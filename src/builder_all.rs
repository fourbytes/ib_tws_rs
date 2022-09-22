use bytes::{BufMut, BytesMut};
use failure;
use futures::{Async, future::Future, Poll, Sink, StartSend, Stream};
use futures::future::{FutureResult, lazy, Loop, loop_fn};
use futures::sync::mpsc;
use message::message_codec::MessageCodec;
use message::request::*;
use message::response::*;
use message::wire::{TwsWireDecoder, TwsWireEncoder};
use std::error::Error;
use std::fmt;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;
use super::channel::{channel4, CommandChannel, TransportChannel};
use super::client::TwsClient;
use super::task::TwsTask;
use tokio;
use tokio::net::TcpStream;
use tokio::util::FutureExt;
use tokio_io;
use super::framed::Framed;

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
pub struct TwsBuilder {
    timeout: Duration,
    client_id: i32,
}

const REDIRECT_COUNT_MAX: i32 = 2;

impl TwsBuilder {
    pub fn new(timeout: Duration, client_id: i32) -> Self {
        TwsBuilder { timeout, client_id }
    }

    fn do_handshake(
        stream: TcpStream,
        timeout: Duration,
        retry_count: i32,
    ) -> impl Future<Item=HandshakeState, Error=io::Error> {
        let stream = Framed::new(stream, MessageCodec::new());
        let request = Request::Handshake(Handshake{
            min_version: 100,
            max_version: 142,
            option: None
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

    fn do_start_api(
        stream: FramedStream,
        version: i32,
        client_id: i32,
    ) -> impl Future<Item=(TwsClient, TwsTask), Error=io::Error> {
        let request = Request::StartApi(StartApi{
            client_id,
            optional_capabilities: "".to_string(),
        });
        stream.send(request)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "send start_api request timeout"))//failure::Error::from)
            .and_then(move |stream| {
                Self::check_ack(stream, version)
            })
        //.map(move |item| item)
    }

    fn do_start_api_ack(
        stream: FramedStream,
        version: i32,
        mut account: Option<String>,
        mut next_valid_id: Option<i32>,
    ) -> impl Future<Item=StartApiAckState, Error=io::Error> {
        stream
            .into_future()
            .map(move |(frame, stream)| {
                let msg = match frame {
                    Some(msg) => msg,
                    _ => return StartApiAckState::Error(ErrorKind::InvalidStartApiAck),
                };

                match msg {
                    Response::ManagedAcctsMsg(msg) => account = Some(msg.accounts),
                    Response::NextValidIdMsg(msg) => next_valid_id = Some(msg.order_id),
                    _ => return StartApiAckState::Error(ErrorKind::UnknownMessageType),
                }

                if account.is_some() && next_valid_id.is_some() {
                    let (command_channel, transport_channel) = channel4();
                    let account = account.unwrap();
                    let next_valid_id = next_valid_id.unwrap();

                    let client = TwsClient {
                        channel: command_channel,
                        server_version: version,
                        account: account.clone(),
                        next_valid_id,
                    };

                    let task = TwsTask {
                        stream,
                        channel: transport_channel,
                        server_version: version,
                        account: account.clone(),
                        next_valid_id,
                        exiting: false,
                    };

                    StartApiAckState::Verified((client, task))
                } else {
                    StartApiAckState::Continue(stream, version, account, next_valid_id)
                }
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.0)) //::std::convert::From::from(e.0))
    }

    fn check_ack(
        stream: FramedStream,
        version: i32,
    ) -> impl Future<Item=(TwsClient, TwsTask), Error=io::Error> {
        loop_fn((stream, version, None, None), move |state| {
            Self::do_start_api_ack(state.0, state.1, state.2, state.3).and_then(move |ack_state| {
                match ack_state {
                    StartApiAckState::Continue(stream, version, account, next_valid_id) => {
                        Ok(Loop::Continue((stream, version, account, next_valid_id)))
                    }
                    StartApiAckState::Verified(info) => Ok(Loop::Break(info)),
                    StartApiAckState::Error(_) => Err(io::Error::new(io::ErrorKind::Other, "check_ack error")),
                }
            })
        })
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
    ) -> impl Future<Item=(FramedStream, i32), Error=io::Error> {
        let retry = 0;
        loop_fn((addr, self.timeout, retry), move |info| {
            Self::do_connect(info.0, info.1, info.2).and_then(move |state| match state {
                HandshakeState::Connected(stream, version) => Ok(Loop::Break((stream, version))),
                HandshakeState::Redirect(re_addr) => {
                    Ok(Loop::Continue((re_addr, info.1, info.2 + 1)))
                }
                HandshakeState::Error(_) => Err(io::Error::new(io::ErrorKind::Other, "handshake error")),
            })
        })
    }

    pub fn connect(
        &self,
        addr: SocketAddr,
    ) -> impl Future<Item=(TwsClient, TwsTask), Error=io::Error> {
        let client_id = self.client_id;
        self.handshake(addr)
            .and_then(move |(stream, version)| Self::do_start_api(stream, version, client_id))
            .map(move |info| info)
    }

    pub fn connect_to(&self, addr: SocketAddr) -> impl Future<Item=TwsClient, Error=io::Error> {
        let client_id = self.client_id;
        self.handshake(addr)
            .and_then(move |(stream, version)| Self::do_start_api(stream, version, client_id))
            .map(move |info| {
                tokio::spawn(info.1);
                info.0
            })
    }

    /*pub fn dispatch(&self, info: (TwsClient, TwsTask)) -> impl Future<Item=TwsClient, Error=io::Error> {
        let (client, task) = info;
        tokio::spawn(task);
    }*/
}
