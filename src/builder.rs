use bytes::{BufMut, BytesMut};
use failure;
use futures::{Async, Poll, Sink, StartSend, Stream, sync::mpsc};
use futures::future::{Future, FutureResult, lazy, Loop, loop_fn};
use message::message_codec::MessageCodec;
use message::request::*;
use message::response::*;
use std::{error::Error, fmt, io, net::SocketAddr, time::Duration};
use super::channel::{channel4, CommandChannel, TransportChannel};
use super::client::TwsClient;
use super::framed::Framed;
use super::task::TwsTask;
use tokio;
use tokio::net::TcpStream;
use tokio::util::FutureExt;
use tokio_io;
use message::constants::{MIN_VERSION, MAX_VERSION};

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
    fn do_handshake(
        stream: TcpStream,
        timeout: Duration,
        retry_count: i32,
    ) -> impl Future<Item=HandshakeState, Error=io::Error> {
        let stream = Framed::new(stream, MessageCodec::new());
        let request = Request::Handshake(Handshake {
            min_version: MIN_VERSION,
            max_version: MAX_VERSION,
            option: None,
        });
        stream.send(request)
            .timeout(timeout)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "send handshake request timeout"))//failure::Error::from)
            .and_then(move |stream| {
                Self::do_handshake_ack(stream, retry_count)
            })
    }

    fn do_handshake_ack(
        stream: FramedStream,
        retry_count: i32,
    ) -> impl Future<Item=HandshakeState, Error=io::Error> {
        stream
            .into_future()
            .map(move |(frame, stream)| {
                if retry_count > REDIRECT_COUNT_MAX {
                    return HandshakeState::Error(ErrorKind::TooManyRedirect);
                }
                let response = match frame {
                    Some(frame) => frame,
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
                    let _ = stream.into_inner().shutdown(::std::net::Shutdown::Both);
                    HandshakeState::Redirect(re_addr)
                }
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.0)) //::std::convert::From::from(e.0))
    }

    fn do_connect(
        addr: SocketAddr,
        timeout: Duration,
        retry_count: i32,
    ) -> impl Future<Item=HandshakeState, Error=io::Error> {
        TcpStream::connect(&addr)
            .timeout(timeout)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "connect request timeout"))//failure::Error::from)
            .and_then(move |stream| {
                tokio::io::write_all(stream, b"API\0")
                    .timeout(timeout)
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "write API head error")) //failure::Error::from)
            })
            .and_then(move |(stream, _remind_buf)| {
                Self::do_handshake(stream, timeout, retry_count)
            })
            .map(move |state| state)
    }

    pub fn handshake(
        &self,
        addr: SocketAddr,
    ) -> impl Future<Item=TwsClient, Error=io::Error> {
        let retry = 0;
        loop_fn((addr, self.timeout, retry), move |info| {
            Self::do_connect(info.0, info.1, info.2).and_then(move |state| match state {
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

                    Ok(Loop::Break(client))
                }
                HandshakeState::Redirect(re_addr) => {
                    Ok(Loop::Continue((re_addr, info.1, info.2 + 1)))
                }
                HandshakeState::Error(_) => {
                    Err(io::Error::new(io::ErrorKind::Other, "handshake error"))
                }
            })
        })
    }

    pub fn connect(&self, addr: SocketAddr, client_id: i32) -> impl Future<Item=TwsClient, Error=io::Error> {
        self.handshake(addr).map(move |client| {
            client.send_request(Request::StartApi(StartApi {
                client_id,
                optional_capabilities: "".to_string(),
            }));
            client
        })
    }
}
