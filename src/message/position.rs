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

pub fn encode_req_positions_multi(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqPositionsMulti,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_MODELS_SUPPORT {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_MODELS_SUPPORT));
    }

    const VERSION: i32 = 1;

    buf.push_int(REQ_POSITIONS_MULTI);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);
    buf.push_string(&req.account);
    buf.push_string(&req.model_code);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_positions_multi(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelPositionsMulti,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_MODELS_SUPPORT {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_MODELS_SUPPORT));
    }

    const VERSION: i32 = 1;

    buf.push_int(CANCEL_POSITIONS_MULTI);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_position_multi_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let account = buf.read_string()?;

    let mut contract = Contract::default();
    contract.con_id = buf.read_int()?;
    contract.symbol = buf.read_string()?;
    contract.sec_type = buf.read_string()?;
    contract.last_trade_date_or_contract_month = buf.read_string()?;
    contract.strike = buf.read_double()?;
    contract.right = buf.read_string()?;
    contract.multiplier = buf.read_string()?;
    contract.exchange = buf.read_string()?;
    contract.currency = buf.read_string()?;
    contract.local_symbol = buf.read_string()?;
    contract.trading_class = buf.read_string()?;
    let pos = buf.read_double()?;
    let avg_cost = buf.read_double()?;
    let model_code = buf.read_string()?;

    Ok((
        Response::PositionMultiMsg(PositionMultiMsg {
            req_id,
            account,
            model_code,
            contract,
            pos,
            avg_cost,
        }),
        req_id,
    ))
}

pub fn decode_position_multi_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;

    Ok((
        Response::PositionMultiEndMsg(PositionMultiEndMsg { req_id }),
        req_id,
    ))
}

////////////////////////////

pub fn encode_req_positions(
    ctx: &mut Context,
    buf: &mut BytesMut,
    _req: &ReqPositions,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_POSITIONS);
    buf.push_int(VERSION);

    Ok(DispatchId::Global(OPCODE_REQ_POSITIONS)) // GlobalMulti
}

pub fn encode_cancel_positions(
    ctx: &mut Context,
    buf: &mut BytesMut,
    _req: &CancelPositions,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_POSITIONS);
    buf.push_int(VERSION);

    Ok(DispatchId::Global(OPCODE_CANCEL_POSITIONS))
}

// [NO REQ_ID]
pub fn decode_position_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;
    let account = buf.read_string()?;

    let mut contract = Contract::default();
    contract.con_id = buf.read_int()?;
    contract.symbol = buf.read_string()?;
    contract.sec_type = buf.read_string()?;
    contract.last_trade_date_or_contract_month = buf.read_string()?;
    contract.strike = buf.read_double()?;
    contract.right = buf.read_string()?;
    contract.multiplier = buf.read_string()?;
    contract.exchange = buf.read_string()?;
    contract.currency = buf.read_string()?;
    contract.local_symbol = buf.read_string()?;
    if version >= 2 {
        contract.trading_class = buf.read_string()?;
    }

    let pos = if ctx.server_version() >= MIN_SERVER_VER_FRACTIONAL_POSITIONS {
        buf.read_double()?
    } else {
        f64::from(buf.read_int()?)
    };

    let avg_cost = if version >= 3 {
        buf.read_double()?
    } else {
        0.0f64
    };

    Ok((
        Response::PositionMsg(PositionMsg {
            account,
            contract,
            pos,
            avg_cost,
        }),
        OPCODE_REQ_POSITIONS,
    ))
}

// [NO REQ_ID]
pub fn decode_position_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    Ok((
        Response::PositionEndMsg(PositionEndMsg {}),
        OPCODE_REQ_POSITIONS,
    ))
}
