use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use std::io;

const NEXT_VALID_ID: i32 = 9;
const MANAGED_ACCTS: i32 = 15;
pub const START_API: i32 = 71;

#[derive(Debug)]
pub enum Message {
    ManagedAccts(String),
    NextValidId(i32),
}

pub fn encode_handshake(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &Handshake,
) -> Result<DispatchId, EncodeError> {
    let s = match req.option {
        Some(ref option) => format!("v{}..{} {}", req.min_version, req.max_version, option),
        None => format!("v{}..{}", req.min_version, req.max_version),
    };
    buf.push_string(&s);

    Ok(DispatchId::Global(OPCODE_HANDSHAKE))
}

pub fn decode_handshake_ack(ctx: &mut Context, buf: &mut BytesMut) -> Result<HandshakeAck, io::Error> {
    let server_version = buf.read_int()?;
    let addr_or_time = buf.read_string()?;

    Ok(HandshakeAck {
        server_version,
        addr_or_time,
    })
}

pub fn encode_start_api(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &StartApi,
) -> Result<DispatchId, EncodeError> {
    // response is parse_managed_acct_msg
    const VERSION: i32 = 2;

    buf.push_int(START_API);
    buf.push_int(VERSION);
    buf.push_int(req.client_id);
    buf.push_string(&req.optional_capabilities);

    Ok(DispatchId::Global(OPCODE_START_API))
}
