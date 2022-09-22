use super::constants::*;
use domain::*;
use ordered_float::NotNaN;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Response {
    HandshakeAck(HandshakeAck),
    HistoricalTickLastMsg(HistoricalTickLastMsg),
    HistoricalTickBidAskMsg(HistoricalTickBidAskMsg),
    HistoricalTicksMsg(HistoricalTicksMsg),
    MarketRule(MarketRule),
    RerouteMktDepthReq(RerouteMktDepthReq),
    RerouteMktDataReq(RerouteMktDataReq),
    HistoricalDataUpdateMsg(HistoricalDataUpdateMsg),
    PnlSingleMsg(PnlSingleMsg),
    PnlMsg(PnlMsg),
    HistogramDataMsg(HistogramDataMsg),
    HistoricalNewsEndMsg(HistoricalNewsEndMsg),
    HistoricalNewsMsg(HistoricalNewsMsg),
    NewsArticleMsg(NewsArticleMsg),
    NewsProviderMsg(NewsProviderMsg),
    TickNewsMsg(TickNewsMsg),
    HeadTimestampMsg(HeadTimestampMsg),
    MktDepthExchangesMsg(MktDepthExchangesMsg),
    SymbolSamplesMsg(SymbolSamplesMsg),
    FamilyCodesMsg(FamilyCodesMsg),
    SoftDollarTiersMsg(SoftDollarTiersMsg),
    SecurityDefinitionOptionalParameterEndMsg(SecurityDefinitionOptionalParameterEndMsg),
    SecurityDefinitionOptionalParameterMsg(SecurityDefinitionOptionalParameterMsg),
    VerifyAndAuthCompletedMsg(VerifyAndAuthCompletedMsg),
    VerifyAndAuthMessageMsg(VerifyAndAuthMessageMsg),
    DisplayGroupUpdatedMsg(DisplayGroupUpdatedMsg),
    DisplayGroupListMsg(DisplayGroupListMsg),
    VerifyCompletedMsg(VerifyCompletedMsg),
    VerifyMessageApiMsg(VerifyMessageApiMsg),
    CommissionReportMsg(CommissionReportMsg),
    MarketDataTypeMsg(MarketDataTypeMsg),
    TickSnapshotEndMsg(TickSnapshotEndMsg),
    DeltaNeutralValidationMsg(DeltaNeutralValidationMsg),
    ExecutionDataEndMsg(ExecutionDataEndMsg),
    AcctDownloadEndMsg(AcctDownloadEndMsg),
    OpenOrderEndMsg(OpenOrderEndMsg),
    ContractDataEndMsg(ContractDataEndMsg),
    FundamentalDataMsg(FundamentalDataMsg),
    RealTimeBarsMsg(RealTimeBarsMsg),
    CurrentTimeMsg(CurrentTimeMsg),
    ScannerDataMsg(ScannerDataMsg),
    ScannerParametersMsg(ScannerParametersMsg),
    HistoricalDataMsg(HistoricalDataMsg),
    ReceiveFaMsg(ReceiveFaMsg),
    ManagedAcctsMsg(ManagedAcctsMsg),
    NewsBulletinsMsg(NewsBulletinsMsg),
    MarketDepthL2Msg(MarketDepthL2Msg),
    MarketDepthMsg(MarketDepthMsg),
    ExecutionDataMsg(ExecutionDataMsg),
    BondContractDataMsg(BondContractDataMsg),
    ContractDataMsg(ContractDataMsg),
    NextValidIdMsg(NextValidIdMsg),
    OpenOrderMsg(OpenOrderMsg),
    ErrMsgMsg(ErrMsgMsg),
    AcctUpdateTimeMsg(AcctUpdateTimeMsg),
    PortfolioValueMsg(PortfolioValueMsg),
    AcctValueMsg(AcctValueMsg),
    OrderStatusMsg(OrderStatusMsg),
    TickEFPMsg(TickEFPMsg),
    TickStringMsg(TickStringMsg),
    TickGenericMsg(TickGenericMsg),
    TickOptionComputationMsg(TickOptionComputationMsg),
    AccountSummaryEndMsg(AccountSummaryEndMsg),
    AccountSummaryMsg(AccountSummaryMsg),
    PositionEndMsg(PositionEndMsg),
    PositionMsg(PositionMsg),
    TickSizeMsg(TickSizeMsg),
    TickPriceMsg(TickPriceMsg),
    PositionMultiMsg(PositionMultiMsg),
    PositionMultiEndMsg(PositionMultiEndMsg),
    AccountUpdateMultiMsg(AccountUpdateMultiMsg),
    AccountUpdateMultiEndMsg(AccountUpdateMultiEndMsg),
    TickReqParamsMsg(TickReqParamsMsg),
    TickByTickAllLastMsg(TickByTickAllLastMsg),
    TickByTickBidAskMsg(TickByTickBidAskMsg),
    TickByTickMidPointMsg(TickByTickMidPointMsg),
    TickByTickNoneMsg(TickByTickNoneMsg),
    SmartComponentsMsg(SmartComponentsMsg),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HandshakeAck {
    pub server_version: i32,
    pub addr_or_time: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoricalTickLastMsg {
    pub req_id: i32,
    pub ticks: Vec<HistoricalTickLast>,
    pub done: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoricalTickBidAskMsg {
    pub req_id: i32,
    pub ticks: Vec<HistoricalTickBidAsk>,
    pub done: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoricalTicksMsg {
    pub req_id: i32,
    pub ticks: Vec<HistoricalTick>,
    pub done: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MarketRule {
    pub market_rule_id: i32,
    pub price_increments: Vec<PriceIncrement>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RerouteMktDepthReq {
    pub req_id: i32,
    pub con_id: i32,
    pub exchange: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RerouteMktDataReq {
    pub req_id: i32,
    pub con_id: i32,
    pub exchange: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoricalDataUpdateMsg {
    pub req_id: i32,
    pub bar: Bar,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PnlSingleMsg {
    pub req_id: i32,
    pub pos: i32,
    pub daily_pnl: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub value: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PnlMsg {
    pub req_id: i32,
    pub daily_pnl: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistogramDataMsg {
    pub req_id: i32,
    pub items: Vec<HistogramEntry>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoricalNewsEndMsg {
    pub req_id: i32,
    pub has_more: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoricalNewsMsg {
    pub req_id: i32,
    pub time: String,
    pub provider_code: String,
    pub article_id: String,
    pub headline: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NewsArticleMsg {
    pub req_id: i32,
    pub article_type: i32,
    pub article_text: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NewsProviderMsg {
    pub providers: Vec<NewsProvider>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickNewsMsg {
    pub req_id: i32,
    pub time_stamp: i64,
    pub provider_code: String,
    pub article_id: String,
    pub headline: String,
    pub extra_data: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HeadTimestampMsg {
    pub req_id: i32,
    pub head_time_stamp: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MktDepthExchangesMsg {
    pub depth_mkt_data_descriptions: Vec<DepthMktDataDescription>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SymbolSamplesMsg {
    pub req_id: i32,
    pub contract_descriptions: Vec<ContractDescription>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FamilyCodesMsg {
    pub family_codes: Vec<FamilyCode>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SoftDollarTiersMsg {
    pub req_id: i32,
    pub tiers: Vec<SoftDollarTier>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SecurityDefinitionOptionalParameterEndMsg {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SecurityDefinitionOptionalParameterMsg {
    pub req_id: i32,
    pub exchange: String,
    pub underlying_con_id: i32,
    pub trading_class: String,
    pub multiplier: String,
    pub expirations: HashSet<String>,
    pub strikes: HashSet<NotNaN<f64>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyAndAuthCompletedMsg {
    pub is_successful: bool,
    pub error_text: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyAndAuthMessageMsg {
    pub api_data: String,
    pub xyz_challenge: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DisplayGroupUpdatedMsg {
    pub req_id: i32,
    pub contract_info: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DisplayGroupListMsg {
    pub req_id: i32,
    pub groups: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyCompletedMsg {
    pub is_successful: bool,
    pub error_text: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyMessageApiMsg {
    pub api_data: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CommissionReportMsg {
    pub report: CommissionReport,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MarketDataTypeMsg {
    pub req_id: i32,
    pub market_data_type: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickSnapshotEndMsg {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeltaNeutralValidationMsg {
    pub req_id: i32,
    pub delta_neutral_contract: DeltaNeutralContract,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ExecutionDataEndMsg {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AcctDownloadEndMsg {
    pub account_name: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OpenOrderEndMsg {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ContractDataEndMsg {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FundamentalDataMsg {
    pub req_id: i32,
    pub data: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RealTimeBarsMsg {
    pub req_id: i32,
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub wap: f64,
    pub count: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CurrentTimeMsg {
    pub time: i64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ScannerParametersMsg {
    pub xml: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoricalDataMsg {
    pub req_id: i32,
    pub start_date: String,
    pub end_date: String,
    pub bars: Vec<Bar>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReceiveFaMsg {
    pub fa_data_type: i32,
    pub xml: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ManagedAcctsMsg {
    pub accounts: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NewsBulletinsMsg {
    pub req_id: i32,
    pub msg_type: i32,
    pub message: String,
    pub originating_exch: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MarketDepthL2Msg {
    pub id: i32,
    pub position: i32,
    pub market_maker: String,
    pub operation: i32,
    pub side: i32,
    pub price: f64,
    pub size: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MarketDepthMsg {
    pub id: i32,
    pub position: i32,
    pub operation: i32,
    pub side: i32,
    pub price: f64,
    pub size: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ExecutionDataMsg {
    pub req_id: i32,
    pub contract: Contract,
    pub exec: Execution,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BondContractDataMsg {
    pub req_id: i32,
    pub contract_details: ContractDetails,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ContractDataMsg {
    pub req_id: i32,
    pub contract_details: ContractDetails,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ScannerData {
    pub rank: i32,
    pub contract_details: ContractDetails,
    pub distance: String,
    pub benchmark: String,
    pub projection: String,
    pub legs: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ScannerDataMsg {
    pub req_id: i32,
    pub datas: Vec<ScannerData>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NextValidIdMsg {
    pub order_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OpenOrderMsg {
    pub order_id: i32,
    pub contract: Contract,
    pub order: Order,
    pub order_state: OrderState,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ErrMsgMsg {
    pub id: i32,
    pub error_code: i32,
    pub error_message: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AcctUpdateTimeMsg {
    pub time_stamp: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PortfolioValueMsg {
    pub contract: Contract,
    pub position: f64,
    pub market_price: f64,
    pub market_value: f64,
    pub average_cost: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub account_name: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AcctValueMsg {
    pub key: String,
    pub val: String,
    pub cur: String,
    pub account_name: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OrderStatusMsg {
    pub id: i32,
    pub status: String,
    pub filled: f64,
    pub remaining: f64,
    pub avg_fill_price: f64,
    pub perm_id: i32,
    pub parent_id: i32,
    pub last_fill_price: f64,
    pub client_id: i32,
    pub why_held: String,
    pub mkt_cap_price: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickEFPMsg {
    pub req_id: i32,
    pub tick_type: i32,
    pub basis_points: f64,
    pub formatted_basis_points: String,
    pub implied_futures_price: f64,
    pub hold_days: i32,
    pub future_last_trade_date: String,
    pub dividend_impact: f64,
    pub dividends_to_last_trade_date: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickStringMsg {
    pub req_id: i32,
    pub tick_type: i32,
    pub value: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickGenericMsg {
    pub req_id: i32,
    pub tick_type: i32,
    pub value: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickOptionComputationMsg {
    pub req_id: i32,
    pub tick_type: i32,
    pub implied_vol: f64,
    pub delta: f64,
    pub opt_price: f64,
    pub pv_dividend: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    pub und_price: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AccountSummaryEndMsg {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AccountSummaryMsg {
    pub req_id: i32,
    pub account: String,
    pub tag: String,
    pub value: String,
    pub currency: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PositionEndMsg {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PositionMsg {
    pub account: String,
    pub contract: Contract,
    pub pos: f64,
    pub avg_cost: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickSizeMsg {
    pub req_id: i32,
    pub tick_type: i32,
    pub size: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickPriceMsg {
    pub req_id: i32,
    pub tick_type: i32,
    pub price: f64,
    pub size: i32,
    pub attribs: TickAttr,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PositionMultiMsg {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
    pub contract: Contract,
    pub pos: f64,
    pub avg_cost: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PositionMultiEndMsg {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AccountUpdateMultiMsg {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
    pub key: String,
    pub value: String,
    pub currency: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AccountUpdateMultiEndMsg {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SmartComponentsMsg {
    pub req_id: i32,
    pub map: HashMap<i32, (String, u8)>,
    //map: HashMap<i32, Entry<String, char>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickReqParamsMsg {
    pub req_id: i32,
    pub min_tick: f64,
    pub bbo_exchange: String,
    pub snapshot_permissions: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickByTickAllLastMsg {
    pub req_id: i32,
    pub tick_type: i32,
    pub time: i64,
    pub price: f64,
    pub size: i32,
    pub attribs: TickAttr,
    pub exchange: String,
    pub special_conditions: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickByTickBidAskMsg {
    pub req_id: i32,
    pub time: i64,
    pub bid_price: f64,
    pub ask_price: f64,
    pub bid_size: i32,
    pub ask_size: i32,
    pub attribs: TickAttr,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickByTickMidPointMsg {
    pub req_id: i32,
    pub time: i64,
    pub mid_point: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickByTickNoneMsg {}

/*impl Identifiable for Response {
    type Id = i32;

    fn id(&self) -> Self::Id {
        match self {
            Response::HandshakeAck(ref msg) => OPCODE_HANDSHAKE,
    Response::HistoricalTickLastMsg(ref msg) => msg.req_id,
    Response::HistoricalTickBidAskMsg(ref msg) => msg.req_id,
    Response::HistoricalTicksMsg(ref msg) => msg.req_id,
    Response::MarketRule(ref msg) => OPCODE_REQ_MARKET_RULE,
    Response::RerouteMktDepthReq(ref msg) => msg.req_id,
    Response::RerouteMktDataReq(ref msg) => msg.req_id,
    Response::HistoricalDataUpdateMsg(ref msg) => msg.req_id,
    Response::PnlSingleMsg(ref msg) => msg.req_id,
    Response::PnlMsg(ref msg) => msg.req_id,
    Response::HistogramDataMsg(ref msg) => msg.req_id,
    Response::HistoricalNewsEndMsg(ref msg) => msg.req_id,
    Response::HistoricalNewsMsg(ref msg) => msg.req_id,
    Response::NewsArticleMsg(ref msg) => msg.req_id,
    Response::NewsProviderMsg(ref msg) => OPCODE_REQ_NEWS_PROVIDERS,
    Response::TickNewsMsg(ref msg) => msg.req_id,
    Response::HeadTimestampMsg(ref msg) => msg.req_id,
    Response::MktDepthExchangesMsg(ref msg) => OPCODE_REQ_MKT_DEPTH_EXCHANGES,
    Response::SymbolSamplesMsg(ref msg) => msg.req_id,
    Response::FamilyCodesMsg(ref msg) => OPCODE_REQ_FAMILY_CODES,
    Response::SoftDollarTiersMsg(ref msg) => msg.req_id,
    Response::SecurityDefinitionOptionalParameterEndMsg(ref msg) => msg.req_id,
    Response::SecurityDefinitionOptionalParameterMsg(ref msg) => msg.req_id,
    Response::VerifyAndAuthCompletedMsg(ref msg) => OPCODE_VERIFY_AND_AUTH_REQUEST,
    Response::VerifyAndAuthMessageMsg(ref msg) => OPCODE_VERIFY_AND_AUTH_MESSAGE,
    Response::DisplayGroupUpdatedMsg(ref msg) => msg.req_id,
    Response::DisplayGroupListMsg(ref msg) => msg.req_id,
    Response::VerifyCompletedMsg(ref msg) => OPCODE_VERIFY_REQUEST,
    Response::VerifyMessageApiMsg(ref msg) => OPCODE_VERIFY_MESSAGE,
    Response::CommissionReportMsg(ref msg) => msg.req_id,
    Response::MarketDataTypeMsg(ref msg) => msg.req_id,
    Response::TickSnapshotEndMsg(ref msg) => msg.req_id,
    Response::DeltaNeutralValidationMsg(ref msg) => msg.req_id,
    Response::ExecutionDataEndMsg(ref msg) => msg.req_id,
    Response::AcctDownloadEndMsg(ref msg) => msg.req_id,
    Response::OpenOrderEndMsg(ref msg) => msg.req_id,
    Response::ContractDataEndMsg(ref msg) => msg.req_id,
    Response::FundamentalDataMsg(ref msg) => msg.req_id,
    Response::RealTimeBarsMsg(ref msg) => msg.req_id,
    Response::CurrentTimeMsg(ref msg) => OPCODE_REQ_CURRENT_TIME,
    Response::ScannerDataMsg(ref msg) => msg.req_id,
    Response::ScannerParametersMsg(ref msg) => OPCODE_REQ_SCANNER_PARAMETERS,
    Response::HistoricalDataMsg(ref msg) => msg.req_id,
    Response::ReceiveFaMsg(ref msg) => OPCODE_REQUEST_FA,
    Response::ManagedAcctsMsg(ref msg) => OPCODE_REQ_ACCOUNT_UPDATES,
    Response::NewsBulletinsMsg(ref msg) => msg.req_id,
    Response::MarketDepthL2Msg(ref msg) => msg.req_id,
    Response::MarketDepthMsg(ref msg) => msg.id,
    Response::ExecutionDataMsg(ref msg) => msg.req_id,
    Response::BondContractDataMsg(ref msg) => msg.req_id,
    Response::ContractDataMsg(ref msg) => msg.req_id,
    Response::NextValidMsg(ref msg) => OPCODE_REQ_IDS,
    Response::OpenOrderMsg(ref msg) => OPCODE_REQ_OPEN_ORDERS,
    Response::ErrMsgMsg(ref msg) => msg.id,
    Response::AcctUpdateTimeMsg(ref msg) => msg.req_id,
    Response::PortfolioValueMsg(ref msg) => msg.req_id,
    Response::AcctValueMsg(ref msg) => msg.req_id,
    Response::OrderStatusMsg(ref msg) => msg.req_id,
    Response::TickEFPMsg(ref msg) => msg.req_id,
    Response::TickStringMsg(ref msg) => msg.req_id,
    Response::TickGenericMsg(ref msg) => msg.req_id,
    Response::TickOptionComputationMsg(ref msg) => msg.req_id,
    Response::AccountSummaryEndMsg(ref msg) => msg.req_id,
    Response::AccountSummaryMsg(ref msg) => msg.req_id,
    Response::PositionEndMsg(ref msg) => msg.req_id,
    Response::PositionMsg(ref msg) => msg.req_id,
    Response::TickSizeMsg(ref msg) => msg.req_id,
    Response::TickPriceMsg(ref msg) => msg.req_id,
    Response::PositionMultiMsg(ref msg) => msg.req_id,
    Response::PositionMultiEndMsg(ref msg) => msg.req_id,
    Response::AccountUpdateMultiMsg(ref msg) => msg.req_id,
    Response::AccountUpdateMultiEndMsg(ref msg) => msg.req_id,
    Response::TickReqParamsMsg(ref msg) => msg.req_id,
    Response::TickByTickAllLastMsg(ref msg) => msg.req_id,
    Response::TickByTickBidAskMsg(ref msg) => msg.req_id,
    Response::TickByTickMidPointMsg(ref msg) => msg.req_id,
    Response::TickByTickNoneMsg(ref msg) => msg.req_id,
    Response::SmartComponentsMsg(ref msg) => msg.req_id,
    }
}
*/
