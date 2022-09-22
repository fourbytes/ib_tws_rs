use domain::condition::OrderCondition;
use domain::soft_dollar_tier::SoftDollarTier;
use domain::tag_value::TagValue;
use std::fmt;
use std::str::FromStr;
use std::{f64, i32};

#[derive(Debug, Clone)]
pub enum OrderStatus {
    ApiPending,
    ApiCancelled,
    PreSubmitted,
    PendingCancel,
    Cancelled,
    Submitted,
    Filled,
    Inactive,
    PendingSubmit,
    Unknown,
}

impl OrderStatus {
    pub fn is_active(&self) -> bool {
        match self {
            OrderStatus::PreSubmitted => true,
            OrderStatus::PendingCancel => true,
            OrderStatus::Submitted => true,
            OrderStatus::PendingSubmit => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum OrderType {
    None,
    MKT,
    LMT,
    STP,
    STP_LMT,
    REL,
    TRAIL,
    BOX_TOP,
    FIX_PEGGED,
    LIT,
    LMT_PLUS_MKT,
    LOC,
    MIT,
    MKT_PRT,
    MOC,
    MTL,
    PASSV_REL,
    PEG_BENCH,
    PEG_MID,
    PEG_MKT,
    PEG_PRIM,
    PEG_STK,
    REL_PLUS_LMT,
    REL_PLUS_MKT,
    SNAP_MID,
    SNAP_MKT,
    SNAP_PRIM,
    STP_PRT,
    TRAIL_LIMIT,
    TRAIL_LIT,
    TRAIL_LMT_PLUS_MKT,
    TRAIL_MIT,
    TRAIL_REL_PLUS_MKT,
    VOL,
    VWAP,
    QUOTE,
    PEG_PRIM_VOL,
    PEG_MID_VOL,
    PEG_MKT_VOL,
    PEG_SRF_VOL,
}

impl FromStr for OrderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(OrderType::None),
            "MKT" => Ok(OrderType::MKT),
            "LMT" => Ok(OrderType::LMT),
            "STP" => Ok(OrderType::STP),
            "STP LMT" => Ok(OrderType::STP_LMT),
            "REL" => Ok(OrderType::REL),
            "TRAIL" => Ok(OrderType::TRAIL),
            "BOX TOP" => Ok(OrderType::BOX_TOP),
            "FIX PEGGED" => Ok(OrderType::FIX_PEGGED),
            "LIT" => Ok(OrderType::LIT),
            "LMT + MKT" => Ok(OrderType::LMT_PLUS_MKT),
            "LOC" => Ok(OrderType::LOC),
            "MIT" => Ok(OrderType::MIT),
            "MKT PRT" => Ok(OrderType::MKT_PRT),
            "MOC" => Ok(OrderType::MOC),
            "MTL" => Ok(OrderType::MTL),
            "PASSV REL" => Ok(OrderType::PASSV_REL),
            "PEG BENCH" => Ok(OrderType::PEG_BENCH),
            "PEG MID" => Ok(OrderType::PEG_MID),
            "PEG MKT" => Ok(OrderType::PEG_MKT),
            "PEG PRIM" => Ok(OrderType::PEG_PRIM),
            "PEG STK" => Ok(OrderType::PEG_STK),
            "REL + LMT" => Ok(OrderType::REL_PLUS_LMT),
            "REL + MKT" => Ok(OrderType::REL_PLUS_MKT),
            "SNAP MID" => Ok(OrderType::SNAP_MID),
            "SNAP MKT" => Ok(OrderType::SNAP_MKT),
            "SNAP PRIM" => Ok(OrderType::SNAP_PRIM),
            "STP PRT" => Ok(OrderType::STP_PRT),
            "TRAIL LIMIT" => Ok(OrderType::TRAIL_LIMIT),
            "TRAIL LIT" => Ok(OrderType::TRAIL_LIT),
            "TRAIL LMT + MKT" => Ok(OrderType::TRAIL_LMT_PLUS_MKT),
            "TRAIL MIT" => Ok(OrderType::TRAIL_MIT),
            "TRAIL_REL + MKT" => Ok(OrderType::TRAIL_REL_PLUS_MKT),
            "VOL" => Ok(OrderType::VOL),
            "VWAP" => Ok(OrderType::VWAP),
            "QUOTE" => Ok(OrderType::QUOTE),
            "PPV" => Ok(OrderType::PEG_PRIM_VOL),
            "PDV" => Ok(OrderType::PEG_MID_VOL),
            "PMV" => Ok(OrderType::PEG_MKT_VOL),
            "PSV" => Ok(OrderType::PEG_SRF_VOL),
            _ => Err(()),
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderType::None => write!(f, ""),
            OrderType::MKT => write!(f, "MKT"),
            OrderType::LMT => write!(f, "LMT"),
            OrderType::STP => write!(f, "STP"),
            OrderType::STP_LMT => write!(f, "STP LMT"),
            OrderType::REL => write!(f, "REL"),
            OrderType::TRAIL => write!(f, "TRAIL"),
            OrderType::BOX_TOP => write!(f, "BOX TOP"),
            OrderType::FIX_PEGGED => write!(f, "FIX PEGGED"),
            OrderType::LIT => write!(f, "LIT"),
            OrderType::LMT_PLUS_MKT => write!(f, "LMT + MKT"),
            OrderType::LOC => write!(f, "LOC"),
            OrderType::MIT => write!(f, "MIT"),
            OrderType::MKT_PRT => write!(f, "MKT PRT"),
            OrderType::MOC => write!(f, "MOC"),
            OrderType::MTL => write!(f, "MTL"),
            OrderType::PASSV_REL => write!(f, "PASSV REL"),
            OrderType::PEG_BENCH => write!(f, "PEG BENCH"),
            OrderType::PEG_MID => write!(f, "PEG MID"),
            OrderType::PEG_MKT => write!(f, "PEG MKT"),
            OrderType::PEG_PRIM => write!(f, "PEG PRIM"),
            OrderType::PEG_STK => write!(f, "PEG STK"),
            OrderType::REL_PLUS_LMT => write!(f, "REL + LMT"),
            OrderType::REL_PLUS_MKT => write!(f, "REL + MKT"),
            OrderType::SNAP_MID => write!(f, "SNAP MID"),
            OrderType::SNAP_MKT => write!(f, "SNAP MKT"),
            OrderType::SNAP_PRIM => write!(f, "SNAP PRIM"),
            OrderType::STP_PRT => write!(f, "STP PRT"),
            OrderType::TRAIL_LIMIT => write!(f, "TRAIL LIMIT"),
            OrderType::TRAIL_LIT => write!(f, "TRAIL LIT"),
            OrderType::TRAIL_LMT_PLUS_MKT => write!(f, "TRAIL LMT + MKT"),
            OrderType::TRAIL_MIT => write!(f, "TRAIL MIT"),
            OrderType::TRAIL_REL_PLUS_MKT => write!(f, "TRAIL_REL + MKT"),
            OrderType::VOL => write!(f, "VOL"),
            OrderType::VWAP => write!(f, "VWAP"),
            OrderType::QUOTE => write!(f, "QUOTE"),
            OrderType::PEG_PRIM_VOL => write!(f, "PPV"),
            OrderType::PEG_MID_VOL => write!(f, "PDV"),
            OrderType::PEG_MKT_VOL => write!(f, "PMV"),
            OrderType::PEG_SRF_VOL => write!(f, "PSV"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderComboLeg {
    pub price: f64,
}

#[derive(Debug, Clone)]
pub struct Order {
    // order id's
    pub client_id: i32,
    pub order_id: i32,
    pub perm_id: i32,
    pub parent_id: i32, // Parent order Id, to associate Auto STP or TRAIL orders with the original order.

    // primary attributes
    pub action: String,
    pub total_quantity: f64,
    pub display_size: i32,
    pub order_type: String,
    pub lmt_price: f64,
    pub aux_price: f64,
    pub tif: String,

    // Clearing info
    pub account: String,
    pub settling_firm: String,
    pub clearing_account: String,
    pub clearing_intent: String,

    // secondary attributes
    pub all_or_none: bool,
    pub block_order: bool,
    pub hidden: bool,
    pub outside_rth: bool,
    pub sweep_to_fill: bool,
    pub percent_offset: f64,
    pub trailing_percent: f64,
    pub trail_stop_price: f64,
    pub min_qty: i32,
    pub good_after_time: String,
    pub good_till_date: String,
    pub oca_group: String,
    pub order_ref: String,
    pub rule_80a: String,
    pub oca_type: i32,
    pub trigger_method: i32,

    // extended order fields
    pub active_start_time: String,
    pub active_stop_time: String,

    // advisor allocation orders
    pub fa_group: String,
    pub fa_method: String,
    pub fa_percentage: String,
    pub fa_profile: String,

    // volatility orders
    pub volatility: f64,
    pub volatility_type: i32,
    pub continuous_update: i32,
    pub reference_price_type: i32,
    pub delta_neutral_order_type: String,
    pub delta_neutral_aux_price: f64,
    pub delta_neutral_con_id: i32,
    pub delta_neutral_open_close: String,
    pub delta_neutral_short_sale: bool,
    pub delta_neutral_short_sale_slot: i32,
    pub delta_neutral_designated_location: String,

    // scale orders
    pub scale_init_level_size: i32,
    pub scale_subs_level_size: i32,
    pub scale_price_increment: f64,
    pub scale_price_adjust_value: f64,
    pub scale_price_adjust_interval: i32,
    pub scale_profit_offset: f64,
    pub scale_auto_reset: bool,
    pub scale_init_position: i32,
    pub scale_init_fill_qty: i32,
    pub scale_random_percent: bool,
    pub scale_table: String,

    // hedge orders
    pub hedge_type: String,
    pub hedge_param: String,

    // algo orders
    pub algo_strategy: String,
    pub algo_params: Vec<TagValue>,
    pub algo_id: String,

    // combo orders
    pub smart_combo_routing_params: Vec<TagValue>,
    pub order_combo_legs: Vec<OrderComboLeg>,

    // processing control
    pub what_if: bool,
    pub transmit: bool,
    pub override_percentage_constraints: bool,

    // institutional order only
    pub open_close: String,
    pub origin: i32,
    pub short_sale_slot: i32,
    pub designated_location: String,
    pub exempt_code: i32,
    pub delta_neutral_settling_firm: String,
    pub delta_neutral_clearing_account: String,
    pub delta_neutral_clearing_intent: String,

    // SMART routing only
    pub discretionary_amt: f64,
    pub etrade_only: bool,
    pub firm_quote_only: bool,
    pub nbbo_price_cap: f64,
    pub opt_out_smart_routing: bool,

    // BOX or VOL ORDERS ONLY
    pub auction_strategy: i32,

    // BOX ORDERS ONLY
    pub starting_price: f64,
    pub stock_ref_price: f64,
    pub delta: f64,

    // pegged to stock or VOL orders
    pub stock_range_lower: f64,
    pub stock_range_upper: f64,

    // COMBO ORDERS ONLY
    pub basis_points: f64,
    pub basis_points_type: i32,

    // Not Held
    pub not_held: bool,

    // order misc options
    pub order_misc_options: Vec<TagValue>,

    //order algo id
    pub solicited: bool,

    pub randomize_size: bool,
    pub randomize_price: bool,

    //VER PEG2BENCH fields:
    pub reference_contract_id: i32,
    pub pegged_change_amount: f64,
    pub is_pegged_change_amount_decrease: bool,
    pub reference_change_amount: f64,
    pub reference_exchange_id: String,
    pub adjusted_order_type: String,
    pub trigger_price: f64,
    pub adjusted_stop_price: f64,
    pub adjusted_stop_limit_price: f64,
    pub adjusted_trailing_amount: f64,
    pub adjustable_trailing_unit: i32,
    pub lmt_price_offset: f64,

    pub conditions: Vec<OrderCondition>,
    pub conditions_cancel_order: bool,
    pub conditions_ignore_rth: bool,

    // models
    pub model_code: String,
    pub ext_operator: String,
    pub soft_dollar_tier: SoftDollarTier,

    // native cash quantity
    pub cash_qty: f64,
    pub mifid2_decision_maker: String,
    pub mified2_decision_algo: String,
    pub mified2_execution_trader: String,
    pub mified2_execution_algo: String,

    // don't use auto price for hedge
    pub dont_use_auto_price_for_hedge: bool,
}

impl Default for Order {
    fn default() -> Order {
        Order {
            client_id: 0,
            order_id: 0,
            perm_id: 0,
            parent_id: 0,

            // primary attributes
            action: "BUY".to_string(),
            total_quantity: 0.0,
            display_size: 0,
            order_type: "LMT".to_string(),
            lmt_price: f64::MAX,
            aux_price: f64::MAX,
            tif: "DAY".to_string(),

            // Clearing info
            account: "".to_string(), // IB account
            settling_firm: "".to_string(),
            clearing_account: "".to_string(), // True beneficiary of the order
            clearing_intent: "".to_string(),  // "" (Default), "IB", "Away", "PTA" (PostTrade)

            // secondary attributes
            all_or_none: false,
            block_order: false,
            hidden: false,
            outside_rth: false,
            sweep_to_fill: false,
            percent_offset: f64::MAX,
            trailing_percent: f64::MAX,
            trail_stop_price: f64::MAX,
            min_qty: i32::MAX,
            good_after_time: "".to_string(), // FORMAT: 20060505 08:00:00 EST
            good_till_date: "".to_string(),  // FORMAT: 20060505 08:00:00 EST or 20060505
            oca_group: "".to_string(),       // one cancels all group name
            order_ref: "".to_string(),
            rule_80a: "".to_string(),
            oca_type: 0,
            trigger_method: 0,

            // extended order fields
            active_start_time: "".to_string(),
            active_stop_time: "".to_string(),

            // advisor allocation orders
            fa_group: "".to_string(),
            fa_method: "".to_string(),
            fa_percentage: "".to_string(),
            fa_profile: "".to_string(),

            // volatility orders
            volatility: f64::MAX,
            volatility_type: i32::MAX,
            continuous_update: 0,
            reference_price_type: i32::MAX,
            delta_neutral_order_type: "".to_string(),
            delta_neutral_aux_price: f64::MAX,
            delta_neutral_con_id: 0,
            delta_neutral_open_close: "".to_string(),
            delta_neutral_short_sale: false,
            delta_neutral_short_sale_slot: 0,
            delta_neutral_designated_location: "".to_string(),

            // scale orders
            scale_init_level_size: i32::MAX,
            scale_subs_level_size: i32::MAX,
            scale_price_increment: f64::MAX,
            scale_price_adjust_value: f64::MAX,
            scale_price_adjust_interval: i32::MAX,
            scale_profit_offset: f64::MAX,
            scale_auto_reset: false,
            scale_init_position: i32::MAX,
            scale_init_fill_qty: i32::MAX,
            scale_random_percent: false,
            scale_table: "".to_string(),

            // hedge orders
            hedge_type: "".to_string(),
            hedge_param: "".to_string(), // beta value for beta hedge (in range 0-1), ratio for pair hedge

            // algo orders
            algo_strategy: "".to_string(),
            algo_params: vec![],
            algo_id: "".to_string(),

            // combo orders
            smart_combo_routing_params: vec![],
            order_combo_legs: vec![],

            // processing control
            what_if: false,
            transmit: true, // if false, order will be sent to TWS but not transmitted to server
            override_percentage_constraints: false,

            // institutional order only
            open_close: "O".to_string(),         // O=Open, C=Close
            origin: 0, // 0 CUSTOM 1 firm                      // 0=Customer, 1=Firm
            short_sale_slot: 0, // 1 if you hold the shares, 2 if they will be delivered from elsewhere.  Only for Action="SSHORT
            designated_location: "".to_string(), // set when slot=2 only.
            exempt_code: -1,
            delta_neutral_settling_firm: "".to_string(),
            delta_neutral_clearing_account: "".to_string(),
            delta_neutral_clearing_intent: "".to_string(),

            // SMART routing only
            discretionary_amt: f64::MAX,
            etrade_only: false,
            firm_quote_only: false,
            nbbo_price_cap: f64::MAX,
            opt_out_smart_routing: false,

            // BOX or VOL ORDERS ONLY
            auction_strategy: 0, // 1=AUCTION_MATCH, 2=AUCTION_IMPROVEMENT, 3=AUCTION_TRANSPARENT

            // BOX ORDERS ONLY
            starting_price: f64::MAX,
            stock_ref_price: f64::MAX,
            delta: f64::MAX,

            // pegged to stock or VOL orders
            stock_range_lower: f64::MAX,
            stock_range_upper: f64::MAX,

            // COMBO ORDERS ONLY
            basis_points: f64::MAX,      // EFP orders only, download only
            basis_points_type: i32::MAX, // EFP orders only, download only

            // Not Held
            not_held: false,

            // order misc options
            order_misc_options: vec![],

            //order algo id
            solicited: false,

            randomize_size: false,
            randomize_price: false,

            //VER PEG2BENCH fields:
            reference_contract_id: 0,
            pegged_change_amount: 0.0,
            is_pegged_change_amount_decrease: false,
            reference_change_amount: 0.0,
            reference_exchange_id: "".to_string(),
            adjusted_order_type: "".to_string(),
            trigger_price: f64::MAX,
            adjusted_stop_price: f64::MAX,
            adjusted_stop_limit_price: f64::MAX,
            adjusted_trailing_amount: f64::MAX,
            adjustable_trailing_unit: 0,
            lmt_price_offset: f64::MAX,

            conditions: vec![],
            conditions_cancel_order: false,
            conditions_ignore_rth: false,

            // models
            model_code: "".to_string(),
            ext_operator: "".to_string(),
            soft_dollar_tier: SoftDollarTier::new("", "", ""),

            // native cash quantity
            cash_qty: f64::MAX,
            mifid2_decision_maker: "".to_string(),
            mified2_decision_algo: "".to_string(),
            mified2_execution_trader: "".to_string(),
            mified2_execution_algo: "".to_string(),

            // don't use auto price for hedge
            dont_use_auto_price_for_hedge: false,
        }
    }
}
