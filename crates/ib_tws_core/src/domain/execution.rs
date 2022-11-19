use std::default::Default;
use std::io;

#[derive(Debug, Clone, Default)]
pub struct OrderState {
    pub status: String,
    pub init_margin_before: String,
    pub maint_margin_before: String,
    pub equity_with_loan_before: String,
    pub init_margin_change: String,
    pub maint_margin_change: String,
    pub equity_with_loan_change: String,
    pub init_margin_after: String,
    pub maint_margin_after: String,
    pub equity_with_loan_after: String,
    pub commission: f64,
    pub min_commission: f64,
    pub max_commission: f64,
    pub commission_currency: String,
    pub warning_text: String,
}

#[derive(Debug, Clone)]
pub struct CommissionReport {
    pub exec_id: String,
    pub commission: f64,
    pub currency: String,
    pub realized_pnl: f64,
    pub yield_value: f64,
    pub yield_redemption_date: i32,
}

#[derive(Debug, Clone)]
#[repr(i32)]
pub enum Liquidities {
    None,
    Added,
    Removed,
    RoudedOut,
}

impl Liquidities {
    pub fn from_code(v: i32) -> Result<Liquidities, io::Error> {
        match v {
            0 => Ok(Liquidities::None),
            1 => Ok(Liquidities::Added),
            2 => Ok(Liquidities::Removed),
            3 => Ok(Liquidities::RoudedOut),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unknown liquidities code",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Execution {
    pub order_id: i32,
    pub client_id: i32,
    pub exec_id: String,
    pub time: String,
    pub acct_number: String,
    pub exchange: String,
    pub side: String,
    pub shares: f64,
    pub price: f64,
    pub perm_id: i32,
    pub liquidation: i32,
    pub cum_qty: f64,
    pub avg_price: f64,
    pub order_ref: String,
    pub ev_rule: String,
    pub ev_multiplier: f64,
    pub model_code: String,
    pub last_liquidity: Liquidities,
}

#[derive(Debug, Clone)]
pub struct ExecutionFilter {
    pub client_id: i32,
    // zero means no filtering on this field
    pub acct_code: String,
    pub time: String,
    pub symbol: String,
    pub sec_type: String,
    pub exchange: String,
    pub side: String,
}

impl Default for Execution {
    fn default() -> Self {
        Execution {
            order_id: 0,
            client_id: 0,
            exec_id: "".to_string(),
            time: "".to_string(),
            acct_number: "".to_string(),
            exchange: "".to_string(),
            side: "".to_string(),
            shares: 0.0,
            price: 0.0,
            perm_id: 0,
            liquidation: 0,
            cum_qty: 0.0,
            avg_price: 0.0,
            order_ref: "".to_string(),
            ev_rule: "".to_string(),
            ev_multiplier: 0.0,
            model_code: "".to_string(),
            last_liquidity: Liquidities::None,
        }
    }
}
