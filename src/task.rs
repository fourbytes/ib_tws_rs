use std::io;
use std::ops::Drop;
use std::pin::Pin;
use std::task::{Poll, Context};

use futures::{Future, Sink, Stream, SinkExt, StreamExt, TryStreamExt};
use futures::channel::mpsc;

use crate::{FramedStream, TransportChannel};
use crate::{Decoder, Encoder, Framed};
use crate::message::request::*;
use crate::message::response::*;

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
    fn poll_request(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        loop {
            let response = match self.channel.poll_next_unpin(cx) {
                Poll::Ready(Some(r)) => r,
                Poll::Ready(None) => {
                    return Poll::Ready(Err(()));
                },
                Poll::Pending => return Poll::Pending
            };

            self.stream.start_send(response).map_err(|_| ())?;
        }
    }

    fn poll_read(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        // exiting = true in some place???
        loop {
            let item = match self.stream.try_poll_next_unpin(cx) {
                Poll::Ready(Some(item)) => item,
                Poll::Ready(None) => return Err(()),
                Poll::Pending => return Poll::Pending,
            };

            let result = self.channel.unbounded_send(item).map_err(|_| ())?;

            if self.exiting {
                return Ok(Poll::Ready(()))
            }
        }
    }

    fn poll_write(&mut self) -> Poll<()> {
        self.stream.poll_complete().map_err(|_| ())
    }
}

impl Future for TwsTask {
    type Output = Result<(), ()>;

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.exiting {
            match self.poll_request(cx) {
                Ok(_) => {}
                Err(()) => {
                    // no more requests will be enqueued
                    self.exiting = true;
                }
            }
        }

        let r = self.poll_read(cx)?;
        let w = self.poll_write()?;

        match (r, w) {
            (Poll::Ready(()), Poll::Ready(())) if self.exiting => {
                println!("task done");
                Ok(Poll::Ready(()))
            }
            (Poll::Ready(()), Poll::Ready(())) => {
                Ok(Poll::Pending)
            }
            (Poll::Ready(()), _) => panic!("outstanding requests, but response channel closed"),
            (_, Poll::Ready(())) if self.exiting => {
                Ok(Poll::Pending)
            }
            _ => {
                Ok(Poll::Pending)
            }
        }
    }
}

impl Sink<Request> for TwsTask {
    type Error = io::Error;

    fn start_send(&mut self, item: Request) -> Result<Request, Self::Error> {
        self.stream.start_send(item)
    }

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.stream.poll_ready_unpin(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.stream.poll_close_unpin(cx)
    }
}

impl Drop for TwsTask {
    fn drop(&mut self) {
        println!("\n\ndrop task\n\n");
    }
}
