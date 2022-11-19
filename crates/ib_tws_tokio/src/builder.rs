use std::{io, net::SocketAddr, time::Duration};

use futures::{SinkExt, TryStreamExt};
use ib_tws_core::channel::channel4;
use ib_tws_core::message::constants::{MAX_VERSION, MIN_VERSION};
use ib_tws_core::message::message_codec::MessageCodec;
use ib_tws_core::message::request::*;
use ib_tws_core::message::response::*;
use ib_tws_core::message::Request;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use super::client::Client;
use super::task::TwsTask;

pub type FramedStream = Framed<TcpStream, MessageCodec>;

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

#[derive(Debug)]
pub enum HandshakeState {
    Connected(FramedStream, i32),
    Redirect(SocketAddr),
    Error(ErrorKind),
}

#[derive(Debug)]
pub enum StartApiAckState {
    Verified((Client, TwsTask)),
    Continue(FramedStream, i32, Option<String>, Option<i32>),
    Error(ErrorKind),
}

#[derive(Debug)]
pub struct Builder {
    client_id: i32,
    timeout: Duration,
}

const REDIRECT_COUNT_MAX: i32 = 2;
const DEFAULT_TIMEOUT_SECS: u64 = 2;

impl Builder {
    pub fn new(client_id: i32) -> Self {
        Builder {
            client_id,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    pub fn new_with_timeout(client_id: i32, timeout: Duration) -> Self {
        Builder { client_id, timeout }
    }

    #[instrument(skip(stream))]
    async fn do_handshake(
        stream: TcpStream,
        timeout: Duration,
        retry_count: i32,
    ) -> Result<HandshakeState, io::Error> {
        let mut stream = Framed::new(stream, MessageCodec::new());
        let request = Request::Handshake(Handshake {
            min_version: MIN_VERSION,
            max_version: MAX_VERSION,
            option: None,
        });
        let _handshake_state = tokio::time::timeout(timeout, stream.send(request))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "send handshake request timeout"))?; //failure::Error::from)?
        Self::do_handshake_ack(stream, timeout, retry_count).await
    }

    #[instrument(skip(stream))]
    async fn do_handshake_ack(
        mut stream: FramedStream,
        timeout: Duration,
        retry_count: i32,
    ) -> Result<HandshakeState, io::Error> {
        trace!("wait for handshake ack");
        let ack = tokio::time::timeout(timeout, stream.try_next())
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "recv handshake ack timeout"))?;
        if let Some(response) = ack? {
            info!(?response);
            if retry_count > REDIRECT_COUNT_MAX {
                return Ok(HandshakeState::Error(ErrorKind::TooManyRedirect));
            }

            let (version, addr_or_time) = match response {
                Response::HandshakeAck(ack) => (ack.server_version, ack.addr_or_time),
                _ => return Ok(HandshakeState::Error(ErrorKind::InvalidHandshakeAck)),
            };

            if version > 0 {
                Ok(HandshakeState::Connected(stream, version))
            } else {
                let re_addr = match addr_or_time.parse::<SocketAddr>() {
                    Ok(addr) => addr,
                    _ => return Ok(HandshakeState::Error(ErrorKind::InvalidRedirectAddr)),
                };
                let _ = stream.into_inner().shutdown();
                Ok(HandshakeState::Redirect(re_addr))
            }
        } else {
            Ok(HandshakeState::Error(ErrorKind::MissingFrame))
        }
    }

    #[instrument(err)]
    async fn do_connect(
        addr: SocketAddr,
        timeout: Duration,
        retry_count: i32,
    ) -> Result<HandshakeState, io::Error> {
        let mut stream = tokio::time::timeout(timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "connect request timeout"))??;

        tokio::time::timeout(timeout, stream.write_all(b"API\0"))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "write API head error"))?;

        Self::do_handshake(stream, timeout, retry_count).await
    }

    pub async fn handshake(&self, addr: SocketAddr) -> Result<Client, io::Error> {
        let mut retry = 0;

        loop {
            let state = Self::do_connect(addr, self.timeout, retry).await?;
            match state {
                //HandshakeState::Connected(stream, version) => Ok(Loop::Break((stream, version))),
                HandshakeState::Connected(stream, version) => {
                    let (command_channel, transport_channel) = channel4();

                    let client = Client {
                        channel: command_channel,
                        server_version: version,
                    };

                    let task = TwsTask {
                        stream,
                        channel: transport_channel,
                        exiting: false,
                    };

                    tokio::spawn(task);

                    return Ok(client);
                }
                HandshakeState::Redirect(re_addr) => {
                    retry += 1;
                }
                HandshakeState::Error(_) => {
                    return Err(io::Error::new(io::ErrorKind::Other, "handshake error"))
                }
            }
        }
    }

    pub async fn connect(&self, addr: SocketAddr, client_id: i32) -> Result<Client, io::Error> {
        info!("connecting to client");
        let client = self.handshake(addr).await?;
        client.send_request(Request::StartApi(StartApi {
            client_id,
            optional_capabilities: "".to_string(),
        }));
        Ok(client)
    }
}
