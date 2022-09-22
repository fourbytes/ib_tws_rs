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

pub fn encode_req_executions(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqExecutions,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 3;

    buf.push_int(REQ_EXECUTIONS);
    buf.push_int(VERSION);

    buf.push_int(req.req_id);

    buf.push_int(req.filter.client_id);
    buf.push_string(&req.filter.acct_code);
    buf.push_string(&req.filter.time);
    buf.push_string(&req.filter.symbol);
    buf.push_string(&req.filter.sec_type);
    buf.push_string(&req.filter.exchange);
    buf.push_string(&req.filter.side);

    Ok(DispatchId::Oneshot(req.req_id))
}

// [NO REQ_ID]

pub fn decode_commission_report_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;

    let exec_id = buf.read_string()?;
    let commission = buf.read_double()?;
    let currency = buf.read_string()?;
    let realized_pnl = buf.read_double()?;
    let yield_value = buf.read_double()?;
    let yield_redemption_date = buf.read_int()?;

    // TODO:
    let req_id = ctx.get_req_id(&exec_id);
    assert!(req_id.is_some());
    let req_id = req_id.unwrap();
    Ok((
        Response::CommissionReportMsg(CommissionReportMsg {
            report: CommissionReport {
                exec_id,
                commission,
                currency,
                realized_pnl,
                yield_value,
                yield_redemption_date,
            },
        }),
        req_id,
    ))
}

pub fn decode_execution_data_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    Ok((
        Response::ExecutionDataEndMsg(ExecutionDataEndMsg { req_id }),
        req_id,
    ))
}

pub fn decode_execution_data_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let version = if ctx.server_version() < MIN_SERVER_VER_LAST_LIQUIDITY {
        buf.read_int()?
    } else {
        ctx.server_version()
    };

    let req_id = if version >= 7 { buf.read_int()? } else { -1 };

    let order_id = buf.read_int()?;

    // read contract fields
    let mut contract: Contract = Default::default();
    if version >= 5 {
        contract.con_id = buf.read_int()?;
    }
    contract.symbol = buf.read_string()?;
    contract.sec_type = buf.read_string()?;
    contract.last_trade_date_or_contract_month = buf.read_string()?;
    contract.strike = buf.read_double()?;
    contract.right = buf.read_string()?;
    if version >= 9 {
        contract.multiplier = buf.read_string()?;
    }
    contract.exchange = buf.read_string()?;
    contract.currency = buf.read_string()?;
    contract.local_symbol = buf.read_string()?;
    if version >= 10 {
        contract.trading_class = buf.read_string()?;
    }

    let mut exec: Execution = Default::default();
    exec.order_id = order_id;
    exec.exec_id = buf.read_string()?;
    exec.time = buf.read_string()?;
    exec.acct_number = buf.read_string()?;
    exec.exchange = buf.read_string()?;
    exec.side = buf.read_string()?;

    exec.shares = if ctx.server_version() >= MIN_SERVER_VER_FRACTIONAL_POSITIONS {
        buf.read_double()?
    } else {
        f64::from(buf.read_int()?)
    };

    exec.price = buf.read_double()?;
    if version >= 2 {
        exec.perm_id = buf.read_int()?;
    }
    if version >= 3 {
        exec.client_id = buf.read_int()?;
    }
    if version >= 4 {
        exec.liquidation = buf.read_int()?;
    }
    if version >= 6 {
        exec.cum_qty = buf.read_double()?;
        exec.avg_price = buf.read_double()?;
    }
    if version >= 8 {
        exec.order_ref = buf.read_string()?;
    }
    if version >= 9 {
        exec.ev_rule = buf.read_string()?;
        exec.ev_multiplier = buf.read_double()?;
    }
    if ctx.server_version() >= MIN_SERVER_VER_MODELS_SUPPORT {
        exec.model_code = buf.read_string()?;
    }

    if ctx.server_version() >= MIN_SERVER_VER_LAST_LIQUIDITY {
        exec.last_liquidity = Liquidities::from_code(buf.read_int()?)?;
    }

    Ok((
        Response::ExecutionDataMsg(ExecutionDataMsg {
            req_id,
            contract,
            exec,
        }),
        req_id,
    ))
}
