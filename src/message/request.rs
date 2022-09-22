use domain::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Request {
    Handshake(Handshake),
    StartApi(StartApi),
    CancelScannerSubscription(CancelScannerSubscription),
    ReqScannerParameters(ReqScannerParameters),
    ReqScannerSubscription(ReqScannerSubscription),
    ReqMktData(ReqMktData),
    CancelHistoricalData(CancelHistoricalData),
    CancelRealtimeBars(CancelRealtimeBars),
    ReqHistoricalData(ReqHistoricalData),
    ReqHeadTimestamp(ReqHeadTimestamp),
    CancelHeadTimestamp(CancelHeadTimestamp),
    ReqRealtimeBars(ReqRealtimeBars),
    ReqContractDetails(ReqContractDetails),
    ReqMktDepth(ReqMktDepth),
    CancelMktData(CancelMktData),
    CancelMktDepth(CancelMktDepth),
    ExerciseOptions(ExerciseOptions),
    PlaceOrder(PlaceOrder),
    ReqAccountUpdates(ReqAccountUpdates),
    ReqExecutions(ReqExecutions),
    CacelOrder(CacelOrder),
    ReqOpenOrders(ReqOpenOrders),
    ReqIds(ReqIds),
    ReqNewsBulletins(ReqNewsBulletins),
    CancelNewsBulletins(CancelNewsBulletins),
    SetServerLogLevel(SetServerLogLevel),
    ReqAutoOpenOrders(ReqAutoOpenOrders),
    ReqAllOpenOrders(ReqAllOpenOrders),
    ReqManagedAccts(ReqManagedAccts),
    RequestFA(RequestFA),
    ReplaceFA(ReplaceFA),
    ReqCurrentTime(ReqCurrentTime),
    ReqFundamentalData(ReqFundamentalData),
    CancelFundamentalData(CancelFundamentalData),
    CalculateImpliedVolatility(CalculateImpliedVolatility),
    CancelCalculateImpliedVolatility(CancelCalculateImpliedVolatility),
    CalculateOptionPrice(CalculateOptionPrice),
    CancelCalculateOptionPrice(CancelCalculateOptionPrice),
    ReqGlobalCancel(ReqGlobalCancel),
    ReqMarketDataType(ReqMarketDataType),
    ReqPositions(ReqPositions),
    ReqSecDefOptParams(ReqSecDefOptParams),
    ReqSoftDollarTiers(ReqSoftDollarTiers),
    CancelPositions(CancelPositions),
    ReqPositionsMulti(ReqPositionsMulti),
    CancelPositionsMulti(CancelPositionsMulti),
    CancelAccountUpdatesMulti(CancelAccountUpdatesMulti),
    ReqAccountUpdatesMulti(ReqAccountUpdatesMulti),
    ReqAccountSummary(ReqAccountSummary),
    CancelAccountSummary(CancelAccountSummary),
    VerifyRequest(VerifyRequest),
    VerifyMessage(VerifyMessage),
    VerfyAndAuthRequest(VerfyAndAuthRequest),
    VerifyAndAuthMessage(VerifyAndAuthMessage),
    QueryDisplayGroups(QueryDisplayGroups),
    SubscribeToGroupEvent(SubscribeToGroupEvent),
    UpdateDisplayGroup(UpdateDisplayGroup),
    UbsubscribeFromGroupEvents(UbsubscribeFromGroupEvents),
    MatchingSymbol(MatchingSymbol),
    ReqFamilyCodes(ReqFamilyCodes),
    ReqMktDepthExchanges(ReqMktDepthExchanges),
    ReqSmartComponents(ReqSmartComponents),
    ReqNewsProvider(ReqNewsProvider),
    ReqNewsArticle(ReqNewsArticle),
    ReqHistoricalNews(ReqHistoricalNews),
    ReqHistogramData(ReqHistogramData),
    CancelHistogramData(CancelHistogramData),
    ReqMarketRule(ReqMarketRule),
    ReqPnl(ReqPnl),
    CancelPnl(CancelPnl),
    ReqPnlSingle(ReqPnlSingle),
    CancelPnlSingle(CancelPnlSingle),
    ReqHistoricalTicks(ReqHistoricalTicks),
    ReqTickByTickData(ReqTickByTickData),
    CancelTickByTickData(CancelTickByTickData),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Handshake {
    pub min_version: i32,
    pub max_version: i32,
    pub option: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StartApi {
    pub client_id: i32,
    pub optional_capabilities: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelScannerSubscription {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqScannerParameters {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqScannerSubscription {
    pub req_id: i32,
    pub subscribe: ScannerSubscription,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMktData {
    pub req_id: i32,
    pub contract: Contract,
    pub generic_tick_list: String,
    pub snapshot: bool,
    pub regulatory_snapshot: bool,
    pub mkt_data_options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelHistoricalData {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelRealtimeBars {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHistoricalData {
    pub req_id: i32,
    pub contract: Contract,
    pub end_date_time: String,
    pub duration_str: String,
    pub bar_size_setting: String,
    pub what_to_show: String,
    pub use_rth: i32,
    pub format_date: i32,
    pub keepup_to_date: bool,
    pub chart_options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHeadTimestamp {
    pub req_id: i32,
    pub contract: Contract,
    pub what_to_show: String,
    pub use_rth: i32,
    pub format_date: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelHeadTimestamp {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqRealtimeBars {
    pub req_id: i32,
    pub contract: Contract,
    pub bar_size: i32,
    pub what_to_show: String,
    pub use_rth: bool,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqContractDetails {
    pub req_id: i32,
    pub contract: Contract,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMktDepth {
    pub req_id: i32,
    pub contract: Contract,
    pub num_rows: i32,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelMktData {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelMktDepth {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ExerciseOptions {
    pub req_id: i32,
    pub contract: Contract,
    pub exercise_action: i32,
    pub exercise_quantity: i32,
    pub account: String,
    pub overriden: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PlaceOrder {
    pub id: i32,
    pub contract: Contract,
    pub order: Order,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAccountUpdates {
    pub subscribe: bool,
    pub acct_code: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqExecutions {
    pub req_id: i32,
    pub filter: ExecutionFilter,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CacelOrder {
    pub id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqOpenOrders {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqIds {
    pub num_ids: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqNewsBulletins {
    pub all_msgs: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelNewsBulletins {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SetServerLogLevel {
    pub log_level: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAutoOpenOrders {
    pub auto_bind: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAllOpenOrders {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqManagedAccts {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RequestFA {
    pub fa_data_type: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReplaceFA {
    pub fa_data_type: i32,
    pub xml: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqCurrentTime {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqFundamentalData {
    pub req_id: i32,
    pub contract: Contract,
    pub report_type: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelFundamentalData {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CalculateImpliedVolatility {
    pub req_id: i32,
    pub contract: Contract,
    pub option_price: f64,
    pub under_price: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelCalculateImpliedVolatility {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CalculateOptionPrice {
    pub req_id: i32,
    pub contract: Contract,
    pub volatility: f64,
    pub under_price: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelCalculateOptionPrice {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqGlobalCancel {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMarketDataType {
    pub market_data_type: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqPositions {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqSecDefOptParams {
    pub req_id: i32,
    pub underlying_symbol: String,
    pub fut_fop_exchange: String,
    pub underlying_sec_type: String,
    pub underlying_con_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqSoftDollarTiers {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelPositions {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqPositionsMulti {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelPositionsMulti {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelAccountUpdatesMulti {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAccountUpdatesMulti {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
    pub ledger_and_nlv: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAccountSummary {
    pub req_id: i32,
    pub group: String,
    pub tags: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelAccountSummary {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyRequest {
    pub api_name: String,
    pub api_version: String,
    pub extra_auth: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyMessage {
    pub api_data: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerfyAndAuthRequest {
    pub api_name: String,
    pub api_version: String,
    pub opaque_is_vkey: String,
    pub extra_auth: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyAndAuthMessage {
    pub api_data: String,
    pub xyz_response: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryDisplayGroups {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SubscribeToGroupEvent {
    pub req_id: i32,
    pub group_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateDisplayGroup {
    pub req_id: i32,
    pub contract_info: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UbsubscribeFromGroupEvents {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MatchingSymbol {
    pub req_id: i32,
    pub pattern: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqFamilyCodes {
    pub server_version: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMktDepthExchanges {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqSmartComponents {
    pub req_id: i32,
    pub bbo_exchange: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqNewsProvider {
    pub server_version: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqNewsArticle {
    pub req_id: i32,
    pub provider_code: String,
    pub article_id: String,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHistoricalNews {
    pub req_id: i32,
    pub con_id: i32,
    pub provider_code: String,
    pub start_time: String,
    pub end_time: String,
    pub total_results: i32,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHistogramData {
    pub req_id: i32,
    pub contract: Contract,
    pub use_rth: bool,
    pub time_period: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelHistogramData {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMarketRule {
    pub market_rule_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqPnl {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelPnl {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqPnlSingle {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
    pub con_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelPnlSingle {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHistoricalTicks {
    pub req_id: i32,
    pub contract: Contract,
    pub start_time: String,
    pub end_time: String,
    pub num_of_ticks: i32,
    pub what_to_show: String,
    pub use_rth: i32,
    pub ignore_size: bool,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqTickByTickData {
    pub req_id: i32,
    pub contract: Contract,
    pub tick_type: String,
    pub num_of_ticks: i32,
    pub ignore_size: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelTickByTickData {
    pub req_id: i32,
}
