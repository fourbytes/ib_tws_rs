use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::*;
use std::io;

pub fn encode_request_fa(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &RequestFA,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_FA);
    buf.push_int(VERSION);
    buf.push_int(req.fa_data_type);

    Ok(DispatchId::Global(OPCODE_REQUEST_FA))
}

pub fn encode_replace_fa(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReplaceFA,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REPLACE_FA);
    buf.push_int(VERSION);
    buf.push_int(req.fa_data_type);
    buf.push_string(&req.xml);

    Ok(DispatchId::Global(OPCODE_REPLACE_FA))
}

// [NO REQ_ID]
pub fn decode_receive_fa_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let fa_data_type = buf.read_int()?;
    let xml = buf.read_string()?;

    Ok((
        Response::ReceiveFaMsg(ReceiveFaMsg { fa_data_type, xml }),
        OPCODE_REQUEST_FA,
    ))
}
