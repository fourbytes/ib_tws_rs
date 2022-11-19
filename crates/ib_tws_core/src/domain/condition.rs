//use bytes::{Buf, BufMut, Bytes, BytesMut};
//use encoder::buf::TwsEncoder;

use std::convert::From;
use std::{f64, i32};
// PriceConditionTriggerMode

pub const PTM_DEFAULT: i32 = 0;
pub const PTM_DOUBLE_ASK_BID: i32 = 1;
pub const PTM_LAST: i32 = 2;
pub const PTM_DOUBLE_LAST: i32 = 3;
pub const PTM_BID_ASK: i32 = 4;
pub const PTM_LAST_OF_BID_ASK: i32 = 7;
pub const PTM_MID_POINT: i32 = 8;

#[derive(Debug, Clone)]
pub struct PriceCondition {
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub conid: i32,
    pub exchange: String,
    pub price: f64,
    pub trigger_mode: i32,
}

#[derive(Debug, Clone)]
pub struct TimeCondition {
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub time: String,
}

#[derive(Debug, Clone)]
pub struct MarginCondition {
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub percent: i32,
}

#[derive(Debug, Clone)]
pub struct ExecutionCondition {
    // inherit orderCondition
    pub is_conjunction_connection: bool,
    pub sec_type: String,
    pub exchange: String,
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct VolumeCondition {
    // inherit ContractCondition
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub conid: i32,
    pub exchange: String,
    pub volume: i32,
}

#[derive(Debug, Clone)]
pub struct PercentChangeCondition {
    // inherit ContractCondition
    pub is_conjunction_connection: bool,
    pub is_more: bool,
    pub conid: i32,
    pub exchange: String,
    pub change_percent: f64,
}

#[derive(Debug, Clone)]
pub enum OrderCondition {
    PriceCondition(PriceCondition),
    TimeCondition(TimeCondition),
    MarginCondition(MarginCondition),
    ExecutionCondition(ExecutionCondition),
    VolumeCondition(VolumeCondition),
    PercentChangeCondition(PercentChangeCondition),
}

impl OrderCondition {
    pub fn type_val(&self) -> i32 {
        match self {
            OrderCondition::PriceCondition(_) => 1,
            OrderCondition::TimeCondition(_) => 3,
            OrderCondition::MarginCondition(_) => 4,
            OrderCondition::ExecutionCondition(_) => 5,
            OrderCondition::VolumeCondition(_) => 6,
            OrderCondition::PercentChangeCondition(_) => 7,
        }
    }
}

impl From<OrderCondition> for i32 {
    fn from(condition: OrderCondition) -> i32 {
        match condition {
            OrderCondition::PriceCondition(_) => 1,
            OrderCondition::TimeCondition(_) => 3,
            OrderCondition::MarginCondition(_) => 4,
            OrderCondition::ExecutionCondition(_) => 5,
            OrderCondition::VolumeCondition(_) => 6,
            OrderCondition::PercentChangeCondition(_) => 7,
        }
    }
}
