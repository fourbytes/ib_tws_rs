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

pub fn decode_historical_ticks_bid_ask(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let tick_count = buf.read_int()?;

    let mut ticks = Vec::with_capacity(tick_count as usize);

    for _ in 0..tick_count {
        let time = buf.read_long()?;
        let mask = buf.read_int()?;
        let price_bid = buf.read_double()?;
        let price_ask = buf.read_double()?;
        let size_bid = buf.read_long()?;
        let size_ask = buf.read_long()?;

        let tick = HistoricalTickBidAsk {
            time,
            mask,
            price_bid,
            price_ask,
            size_bid,
            size_ask,
        };

        ticks.push(tick);
    }

    let done = buf.read_bool()?;

    Ok((
        Response::HistoricalTickBidAskMsg(HistoricalTickBidAskMsg {
            req_id,
            ticks,
            done,
        }),
        req_id,
    ))
}

pub fn decode_historical_ticks(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let tick_count = buf.read_int()?;

    let mut ticks = Vec::with_capacity(tick_count as usize);

    for _ in 0..tick_count {
        let time = buf.read_long()?;
        let _ = buf.read_int()?;
        let price = buf.read_double()?;
        let size = buf.read_long()?;

        let tick = HistoricalTick { time, price, size };

        ticks.push(tick);
    }

    let done = buf.read_bool()?;

    Ok((
        Response::HistoricalTicksMsg(HistoricalTicksMsg {
            req_id,
            ticks,
            done,
        }),
        req_id,
    ))
}

pub fn decode_historical_data_update_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let bar_count = buf.read_int()?;
    let time = buf.read_string()?;
    let open = buf.read_double()?;
    let close = buf.read_double()?;
    let high = buf.read_double()?;
    let low = buf.read_double()?;
    let wap = buf.read_double()?;
    let volume = buf.read_long()?;

    Ok((
        Response::HistoricalDataUpdateMsg(HistoricalDataUpdateMsg {
            req_id,
            bar: Bar {
                time,
                open,
                high,
                low,
                close,
                volume,
                wap,
                count: bar_count,
            },
        }),
        req_id,
    ))
}

pub fn decode_historical_news_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let has_more = buf.read_bool()?;

    Ok((
        Response::HistoricalNewsEndMsg(HistoricalNewsEndMsg { req_id, has_more }),
        req_id,
    ))
}

pub fn decode_historical_news_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let time = buf.read_string()?;
    let provider_code = buf.read_string()?;
    let article_id = buf.read_string()?;
    let headline = buf.read_string()?;

    Ok((
        Response::HistoricalNewsMsg(HistoricalNewsMsg {
            req_id,
            time,
            provider_code,
            article_id,
            headline,
        }),
        req_id,
    ))
}

pub fn decode_historical_data_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let version = if ctx.server_version() < MIN_SERVER_VER_SYNT_REALTIME_BARS {
        buf.read_int()?
    } else {
        i32::MAX
    };

    let req_id = buf.read_int()?;
    let mut start_date: String = "".to_string();
    let mut end_date: String = "".to_string();
    let mut bars = Vec::new();

    if version >= 2 {
        start_date = buf.read_string()?;
        end_date = buf.read_string()?;
    }
    let item_count = buf.read_int()?;
    for _ in 0..item_count {
        let date = buf.read_string()?;
        let open = buf.read_double()?;
        let high = buf.read_double()?;
        let low = buf.read_double()?;
        let close = buf.read_double()?;
        let volume = if ctx.server_version() < MIN_SERVER_VER_SYNT_REALTIME_BARS {
            i64::from(buf.read_int()?)
        } else {
            buf.read_long()?
        };

        let wap = buf.read_double()?;

        if ctx.server_version() < MIN_SERVER_VER_SYNT_REALTIME_BARS {
            let _has_gaps = buf.read_string()?;
        }

        let bar_count =
            if version >= 3 { buf.read_int()? } else { -1 };

        let historical_bar = Bar {
            time: date,
            open,
            high,
            low,
            close,
            volume,
            count: bar_count,
            wap,
        };

        bars.push(historical_bar);
    }
    Ok((
        Response::HistoricalDataMsg(HistoricalDataMsg {
            req_id,
            start_date,
            end_date,
            bars,
        }),
        req_id,
    ))
}

pub fn decode_historical_ticks_last(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let tick_count = buf.read_int()?;

    let mut ticks = Vec::with_capacity(tick_count as usize);

    for _ in 0..tick_count {
        let time = buf.read_long()?;
        let mask = buf.read_int()?;
        let price = buf.read_double()?;
        let size = buf.read_long()?;
        let exchange = buf.read_string()?;
        let special_conditions = buf.read_string()?;

        let tick = HistoricalTickLast {
            time,
            mask,
            price,
            size,
            exchange,
            special_conditions,
        };

        ticks.push(tick);
    }

    let done = buf.read_bool()?;

    Ok((
        Response::HistoricalTickLastMsg(HistoricalTickLastMsg {
            req_id,
            ticks,
            done,
        }),
        req_id,
    ))
}

pub fn encode_req_historical_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqHistoricalData,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 6;

    buf.push_int(REQ_HISTORICAL_DATA);

    if ctx.server_version() < MIN_SERVER_VER_SYNT_REALTIME_BARS {
        buf.push_int(VERSION);
    }

    buf.push_int(req.req_id);

    encode_contract(buf, &req.contract);

    buf.push_bool(req.contract.include_expired);

    buf.push_string(&req.end_date_time);
    buf.push_string(&req.bar_size_setting);

    buf.push_string(&req.duration_str);
    buf.push_int(req.use_rth); //TODO
    buf.push_string(&req.what_to_show);

    buf.push_int(req.format_date);

    if req.contract.sec_type.to_uppercase() == "BAG" {
        buf.push_int(req.contract.combo_legs.len() as i32);
        for elem in &req.contract.combo_legs {
            buf.push_int(elem.con_id);
            buf.push_int(elem.ratio);
            buf.push_string(&elem.action);
            buf.push_string(&elem.exchange);
        }
    }

    if ctx.server_version() >= MIN_SERVER_VER_SYNT_REALTIME_BARS {
        buf.push_bool(req.keepup_to_date);
    }

    encode_tagvalue_as_string(buf, &req.chart_options);

    Ok(DispatchId::Multi(req.req_id))
}

pub fn encode_req_historical_news(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqHistoricalNews,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_HISTORICAL_NEWS {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_REQ_HISTORICAL_NEWS,
        ));
    }

    buf.push_int(REQ_HISTORICAL_NEWS);
    buf.push_int(req.req_id);
    buf.push_int(req.con_id);
    buf.push_string(&req.provider_code);
    buf.push_string(&req.start_time);
    buf.push_string(&req.end_time);
    buf.push_int(req.total_results);

    if ctx.server_version() >= MIN_SERVER_VER_NEWS_QUERY_ORIGINS {
        encode_tagvalue_as_string(buf, &req.options);
    }

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_req_historical_ticks(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqHistoricalTicks,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_HISTORICAL_TICKS {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_HISTORICAL_TICKS,
        ));
    }

    buf.push_int(REQ_HISTORICAL_TICKS);
    buf.push_int(req.req_id);
    encode_contract(buf, &req.contract);
    buf.push_bool(req.contract.include_expired);
    buf.push_string(&req.start_time);
    buf.push_string(&req.end_time);
    buf.push_int(req.num_of_ticks);
    buf.push_string(&req.what_to_show);
    buf.push_int(req.use_rth);
    buf.push_bool(req.ignore_size);
    encode_tagvalue_as_string(buf, &req.options);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_historical_data(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelHistoricalData,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_HISTORICAL_DATA);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}
