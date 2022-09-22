use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use bytes::{BufMut, BytesMut};
use domain::*;
// [NO REQ_ID]
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use std::io;

pub fn encode_req_market_rule(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqMarketRule,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_MARKET_RULES {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_MARKET_RULES));
    }

    buf.push_int(REQ_MARKET_RULE);
    buf.push_int(req.market_rule_id);

    Ok(DispatchId::Global(OPCODE_REQ_MARKET_RULE))
}

pub fn decode_market_rule(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let market_rule_id = buf.read_int()?;
    let count = buf.read_int()?;

    let mut price_increments = Vec::new();
    for i in 0..count {
        let low_edge = buf.read_double()?;
        let increment = buf.read_double()?;

        let price_increment = PriceIncrement {
            low_edge,
            increment,
        };

        price_increments.push(price_increment);
    }

    Ok((
        Response::MarketRule(MarketRule {
            market_rule_id,
            price_increments,
        }),
        OPCODE_REQ_MARKET_RULE,
    ))
}
