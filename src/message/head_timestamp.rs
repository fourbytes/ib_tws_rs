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

pub fn encode_req_head_timestamp(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqHeadTimestamp,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_HEAD_TIMESTAMP {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_REQ_HEAD_TIMESTAMP,
        ));
    }
    buf.push_int(REQ_HEAD_TIMESTAMP);
    buf.push_int(req.req_id);

    encode_contract(buf, &req.contract);
    buf.push_bool(req.contract.include_expired);

    buf.push_int(req.use_rth);
    buf.push_string(&req.what_to_show);
    buf.push_int(req.format_date);

    Ok(DispatchId::Stream(req.req_id))
}

pub fn encode_cancel_head_timestamp(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelHeadTimestamp,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_HEAD_TIMESTAMP {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_REQ_HEAD_TIMESTAMP,
        ));
    }
    buf.push_int(CANCEL_HEAD_TIMESTAMP);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_head_timestamp_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let head_time_stamp = buf.read_string()?;
    Ok((
        Response::HeadTimestampMsg(HeadTimestampMsg {
            req_id,
            head_time_stamp,
        }),
        req_id,
    ))
}
