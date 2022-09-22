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
use std::{f64, i32};
use std::io;

// [NO REQ_ID]
pub fn decode_open_order_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;

    let mut order: Order = Default::default();
    order.order_id = buf.read_int()?;

    let mut contract: Contract = Default::default();
    if version >= 17 {
        contract.con_id = buf.read_int()?;
    }
    contract.symbol = buf.read_string()?;
    contract.sec_type = buf.read_string()?;
    contract.last_trade_date_or_contract_month = buf.read_string()?;
    contract.strike = buf.read_double()?;
    contract.right = buf.read_string()?;
    if version >= 32 {
        contract.multiplier = buf.read_string()?;
    }
    contract.exchange = buf.read_string()?;
    contract.currency = buf.read_string()?;
    if version >= 2 {
        contract.local_symbol = buf.read_string()?;
    }
    if version >= 32 {
        contract.trading_class = buf.read_string()?;
    }

    // read order fields
    order.action = buf.read_string()?;

    order.total_quantity = if ctx.server_version() >= MIN_SERVER_VER_FRACTIONAL_POSITIONS {
        buf.read_double()?
    } else {
        f64::from(buf.read_int()?)
    };

    order.order_type = buf.read_string()?;
    if version < 29 {
        order.lmt_price = buf.read_double()?;
    } else {
        order.lmt_price = buf.read_double_max()?;
    }
    if version < 30 {
        order.aux_price = buf.read_double()?;
    } else {
        order.aux_price = buf.read_double_max()?;
    }
    order.tif = buf.read_string()?;
    order.oca_group = buf.read_string()?;
    order.account = buf.read_string()?;
    order.open_close = buf.read_string()?;
    order.origin = buf.read_int()?;
    order.order_ref = buf.read_string()?;

    if version >= 3 {
        order.client_id = buf.read_int()?;
    }

    if version >= 4 {
        order.perm_id = buf.read_int()?;
        if version < 18 {
            // will never happen
            let _ = buf.read_bool();
        } else {
            order.outside_rth = buf.read_bool()?;
        }
        order.hidden = buf.read_bool()?;
        order.discretionary_amt = buf.read_double()?;
    }

    if version >= 5 {
        order.good_after_time = buf.read_string()?;
    }

    if version >= 6 {
        // skip deprecated sharesAllocation field
        let _ = buf.read_string()?;
    }

    if version >= 7 {
        order.fa_group = buf.read_string()?;
        order.fa_method = buf.read_string()?;
        order.fa_percentage = buf.read_string()?;
        order.fa_profile = buf.read_string()?;
    }

    if ctx.server_version() >= MIN_SERVER_VER_MODELS_SUPPORT {
        order.model_code = buf.read_string()?;
    }

    if version >= 8 {
        order.good_till_date = buf.read_string()?;
    }

    if version >= 9 {
        order.rule_80a = buf.read_string()?;
        order.percent_offset = buf.read_double_max()?;
        order.settling_firm = buf.read_string()?;
        order.short_sale_slot = buf.read_int()?;
        order.designated_location = buf.read_string()?;
        if ctx.server_version() == 51 {
            let _ = buf.read_int()?; // exemptCode
        } else if version >= 23 {
            order.exempt_code = buf.read_int()?;
        }
        order.auction_strategy = buf.read_int()?;
        order.starting_price = buf.read_double_max()?;
        order.stock_ref_price = buf.read_double_max()?;
        order.delta = buf.read_double_max()?;
        order.stock_range_lower = buf.read_double_max()?;
        order.stock_range_upper = buf.read_double_max()?;
        order.display_size = buf.read_int()?;
        if version < 18 {
            // will never happen
            /* order.m_rthOnly = */            let _ = buf.read_bool();
        }
        order.block_order = buf.read_bool()?;
        order.sweep_to_fill = buf.read_bool()?;
        order.all_or_none = buf.read_bool()?;
        order.min_qty = buf.read_int_max()?;
        order.oca_type = buf.read_int()?;
        order.etrade_only = buf.read_bool()?;
        order.firm_quote_only = buf.read_bool()?;
        order.nbbo_price_cap = buf.read_double_max()?;
    }

    if version >= 10 {
        order.parent_id = buf.read_int()?;
        order.trigger_method = buf.read_int()?;
    }

    if version >= 11 {
        order.volatility = buf.read_double_max()?;
        order.volatility_type = buf.read_int()?;
        if version == 11 {
            let received_int = buf.read_int()?;
            order.delta_neutral_order_type = if received_int == 0 {
                "NONE".to_string()
            } else {
                "MKT".to_string()
            };
        } else {
            // version 12 and up
            order.delta_neutral_order_type = buf.read_string()?;
            order.delta_neutral_aux_price = buf.read_double_max()?;

            if version >= 27 && !order.delta_neutral_order_type.is_empty() {
                order.delta_neutral_con_id = buf.read_int()?;
                order.delta_neutral_settling_firm = buf.read_string()?;
                order.delta_neutral_clearing_account = buf.read_string()?;
                order.delta_neutral_clearing_intent = buf.read_string()?;
            }

            if version >= 31 && !order.delta_neutral_order_type.is_empty() {
                order.delta_neutral_open_close = buf.read_string()?;
                order.delta_neutral_short_sale = buf.read_bool()?;
                order.delta_neutral_short_sale_slot = buf.read_int()?;
                order.delta_neutral_designated_location = buf.read_string()?;
            }
        }
        order.continuous_update = buf.read_int()?;
        if ctx.server_version() == 26 {
            order.stock_range_lower = buf.read_double()?;
            order.stock_range_upper = buf.read_double()?;
        }
        order.reference_price_type = buf.read_int()?;
    }

    if version >= 13 {
        order.trail_stop_price = buf.read_double_max()?;
    }

    if version >= 30 {
        order.trailing_percent = buf.read_double_max()?;
    }

    if version >= 14 {
        order.basis_points = buf.read_double_max()?;
        order.basis_points_type = buf.read_int_max()?;
        contract.combo_legs_descrip = buf.read_string()?;
    }

    if version >= 29 {
        let count = buf.read_int()?;
        for _ in 0..count {
            let con_id = buf.read_int()?;
            let ratio = buf.read_int()?;
            let action = buf.read_string()?;
            let exchange = buf.read_string()?;
            let open_close = buf.read_int()?;
            let short_sale_slot = buf.read_int()?;
            let designated_location = buf.read_string()?;
            let exempt_code = buf.read_int()?;

            contract.combo_legs.push(ComboLeg {
                con_id,
                ratio,
                action,
                exchange,
                open_close,
                short_sale_slot,
                designated_location,
                exempt_code,
            });
        }

        let order_combo_legs_count = buf.read_int()?;

        for _ in 0..order_combo_legs_count {
            let price = buf.read_double_max()?;
            order.order_combo_legs.push(OrderComboLeg { price });
        }
    }

    if version >= 26 {
        let count = buf.read_int()?;

        for _ in 0..count {
            let tag = buf.read_string()?;
            let value = buf.read_string()?;
            order
                .smart_combo_routing_params
                .push(TagValue { tag, value });
        }
    }

    if version >= 15 {
        if version >= 20 {
            order.scale_init_level_size = buf.read_int_max()?;
            order.scale_subs_level_size = buf.read_int_max()?;
        } else {
            /* int notSuppScaleNumComponents = */
            let _ = buf.read_int_max()?;
            order.scale_init_level_size = buf.read_int_max()?;
        }
        order.scale_price_increment = buf.read_double_max()?;
    }

    if version >= 28 && order.scale_price_increment > 0.0 && order.scale_price_increment != f64::MAX
    {
        order.scale_price_adjust_value = buf.read_double_max()?;
        order.scale_price_adjust_interval = buf.read_int_max()?;
        order.scale_profit_offset = buf.read_double_max()?;
        order.scale_auto_reset = buf.read_bool()?;
        order.scale_init_position = buf.read_int_max()?;
        order.scale_init_fill_qty = buf.read_int_max()?;
        order.scale_random_percent = buf.read_bool()?;
    }

    if version >= 24 {
        order.hedge_type = buf.read_string()?;
        if !order.hedge_type.is_empty() {
            order.hedge_param = buf.read_string()?;
        }
    }

    if version >= 25 {
        order.opt_out_smart_routing = buf.read_bool()?;
    }

    if version >= 19 {
        order.clearing_account = buf.read_string()?;
        order.clearing_intent = buf.read_string()?;
    }

    if version >= 22 {
        order.not_held = buf.read_bool()?;
    }

    if version >= 20 {
        if buf.read_bool()? {
            let con_id = buf.read_int()?;
            let delta = buf.read_double()?;
            let price = buf.read_double()?;
            contract.delta_neutral_contract = Some(DeltaNeutralContract {
                con_id,
                delta,
                price,
            });
        }
    }

    if version >= 21 {
        order.algo_strategy = buf.read_string()?;
        if !order.algo_strategy.is_empty() {
            let count = buf.read_int()?;

            for _ in 0..count {
                let tag = buf.read_string()?;
                let value = buf.read_string()?;
                order.algo_params.push(TagValue { tag, value });
            }
        }
    }

    if version >= 33 {
        order.solicited = buf.read_bool()?;
    }

    let mut order_state: OrderState = Default::default();

    if version >= 16 {
        order.what_if = buf.read_bool()?;

        order_state.status = buf.read_string()?;

        if ctx.server_version() >= MIN_SERVER_VER_WHAT_IF_EXT_FIELDS {
            order_state.init_margin_before = buf.read_string()?;
            order_state.maint_margin_before = buf.read_string()?;
            order_state.equity_with_loan_before = buf.read_string()?;
            order_state.init_margin_change = buf.read_string()?;
            order_state.maint_margin_change = buf.read_string()?;
            order_state.equity_with_loan_change = buf.read_string()?;
        }

        order_state.init_margin_after = buf.read_string()?;
        order_state.maint_margin_after = buf.read_string()?;
        order_state.equity_with_loan_after = buf.read_string()?;
        order_state.commission = buf.read_double_max()?;
        order_state.min_commission = buf.read_double_max()?;
        order_state.max_commission = buf.read_double_max()?;
        order_state.commission_currency = buf.read_string()?;
        order_state.warning_text = buf.read_string()?;
    }

    if version >= 34 {
        order.randomize_size = buf.read_bool()?;
        order.randomize_price = buf.read_bool()?;
    }

    if ctx.server_version() >= MIN_SERVER_VER_PEGGED_TO_BENCHMARK {
        if order.order_type == OrderType::PEG_BENCH.to_string() {
            order.reference_contract_id = buf.read_int()?;
            order.is_pegged_change_amount_decrease = buf.read_bool()?;
            order.pegged_change_amount = buf.read_double()?;
            order.reference_change_amount = buf.read_double()?;
            order.reference_exchange_id = buf.read_string()?;
        }

        let conditions_count = buf.read_int()?;

        if conditions_count > 0 {
            for _ in 0..conditions_count {
                let order_condition_type = buf.read_int()?;
                let condition = order_condition_read(buf, order_condition_type)?;
                order.conditions.push(condition);
            }

            order.conditions_ignore_rth = buf.read_bool()?;
            order.conditions_cancel_order = buf.read_bool()?;
        }

        order.adjusted_order_type = buf.read_string()?;
        order.trigger_price = buf.read_double_max()?;
        order.trail_stop_price = buf.read_double_max()?;
        order.lmt_price_offset = buf.read_double_max()?;
        order.adjusted_stop_price = buf.read_double_max()?;
        order.adjusted_stop_limit_price = buf.read_double_max()?;
        order.adjusted_trailing_amount = buf.read_double_max()?;
        order.adjustable_trailing_unit = buf.read_int()?;
    }

    if ctx.server_version() >= MIN_SERVER_VER_SOFT_DOLLAR_TIER {
        let name = buf.read_string()?;
        let value = buf.read_string()?;
        let display_name = buf.read_string()?;
        order.soft_dollar_tier = SoftDollarTier {
            name,
            value,
            display_name,
        };
    }

    if ctx.server_version() >= MIN_SERVER_VER_CASH_QTY {
        order.cash_qty = buf.read_double_max()?;
    }

    if ctx.server_version() >= MIN_SERVER_VER_AUTO_PRICE_FOR_HEDGE {
        order.dont_use_auto_price_for_hedge = buf.read_bool()?;
    }

    Ok((
        Response::OpenOrderMsg(OpenOrderMsg {
            order_id: order.order_id,
            contract,
            order,
            order_state,
        }),
        OPCODE_REQ_OPEN_ORDERS,
    ))
}

pub fn decode_order_status_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let version = if ctx.server_version() >= MIN_SERVER_VER_MARKET_CAP_PRICE {
        i32::MAX
    } else {
        buf.read_int()?
    };
    let id = buf.read_int()?;
    let status = buf.read_string()?;

    let filled = if ctx.server_version() >= MIN_SERVER_VER_FRACTIONAL_POSITIONS {
        buf.read_double()?
    } else {
        f64::from(buf.read_int()?)
    };

    let remaining = if ctx.server_version() >= MIN_SERVER_VER_FRACTIONAL_POSITIONS {
        buf.read_double()?
    } else {
        f64::from(buf.read_int()?)
    };

    let avg_fill_price = buf.read_double()?;

    let perm_id =
        if version >= 2 { buf.read_int()? } else { 0 };

    let parent_id =
        if version >= 3 { buf.read_int()? } else { 0 };

    let last_fill_price = if version >= 4 {
        buf.read_double()?
    } else {
        0.0
    };

    let client_id =
        if version >= 5 { buf.read_int()? } else { 0 };

    let why_held = if version >= 6 {
        buf.read_string()?
    } else {
        "".to_string()
    };

    let mkt_cap_price = if ctx.server_version() >= MIN_SERVER_VER_MARKET_CAP_PRICE {
        buf.read_double()?
    } else {
        f64::MAX
    };

    Ok((
        Response::OrderStatusMsg(OrderStatusMsg {
            id,
            status,
            filled,
            remaining,
            avg_fill_price,
            perm_id,
            parent_id,
            last_fill_price,
            client_id,
            why_held,
            mkt_cap_price,
        }),
        id, // TODO: order_id
    ))
}

fn order_condition_read(buf: &mut BytesMut, condition_type: i32) -> Result<OrderCondition, io::Error> {
    match condition_type {
        1 => {
            // PriceCondition
            let is_conjunction_connection = buf.read_string()?.to_lowercase() == "a";
            let is_more = buf.read_bool()?;
            let price = buf.read_double()?;

            let conid = buf.read_int()?;
            let exchange = buf.read_string()?;
            let trigger_mode = buf.read_int()?;
            Ok(OrderCondition::PriceCondition(PriceCondition {
                is_conjunction_connection,
                is_more,
                conid,
                exchange,
                price,
                trigger_mode,
            }))
        }
        3 => {
            // TimeCondition
            let is_conjunction_connection = buf.read_string()?.to_lowercase() == "a";
            let is_more = buf.read_bool()?;
            let time = buf.read_string()?;
            Ok(OrderCondition::TimeCondition(TimeCondition {
                is_conjunction_connection,
                is_more,
                time,
            }))
        }
        4 => {
            // MarginCondition
            let is_conjunction_connection = buf.read_string()?.to_lowercase() == "a";
            let is_more = buf.read_bool()?;
            let percent = buf.read_int()?;
            Ok(OrderCondition::MarginCondition(MarginCondition {
                is_conjunction_connection,
                is_more,
                percent,
            }))
        }
        5 => {
            // ExecutionCondition
            let is_conjunction_connection = buf.read_string()?.to_lowercase() == "a";
            let sec_type = buf.read_string()?;
            let exchange = buf.read_string()?;
            let symbol = buf.read_string()?;
            Ok(OrderCondition::ExecutionCondition(ExecutionCondition {
                is_conjunction_connection,
                sec_type,
                exchange,
                symbol,
            }))
        }
        6 => {
            // VolumeCondition
            let is_conjunction_connection = buf.read_string()?.to_lowercase() == "a";
            let is_more = buf.read_bool()?;
            let volume = buf.read_int()?;

            let conid = buf.read_int()?;
            let exchange = buf.read_string()?;

            Ok(OrderCondition::VolumeCondition(VolumeCondition {
                is_conjunction_connection,
                is_more,
                conid,
                exchange,
                volume,
            }))
        }
        7 => {
            //PercentChangeCondition
            let is_conjunction_connection = buf.read_string()?.to_lowercase() == "a";
            let is_more = buf.read_bool()?;
            let change_percent = buf.read_double()?;

            let conid = buf.read_int()?;
            let exchange = buf.read_string()?;

            Ok(OrderCondition::PercentChangeCondition(
                PercentChangeCondition {
                    is_conjunction_connection,
                    is_more,
                    conid,
                    exchange,
                    change_percent,
                },
            ))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unknown order condition type",
        )),
    }
}

// [NO REQ_ID]
pub fn decode_open_order_end_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    Ok((
        Response::OpenOrderEndMsg(OpenOrderEndMsg {}),
        OPCODE_REQ_OPEN_ORDERS,
    ))
}

fn encoder_order_condition(buf: &mut BytesMut, order_condition: &OrderCondition) {
    match order_condition {
        OrderCondition::PriceCondition(ref pc) => {
            if pc.is_conjunction_connection {
                buf.push_string("a")
            } else {
                buf.push_string("o")
            }
            buf.push_bool(pc.is_more);
            buf.push_double(pc.price); // value
            buf.push_int(pc.conid);
            buf.push_string(&pc.exchange);
            buf.push_int(pc.trigger_mode);
        }
        OrderCondition::TimeCondition(ref tc) => {
            if tc.is_conjunction_connection {
                buf.push_string("a")
            } else {
                buf.push_string("o")
            }
            buf.push_bool(tc.is_more);
            buf.push_string(&tc.time); // value
        }
        OrderCondition::MarginCondition(ref mc) => {
            if mc.is_conjunction_connection {
                buf.push_string("a")
            } else {
                buf.push_string("o")
            }
            buf.push_bool(mc.is_more);
            buf.push_int(mc.percent); // value
        }
        OrderCondition::ExecutionCondition(ref ec) => {
            if ec.is_conjunction_connection {
                buf.push_string("a")
            } else {
                buf.push_string("o")
            }
            buf.push_string(&ec.sec_type);
            buf.push_string(&ec.exchange);
            buf.push_string(&ec.symbol);
        }
        OrderCondition::VolumeCondition(ref vc) => {
            if vc.is_conjunction_connection {
                buf.push_string("a")
            } else {
                buf.push_string("o")
            }
            buf.push_bool(vc.is_more);
            buf.push_int(vc.volume); // value
            buf.push_int(vc.conid);
            buf.push_string(&vc.exchange);
        }
        OrderCondition::PercentChangeCondition(pcc) => {
            if pcc.is_conjunction_connection {
                buf.push_string("a")
            } else {
                buf.push_string("o")
            }
            buf.push_bool(pcc.is_more);
            buf.push_double(pcc.change_percent); // value
            buf.push_int(pcc.conid);
            buf.push_string(&pcc.exchange);
        }
    };
}

pub fn encode_place_order(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &PlaceOrder,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 45;

    buf.push_int(PLACE_ORDER);
    buf.push_int(VERSION);
    buf.push_int(req.id);

    encode_contract(buf, &req.contract);

    buf.push_string(&req.contract.sec_id_type);
    buf.push_string(&req.contract.sec_id);

    buf.push_string(&req.order.action);

    if ctx.server_version() >= MIN_SERVER_VER_FRACTIONAL_POSITIONS {
        buf.push_double(req.order.total_quantity)
    } else {
        buf.push_int(req.order.total_quantity as i32);
    }

    buf.push_string(&req.order.order_type);
    buf.push_double_max(req.order.lmt_price);
    buf.push_double_max(req.order.aux_price);

    buf.push_string(&req.order.tif);
    buf.push_string(&req.order.oca_group);
    buf.push_string(&req.order.account);
    buf.push_string(&req.order.open_close);
    buf.push_int(req.order.origin);
    buf.push_string(&req.order.order_ref);
    buf.push_bool(req.order.transmit);
    buf.push_int(req.order.parent_id);

    buf.push_bool(req.order.block_order);
    buf.push_bool(req.order.sweep_to_fill);
    buf.push_int(req.order.display_size);
    buf.push_int(req.order.trigger_method);
    buf.push_bool(req.order.outside_rth);
    buf.push_bool(req.order.hidden);

    if req.contract.sec_type.to_uppercase() == "BAG" {
        buf.push_int(req.contract.combo_legs.len() as i32);
        for elem in &req.contract.combo_legs {
            buf.push_int(elem.con_id);
            buf.push_int(elem.ratio);
            buf.push_string(&elem.action);
            buf.push_string(&elem.exchange);
            buf.push_int(elem.open_close);
            buf.push_int(elem.short_sale_slot);
            buf.push_string(&elem.designated_location);
            buf.push_int(elem.exempt_code);
        }

        buf.push_int(req.order.order_combo_legs.len() as i32);
        for elem in &req.order.order_combo_legs {
            buf.push_double_max(elem.price);
        }

        buf.push_int(req.order.smart_combo_routing_params.len() as i32);
        for elem in &req.order.smart_combo_routing_params {
            buf.push_string(&elem.tag);
            buf.push_string(&elem.value);
        }
    }

    buf.push_string("");
    buf.push_double(req.order.discretionary_amt);
    buf.push_string(&req.order.good_after_time);
    buf.push_string(&req.order.good_till_date);
    buf.push_string(&req.order.fa_group);
    buf.push_string(&req.order.fa_method);
    buf.push_string(&req.order.fa_percentage);
    buf.push_string(&req.order.fa_profile);

    if ctx.server_version() >= MIN_SERVER_VER_MODELS_SUPPORT {
        buf.push_string(&req.order.model_code);
    }

    buf.push_int(req.order.short_sale_slot);
    buf.push_string(&req.order.designated_location);
    buf.push_int(req.order.exempt_code);

    buf.push_int(req.order.oca_type);
    buf.push_string(&req.order.rule_80a);
    buf.push_string(&req.order.settling_firm);
    buf.push_bool(req.order.all_or_none);
    buf.push_int_max(req.order.min_qty);
    buf.push_double_max(req.order.percent_offset);
    buf.push_bool(req.order.etrade_only);
    buf.push_bool(req.order.firm_quote_only);
    buf.push_double_max(req.order.nbbo_price_cap);
    buf.push_int_max(req.order.auction_strategy);
    buf.push_double_max(req.order.starting_price);
    buf.push_double_max(req.order.stock_ref_price);
    buf.push_double_max(req.order.delta);
    buf.push_double_max(req.order.stock_range_lower);
    buf.push_double_max(req.order.stock_range_upper);
    buf.push_bool(req.order.override_percentage_constraints);
    buf.push_double_max(req.order.volatility);
    buf.push_int(req.order.volatility_type);
    buf.push_string(&req.order.delta_neutral_order_type);
    buf.push_double_max(req.order.delta_neutral_aux_price);

    if !req.order.delta_neutral_order_type.is_empty() {
        buf.push_int(req.order.delta_neutral_con_id);
        buf.push_string(&req.order.delta_neutral_settling_firm);
        buf.push_string(&req.order.delta_neutral_clearing_account);
        buf.push_string(&req.order.delta_neutral_clearing_intent);

        buf.push_string(&req.order.delta_neutral_open_close);
        buf.push_bool(req.order.delta_neutral_short_sale);
        buf.push_int(req.order.delta_neutral_short_sale_slot);
        buf.push_string(&req.order.delta_neutral_designated_location);
    }

    buf.push_int(req.order.continuous_update);
    buf.push_int(req.order.reference_price_type);
    buf.push_double_max(req.order.trail_stop_price);
    buf.push_double_max(req.order.trailing_percent);
    buf.push_int_max(req.order.scale_init_level_size);
    buf.push_int_max(req.order.scale_subs_level_size);
    buf.push_double_max(req.order.scale_price_increment);

    if req.order.scale_price_increment > 0.0 && req.order.scale_price_increment != f64::MAX {
        buf.push_double_max(req.order.scale_price_adjust_value);
        buf.push_int_max(req.order.scale_price_adjust_interval);
        buf.push_double_max(req.order.scale_profit_offset);
        buf.push_bool(req.order.scale_auto_reset);
        buf.push_int_max(req.order.scale_init_position);
        buf.push_int_max(req.order.scale_init_fill_qty);
        buf.push_bool(req.order.scale_random_percent);
    }

    buf.push_string(&req.order.scale_table);
    buf.push_string(&req.order.active_start_time);
    buf.push_string(&req.order.active_stop_time);

    buf.push_string(&req.order.hedge_type);
    if !req.order.hedge_type.is_empty() {
        buf.push_string(&req.order.hedge_param);
    }

    buf.push_bool(req.order.opt_out_smart_routing);
    buf.push_string(&req.order.clearing_account);
    buf.push_string(&req.order.clearing_intent);
    buf.push_bool(req.order.not_held);

    if let Some(ref uc) = req.contract.delta_neutral_contract {
        buf.push_bool(true);
        buf.push_int(uc.con_id);
        buf.push_double(uc.delta);
        buf.push_double(uc.price);
    }

    buf.push_string(&req.order.algo_strategy);
    if !req.order.algo_strategy.is_empty() {
        buf.push_int(req.order.algo_params.len() as i32);
        for elem in &req.order.algo_params {
            buf.push_string(&elem.tag);
            buf.push_string(&elem.value);
        }
    }

    buf.push_string(&req.order.algo_id);
    buf.push_bool(req.order.what_if);

    encode_tagvalue_as_string(buf, &req.order.order_misc_options);

    buf.push_bool(req.order.solicited);
    buf.push_bool(req.order.randomize_size);
    buf.push_bool(req.order.randomize_price);

    if ctx.server_version() >= MIN_SERVER_VER_PEGGED_TO_BENCHMARK {
        if req.order.order_type == "PEG BENCH" {
            buf.push_int(req.order.reference_contract_id);
            buf.push_bool(req.order.is_pegged_change_amount_decrease);
            buf.push_double(req.order.pegged_change_amount);
            buf.push_double(req.order.reference_change_amount);
            buf.push_string(&req.order.reference_exchange_id);
        }

        buf.push_int(req.order.conditions.len() as i32);

        // TODO
        for item in &req.order.conditions {
            buf.push_int(item.type_val());
            encoder_order_condition(buf, &item);
        }

        buf.push_bool(req.order.conditions_ignore_rth);
        buf.push_bool(req.order.conditions_cancel_order);

        buf.push_string(req.order.adjusted_order_type.to_string().as_str());
        buf.push_double(req.order.trigger_price);
        buf.push_double(req.order.lmt_price_offset);
        buf.push_double(req.order.adjusted_stop_price);
        buf.push_double(req.order.adjusted_stop_limit_price);
        buf.push_double(req.order.adjusted_trailing_amount);
        buf.push_int(req.order.adjustable_trailing_unit);
    }

    if ctx.server_version() >= MIN_SERVER_VER_EXT_OPERATOR {
        buf.push_string(&req.order.ext_operator);
    }

    if ctx.server_version() >= MIN_SERVER_VER_SOFT_DOLLAR_TIER {
        buf.push_string(&req.order.soft_dollar_tier.name);
        buf.push_string(&req.order.soft_dollar_tier.value);
    }

    Ok(DispatchId::Oneshot(req.id))
}

pub fn encode_cancel_order(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CacelOrder,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_ORDER);
    buf.push_int(VERSION);
    buf.push_int(req.id);

    Ok(DispatchId::Oneshot(req.id))
}

pub fn encode_req_open_orders(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqOpenOrders,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_OPEN_ORDERS);
    buf.push_int(VERSION);

    Ok(DispatchId::Global(OPCODE_REQ_OPEN_ORDERS))
}

pub fn encode_req_auto_open_orders(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqAutoOpenOrders,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_AUTO_OPEN_ORDERS);
    buf.push_int(VERSION);
    buf.push_bool(req.auto_bind);

    Ok(DispatchId::Global(OPCODE_REQ_AUTO_OPEN_ORDERS))
}

pub fn encode_req_all_open_orders(
    ctx: &mut Context,
    buf: &mut BytesMut,
    _req: &ReqAllOpenOrders,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_ALL_OPEN_ORDERS);
    buf.push_int(VERSION);

    Ok(DispatchId::Global(OPCODE_REQ_ALL_OPEN_ORDERS))
}
