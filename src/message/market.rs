use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bit::BitIndex;
use bytes::{BufMut, BytesMut};
use domain::*;
use std::{f64, i32};
use std::io;

pub fn encode_req_mkt_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqMktData,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 11;

    buf.push_int(REQ_MKT_DATA);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    encode_contract(buf, &req.contract);

    if req.contract.sec_type.to_uppercase() == "BAG" {
        buf.push_int(req.contract.combo_legs.len() as i32);
        for elem in &req.contract.combo_legs {
            buf.push_int(elem.con_id);
            buf.push_int(elem.ratio);
            buf.push_string(&elem.action);
            buf.push_string(&elem.exchange);
        }
    }

    if let Some(ref comp) = req.contract.delta_neutral_contract {
        buf.push_bool(true);
        buf.push_int(comp.con_id);
        buf.push_double(comp.delta);
        buf.push_double(comp.price);
    } else {
        buf.push_bool(false);
    }

    buf.push_string(&req.generic_tick_list);

    buf.push_bool(req.snapshot);

    if ctx.server_version() >= MIN_SERVER_VER_REQ_SMART_COMPONENTS {
        buf.push_bool(req.regulatory_snapshot);
    }

    encode_tagvalue_as_string(buf, &req.mkt_data_options);

    Ok(DispatchId::Stream(req.req_id))
}

pub fn decode_tick_efp_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let tick_type = buf.read_int()?;
    let basis_points = buf.read_double()?;
    let formatted_basis_points = buf.read_string()?;
    let implied_futures_price = buf.read_double()?;
    let hold_days = buf.read_int()?;
    let future_last_trade_date = buf.read_string()?;
    let dividend_impact = buf.read_double()?;
    let dividends_to_last_trade_date = buf.read_double()?;
    Ok((
        Response::TickEFPMsg(TickEFPMsg {
            req_id,
            tick_type,
            basis_points,
            formatted_basis_points,
            implied_futures_price,
            hold_days,
            future_last_trade_date,
            dividend_impact,
            dividends_to_last_trade_date,
        }),
        req_id,
    ))
}

pub fn decode_tick_string_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let tick_type = buf.read_int()?;
    let value = buf.read_string()?;

    Ok((
        Response::TickStringMsg(TickStringMsg {
            req_id,
            tick_type,
            value,
        }),
        req_id,
    ))
}

pub fn decode_tick_generic_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let tick_type = buf.read_int()?;
    let value = buf.read_double()?;

    Ok((
        Response::TickGenericMsg(TickGenericMsg {
            req_id,
            tick_type,
            value,
        }),
        req_id,
    ))
}

pub fn decode_tick_option_computation_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let tick_type = buf.read_int()?;
    let mut implied_vol = buf.read_double()?;
    if abs_diff_eq!(implied_vol, -1.0f64) {
        // -1 is the "not yet computed" indicator
        implied_vol = f64::MAX;
    }

    let mut delta = buf.read_double()?;
    if abs_diff_eq!(delta, -2.0f64) {
        // -2 is the "not yet computed" indicator
        delta = f64::MAX;
    }
    let mut opt_price = f64::MAX;
    let mut pv_dividend = f64::MAX;
    let mut gamma = f64::MAX;
    let mut vega = f64::MAX;
    let mut theta = f64::MAX;
    let mut und_price = f64::MAX;
    if version >= 6
        || tick_type == TickType::MODEL_OPTION as i32
        || tick_type == TickType::DELAYED_MODEL_OPTION as i32
    {
        // introduced in version == 5
        opt_price = buf.read_double()?;
        if abs_diff_eq!(opt_price, -1.0f64) {
            // -1 is the "not yet computed" indicator
            opt_price = f64::MAX;
        }
        pv_dividend = buf.read_double()?;
        if abs_diff_eq!(pv_dividend, -1.0f64) {
            // -1 is the "not yet computed" indicator
            pv_dividend = f64::MAX;
        }
    }
    if version >= 6 {
        gamma = buf.read_double()?;
        if abs_diff_eq!(gamma, -2.0f64) {
            // -2 is the "not yet computed" indicator
            gamma = f64::MAX;
        }
        vega = buf.read_double()?;
        if abs_diff_eq!(vega, -2.0f64) {
            // -2 is the "not yet computed" indicator
            vega = f64::MAX;
        }
        theta = buf.read_double()?;
        if abs_diff_eq!(theta, -2.0f64) {
            // -2 is the "not yet computed" indicator
            theta = f64::MAX;
        }
        und_price = buf.read_double()?;
        if abs_diff_eq!(und_price, -1.0f64) {
            // -1 is the "not yet computed" indicator
            und_price = f64::MAX;
        }
    }

    Ok((
        Response::TickOptionComputationMsg(TickOptionComputationMsg {
            req_id,
            tick_type,
            implied_vol,
            delta,
            opt_price,
            pv_dividend,
            gamma,
            vega,
            theta,
            und_price,
        }),
        req_id,
    ))
}

pub fn decode_tick_size_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let tick_type = buf.read_int()?;
    let size = buf.read_int()?;

    Ok((
        Response::TickSizeMsg(TickSizeMsg {
            req_id,
            tick_type,
            size,
        }),
        req_id,
    ))
}

pub fn decode_tick_price_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let tick_type = buf.read_int()?;
    let price = buf.read_double()?;
    let mut size = 0;
    let mut attribs: TickAttr = Default::default();
    if version >= 2 {
        size = buf.read_int()?;
    }
    if version >= 3 {
        let attr_mask = buf.read_int()?;

        attribs.can_auto_execute = attr_mask == 1;
        if ctx.server_version() >= MIN_SERVER_VER_PAST_LIMIT {
            let mask = attr_mask as u32;
            attribs.can_auto_execute = mask.bit(0);
            attribs.past_limit = mask.bit(1);
            if ctx.server_version() >= MIN_SERVER_VER_PRE_OPEN_BID_ASK {
                attribs.pre_open = mask.bit(2);
            }
        }
    }

    Ok((
        Response::TickPriceMsg(TickPriceMsg {
            req_id,
            tick_type,
            price,
            size,
            attribs,
        }),
        req_id,
    ))
}

pub fn decode_tick_req_params_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    println!("decode_tick_req_params_msg");
    let req_id = buf.read_int()?;
    let min_tick = buf.read_double()?;
    let bbo_exchange = buf.read_string()?;
    let snapshot_permissions = buf.read_int()?;

    Ok((
        Response::TickReqParamsMsg(TickReqParamsMsg {
            req_id,
            min_tick,
            bbo_exchange,
            snapshot_permissions,
        }),
        req_id,
    ))
}

pub fn decode_tick_by_tick_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let tick_type = buf.read_int()?;
    let time = buf.read_long()?;

    match tick_type {
        //0 => Ok(Response::TickByTickNoneMsg(TickByTickNoneMsg {})),
        1 | 2 => {
            let price = buf.read_double()?;
            let size = buf.read_int()?;
            let mask = buf.read_int()? as u32;
            let mut attribs: TickAttr = Default::default();
            attribs.past_limit = mask.bit(0);
            attribs.unreported = mask.bit(1);
            let exchange = buf.read_string()?;
            let special_conditions = buf.read_string()?;
            Ok((
                Response::TickByTickAllLastMsg(TickByTickAllLastMsg {
                    req_id,
                    tick_type,
                    time,
                    price,
                    size,
                    attribs,
                    exchange,
                    special_conditions,
                }),
                req_id,
            ))
        }
        3 => {
            // BidAsk
            let bid_price = buf.read_double()?;
            let ask_price = buf.read_double()?;
            let bid_size = buf.read_int()?;
            let ask_size = buf.read_int()?;
            let mask = buf.read_int()? as u32;
            let mut attribs: TickAttr = Default::default();
            attribs.bid_past_low = mask.bit(0);
            attribs.ask_past_high = mask.bit(1);
            Ok((
                Response::TickByTickBidAskMsg(TickByTickBidAskMsg {
                    req_id,
                    time,
                    bid_price,
                    ask_price,
                    bid_size,
                    ask_size,
                    attribs,
                }),
                req_id,
            ))
        }
        4 => {
            // MidPoint
            let mid_point = buf.read_double()?;
            Ok((
                Response::TickByTickMidPointMsg(TickByTickMidPointMsg {
                    req_id,
                    time,
                    mid_point,
                }),
                req_id,
            ))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unknown tick_by_tick tick_type",
        )),
    }
}

pub fn decode_tick_snapshot_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;

    Ok((
        Response::TickSnapshotEndMsg(TickSnapshotEndMsg { req_id }),
        req_id,
    ))
}

pub fn decode_tick_news_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let time_stamp = buf.read_long()?;
    let provider_code = buf.read_string()?;
    let article_id = buf.read_string()?;
    let headline = buf.read_string()?;
    let extra_data = buf.read_string()?;

    Ok((
        Response::TickNewsMsg(TickNewsMsg {
            req_id,
            time_stamp,
            provider_code,
            article_id,
            headline,
            extra_data,
        }),
        req_id,
    ))
}

pub fn decode_market_depth_l2_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;

    let position = buf.read_int()?;
    let market_maker = buf.read_string()?;
    let operation = buf.read_int()?;
    let side = buf.read_int()?;
    let price = buf.read_double()?;
    let size = buf.read_int()?;

    Ok((
        Response::MarketDepthL2Msg(MarketDepthL2Msg {
            id: req_id,
            position,
            market_maker,
            operation,
            side,
            price,
            size,
        }),
        req_id,
    ))
}

pub fn decode_market_depth_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;

    let position = buf.read_int()?;
    let operation = buf.read_int()?;
    let side = buf.read_int()?;
    let price = buf.read_double()?;
    let size = buf.read_int()?;

    Ok((
        Response::MarketDepthMsg(MarketDepthMsg {
            id: req_id,
            position,
            operation,
            side,
            price,
            size,
        }),
        req_id,
    ))
}

pub fn encode_req_tick_by_tick_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqTickByTickData,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_TICK_BY_TICK {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_TICK_BY_TICK));
    }

    if ctx.server_version() < MIN_SERVER_VER_TICK_BY_TICK_IGNORE_SIZE
        && (req.num_of_ticks != 0 || req.ignore_size)
    {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_TICK_BY_TICK_IGNORE_SIZE,
        ));
    }

    buf.push_int(REQ_TICK_BY_TICK_DATA);
    buf.push_int(req.req_id);
    encode_contract(buf, &req.contract);

    buf.push_string(&req.tick_type);

    if ctx.server_version() >= MIN_SERVER_VER_TICK_BY_TICK_IGNORE_SIZE {
        buf.push_int(req.num_of_ticks);
        buf.push_bool(req.ignore_size);
    }
    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_tick_by_tick_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelTickByTickData,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_TICK_BY_TICK {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_TICK_BY_TICK));
    }

    buf.push_int(CANCEL_TICK_BY_TICK_DATA);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_realtime_bars(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelRealtimeBars,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_REAL_TIME_BARS);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_req_realtime_bars(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqRealtimeBars,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 3;

    buf.push_int(REQ_REAL_TIME_BARS);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    encode_contract(buf, &req.contract);

    buf.push_int(req.bar_size);
    buf.push_string(&req.what_to_show);
    buf.push_bool(req.use_rth);

    encode_tagvalue_as_string(buf, &req.options);

    Ok(DispatchId::Stream(req.req_id))
}

pub fn encode_req_mkt_depth(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqMktDepth,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 5;

    buf.push_int(REQ_MKT_DEPTH);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    encode_contract_without_primary_exch(buf, &req.contract);

    buf.push_int(req.num_rows);

    encode_tagvalue_as_string(buf, &req.options);

    Ok(DispatchId::Stream(req.req_id))
}

pub fn encode_cancel_mkt_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelMktData,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_MKT_DATA);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_mkt_depth(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelMktDepth,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_MKT_DEPTH);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_calculate_implied_volatility(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CalculateImpliedVolatility,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 2;

    buf.push_int(REQ_CALC_IMPLIED_VOLAT);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    encode_contract(buf, &req.contract);

    buf.push_double(req.option_price);
    buf.push_double(req.under_price);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_calculate_implied_volatility(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelCalculateImpliedVolatility,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_CALC_IMPLIED_VOLAT);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_calculate_option_price(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CalculateOptionPrice,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 2;

    buf.push_int(REQ_CALC_OPTION_PRICE);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    encode_contract(buf, &req.contract);

    buf.push_double(req.volatility);
    buf.push_double(req.under_price);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_calculate_option_price(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelCalculateOptionPrice,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_CALC_OPTION_PRICE);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_req_market_data_type(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqMarketDataType,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_MARKET_DATA_TYPE);
    buf.push_int(VERSION);
    buf.push_int(req.market_data_type);

    Ok(DispatchId::Global(OPCODE_REQ_MARKET_DATA_TYPE))
}

pub fn decode_market_data_type_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let market_data_type = buf.read_int()?;

    Ok((
        Response::MarketDataTypeMsg(MarketDataTypeMsg {
            req_id,
            market_data_type,
        }),
        req_id,
    ))
}

pub fn decode_realtime_bars_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let time = buf.read_long()?;
    let open = buf.read_double()?;
    let high = buf.read_double()?;
    let low = buf.read_double()?;
    let close = buf.read_double()?;
    let volume = buf.read_long()?;
    let wap = buf.read_double()?;
    let count = buf.read_int()?;
    Ok((
        Response::RealTimeBarsMsg(RealTimeBarsMsg {
            req_id,
            time,
            open,
            high,
            low,
            close,
            volume,
            wap,
            count,
        }),
        req_id,
    ))
}
