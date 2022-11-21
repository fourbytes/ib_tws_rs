use std::io;

use futures::{
    channel::mpsc,
    task::{Spawn, SpawnExt},
    Future, Sink, SinkExt, Stream, StreamExt,
};

use crate::message::{request::StartApi, Request, Response};

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {}

pub trait RequestSink = Sink<Request>;
pub trait ResponseStream = Stream<Item = Result<Response, io::Error>>;

#[derive(Debug)]
pub enum ErrorKind {
    MissingFrame,
    InvalidHandshakeAck,
    InvalidStartApiAck,
    TooManyMessages,
    TooManyRedirect,
    InvalidRedirectAddr,
    UnknownMessageType,
}

pub struct AsyncClient {
    request_tx: mpsc::UnboundedSender<Request>,
    response_rx: mpsc::UnboundedReceiver<Response>,
}

async fn request_forwarder<S: Sink<Request>>(
    mut request_rx: mpsc::UnboundedReceiver<Request>,
    transport_rx: S,
) -> Result<(), S::Error>
where
    S::Error: Send,
{
    let mut transport_rx = Box::pin(transport_rx);
    while let Some(request) = request_rx.next().await {
        transport_rx.send(request).await?;
    }
    Ok(())
}

impl AsyncClient {
    /// Setup a new client with a specified transport.
    pub async fn setup<T>(transport: T, client_id: i32) -> Result<Self, Error>
    where
        T: RequestSink + ResponseStream + SpawnTask + Send + 'static, <T as Sink<Request>>::Error: std::marker::Send
    {
        info!("setting up client");

        let (transport_tx, transport_rx) = transport.split();
        let (request_tx, request_rx) = mpsc::unbounded();
        let (response_tx, response_rx) = mpsc::unbounded();

        let _request_forwarder = T::spawn_task("request_forwarder", async move {
            request_forwarder(request_rx, transport_tx).await
        });

        let mut client = Self {
            request_tx,
            response_rx,
        };
        // client.handshake().await?;
        client
            .send(Request::StartApi(StartApi {
                client_id,
                optional_capabilities: "".to_string(),
            }))
            .await;

        Ok(client)
    }

    pub async fn send(&mut self, request: Request) {
        info!(?request, "sending message");
        self.request_tx.send(request).await;
    }
}

pub trait SpawnTask {
    type JoinHandle<T>;

    fn spawn_task<F, T>(name: &str, future: F) -> Self::JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
}
