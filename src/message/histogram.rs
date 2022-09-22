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

pub fn encode_req_histogram_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqHistogramData,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_HISTOGRAM {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_REQ_HISTOGRAM));
    }

    buf.push_int(REQ_HISTOGRAM_DATA);
    buf.push_int(req.req_id);
    encode_contract(buf, &req.contract);
    buf.push_bool(req.contract.include_expired);
    buf.push_bool(req.use_rth);
    buf.push_string(&req.time_period);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_histogram_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelHistogramData,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_HISTOGRAM {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_REQ_HISTOGRAM));
    }

    buf.push_int(CANCEL_HISTOGRAM_DATA);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_histogram_data_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let n = buf.read_int()?;
    let mut items = Vec::new();

    for _ in 0..n {
        let price = buf.read_double()?;
        let size = buf.read_long()?;
        items.push(HistogramEntry { price, size })
    }

    Ok((
        Response::HistogramDataMsg(HistogramDataMsg { req_id, items }),
        req_id,
    ))
}
