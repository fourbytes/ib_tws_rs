use std::{
    pin::{self, Pin},
    task::{Context, Poll},
};

use futures::{
    channel::{mpsc, oneshot},
    future::Either,
    stream, Future, FutureExt, Stream, StreamExt, TryFutureExt, TryStreamExt,
};

#[derive(Debug, Clone, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error("failed to receive oneshot response")]
    FailedToReceiveOneshotResponse,
    #[error("failed to send new request")]
    FailedToSendNewRequest,
    #[error("failed to subscribe new request")]
    FailedToSubscribeNewRequest,
}
type Result<T, E = Error> = std::result::Result<T, E>;

// T: Request
// R: Response
// E: Error

pub type OneshotReplySender<R, E> = oneshot::Sender<Result<R, E>>;
pub type UnboundedReplySender<R, E> = mpsc::UnboundedSender<Result<R, E>>;
#[derive(Debug)]
pub enum ReplySender<R, E> {
    Oneshot(OneshotReplySender<R, E>),
    Stream(UnboundedReplySender<R, E>),
}

pub type Command<T, R, E> = (T, ReplySender<R, E>);

pub type CommandSender<T, R, E> = mpsc::UnboundedSender<Command<T, R, E>>;
pub type CommandReceiver<T, R, E> = mpsc::UnboundedReceiver<Command<T, R, E>>;

#[derive(Debug)]
pub struct Commander<T, R, E>(pub CommandSender<T, R, E>);

impl<T, R, E> Commander<T, R, E> {
    pub async fn send(&self, req: T) -> Result<Result<R, E>> {
        let (tx, rx) = oneshot::channel();
        match self.0.unbounded_send((req, ReplySender::Oneshot(tx))) {
            Ok(()) => Either::Left(rx.map_err(|e| Error::FailedToReceiveOneshotResponse)),
            Err(e) => Either::Right(async { Err(Error::FailedToSendNewRequest) }),
        }
        .await
    }

    pub fn subscribe(&self, req: T) -> impl Stream<Item = Result<Result<R, E>>> {
        let (tx, rx) = mpsc::unbounded();

        match self.0.unbounded_send((req, ReplySender::Stream(tx))) {
            Ok(()) => Either::Left(rx.map(|o| Ok(o))),
            Err(e) => Either::Right(stream::once(async {
                Err(Error::FailedToSubscribeNewRequest)
            })),
        }
    }
}

impl<T, R, E> Clone for Commander<T, R, E> {
    fn clone(&self) -> Self {
        Commander(self.0.clone())
    }
}
