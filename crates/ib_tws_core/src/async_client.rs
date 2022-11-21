use std::{io, sync::{atomic::{AtomicI32, Ordering}, Arc}};

use flume::SendError;
use futures::{
    channel::mpsc,
    Future, Sink, SinkExt, Stream, StreamExt, TryStreamExt, lock::Mutex,
};

use crate::message::{request::{StartApi, Handshake}, Request, Response, constants::{MIN_VERSION, MAX_VERSION}};

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error("request channel closed")]
    RequestChannelClosed,
    #[error("response channel closed")]
    ResponseChannelClosed,
    #[error("transport io error: {0}")]
    TransportIo(#[from] io::Error)
}

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
    response_rx: flume::Receiver<Response>,

    request_id: AtomicI32,
    managed_accounts: Arc<Mutex<Vec<String>>>,
    next_valid_order_id: AtomicI32,
}

async fn request_forwarder<S: Sink<Request>>(
    mut request_rx: mpsc::UnboundedReceiver<Request>,
    transport_tx: S,
) -> Result<(), S::Error>
where
    S::Error: Send,
{
    let mut transport_tx = Box::pin(transport_tx);
    while let Some(message) = request_rx.next().await {
        transport_tx.send(message).await?;
    }
    Ok(())
}

async fn response_forwarder<S: Stream<Item = Result<Response, io::Error>>, T>(
    response_tx: flume::Sender<Response>,
    transport_rx: S,
) -> Result<(), Error>
where
    T: Send, SendError<T>: From<SendError<Request>>
{
    let mut transport_rx = Box::pin(transport_rx);
    while let Some(message) = transport_rx.try_next().await.map_err(Error::TransportIo)? {
        response_tx.send_async(message).await.map_err(|_| Error::ResponseChannelClosed)?;
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
        let (response_tx, response_rx) = flume::unbounded();

        let _request_forwarder = T::spawn_task("request_forwarder", async move {
            request_forwarder(request_rx, transport_tx).await
        });
        let _response_forwarder = T::spawn_task("response_forwarder", async move {
            response_forwarder(response_tx, transport_rx).await
        });

        let client = Self {
            request_tx,
            response_rx,
            request_id: AtomicI32::new(0),
            managed_accounts: Arc::default(),
            next_valid_order_id: AtomicI32::new(0),
        };
        let handshake_ack = client.handshake().await?;
        info!(?handshake_ack);
        client.start_api(client_id).await?;
        
        Ok(client)
    }

    pub async fn send(&self, mut request: Request) -> Result<i32, Error> {
        let request_id = self.request_id.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |request_id| {
            Some(request_id + 1)
        }).unwrap();
        request.set_request_id(request_id);
        info!(?request, "sending message");

        self.request_tx.clone().send(request).await
            .map_err(|_error| Error::RequestChannelClosed)?;

        Ok(request_id)
    }

    /// Get a cloned receiver for the response channel.
    pub fn response_rx(&self) -> flume::Receiver<Response> {
        self.response_rx.clone()
    }

    fn stream_by_request_id(&self, request_id: Option<i32>) -> impl Stream<Item = Response> + '_ {
        self.response_rx.stream()
            .filter(move |response| {
                let response_request_id = response.request_id();
                async move {
                    response_request_id == request_id
                }
            })
    }

    #[instrument(skip(self))]
    async fn start_api(&self, client_id: i32) -> Result<(), Error> {
        debug!("requesting start api");
        self
            .send(Request::StartApi(StartApi {
                client_id,
                optional_capabilities: "".to_string(),
            }))
            .await?;

        let (managed_accts_msg, next_valid_id_msg) = {
            let mut managed_accts_stream = Box::pin(self.response_rx.stream().filter_map(|response| {
                async move {
                    match response {
                        Response::ManagedAcctsMsg(msg) => Some(msg),
                        _ => None,
                    }
                }
            }));
            let mut next_valid_id_stream = Box::pin(self.response_rx.stream().filter_map(|response| {
                async move {
                    match response {
                        Response::NextValidIdMsg(msg) => Some(msg),
                        _ => None,
                    }
                }
            }));
            futures::join!(managed_accts_stream.next(), next_valid_id_stream.next())
        };

        let (managed_accts_msg, next_valid_id_msg) = (managed_accts_msg.ok_or(Error::ResponseChannelClosed)?, next_valid_id_msg.ok_or(Error::ResponseChannelClosed)?);
        {
            let accounts = managed_accts_msg.accounts.split(',').map(String::from).collect();
            info!(?accounts, "updating managed accounts");
            *self.managed_accounts.lock().await = accounts;
        }

        {
            let order_id = next_valid_id_msg.order_id;
            info!(?order_id, "updating next valid id");
            self.next_valid_order_id.swap(order_id, Ordering::Relaxed);
        }

        Ok(())
    }

    async fn handshake(&self) -> Result<Response, Error> {
        debug!("performing handshake");
        self
            .send(Request::Handshake(Handshake {
                min_version: MIN_VERSION,
                max_version: MAX_VERSION,
                option: None
            }))
            .await?;

        let mut stream = Box::pin(self.response_rx.stream().filter(|response| {
            // FIXME: Would rather avoid this clone.
            let response = response.clone();
            async move {
                matches!(response, Response::HandshakeAck(_))
            }
        }));
        stream.next().await.ok_or(Error::ResponseChannelClosed)
    }
}

pub trait SpawnTask {
    type JoinHandle<T>;

    fn spawn_task<F, T>(name: &str, future: F) -> Self::JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
}
