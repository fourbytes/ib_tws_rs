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

pub fn encode_req_fundamental_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqFundamentalData,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 2;

    buf.push_int(REQ_FUNDAMENTAL_DATA);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    buf.push_int(req.contract.con_id);

    buf.push_string(&req.contract.symbol);
    buf.push_string(&req.contract.sec_type);
    buf.push_string(&req.contract.exchange);
    buf.push_string(&req.contract.primary_exch);
    buf.push_string(&req.contract.currency);
    buf.push_string(&req.contract.local_symbol);

    buf.push_string(&req.report_type);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_fundamental_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelFundamentalData,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_FUNDAMENTAL_DATA);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_fundamental_data_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let data = buf.read_string()?;
    Ok((
        Response::FundamentalDataMsg(FundamentalDataMsg { req_id, data }),
        req_id,
    ))
}
