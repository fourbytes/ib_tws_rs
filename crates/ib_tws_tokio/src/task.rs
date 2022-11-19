use std::io;
use std::ops::Drop;
use std::pin::Pin;
use std::task::{Poll, Context};

use futures::{Future, Sink, SinkExt, StreamExt, TryStreamExt};

use crate::FramedStream;
use ib_tws_core::TransportChannel;
use ib_tws_core::message::request::*;
use ib_tws_core::message::response::*;

#[derive(Debug)]
pub struct TwsTask {
    pub stream: FramedStream,
    pub channel: TransportChannel<Request, Response>,
    pub exiting: bool,
    //pub server_version: i32,
    //pub account: String,
    //pub next_valid_id: i32,
}

impl TwsTask {
    fn poll_request(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), ()>> {
        loop {
            let response = match self.channel.poll_next_unpin(cx) {
                Poll::Ready(Some(r)) => r,
                Poll::Ready(None) => {
                    return Poll::Ready(Err(()));
                },
                Poll::Pending => return Poll::Pending
            };

            Pin::new(&mut self.stream).start_send(response).map_err(|_| ())?;
        }
    }

    fn poll_read(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), ()>> {
        // exiting = true in some place???
        loop {
            let item = match self.stream.try_poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(item))) => item,
                Poll::Ready(Some(Err(error))) => return Poll::Ready(Err(())),
                Poll::Ready(None) => return Poll::Ready(Err(())),
                Poll::Pending => return Poll::Pending,
            };

            let result = self.channel.unbounded_send(item).map_err(|_| ())?;

            if self.exiting {
                return Poll::Ready(Ok(()))
            }
        }
    }

    fn poll_write(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), ()>> {
        self.stream.poll_ready_unpin(cx).map_err(|_| ())
    }
}

impl Future for TwsTask {
    type Output = Result<(), ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.exiting {
            match self.poll_request(cx) {
                Poll::Ready(Ok(_)) => {}
                Poll::Ready(Err(())) => {
                    // no more requests will be enqueued
                    self.exiting = true;
                },
                Poll::Pending => {}
            }
        }

        let r = self.poll_read(cx);
        let w = self.poll_write(cx);

        match (r, w) {
            (Poll::Ready(Ok(())), Poll::Ready(Ok(()))) if self.exiting => {
                trace!("task done");
                Poll::Ready(Ok(()))
            }
            (Poll::Ready(Ok(())), Poll::Ready(Ok(()))) => {
                Poll::Pending
            }
            (Poll::Ready(Ok(())), _) => panic!("outstanding requests, but response channel closed"),
            (_, Poll::Ready(Ok(()))) if self.exiting => {
                Poll::Pending
            }
            _ => {
                Poll::Pending
            }
        }
    }
}

impl Sink<Request> for TwsTask {
    type Error = io::Error;

    fn start_send(mut self: Pin<&mut Self>, item: Request) -> Result<(), Self::Error> {
        Pin::new(&mut self.stream).start_send(item)
    }

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.stream.poll_ready_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.stream.poll_close_unpin(cx)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.stream.poll_flush_unpin(cx)
    }
}

impl Drop for TwsTask {
    fn drop(&mut self) {
        trace!("\n\ndrop task\n\n");
    }
}
