use std::{
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures::{Future, Sink, Stream, StreamExt, SinkExt};
use ib_tws_core::{
    SpawnTask,
    message::{Request, Response},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio_util::codec::Framed;

use crate::Codec;

pub type FramedStream = Framed<TcpStream, Codec>;

pub struct Transport {
    framed_stream: FramedStream,
}

impl Transport {
    /// # Errors
    /// Returns a `std::io::Error` upon timeout or TCP connection failure.
    pub async fn connect(
        addr: SocketAddr,
        timeout_duration: Duration,
    ) -> Result<Transport, io::Error> {
        let mut stream = tokio::time::timeout(timeout_duration, TcpStream::connect(&addr))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "connect request timeout"))??;

        tokio::time::timeout(timeout_duration, stream.write_all(b"API\0"))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "write API head error"))??;

        let framed_stream = Framed::new(stream, Codec::default());

        Ok(Transport {
            framed_stream,
        })
    }
}

impl Stream for Transport {
    type Item = Result<Response, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.framed_stream.poll_next_unpin(cx)
    }
}

impl Sink<Request> for Transport {
    type Error = io::Error;

    fn start_send(mut self: Pin<&mut Self>, item: Request) -> Result<(), Self::Error> {
        self.framed_stream.start_send_unpin(item)
    }

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.framed_stream.poll_ready_unpin(cx)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.framed_stream.poll_flush_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.framed_stream.poll_close_unpin(cx)
    }
}

impl SpawnTask for Transport {
    type JoinHandle<T> = tokio::task::JoinHandle<T>;

    fn spawn_task<F, T>(name: &str, future: F) -> Self::JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        info!(%name, "spawning task");
        tokio::task::spawn(future)
    }
}
