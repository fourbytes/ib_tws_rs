use std::task::Poll;
use std::ops::Drop;
use std::default::Default;

use bytes::BytesMut;
use futures::channel::mpsc;
use futures::{Sink, Stream, StreamExt};
use tokio::spawn;
use crate::CommandChannel;
use crate::message::response::*;
use crate::message::request::*;

#[derive(Debug)]
pub struct TwsClient {
    pub channel: CommandChannel<Request, Response>,
    pub server_version: i32,
    //pub account: String,
    //pub next_valid_id: i32,
}

impl TwsClient {
    pub fn send_request(&self, req: Request) {
        let _ = self.channel.tx.unbounded_send(req);
    }
}

impl Stream for TwsClient {
    type Item = Response;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        self.channel.rx.poll_next_unpin(cx)
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
