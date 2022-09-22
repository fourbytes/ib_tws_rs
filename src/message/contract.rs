use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::*;
use ordered_float::NotNaN;
use std::collections::HashSet;
use std::io;

//  IBApi.EWrapper.contractDetails. Once all contracts have been delivered the IBApi.EWrapper.contractDetailsEnd
pub fn encode_req_contract_details(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqContractDetails,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 8;

    buf.push_int(REQ_CONTRACT_DATA);
    buf.push_int(VERSION);

    buf.push_int(req.req_id);

    encode_contract(buf, &req.contract);

    buf.push_bool(req.contract.include_expired);

    buf.push_string(&req.contract.sec_id_type);
    buf.push_string(&req.contract.sec_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_contract_data_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    Ok((
        Response::ContractDataEndMsg(ContractDataEndMsg { req_id }),
        req_id,
    ))
}

pub fn decode_bond_contract_data_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;

    let req_id = if version >= 3 { buf.read_int()? } else { -1 };

    let mut contract_details: ContractDetails = Default::default();

    contract_details.contract.symbol = buf.read_string()?;
    contract_details.contract.sec_type = buf.read_string()?;
    contract_details.cusip = buf.read_string()?;
    contract_details.coupon = buf.read_double()?;
    read_last_trade_date(buf, &mut contract_details, true)?;
    contract_details.issue_date = buf.read_string()?;
    contract_details.bond_type = buf.read_string()?;
    contract_details.coupon_type = buf.read_string()?;
    contract_details.convertible = buf.read_bool()?;
    contract_details.callable = buf.read_bool()?;
    contract_details.putable = buf.read_bool()?;
    contract_details.desc_append = buf.read_string()?;
    contract_details.contract.exchange = buf.read_string()?;
    contract_details.contract.currency = buf.read_string()?;
    contract_details.market_name = buf.read_string()?;
    contract_details.contract.trading_class = buf.read_string()?;
    contract_details.contract.con_id = buf.read_int()?;
    contract_details.min_tick = buf.read_double()?;
    if ctx.server_version() >= MIN_SERVER_VER_MD_SIZE_MULTIPLIER {
        contract_details.md_size_multiplier = buf.read_int()?;
    }
    contract_details.order_types = buf.read_string()?;
    contract_details.valid_exchanges = buf.read_string()?;
    if version >= 2 {
        contract_details.next_option_date = buf.read_string()?;
        contract_details.next_option_type = buf.read_string()?;
        contract_details.next_option_partial = buf.read_bool()?;
        contract_details.notes = buf.read_string()?;
    }
    if version >= 4 {
        contract_details.long_name = buf.read_string()?;
    }
    if version >= 6 {
        contract_details.ev_rule = buf.read_string()?;
        contract_details.ev_multiplier = buf.read_double()?;
    }
    if version >= 5 {
        let count = buf.read_int()?;
        for _ in 0..count {
            let tag = buf.read_string()?;
            let value = buf.read_string()?;
            contract_details.sec_id_list.push(TagValue { tag, value });
        }
    }
    if ctx.server_version() >= MIN_SERVER_VER_AGG_GROUP {
        contract_details.agg_group = buf.read_int()?;
    }
    if ctx.server_version() >= MIN_SERVER_VER_MARKET_RULES {
        contract_details.market_rule_ids = buf.read_string()?;
    }

    Ok((
        Response::BondContractDataMsg(BondContractDataMsg {
            req_id,
            contract_details,
        }),
        req_id,
    ))
}

pub fn decode_contract_data_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;

    let req_id = if version >= 3 { buf.read_int()? } else { -1 };

    let mut contract_details: ContractDetails = Default::default();
    contract_details.contract.symbol = buf.read_string()?;
    contract_details.contract.sec_type = buf.read_string()?;
    read_last_trade_date(buf, &mut contract_details, false)?;
    contract_details.contract.strike = buf.read_double()?;
    contract_details.contract.right = buf.read_string()?;
    contract_details.contract.exchange = buf.read_string()?;
    contract_details.contract.currency = buf.read_string()?;
    contract_details.contract.local_symbol = buf.read_string()?;
    contract_details.market_name = buf.read_string()?;
    contract_details.contract.trading_class = buf.read_string()?;
    contract_details.contract.con_id = buf.read_int()?;
    contract_details.min_tick = buf.read_double()?;
    if ctx.server_version() >= MIN_SERVER_VER_MD_SIZE_MULTIPLIER {
        contract_details.md_size_multiplier = buf.read_int()?;
    }
    contract_details.contract.multiplier = buf.read_string()?;
    contract_details.order_types = buf.read_string()?;
    contract_details.valid_exchanges = buf.read_string()?;
    if version >= 2 {
        contract_details.price_magnifier = buf.read_int()?;
    }
    if version >= 4 {
        contract_details.under_con_id = buf.read_int()?;
    }
    if version >= 5 {
        contract_details.long_name = buf.read_string()?;
        contract_details.contract.primary_exch = buf.read_string()?;
    }
    if version >= 6 {
        contract_details.contract_month = buf.read_string()?;
        contract_details.industry = buf.read_string()?;
        contract_details.category = buf.read_string()?;
        contract_details.sub_category = buf.read_string()?;
        contract_details.timezone_id = buf.read_string()?;
        contract_details.trading_hours = buf.read_string()?;
        contract_details.liquid_hours = buf.read_string()?;
    }
    if version >= 8 {
        contract_details.ev_rule = buf.read_string()?;
        contract_details.ev_multiplier = buf.read_double()?;
    }
    if version >= 7 {
        let count = buf.read_int()?;
        for _ in 0..count {
            let tag = buf.read_string()?;
            let value = buf.read_string()?;
            contract_details.sec_id_list.push(TagValue { tag, value });
        }
    }
    if ctx.server_version() >= MIN_SERVER_VER_AGG_GROUP {
        contract_details.agg_group = buf.read_int()?;
    }
    if ctx.server_version() >= MIN_SERVER_VER_UNDERLYING_INFO {
        contract_details.under_symbol = buf.read_string()?;
        contract_details.under_sec_type = buf.read_string()?;
    }
    if ctx.server_version() >= MIN_SERVER_VER_MARKET_RULES {
        contract_details.market_rule_ids = buf.read_string()?;
    }
    if ctx.server_version() >= MIN_SERVER_VER_REAL_EXPIRATION_DATE {
        contract_details.real_expiration_date = buf.read_string()?;
    }

    Ok((
        Response::ContractDataMsg(ContractDataMsg {
            req_id,
            contract_details,
        }),
        req_id,
    ))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn encode_matching_symbol(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &MatchingSymbol,
) -> Result<DispatchId, EncodeError> {
    buf.push_int(REQ_MATCHING_SYMBOLS);
    buf.push_int(req.req_id);
    buf.push_string(&req.pattern);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_symbol_sample_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let count = buf.read_int()?;

    let mut contract_descriptions = Vec::new();

    for _ in 0..count {
        let mut contract: Contract = Default::default();
        contract.con_id = buf.read_int()?;
        contract.symbol = buf.read_string()?;
        contract.sec_type = buf.read_string()?;
        contract.primary_exch = buf.read_string()?;
        contract.currency = buf.read_string()?;

        // read derivative sec types list
        let types_count = buf.read_int()?;
        let mut derivative_sec_types = Vec::new();
        for _ in 0..types_count {
            derivative_sec_types.push(buf.read_string()?);
        }
        let description = ContractDescription {
            contract,
            derivative_sec_types,
        };

        contract_descriptions.push(description);
    }

    Ok((
        Response::SymbolSamplesMsg(SymbolSamplesMsg {
            req_id,
            contract_descriptions,
        }),
        req_id,
    ))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn encode_req_sec_def_opt_params(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqSecDefOptParams,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_SEC_DEF_OPT_PARAMS_REQ {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_SEC_DEF_OPT_PARAMS_REQ,
        ));
    }

    buf.push_int(REQ_SEC_DEF_OPT_PARAMS);
    buf.push_int(req.req_id);
    buf.push_string(&req.underlying_symbol);
    buf.push_string(&req.fut_fop_exchange);
    buf.push_string(&req.underlying_sec_type);
    buf.push_int(req.underlying_con_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_security_definition_optional_parameter_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;

    Ok((
        Response::SecurityDefinitionOptionalParameterEndMsg(
            SecurityDefinitionOptionalParameterEndMsg { req_id },
        ),
        req_id,
    ))
}

pub fn decode_security_definition_optional_parameter_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let exchange = buf.read_string()?;
    let underlying_con_id = buf.read_int()?;
    let trading_class = buf.read_string()?;
    let multiplier = buf.read_string()?;
    let expirations_size = buf.read_int()?;
    let mut expirations = HashSet::new();
    let mut strikes = HashSet::<NotNaN<f64>>::new();

    for _ in 0..expirations_size {
        //TODO insert return bool
        expirations.insert(buf.read_string()?);
    }

    let strikes_size = buf.read_int()?;

    for _ in 0..strikes_size {
        //TODO insert return bool
        let f = buf.read_double()?;
        let not_nan_result = NotNaN::new(f);
        let not_nan = match not_nan_result {
            Ok(v) => v,
            Err(_) => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "not nan error",
            )),
        };
        strikes.insert(not_nan);
    }

    Ok((
        Response::SecurityDefinitionOptionalParameterMsg(SecurityDefinitionOptionalParameterMsg {
            req_id,
            exchange,
            underlying_con_id,
            trading_class,
            multiplier,
            expirations,
            strikes,
        }),
        req_id,
    ))
}

pub fn decode_delta_neutral_validation_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;

    let con_id = buf.read_int()?;
    let delta = buf.read_double()?;
    let price = buf.read_double()?;

    let delta_neutral_contract = DeltaNeutralContract {
        con_id,
        delta,
        price,
    };

    Ok((
        Response::DeltaNeutralValidationMsg(DeltaNeutralValidationMsg {
            req_id,
            delta_neutral_contract,
        }),
        req_id,
    ))
}

// helper
