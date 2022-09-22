use domain::tag_value::TagValue;
use std::default::Default;
use std::io;

#[derive(Debug, Clone, Default)]
pub struct DeltaNeutralContract {
    pub con_id: i32,
    pub delta: f64,
    pub price: f64,
}

impl DeltaNeutralContract {
    pub fn new(con_id: i32, delta: f64, price: f64) -> Self {
        DeltaNeutralContract {
            con_id,
            delta,
            price,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComboLeg {
    pub con_id: i32,
    pub ratio: i32,
    pub action: String,
    // BUY/SELL/SSHORT/SSHORTX
    pub exchange: String,
    pub open_close: i32,
    // for stock legs when doing short sale
    pub short_sale_slot: i32,
    // 1 = clearing broker, 2 = third party
    pub designated_location: String,
    pub exempt_code: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Contract {
    pub con_id: i32,
    pub symbol: String,
    pub sec_type: String,
    pub last_trade_date_or_contract_month: String,
    pub strike: f64,
    pub right: String,
    pub multiplier: String,
    // should be double
    pub exchange: String,
    pub primary_exch: String,
    // pick a non-aggregate (ie not the SMART exchange) exchange that the contract trades on.  DO NOT SET TO SMART.
    pub currency: String,
    pub local_symbol: String,
    pub trading_class: String,
    pub sec_id_type: String,
    // CUSIP;SEDOL;ISIN;RIC
    pub sec_id: String,

    pub delta_neutral_contract: Option<DeltaNeutralContract>,
    pub include_expired: bool,
    // can not be set to true for orders
    // COMBOS
    pub combo_legs_descrip: String,
    // received in open order version 14 and up for all combos
    pub combo_legs: Vec<ComboLeg>,
}

impl Contract {
    pub fn new() -> Self {
        Contract {
            ..Default::default()
        }
    }

    pub fn is_combo(&self) -> bool {
        !self.combo_legs.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct ContractDescription {
    pub contract: Contract,
    pub derivative_sec_types: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ContractDetails {
    pub contract: Contract,
    pub market_name: String,
    pub min_tick: f64,
    pub price_magnifier: i32,
    pub order_types: String,
    pub valid_exchanges: String,
    pub under_con_id: i32,
    pub long_name: String,
    pub contract_month: String,
    pub industry: String,
    pub category: String,
    pub sub_category: String,
    pub timezone_id: String,
    pub trading_hours: String,
    pub liquid_hours: String,
    pub ev_rule: String,
    pub ev_multiplier: f64,
    pub md_size_multiplier: i32,
    pub sec_id_list: Vec<TagValue>,
    // CUSIP/ISIN/etc.
    pub agg_group: i32,
    pub under_symbol: String,
    pub under_sec_type: String,
    pub market_rule_ids: String,
    pub real_expiration_date: String,
    pub last_trade_time: String,

    // BOND values
    pub cusip: String,
    pub ratings: String,
    pub desc_append: String,
    pub bond_type: String,
    pub coupon_type: String,
    pub callable: bool,
    pub putable: bool,
    pub coupon: f64,
    pub convertible: bool,
    pub maturity: String,
    pub issue_date: String,
    pub next_option_date: String,
    pub next_option_type: String,
    pub next_option_partial: bool,
    pub notes: String,
}

impl Contract {
    pub fn new_forex(symbol: &str) -> Result<Contract, io::Error> {
        let vec: Vec<&str> = symbol.split(|c| c == '/' || c == '.').collect();
        if vec.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "parse forex symbol error",
            ));
        }
        let mut contract = Contract::new();
        contract.symbol = vec[0].to_string();
        contract.currency = vec[1].to_string();
        contract.sec_type = "CASH".to_string();
        contract.exchange = "IDEALPRO".to_string();
        Ok(contract)
    }

    pub fn new_stock(symbol:&str, exchange:&str, currency: &str) ->Result<Contract, io::Error> {
        let mut contract = Contract::new();
        contract.symbol = symbol.to_string();
        contract.currency = currency.to_string();
        //In the API side, NASDAQ is always defined as ISLAND
        contract.exchange = exchange.to_string();
        contract.sec_type = "STK".to_string();
        Ok(contract)
    }
}
