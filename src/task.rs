use {FramedStream, TransportChannel};
use {Decoder, Encoder, Framed};
use futures::{Async, Future, Poll, Sink, StartSend, Stream};
use futures::sync::mpsc;
use message::request::*;
use message::response::*;
use std::io;
use std::ops::Drop;

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
    fn poll_request(&mut self) -> Poll<(), ()> {
        loop {
            let response = match try_ready!(self.channel.poll()) {
                Some(r) => r,
                None => {
                    return Err(());
                }
            };

            self.stream.start_send(response).map_err(|_| ())?;
        }
    }

    fn poll_read(&mut self) -> Poll<(), ()> {
        // exiting = true in some place???
        loop {
            let item = match try_ready!(self.stream.poll().map_err(|_|())) {
                Some(item) => item,
                None => {
                    return Err(());
                }
            };

            let result = self.channel.unbounded_send(item).map_err(|_| ())?;

            if self.exiting {
                return Ok(Async::Ready(()))
            }
        }
    }

    fn poll_write(&mut self) -> Poll<(), ()> {
        self.stream.poll_complete().map_err(|_| ())
    }
}

impl Future for TwsTask {
    type Item = ();
    type Error = ();


    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        if !self.exiting {
            match self.poll_request() {
                Ok(_) => {}
                Err(()) => {
                    // no more requests will be enqueued
                    self.exiting = true;
                }
            }
        }

        let r = self.poll_read()?;
        let w = self.poll_write()?;

        match (r, w) {
            (Async::Ready(()), Async::Ready(())) if self.exiting => {
                println!("task done");
                Ok(Async::Ready(()))
            }
            (Async::Ready(()), Async::Ready(())) => {
                Ok(Async::NotReady)
            }
            (Async::Ready(()), _) => panic!("outstanding requests, but response channel closed"),
            (_, Async::Ready(())) if self.exiting => {
                Ok(Async::NotReady)
            }
            _ => {
                Ok(Async::NotReady)
            }
        }
    }
}

impl Sink for TwsTask {
    type SinkItem = Request;
    type SinkError = io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.stream.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.stream.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.stream.close()
    }
}

impl Drop for TwsTask {
    fn drop(&mut self) {
        println!("\n\ndrop task\n\n");
    }
}