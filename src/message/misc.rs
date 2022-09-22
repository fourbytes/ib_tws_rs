use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::*;
use std::collections::HashMap;
use std::io;

pub fn encode_req_ids(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqIds,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_IDS);
    buf.push_int(VERSION);
    buf.push_int(req.num_ids);

    Ok(DispatchId::Global(OPCODE_REQ_IDS))
}

pub fn encode_set_server_log_level(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &SetServerLogLevel,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(SET_SERVER_LOGLEVEL);
    buf.push_int(VERSION);
    buf.push_int(req.log_level);

    Ok(DispatchId::Global(OPCODE_SET_SERVER_LOG_LEVEL))
}

pub fn encode_req_current_time(
    ctx: &mut Context,
    buf: &mut BytesMut,
    _req: &ReqCurrentTime,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_CURRENT_TIME);
    buf.push_int(VERSION);

    Ok(DispatchId::Global(OPCODE_REQ_CURRENT_TIME))
}

pub fn encode_req_global_cancel(
    ctx: &mut Context,
    buf: &mut BytesMut,
    _req: &ReqGlobalCancel,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_GLOBAL_CANCEL);
    buf.push_int(VERSION);

    Ok(DispatchId::Global(OPCODE_REQ_GLOBAL_CANCEL))
}

pub fn encode_req_soft_dollar_tiers(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqSoftDollarTiers,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_SOFT_DOLLAR_TIER {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_SOFT_DOLLAR_TIER,
        ));
    }

    buf.push_int(REQ_SOFT_DOLLAR_TIERS);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_soft_dollar_tiers_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let count = buf.read_int()?;
    let mut tiers = Vec::new();

    for _ in 0..count {
        let name = buf.read_string()?;
        let value = buf.read_string()?;
        let display_name = buf.read_string()?;
        let tier = SoftDollarTier {
            name,
            value,
            display_name,
        };
        tiers.push(tier);
    }

    Ok((
        Response::SoftDollarTiersMsg(SoftDollarTiersMsg { req_id, tiers }),
        req_id,
    ))
}

// [NO REQ_ID]
pub fn decode_current_time_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let time = buf.read_long()?;
    Ok((
        Response::CurrentTimeMsg(CurrentTimeMsg { time }),
        OPCODE_REQ_CURRENT_TIME,
    ))
}

// [NO REQ_ID]
pub fn decode_next_valid_id_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let valid_id = buf.read_int()?;
    Ok((
        Response::NextValidIdMsg(NextValidIdMsg {
            order_id: valid_id, // valid_id only use for order id
        }),
        OPCODE_REQ_IDS,
    ))
}

pub fn decode_smart_components_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let n = buf.read_int()?;
    //Map<Integer, Entry<String, char>> map = new HashMap<>();
    let mut map = HashMap::new();

    for _ in 0..n {
        let bit_number = buf.read_int()?;
        let exchange = buf.read_string()?;
        let exchange_letter = buf.read_string()?.as_bytes()[0];

        map.insert(bit_number, (exchange, exchange_letter));
    }

    Ok((
        Response::SmartComponentsMsg(SmartComponentsMsg { req_id, map }),
        req_id,
    ))
}

// helper functions

pub fn encode_req_smart_components(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqSmartComponents,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_SMART_COMPONENTS {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_REQ_SMART_COMPONENTS,
        ));
    }

    buf.push_int(REQ_SMART_COMPONENTS);
    buf.push_int(req.req_id);
    buf.push_string(&req.bbo_exchange);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_exercise_options(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ExerciseOptions,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 2;

    buf.push_int(EXERCISE_OPTIONS);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    encode_contract_without_primary_exch(buf, &req.contract);

    buf.push_int(req.exercise_action);
    buf.push_int(req.exercise_quantity);
    buf.push_string(&req.account);
    buf.push_int(req.overriden);

    Ok(DispatchId::Oneshot(req.req_id))
}
