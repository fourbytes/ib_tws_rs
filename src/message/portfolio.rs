use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::*;
use std::{f64, i32};
use std::io;

pub fn encode_req_pnl(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqPnl,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_PNL {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_PNL));
    }

    buf.push_int(REQ_PNL);
    buf.push_int(req.req_id);
    buf.push_string(&req.account);
    buf.push_string(&req.model_code);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_pnl(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelPnl,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_PNL {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_PNL));
    }

    buf.push_int(CANCEL_PNL);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_pnl_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let daily_pnl = buf.read_double()?;
    let mut unrealized_pnl = f64::MAX;
    let mut realized_pnl = f64::MAX;

    if ctx.server_version() >= MIN_SERVER_VER_UNREALIZED_PNL {
        unrealized_pnl = buf.read_double()?;
    }

    if ctx.server_version() >= MIN_SERVER_VER_REALIZED_PNL {
        realized_pnl = buf.read_double()?;
    }

    Ok((
        Response::PnlMsg(PnlMsg {
            req_id,
            daily_pnl,
            unrealized_pnl,
            realized_pnl,
        }),
        req_id,
    ))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn encode_req_pnl_single(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqPnlSingle,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_PNL {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_PNL));
    }

    buf.push_int(REQ_PNL_SINGLE);
    buf.push_int(req.req_id);
    buf.push_string(&req.account);
    buf.push_string(&req.model_code);
    buf.push_int(req.con_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_pnl_single(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelPnlSingle,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_PNL {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_PNL));
    }

    buf.push_int(CANCEL_PNL_SINGLE);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_pnl_single_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let pos = buf.read_int()?;
    let daily_pnl = buf.read_double()?;
    let mut unrealized_pnl = f64::MAX;
    let mut realized_pnl = f64::MAX;

    if ctx.server_version() >= MIN_SERVER_VER_UNREALIZED_PNL {
        unrealized_pnl = buf.read_double()?;
    }

    if ctx.server_version() >= MIN_SERVER_VER_REALIZED_PNL {
        realized_pnl = buf.read_double()?;
    }

    let value = buf.read_double()?;

    Ok((
        Response::PnlSingleMsg(PnlSingleMsg {
            req_id,
            pos,
            daily_pnl,
            unrealized_pnl,
            realized_pnl,
            value,
        }),
        req_id,
    ))
}
