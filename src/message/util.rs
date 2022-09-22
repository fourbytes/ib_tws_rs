use super::error::EncodeError;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::{Contract, ContractDetails, TagValue};
use std::io;

pub fn encode_contract(buf: &mut BytesMut, contract: &Contract) {
    buf.push_int(contract.con_id);

    buf.push_string(&contract.symbol);
    buf.push_string(&contract.sec_type);
    buf.push_string(&contract.last_trade_date_or_contract_month);
    buf.push_double(contract.strike);
    buf.push_string(&contract.right);
    buf.push_string(&contract.multiplier);
    buf.push_string(&contract.exchange);
    buf.push_string(&contract.primary_exch);
    buf.push_string(&contract.currency);
    buf.push_string(&contract.local_symbol);

    buf.push_string(&contract.trading_class);
}

pub fn read_last_trade_date(
    buf: &mut BytesMut,
    contract: &mut ContractDetails,
    is_bond: bool,
) -> Result<(), io::Error> {
    let last_trade_date_or_contract_month = buf.read_string()?;
    if !last_trade_date_or_contract_month.is_empty() {
        let splitted: Vec<&str> = last_trade_date_or_contract_month.split("\\s+").collect();
        if !splitted.is_empty() {
            if is_bond {
                contract.maturity = splitted[0].to_string();
            } else {
                contract.contract.last_trade_date_or_contract_month = splitted[0].to_string();
            }
        }
        if splitted.len() > 1 {
            contract.last_trade_time = splitted[1].to_string();
        }
        if is_bond && splitted.len() > 2 {
            contract.timezone_id = splitted[2].to_string();
        }
    }
    Ok(())
}

pub fn encode_contract_without_primary_exch(buf: &mut BytesMut, contract: &Contract) {
    buf.push_int(contract.con_id);

    buf.push_string(&contract.symbol);
    buf.push_string(&contract.sec_type);
    buf.push_string(&contract.last_trade_date_or_contract_month);
    buf.push_double(contract.strike);
    buf.push_string(&contract.right);
    buf.push_string(&contract.multiplier);
    buf.push_string(&contract.exchange);
    buf.push_string(&contract.currency);
    buf.push_string(&contract.local_symbol);

    buf.push_string(&contract.trading_class);
}

pub fn encode_tagvalue_as_string(buf: &mut BytesMut, options: &[TagValue]) {
    let count = options.len() as i32;
    //println!("count: {} content: {:?}", count, options);
    let mut sb = "".to_string();
    if count > 0 {
        for elem in options {
            let s = format!("{}={};", elem.tag, elem.value);
            sb.push_str(&s);
        }
    }
    buf.push_string(&sb)
}
