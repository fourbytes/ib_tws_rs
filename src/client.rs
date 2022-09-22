use bytes::BytesMut;
use futures::sync::mpsc;
use futures::{Poll, Sink, StartSend, Stream};
use tokio::spawn;
use std::default::Default;
use {CommandChannel};
use message::response::*;
use message::request::*;
use std::ops::Drop;

#[derive(Debug)]
pub struct TwsClient {
    pub channel: CommandChannel<Request, Response>,
    pub server_version: i32,
    //pub account: String,
    //pub next_valid_id: i32,
}

impl TwsClient {
    pub fn send_request(&self, req: Request) {
        let _ =self.channel.tx.unbounded_send(req);
    }
}

impl Stream for TwsClient {
    type Item = Response;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.channel.rx.poll()
    }
}

/*impl Sink for TwsClient {
    type SinkItem = Request;
    type SinkError = mpsc::SendError<Self::SinkItem>;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.channel.tx.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.channel.tx.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.channel.tx.close()
    }
}*/

impl Drop for TwsClient {
    fn drop(&mut self) {
        println!("drop client");
    }
}