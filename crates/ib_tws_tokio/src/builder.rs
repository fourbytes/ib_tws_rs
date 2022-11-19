use std::{error::Error, fmt, io, net::SocketAddr, time::Duration};

use bytes::{BufMut, BytesMut};
use futures::{StreamExt, FutureExt, TryFutureExt, SinkExt};
use futures::{Sink, Stream, channel::mpsc};
use futures::future::{Future, lazy};
use tokio::io::AsyncWriteExt;

use ib_tws_core::message::Request;
use ib_tws_core::message::message_codec::MessageCodec;
use ib_tws_core::message::request::*;
use ib_tws_core::message::response::*;
use ib_tws_core::message::constants::{MIN_VERSION, MAX_VERSION};
use ib_tws_core::channel::{channel4, CommandChannel, TransportChannel};

use super::client::TwsClient;
use super::framed::Framed;
use super::task::TwsTask;
use tokio;
use tokio::net::TcpStream;

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
    Verified((TwsClient, TwsTask)),
    Continue(FramedStream, i32, Option<String>, Option<i32>),
    Error(ErrorKind),
}

#[derive(Debug)]
pub struct TwsClientBuilder {
    client_id: i32,
    timeout: Duration,
}

const REDIRECT_COUNT_MAX: i32 = 2;
const DEFAULT_TIMEOUT_SECS: u64 = 2;

impl TwsClientBuilder {
    pub fn new(client_id: i32) -> Self {
        TwsClientBuilder {
            client_id,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    pub fn new_with_timeout(client_id: i32, timeout: Duration) -> Self {
        TwsClientBuilder {
            client_id,
            timeout,
        }
    }

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
        let handshake_state = tokio::time::timeout(timeout, stream.send(request))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "send handshake request timeout"))?;//failure::Error::from)?
        Self::do_handshake_ack(stream, retry_count).await
    }

    async fn do_handshake_ack(
        stream: FramedStream,
        retry_count: i32,
    ) -> Result<HandshakeState, io::Error> {
        stream
            .into_future()
            .then(move |(frame, stream)| async {
                if retry_count > REDIRECT_COUNT_MAX {
                    return HandshakeState::Error(ErrorKind::TooManyRedirect);
                }
                let response = match frame {
                    Some(frame) => frame?,
                    None => return HandshakeState::Error(ErrorKind::MissingFrame),
                };

                let (version, addr_or_time) = match response {
                    Response::HandshakeAck(ack) => (ack.server_version, ack.addr_or_time),
                    _ => return HandshakeState::Error(ErrorKind::InvalidHandshakeAck),
                };

                if version > 0 {
                    HandshakeState::Connected(stream, version)
                } else {
                    let re_addr = match addr_or_time.parse::<SocketAddr>() {
                        Ok(addr) => addr,
                        _ => return HandshakeState::Error(ErrorKind::InvalidRedirectAddr),
                    };
                    let _ = stream.into_inner().shutdown();
                    HandshakeState::Redirect(re_addr)
                }
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.0)) //::std::convert::From::from(e.0))
    }

    async fn do_connect(
        addr: SocketAddr,
        timeout: Duration,
        retry_count: i32,
    ) -> Result<HandshakeState, io::Error> {
        tokio::time::timeout(
            timeout, 
            TcpStream::connect(&addr)
        ).await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "connect request timeout"))?
            .and_then(move |stream| {
                tokio::time::timeout(timeout, stream.write_all(b"API\0"))
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "write API head error")) //failure::Error::from)
            })?
            .and_then(move |(stream, _remind_buf)| {
                Self::do_handshake(stream, timeout, retry_count)
            })
            .map(move |state| state)
    }

    pub async fn handshake(
        &self,
        addr: SocketAddr,
    ) -> Result<TwsClient, io::Error> {
        let mut retry = 0;

        loop {
            let state = Self::do_connect(addr, self.timeout, retry)
                .await?;
            match state {
                //HandshakeState::Connected(stream, version) => Ok(Loop::Break((stream, version))),
                HandshakeState::Connected(stream, version) => {
                    let (command_channel, transport_channel) = channel4();

                    let client = TwsClient {
                        channel: command_channel,
                        server_version: version,
                    };

                    let task = TwsTask {
                        stream,
                        channel: transport_channel,
                        exiting: false,
                    };

                    tokio::spawn(task);

                    return Ok(client)
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

    pub async fn connect(&self, addr: SocketAddr, client_id: i32) -> Result<TwsClient, io::Error> {
        let client = self.handshake(addr).await?;
        client.send_request(Request::StartApi(StartApi {
            client_id,
            optional_capabilities: "".to_string(),
        }));
        Ok(client)
    }
}
