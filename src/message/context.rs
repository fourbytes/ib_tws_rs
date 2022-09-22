// Context for Request & Response

use bit::BitIndex;
use bytes::{BufMut, BytesMut};
use domain::contract::*;
use domain::OrderCondition;
use domain::tag_value::*;
use ordered_float::NotNaN;
use std::{f64, i32};
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::error;
use std::fmt;
use std::io;
use std::str;
use super::account::*;
use super::auth::*;
use super::bulletins::*;
use super::constants::*;
use super::contract::*;
use super::depth_exchange::*;
use super::display_group::*;
use super::err_msg::*;
use super::execution::*;
use super::fa::*;
use super::family_code::*;
use super::fundamental::*;
use super::handshake::*;
use super::head_timestamp::*;
use super::histogram::*;
use super::historical::*;
use super::market::*;
use super::market_rule::*;
use super::misc::*;
use super::news::*;
use super::order::*;
use super::portfolio::*;
use super::position::*;
use super::request::*;
use super::reroute::*;
use super::response::*;
use super::scanner::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};

#[derive(Debug)]
pub enum DispatchId {
    Oneshot(i32),
    Stream(i32),
    Global(i32),
    Multi(i32),
}

#[derive(Debug)]
pub struct Context {
    pub exec_id_to_req_id_map: HashMap<String, i32>,
    pub server_version: i32,
    pub extra_auth: bool,
    pub next_valid_id: i32,
    pub accounts: String,
}

impl Context {
    pub fn new() -> Self {
        Context {
            exec_id_to_req_id_map: HashMap::new(),
            server_version: -1,
            extra_auth: false,
            next_valid_id: -1,
            accounts: "".to_string(),
        }
    }

    pub fn server_version(&self) -> i32 {
        self.server_version
    }

    pub fn set_server_version(&mut self, version: i32) {
        self.server_version = version
    }

    pub fn extra_auth(&self) -> bool {
        self.extra_auth
    }

    pub fn is_connect(&self) -> bool {
        self.next_valid_id > 0
    }

    pub fn set_extra_auth(&mut self, extra_auth: bool) {
        self.extra_auth = extra_auth
    }

    pub fn register_request(&mut self, request: &Request) {}

    pub fn process_response(&mut self, response: &Response) {}

    pub fn register(&mut self, req_id: i32, exec_id: &str) {
        self.exec_id_to_req_id_map
            .insert(exec_id.to_string(), req_id);
    }

    pub fn unregister(&mut self, req_id: i32) {}

    pub fn get_req_id(&self, exec_id: &str) -> Option<i32> {
        self.exec_id_to_req_id_map.get(exec_id).map(|i| *i)
    }

    pub fn encode_message(&mut self, req: &Request) -> Result<BytesMut, io::Error> {
        let mut buf = BytesMut::new();
        let request = match req {
            Request::Handshake(ref req) => encode_handshake(self, &mut buf, req),
            Request::StartApi(ref req) => encode_start_api(self, &mut buf, req),
            Request::CancelScannerSubscription(ref req) => {
                encode_cancel_scanner_subscription(self, &mut buf, req)
            }

            Request::ReqScannerParameters(ref req) => {
                encode_req_scanner_parameters(self, &mut buf, req)
            }

            Request::ReqScannerSubscription(ref req) => {
                encode_req_scanner_subscription(self, &mut buf, req)
            }

            Request::ReqMktData(ref req) => encode_req_mkt_data(self, &mut buf, req),
            Request::CancelHistoricalData(ref req) => {
                encode_cancel_historical_data(self, &mut buf, req)
            }
            Request::CancelRealtimeBars(ref req) => {
                encode_cancel_realtime_bars(self, &mut buf, req)
            }
            Request::ReqHistoricalData(ref req) => encode_req_historical_data(self, &mut buf, req),
            Request::ReqHeadTimestamp(ref req) => encode_req_head_timestamp(self, &mut buf, req),
            Request::CancelHeadTimestamp(ref req) => {
                encode_cancel_head_timestamp(self, &mut buf, req)
            }
            Request::ReqRealtimeBars(ref req) => encode_req_realtime_bars(self, &mut buf, req),
            Request::ReqContractDetails(ref req) => {
                encode_req_contract_details(self, &mut buf, req)
            }
            Request::ReqMktDepth(ref req) => encode_req_mkt_depth(self, &mut buf, req),
            Request::CancelMktData(ref req) => encode_cancel_mkt_data(self, &mut buf, req),
            Request::CancelMktDepth(ref req) => encode_cancel_mkt_depth(self, &mut buf, req),
            Request::ExerciseOptions(ref req) => encode_exercise_options(self, &mut buf, req),
            Request::PlaceOrder(ref req) => encode_place_order(self, &mut buf, req),
            Request::ReqAccountUpdates(ref req) => encode_req_account_updates(self, &mut buf, req),
            Request::ReqExecutions(ref req) => encode_req_executions(self, &mut buf, req),
            Request::CacelOrder(ref req) => encode_cancel_order(self, &mut buf, req),
            Request::ReqOpenOrders(ref req) => encode_req_open_orders(self, &mut buf, req),
            Request::ReqIds(ref req) => encode_req_ids(self, &mut buf, req),
            Request::ReqNewsBulletins(ref req) => encode_req_news_bulletins(self, &mut buf, req),
            Request::CancelNewsBulletins(ref req) => {
                encode_cancel_news_bulletins(self, &mut buf, req)
            }
            Request::SetServerLogLevel(ref req) => encode_set_server_log_level(self, &mut buf, req),
            Request::ReqAutoOpenOrders(ref req) => encode_req_auto_open_orders(self, &mut buf, req),
            Request::ReqAllOpenOrders(ref req) => encode_req_all_open_orders(self, &mut buf, req),
            Request::ReqManagedAccts(ref req) => encode_req_managed_accts(self, &mut buf, req),
            Request::RequestFA(ref req) => encode_request_fa(self, &mut buf, req),
            Request::ReplaceFA(ref req) => encode_replace_fa(self, &mut buf, req),
            Request::ReqCurrentTime(ref req) => encode_req_current_time(self, &mut buf, req),
            Request::ReqFundamentalData(ref req) => {
                encode_req_fundamental_data(self, &mut buf, req)
            }
            Request::CancelFundamentalData(ref req) => {
                encode_cancel_fundamental_data(self, &mut buf, req)
            }
            Request::CalculateImpliedVolatility(ref req) => {
                encode_calculate_implied_volatility(self, &mut buf, req)
            }
            Request::CancelCalculateImpliedVolatility(ref req) => {
                encode_cancel_calculate_implied_volatility(self, &mut buf, req)
            }
            Request::CalculateOptionPrice(ref req) => {
                encode_calculate_option_price(self, &mut buf, req)
            }
            Request::CancelCalculateOptionPrice(ref req) => {
                encode_cancel_calculate_option_price(self, &mut buf, req)
            }
            Request::ReqGlobalCancel(ref req) => encode_req_global_cancel(self, &mut buf, req),
            Request::ReqMarketDataType(ref req) => encode_req_market_data_type(self, &mut buf, req),
            Request::ReqPositions(ref req) => encode_req_positions(self, &mut buf, req),
            Request::ReqSecDefOptParams(ref req) => {
                encode_req_sec_def_opt_params(self, &mut buf, req)
            }
            Request::ReqSoftDollarTiers(ref req) => {
                encode_req_soft_dollar_tiers(self, &mut buf, req)
            }
            Request::CancelPositions(ref req) => encode_cancel_positions(self, &mut buf, req),
            Request::ReqPositionsMulti(ref req) => encode_req_positions_multi(self, &mut buf, req),
            Request::CancelPositionsMulti(ref req) => {
                encode_cancel_positions_multi(self, &mut buf, req)
            }
            Request::CancelAccountUpdatesMulti(ref req) => {
                encode_cancel_account_updates_multi(self, &mut buf, req)
            }
            Request::ReqAccountUpdatesMulti(ref req) => {
                encode_req_account_updates_multi(self, &mut buf, req)
            }
            Request::ReqAccountSummary(ref req) => encode_req_account_summary(self, &mut buf, req),
            Request::CancelAccountSummary(ref req) => {
                encode_cancel_account_summary(self, &mut buf, req)
            }
            Request::VerifyRequest(ref req) => encode_verify_request(self, &mut buf, req),
            Request::VerifyMessage(ref req) => encode_verify_message(self, &mut buf, req),
            Request::VerfyAndAuthRequest(ref req) => {
                encode_verify_and_auth_request(self, &mut buf, req)
            }
            Request::VerifyAndAuthMessage(ref req) => {
                encode_verify_and_auth_message(self, &mut buf, req)
            }
            Request::QueryDisplayGroups(ref req) => {
                encode_query_display_groups(self, &mut buf, req)
            }
            Request::SubscribeToGroupEvent(ref req) => {
                encode_subscribe_to_group_event(self, &mut buf, req)
            }
            Request::UpdateDisplayGroup(ref req) => {
                encode_update_display_group(self, &mut buf, req)
            }
            Request::UbsubscribeFromGroupEvents(ref req) => {
                encode_unsubscribe_from_group_events(self, &mut buf, req)
            }
            Request::MatchingSymbol(ref req) => encode_matching_symbol(self, &mut buf, req),
            Request::ReqFamilyCodes(ref req) => encode_req_family_codes(self, &mut buf, req),
            Request::ReqMktDepthExchanges(ref req) => {
                encode_req_mkt_depth_exchanges(self, &mut buf, req)
            }
            Request::ReqSmartComponents(ref req) => {
                encode_req_smart_components(self, &mut buf, req)
            }
            Request::ReqNewsProvider(ref req) => encode_req_news_provider(self, &mut buf, req),
            Request::ReqNewsArticle(ref req) => encode_req_news_article(self, &mut buf, req),
            Request::ReqHistoricalNews(ref req) => encode_req_historical_news(self, &mut buf, req),
            Request::ReqHistogramData(ref req) => encode_req_histogram_data(self, &mut buf, req),
            Request::CancelHistogramData(ref req) => {
                encode_cancel_histogram_data(self, &mut buf, req)
            }
            Request::ReqMarketRule(ref req) => encode_req_market_rule(self, &mut buf, req),
            Request::ReqPnl(ref req) => encode_req_pnl(self, &mut buf, req),
            Request::CancelPnl(ref req) => encode_cancel_pnl(self, &mut buf, req),
            Request::ReqPnlSingle(ref req) => encode_req_pnl_single(self, &mut buf, req),
            Request::CancelPnlSingle(ref req) => encode_cancel_pnl_single(self, &mut buf, req),
            Request::ReqHistoricalTicks(ref req) => {
                encode_req_historical_ticks(self, &mut buf, req)
            }
            Request::ReqTickByTickData(ref req) => {
                encode_req_tick_by_tick_data(self, &mut buf, req)
            }
            Request::CancelTickByTickData(ref req) => {
                encode_cancel_tick_by_tick_data(self, &mut buf, req)
            }
        };

        //println!("request {:?}\n buf:{:?}", request, buf);
        match request {
            Ok(_) => Ok(buf),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "encoder error")),
        }
    }

    pub fn decode_message(&mut self, buf: &mut BytesMut) -> Result<Response, io::Error> {
        let (response, id) = self.parse_message(buf)?;
        Ok(response)
    }

    /*pub fn parse_message_warp(&mut self, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
        let result = self.parse_message(buf);

        match result {
            Err(ref e) => {
                println!("parse_message error:{:?}", e);
            }
            _ => {}
        }
        result
    }*/

    pub fn parse_message(&mut self, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
        if self.server_version < 0 {
            let ack = decode_handshake_ack(self, buf)?;
            self.server_version = ack.server_version;
            Ok((Response::HandshakeAck(ack), OPCODE_HANDSHAKE))
        } else {
            let msg_id = buf.read_int()?;
            //println!("msg_id: {}", msg_id);
            let result = match msg_id {
                TICK_PRICE => decode_tick_price_msg(self, buf)?,
                TICK_SIZE => decode_tick_size_msg(self, buf)?,
                ORDER_STATUS => decode_order_status_msg(self, buf)?,
                ERR_MSG => decode_err_msg(self, buf)?,
                OPEN_ORDER => decode_open_order_end_msg(self, buf)?,
                ACCT_VALUE => decode_acct_value_msg(self, buf)?,
                PORTFOLIO_VALUE => decode_portfolio_value_msg(self, buf)?,
                ACCT_UPDATE_TIME => decode_acct_update_time_msg(self, buf)?,
                NEXT_VALID_ID => {
                    let response = decode_next_valid_id_msg(self, buf)?;
                    if let Response::NextValidIdMsg(ref msg) = response.0 {
                        self.next_valid_id = msg.order_id;
                    }
                    response
                }
                CONTRACT_DATA => decode_contract_data_msg(self, buf)?,
                EXECUTION_DATA => decode_execution_data_msg(self, buf)?,
                MARKET_DEPTH => decode_market_depth_msg(self, buf)?,
                MARKET_DEPTH_L2 => decode_market_depth_l2_msg(self, buf)?,
                NEWS_BULLETINS => decode_news_bulletins_msg(self, buf)?,
                MANAGED_ACCTS => {
                    let response = decode_managed_accts_msg(self, buf)?;
                    if let Response::ManagedAcctsMsg(ref msg) = response.0 {
                        self.accounts = msg.accounts.clone();
                    }
                    response
                }
                RECEIVE_FA => decode_receive_fa_msg(self, buf)?,
                HISTORICAL_DATA => decode_historical_data_msg(self, buf)?,
                BOND_CONTRACT_DATA => decode_bond_contract_data_msg(self, buf)?,
                SCANNER_PARAMETERS => decode_scanner_parameters_msg(self, buf)?,
                SCANNER_DATA => decode_scanner_data_msg(self, buf)?,
                TICK_OPTION_COMPUTATION => decode_tick_option_computation_msg(self, buf)?,
                TICK_GENERIC => decode_tick_generic_msg(self, buf)?,
                TICK_STRING => decode_tick_string_msg(self, buf)?,
                TICK_EFP => decode_tick_efp_msg(self, buf)?,
                CURRENT_TIME => decode_current_time_msg(self, buf)?,
                REAL_TIME_BARS => decode_realtime_bars_msg(self, buf)?,
                FUNDAMENTAL_DATA => decode_fundamental_data_msg(self, buf)?,
                CONTRACT_DATA_END => decode_contract_data_end_msg(self, buf)?,
                OPEN_ORDER_END => decode_open_order_end_msg(self, buf)?,
                ACCT_DOWNLOAD_END => decode_acct_download_end_msg(self, buf)?,
                EXECUTION_DATA_END => decode_execution_data_end_msg(self, buf)?,
                DELTA_NEUTRAL_VALIDATION => decode_delta_neutral_validation_msg(self, buf)?,
                TICK_SNAPSHOT_END => decode_tick_snapshot_end_msg(self, buf)?,
                MARKET_DATA_TYPE => decode_market_data_type_msg(self, buf)?,
                COMMISSION_REPORT => decode_commission_report_msg(self, buf)?,
                POSITION => decode_position_msg(self, buf)?,
                POSITION_END => decode_position_end_msg(self, buf)?,
                ACCOUNT_SUMMARY => decode_account_summary_msg(self, buf)?,
                ACCOUNT_SUMMARY_END => decode_account_summary_end_msg(self, buf)?,
                VERIFY_MESSAGE_API => decode_verify_message_api_msg(self, buf)?,
                VERIFY_COMPLETED => decode_verify_completed_msg(self, buf)?,
                DISPLAY_GROUP_LIST => decode_display_group_list_msg(self, buf)?,
                DISPLAY_GROUP_UPDATED => decode_display_group_updated_msg(self, buf)?,
                VERIFY_AND_AUTH_MESSAGE_API => decode_verify_and_auth_message_msg(self, buf)?,
                VERIFY_AND_AUTH_COMPLETED => decode_verify_and_auth_completed_msg(self, buf)?,
                POSITION_MULTI => decode_position_multi_msg(self, buf)?,
                POSITION_MULTI_END => decode_position_multi_end_msg(self, buf)?,
                ACCOUNT_UPDATE_MULTI => decode_account_update_multi_msg(self, buf)?,
                ACCOUNT_UPDATE_MULTI_END => decode_account_update_multi_end_msg(self, buf)?,
                SECURITY_DEFINITION_OPTION_PARAMETER => {
                    decode_security_definition_optional_parameter_msg(self, buf)?
                }
                SECURITY_DEFINITION_OPTION_PARAMETER_END => {
                    decode_security_definition_optional_parameter_end_msg(self, buf)?
                }
                SOFT_DOLLAR_TIERS => decode_soft_dollar_tiers_msg(self, buf)?,
                FAMILY_CODES => decode_family_codes_msg(self, buf)?,
                SYMBOL_SAMPLES => decode_symbol_sample_msg(self, buf)?,
                MKT_DEPTH_EXCHANGES => decode_mkt_depth_exchanges_msg(self, buf)?,
                TICK_REQ_PARAMS => decode_tick_req_params_msg(self, buf)?,
                SMART_COMPONENTS => decode_smart_components_msg(self, buf)?,
                NEWS_ARTICLE => decode_news_article_msg(self, buf)?,
                TICK_NEWS => decode_tick_news_msg(self, buf)?,
                NEWS_PROVIDERS => decode_news_providers_msg(self, buf)?,
                HISTORICAL_NEWS => decode_historical_news_msg(self, buf)?,
                HISTORICAL_NEWS_END => decode_historical_news_end_msg(self, buf)?,
                HEAD_TIMESTAMP => decode_head_timestamp_msg(self, buf)?,
                HISTOGRAM_DATA => decode_historical_data_msg(self, buf)?,
                HISTORICAL_DATA_UPDATE => decode_historical_data_update_msg(self, buf)?,
                REROUTE_MKT_DATA_REQ => decode_reroute_mkt_data_req(self, buf)?,
                REROUTE_MKT_DEPTH_REQ => decode_reroute_mkt_depth_req(self, buf)?,
                MARKET_RULE => decode_market_rule(self, buf)?,
                PNL => decode_pnl_msg(self, buf)?,
                PNL_SINGLE => decode_pnl_single_msg(self, buf)?,
                HISTORICAL_TICKS => decode_historical_ticks(self, buf)?,
                HISTORICAL_TICKS_BID_ASK => decode_historical_ticks_bid_ask(self, buf)?,
                HISTORICAL_TICKS_LAST => decode_historical_ticks_last(self, buf)?,
                TICK_BY_TICK => decode_tick_by_tick_msg(self, buf)?,
                _ => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("unknown message id {}", msg_id.to_string()))),
            };
            Ok(result)
        }
    }
}
