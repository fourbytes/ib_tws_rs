use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::*;
use std::io;

// Starts a subscription to market scan results based on the provided parameters.
pub fn encode_req_scanner_subscription(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqScannerSubscription,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 4;

    buf.push_int(REQ_SCANNER_SUBSCRIPTION);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);
    buf.push_int_max(req.subscribe.number_of_rows);
    buf.push_string(&req.subscribe.instrument);
    buf.push_string(&req.subscribe.location_code);
    buf.push_string(&req.subscribe.scan_code);
    buf.push_double_max(req.subscribe.above_price);
    buf.push_double_max(req.subscribe.below_price);
    buf.push_int_max(req.subscribe.above_volume);
    buf.push_double_max(req.subscribe.market_cap_above);
    buf.push_double_max(req.subscribe.market_cap_below);
    buf.push_string(&req.subscribe.moody_rating_above);
    buf.push_string(&req.subscribe.moody_rating_below);
    buf.push_string(&req.subscribe.sp_rating_above);
    buf.push_string(&req.subscribe.sp_rating_below);
    buf.push_string(&req.subscribe.maturity_date_above);
    buf.push_string(&req.subscribe.maturity_date_below);
    buf.push_double_max(req.subscribe.coupon_rate_above);
    buf.push_double_max(req.subscribe.coupon_rate_below);
    buf.push_string(&req.subscribe.exclude_convertible);
    buf.push_int_max(req.subscribe.average_option_volume_above);
    buf.push_string(&req.subscribe.scanner_setting_pairs);
    buf.push_string(&req.subscribe.stock_type_filter);

    encode_tagvalue_as_string(buf, &req.options);

    Ok(DispatchId::Multi(req.req_id))
}

pub fn encode_cancel_scanner_subscription(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelScannerSubscription,
) -> Result<DispatchId, EncodeError> {
    // no response
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_SCANNER_SUBSCRIPTION);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_scanner_data_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let count = buf.read_int()?;
    let mut data_vec = Vec::new();
    for _ in 0..count {
        let rank = buf.read_int()?;
        let mut contract_details: ContractDetails = Default::default();
        if version >= 3 {
            contract_details.contract.con_id = buf.read_int()?;
        }
        contract_details.contract.symbol = buf.read_string()?;
        contract_details.contract.sec_type = buf.read_string()?;
        contract_details.contract.last_trade_date_or_contract_month = buf.read_string()?;
        contract_details.contract.strike = buf.read_double()?;
        contract_details.contract.right = buf.read_string()?;
        contract_details.contract.exchange = buf.read_string()?;
        contract_details.contract.currency = buf.read_string()?;
        contract_details.contract.local_symbol = buf.read_string()?;
        contract_details.market_name = buf.read_string()?;
        contract_details.contract.trading_class = buf.read_string()?;
        let distance = buf.read_string()?;
        let benchmark = buf.read_string()?;
        let projection = buf.read_string()?;
        let mut legs = "".to_string();
        if version >= 2 {
            legs = buf.read_string()?;
        };

        data_vec.push(ScannerData {
            rank,
            contract_details,
            distance,
            benchmark,
            projection,
            legs,
        });
    }
    Ok((
        Response::ScannerDataMsg(ScannerDataMsg {
            req_id,
            datas: data_vec,
        }),
        req_id,
    ))
}

// Requests an XML list of scanner parameters valid in TWS.
// Not all parameters are valid from API scanner.
//
// Response: decode_scanner_parameters_msg
pub fn encode_req_scanner_parameters(
    ctx: &mut Context,
    buf: &mut BytesMut,
    _req: &ReqScannerParameters,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_SCANNER_PARAMETERS);
    buf.push_int(VERSION);

    Ok(DispatchId::Global(OPCODE_REQ_SCANNER_PARAMETERS))
}

pub fn decode_scanner_parameters_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let xml = buf.read_string()?;
    Ok((
        Response::ScannerParametersMsg(ScannerParametersMsg { xml }),
        OPCODE_REQ_SCANNER_PARAMETERS,
    ))
}
