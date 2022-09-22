use failure;
use futures::{
    stream,
    sync::{mpsc, oneshot},
    Future, IntoFuture, Poll, Stream,
};

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
    pub fn send(&self, req: T) -> impl Future<Item = Result<R, E>, Error = failure::Error> {
        let (tx, rx) = oneshot::channel();
        match self.0.unbounded_send((req, ReplySender::Oneshot(tx))) {
            Ok(()) => Either::A(
                rx.map_err(|e| format_err!("failed to receive oneshot response: {:?}", e)),
            ),
            Err(e) => {
                Either::B(Err(format_err!("failed to send new request: {:?}", e)).into_future())
            }
        }
    }

    pub fn subscribe(&self, req: T) -> impl Stream<Item = Result<R, E>, Error = failure::Error> {
        let (tx, rx) = mpsc::unbounded();

        match self.0.unbounded_send((req, ReplySender::Stream(tx))) {
            Ok(()) => Either::A(
                rx.map_err(|e| format_err!("failed to receive oneshot response: {:?}", e)),
            ),
            Err(e) => Either::B(stream::once(Err(format_err!(
                "failed to subscribe new request: {:?}",
                e
            )))),
        }
    }
}

impl<T, R, E> Clone for Commander<T, R, E> {
    fn clone(&self) -> Self {
        Commander(self.0.clone())
    }
}

// Copy from futures-0.2
/// Combines two different futures yielding the same item and error
/// types into a single type.
#[derive(Debug)]
pub enum Either<A, B> {
    /// First branch of the type
    A(A),
    /// Second branch of the type
    B(B),
}

impl<T, A, B> Either<(T, A), (T, B)> {
    /// Splits out the homogeneous type from an either of tuples.
    ///
    /// This method is typically useful when combined with the `Future::select2`
    /// combinator.
    pub fn split(self) -> (T, Either<A, B>) {
        match self {
            Either::A((a, b)) => (a, Either::A(b)),
            Either::B((a, b)) => (a, Either::B(b)),
        }
    }
}

impl<A, B> Future for Either<A, B>
where
    A: Future,
    B: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<A::Item, A::Error> {
        match *self {
            Either::A(ref mut a) => a.poll(),
            Either::B(ref mut b) => b.poll(),
        }
    }
}

impl<A, B> Stream for Either<A, B>
where
    A: Stream,
    B: Stream<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<A::Item>, A::Error> {
        match *self {
            Either::A(ref mut a) => a.poll(),
            Either::B(ref mut b) => b.poll(),
        }
    }
}
