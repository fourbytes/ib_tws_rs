use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::Contract;
use std::io;

// The reqAccountUpdates function creates a subscription to the TWS through which account
// and portfolio information is delivered. This information is the exact same as the one displayed
// within the TWS' Account Window. Note this function receives a specific account along with a flag
// indicating whether to start or stop the subscription. In a single account structure,
// the account number is not necessary. Just as with the TWS' Account Window, unless there is a
// position change this information is updated at a fixed interval of three minutes.
//
// Resulting account and portfolio information will be delivered via the updateAccountValue,
// updatePortfolio, updateAccountTime and accountDownloadEnd
pub fn encode_req_account_updates(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqAccountUpdates,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 2;

    buf.push_int(REQ_ACCOUNT_DATA);
    buf.push_int(VERSION);
    buf.push_bool(req.subscribe);

    buf.push_string(&req.acct_code);

    Ok(DispatchId::Global(OPCODE_REQ_ACCOUNT_UPDATES))
}

// [NO REQ_ID]
pub fn decode_portfolio_value_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;
    let mut contract: Contract = Default::default();
    if version >= 6 {
        contract.con_id = buf.read_int()?;
    }
    contract.symbol = buf.read_string()?;
    contract.sec_type = buf.read_string()?;
    contract.last_trade_date_or_contract_month = buf.read_string()?;
    contract.strike = buf.read_double()?;
    contract.right = buf.read_string()?;
    if version >= 7 {
        contract.multiplier = buf.read_string()?;
        contract.primary_exch = buf.read_string()?;
    }
    contract.currency = buf.read_string()?;
    if version >= 2 {
        contract.local_symbol = buf.read_string()?;
    }
    if version >= 8 {
        contract.trading_class = buf.read_string()?;
    }

    let position = if ctx.server_version() >= MIN_SERVER_VER_FRACTIONAL_POSITIONS {
        buf.read_double()?
    } else {
        f64::from(buf.read_int()?)
    };

    let market_price = buf.read_double()?;
    let market_value = buf.read_double()?;

    let average_cost = if version >= 3 {
        buf.read_double()?
    } else {
        0.0
    };
    let unrealized_pnl = if version >= 3 {
        buf.read_double()?
    } else {
        0.0
    };
    let realized_pnl = if version >= 3 {
        buf.read_double()?
    } else {
        0.0
    };

    let account_name = if version >= 4 {
        buf.read_string()?
    } else {
        "".to_string()
    };

    if version == 6 && ctx.server_version() == 39 {
        contract.primary_exch = buf.read_string()?;
    }

    Ok((
        Response::PortfolioValueMsg(PortfolioValueMsg {
            contract,
            position,
            market_price,
            market_value,
            average_cost,
            unrealized_pnl,
            realized_pnl,
            account_name,
        }),
        OPCODE_PORTFOLIO_VALUE, // TODO
    ))
}

// [NO REQ_ID]
pub fn decode_acct_update_time_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let time_stamp = buf.read_string()?;
    Ok((
        Response::AcctUpdateTimeMsg(AcctUpdateTimeMsg { time_stamp }),
        OPCODE_REQ_ACCOUNT_UPDATES, // TODO
    ))
}

// [NO REQ_ID]
pub fn decode_acct_value_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;
    let key = buf.read_string()?;
    let val = buf.read_string()?;
    let cur = buf.read_string()?;
    let account_name = if version >= 2 {
        buf.read_string()?
    } else {
        "".to_string()
    };

    Ok((
        Response::AcctValueMsg(AcctValueMsg {
            key,
            val,
            cur,
            account_name,
        }),
        OPCODE_REQ_ACCOUNT_UPDATES,
    ))
}

// [NO REQ_ID]
pub fn decode_acct_download_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let account_name = buf.read_string()?;
    Ok((
        Response::AcctDownloadEndMsg(AcctDownloadEndMsg { account_name }),
        OPCODE_REQ_ACCOUNT_UPDATES, // TODO
    ))
}

////////////////////////////////
// The reqAccountSummary method creates a subscription for the account data displayed in the
// TWS Account Summary window. It is commonly used with multiple-account structures.
// Introducing broker (IBroker) accounts with more than 50 subaccounts or configured for
// on-demand account lookup cannot use reqAccountSummary with group="All".
pub fn encode_req_account_summary(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqAccountSummary,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_ACCOUNT_SUMMARY);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);
    buf.push_string(&req.group);
    buf.push_string(&req.tags);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_cancel_account_summary(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelAccountSummary,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_ACCOUNT_SUMMARY);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_account_summary_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let account = buf.read_string()?;
    let tag = buf.read_string()?;
    let value = buf.read_string()?;
    let currency = buf.read_string()?;
    Ok((
        Response::AccountSummaryMsg(AccountSummaryMsg {
            req_id,
            account,
            tag,
            value,
            currency,
        }),
        req_id,
    ))
}

pub fn decode_account_summary_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    Ok((
        Response::AccountSummaryEndMsg(AccountSummaryEndMsg { req_id }),
        req_id,
    ))
}

////////////////////////////////////////////////////////////////////////////////////
pub fn encode_req_managed_accts(
    ctx: &mut Context,
    buf: &mut BytesMut,
    _req: &ReqManagedAccts,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_MANAGED_ACCTS);
    buf.push_int(VERSION);

    Ok(DispatchId::Global(OPCODE_REQ_MANAGED_ACCTS))
}
///////////////////////////

pub fn encode_cancel_account_updates_multi(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelAccountUpdatesMulti,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_MODELS_SUPPORT {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_MODELS_SUPPORT));
    }

    const VERSION: i32 = 1;

    buf.push_int(CANCEL_ACCOUNT_UPDATES_MULTI);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

// [NO REQ_ID]
pub fn decode_managed_accts_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let accounts = buf.read_string()?;

    Ok((
        Response::ManagedAcctsMsg(ManagedAcctsMsg { accounts }),
        OPCODE_REQ_MANAGED_ACCTS,
    ))
}

///////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////
pub fn encode_req_account_updates_multi(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqAccountUpdatesMulti,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_MODELS_SUPPORT {
        return Err(EncodeError::VersionLessError(MIN_SERVER_VER_MODELS_SUPPORT));
    }

    const VERSION: i32 = 1;

    buf.push_int(REQ_ACCOUNT_UPDATES_MULTI);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);
    buf.push_string(&req.account);
    buf.push_string(&req.model_code);
    buf.push_bool(req.ledger_and_nlv);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_account_update_multi_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let account = buf.read_string()?;
    let model_code = buf.read_string()?;
    let key = buf.read_string()?;
    let value = buf.read_string()?;
    let currency = buf.read_string()?;

    Ok((
        Response::AccountUpdateMultiMsg(AccountUpdateMultiMsg {
            req_id,
            account,
            model_code,
            key,
            value,
            currency,
        }),
        req_id,
    ))
}

pub fn decode_account_update_multi_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;

    Ok((
        Response::AccountUpdateMultiEndMsg(AccountUpdateMultiEndMsg { req_id }),
        req_id,
    ))
}
