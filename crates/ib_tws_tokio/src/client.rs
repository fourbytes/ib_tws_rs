use std::pin::Pin;
use std::task::Poll;
use std::ops::Drop;

use futures::{Sink, SinkExt};
use futures::channel::mpsc;
use futures::{Stream, StreamExt};
use ib_tws_core::CommandChannel;
use ib_tws_core::message::response::*;
use ib_tws_core::message::request::*;

#[derive(Debug)]
pub struct Client {
    pub channel: CommandChannel<Request, Response>,
    pub server_version: i32,
    //pub account: String,
    //pub next_valid_id: i32,
}

impl Client {
    pub fn send_request(&self, req: Request) {
        let _ = self.channel.tx.unbounded_send(req);
    }
}

impl Stream for Client {
    type Item = Response;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        self.channel.rx.poll_next_unpin(cx)
    }
}

impl Sink<Request> for Client {
    type Error = mpsc::SendError;

    fn start_send(mut self: Pin<&mut Self>, item: Request) -> Result<(), Self::Error> {
        self.channel.tx.start_send(item)
    }

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.channel.tx.poll_ready_unpin(cx)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.channel.tx.poll_flush_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.channel.tx.poll_close_unpin(cx)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        trace!("drop client");
    }
}

impl Client {
    pub async fn req_account_summary(&self, msg: ReqMktData) -> Vec<AccountSummaryMsg> {
        vec![]
    }
}
