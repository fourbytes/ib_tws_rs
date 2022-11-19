use std::pin::Pin;
use std::task::{Poll, Context};

use futures::{StreamExt, SinkExt};
use futures::channel::{mpsc, oneshot, mpsc::SendError};
use futures::{future::Future, Sink, Stream};

#[derive(Debug)]
pub struct CommandChannel<C, R> {
    pub tx: mpsc::UnboundedSender<C>,
    pub rx: mpsc::UnboundedReceiver<R>,
}

#[derive(Debug)]
pub struct TransportChannel<C, R> {
    pub tx: mpsc::UnboundedSender<R>,
    pub rx: mpsc::UnboundedReceiver<C>,
}

pub fn channel4<C, R>() -> (CommandChannel<C, R>, TransportChannel<C, R>) {
    let (req_tx, req_rx) = mpsc::unbounded();
    let (watch_tx, watch_rx) = mpsc::unbounded();
    let cc = CommandChannel {
        tx: req_tx,
        rx: watch_rx,
    };

    let tc = TransportChannel {
        tx: watch_tx,
        rx: req_rx,
    };
    (cc, tc)
}

impl<C,R> TransportChannel<C, R> {
    pub fn unbounded_send(&self, msg: R) -> Result<(), SendError> {
        self.tx.unbounded_send(msg)
    }
}

impl<C,R> CommandChannel<C, R> {
    pub fn unbounded_send(&self, msg: C) -> Result<(), SendError> {
        self.tx.unbounded_send(msg)
    }
}

impl<C, R> Stream for TransportChannel<C, R> {
    type Item = C;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

impl<C, R> Sink<R> for TransportChannel<C, R> {
    type Error = mpsc::SendError;


    fn start_send(&mut self, item: R) -> Result<R, Self::Error> {
        self.tx.start_send(item)
    }

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.tx.poll_ready(cx)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.tx.poll_flush_unpin(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.tx.close()
    }
}

impl<C, R> Stream for CommandChannel<C, R> {
    type Item = R;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

impl<C, R> Sink<C> for CommandChannel<C, R> {
    type Error = mpsc::SendError;

    fn start_send(&mut self, item: C) -> Result<C, Self::Error> {
        self.tx.start_send(item)
    }

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.tx.poll_ready(cx)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.tx.poll_flush_unpin(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.tx.poll_close_unpin(cx)
    }
}
