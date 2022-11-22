use std::{
    io,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
};

use async_broadcast::SendError;
use futures::{channel::mpsc, lock::Mutex, Future, Sink, SinkExt, Stream, StreamExt, TryStreamExt};

use crate::{
    domain::{ContractDetails, market_data::MarketDataType, misc::ServerLogLevel},
    message::{
        constants::{MAX_VERSION, MIN_VERSION},
        request::{
            Handshake, ReqAccountSummary, ReqContractDetails, ReqMktData, ReqMktDepthExchanges,
            StartApi, ReqMktDepth, ReqMarketDataType, SetServerLogLevel,
        },
        response::{AccountSummaryMsg, HandshakeAck, MktDepthExchangesMsg},
        Request, Response,
    }, Error,
};

#[derive(Debug)]
pub struct AsyncClient {
    request_tx: mpsc::UnboundedSender<Request>,
    response_rx: async_broadcast::Receiver<Response>,

    request_id: AtomicI32,
    managed_accounts: Arc<Mutex<Vec<String>>>,
    next_valid_order_id: AtomicI32,
    server_version: AtomicI32,
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
    response_tx: async_broadcast::Sender<Response>,
    transport_rx: S,
) -> Result<(), Error>
where
    T: Send,
    SendError<T>: From<SendError<Request>>,
{
    let mut transport_rx = Box::pin(transport_rx);
    while let Some(message) = transport_rx.try_next().await.map_err(Error::TransportIo)? {
        response_tx
            .broadcast(message)
            .await
            .map_err(|_| Error::ResponseChannelClosed)?;
    }
    Ok(())
}

impl AsyncClient {
    /// Setup a new client with a specified transport.
    /// # Errors
    /// Returns an error if either the handshake or initial start API request fails.
    pub async fn setup<T>(transport: T, client_id: i32) -> Result<Self, Error>
    where
        T: Sink<Request> + Stream<Item = Result<Response, io::Error>> + SpawnTask + Send + 'static,
        <T as Sink<Request>>::Error: std::marker::Send,
    {
        info!("setting up client");

        let (transport_tx, transport_rx) = transport.split();
        let (request_tx, request_rx) = mpsc::unbounded();
        let (response_tx, response_rx) = async_broadcast::broadcast(1000);

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
            server_version: AtomicI32::new(0),
        };
        let _handshake_ack = client.handshake().await?;

        client.start_api(client_id).await?;

        Ok(client)
    }

    /// # Errors
    /// Returns an error if the request channel is closed.
    #[allow(clippy::missing_panics_doc)]
    pub async fn send(&self, mut request: Request) -> Result<i32, Error> {
        let request_id = self
            .request_id
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |request_id| {
                Some(request_id + 1)
            })
            .unwrap();
        request.set_request_id(request_id);
        info!(?request, "sending message");

        self.request_tx
            .clone()
            .send(request)
            .await
            .map_err(|_error| Error::RequestChannelClosed)?;

        Ok(request_id)
    }

    /// Get a cloned receiver for the response channel.
    pub fn response_stream(&self) -> async_broadcast::Receiver<Response> {
        self.response_rx.clone()
    }

    fn response_stream_by_id(&self, id: Option<i32>) -> impl Stream<Item = Response> + '_ {
        self.response_stream().filter(move |response| {
            let response_request_id = response.request_id();
            async move { response_request_id == id }
        })
    }

    #[instrument(skip(self))]
    async fn start_api(&self, client_id: i32) -> Result<(), Error> {
        debug!("requesting start api");
        self.send(Request::StartApi(StartApi {
            client_id,
            optional_capabilities: String::new(),
        }))
        .await?;

        let (managed_accts_msg, next_valid_id_msg) = {
            let mut managed_accts_stream =
                Box::pin(self.response_stream().filter_map(|response| async move {
                    match response {
                        Response::ManagedAcctsMsg(msg) => Some(msg),
                        _ => None,
                    }
                }));
            let mut next_valid_id_stream =
                Box::pin(self.response_stream().filter_map(|response| async move {
                    match response {
                        Response::NextValidIdMsg(msg) => Some(msg),
                        _ => None,
                    }
                }));
            futures::join!(managed_accts_stream.next(), next_valid_id_stream.next())
        };

        let (managed_accts_msg, next_valid_id_msg) = (
            managed_accts_msg.ok_or(Error::ResponseChannelClosed)?,
            next_valid_id_msg.ok_or(Error::ResponseChannelClosed)?,
        );
        {
            let accounts = managed_accts_msg
                .accounts
                .split(',')
                .map(String::from)
                .collect();
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

    #[instrument(skip(self))]
    async fn handshake(&self) -> Result<HandshakeAck, Error> {
        debug!("performing handshake");
        self.send(Request::Handshake(Handshake {
            min_version: MIN_VERSION,
            max_version: MAX_VERSION,
            option: None,
        }))
        .await?;

        let mut stream = Box::pin(self.response_rx.clone().filter_map(|response| async move {
            match response {
                Response::HandshakeAck(ack) => Some(ack),
                _ => None,
            }
        }));
        let handshake_ack = stream.next().await.ok_or(Error::ResponseChannelClosed)?;
        debug!(?handshake_ack, "received handshake ack");
        self.server_version
            .store(handshake_ack.server_version, Ordering::Relaxed);
        Ok(handshake_ack)
    }

    pub async fn managed_accounts(&self) -> Vec<String> {
        self.managed_accounts.lock().await.clone()
    }

    pub fn next_valid_order_id(&self) -> i32 {
        self.next_valid_order_id.load(Ordering::Relaxed)
    }

    pub fn server_version(&self) -> i32 {
        self.server_version.load(Ordering::Relaxed)
    }

    pub async fn request_contract_details(
        &self,
        message: ReqContractDetails,
    ) -> Result<ContractDetails, Error> {
        let request_id = self.send(Request::ReqContractDetails(message)).await?;

        Box::pin(
            self.response_stream_by_id(Some(request_id))
                .take_while(|response| {
                    let is_end = matches!(response, Response::ContractDataEndMsg(_));
                    async move { !is_end }
                })
                .filter_map(|response| async move {
                    match response {
                        Response::ErrMsgMsg(err) => Some(Err(Error::ApiError(err))),
                        Response::ContractDataMsg(msg) => Some(Ok(msg.contract_details)),
                        _ => {
                            warn!(?response, "unexpected response for request id");
                            None
                        }
                    }
                }),
        )
        .next()
        .await
        .unwrap()
    }

    #[instrument(skip(self))]
    pub async fn request_account_summary(
        &self,
        message: ReqAccountSummary,
    ) -> Result<impl Stream<Item = Result<AccountSummaryMsg, Error>> + '_, Error> {
        let request_id = self.send(Request::ReqAccountSummary(message)).await?;

        Ok(self
            .response_stream_by_id(Some(request_id))
            .take_while(|response| {
                let is_end = matches!(response, Response::AccountSummaryEndMsg(_));
                async move { !is_end }
            })
            .filter_map(|response| async move {
                match response {
                    Response::ErrMsgMsg(err) => Some(Err(Error::ApiError(err))),
                    Response::AccountSummaryMsg(msg) => Some(Ok(msg)),
                    _ => None,
                }
            }))
    }

    #[instrument(skip(self))]
    pub async fn request_market_depth_exchanges(&self) -> Result<MktDepthExchangesMsg, Error> {
        self.send(Request::ReqMktDepthExchanges(ReqMktDepthExchanges {}))
            .await?;

        Box::pin(self.response_stream().filter_map(|response| async move {
            match response {
                Response::MktDepthExchangesMsg(msg) => Some(Ok(msg)),
                _ => None,
            }
        }))
        .next()
        .await
        .ok_or(Error::ResponseChannelClosed)?
    }

    #[instrument(skip(self))]
    pub async fn request_market_data(
        &self,
        message: ReqMktData,
    ) -> Result<impl Stream<Item = Result<Response, Error>> + '_, Error> {
        let request_id = self.send(Request::ReqMktData(message)).await?;

        Ok(self
            .response_stream_by_id(Some(request_id))
            .take_while(|response| {
                let is_end = matches!(response, Response::TickSnapshotEndMsg(_));
                async move { !is_end }
            })
            .filter_map(|response| async move {
                match response {
                    Response::ErrMsgMsg(err) => Some(Err(Error::ApiError(err))),
                    response @ (Response::TickSizeMsg(_)
                    | Response::MarketDataTypeMsg(_)
                    | Response::TickPriceMsg(_)
                    | Response::TickStringMsg(_)
                    | Response::TickEFPMsg(_)
                    | Response::TickGenericMsg(_)
                    | Response::TickOptionComputationMsg(_)) => Some(Ok(response)),
                    _ => None,
                }
            }))
    }

    #[instrument(skip(self))]
    pub async fn request_market_depth(
        &self,
        message: ReqMktDepth,
    ) -> Result<impl Stream<Item = Result<Response, Error>> + '_, Error> {
        let request_id = self.send(Request::ReqMktDepth(message)).await?;

        Ok(self
            .response_stream_by_id(Some(request_id))
            .filter_map(|response| async move {
                match response {
                    Response::ErrMsgMsg(err) => Some(Err(Error::ApiError(err))),
                    response @ (Response::MarketDepthL2Msg(_)
                    | Response::MarketDepthMsg(_)) => Some(Ok(response)),
                    _ => None,
                }
            }))
    }

    #[instrument(skip(self))]
    pub async fn request_market_data_type(
        &self,
        market_data_type: MarketDataType,
    ) -> Result<(), Error> {
        self.send(Request::ReqMarketDataType(ReqMarketDataType { market_data_type })).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn set_server_log_level(
        &self,
        log_level: ServerLogLevel,
    ) -> Result<(), Error> {
        self.send(Request::SetServerLogLevel(SetServerLogLevel { log_level })).await?;
        Ok(())
    }
}

pub trait SpawnTask {
    type JoinHandle<T>;

    fn spawn_task<F, T>(name: &str, future: F) -> Self::JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
}
