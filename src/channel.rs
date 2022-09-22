use futures::sync::{mpsc, oneshot, mpsc::SendError};
use futures::{future::Future, Async, Poll, Sink, StartSend, Stream};

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
    pub fn unbounded_send(&self, msg: R) -> Result<(), SendError<R>> {
        self.tx.unbounded_send(msg)
    }
}

impl<C,R> CommandChannel<C, R> {
    pub fn unbounded_send(&self, msg: C) -> Result<(), SendError<C>> {
        self.tx.unbounded_send(msg)
    }
}

impl<C, R> Stream for TransportChannel<C, R> {
    type Item = C;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.rx.poll()
    }
}

impl<C, R> Sink for TransportChannel<C, R> {
    type SinkItem = R;
    type SinkError = mpsc::SendError<R>;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.tx.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.tx.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.tx.close()
    }
}

impl<C, R> Stream for CommandChannel<C, R> {
    type Item = R;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.rx.poll()
    }
}

impl<C, R> Sink for CommandChannel<C, R> {
    type SinkItem = C;
    type SinkError = mpsc::SendError<C>;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.tx.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.tx.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.tx.close()
    }
}
