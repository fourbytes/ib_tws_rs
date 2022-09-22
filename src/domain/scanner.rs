use std::default::Default;
use std::{f64, i32};

#[derive(Debug, Clone)]
pub struct ScannerSubscription {
    // The number of rows to be returned for the query.
    pub number_of_rows: i32,
    // The instrument's type for the scan. I.e. STK, FUT.HK, etc.
    pub instrument: String,
    // The request's location (STK.US, STK.US.MAJOR, etc).
    pub location_code: String,
    // Same as TWS Market Scanner's "parameters" field, for example: TOP_PERC_GAIN.
    pub scan_code: String,
    // Filters out Contracts which price is below this value.
    pub above_price: f64,
    // Filters out contracts which price is above this value.
    pub below_price: f64,
    // Filters out Contracts which volume is above this value.
    pub above_volume: i32,
    // Filters out Contracts which option volume is above this value.
    pub average_option_volume_above: i32,
    // Filters out Contracts which market cap is above this value.
    pub market_cap_above: f64,
    // Filters out Contracts which market cap is below this value.
    pub market_cap_below: f64,
    // Filters out Contracts which Moody's rating is below this value.
    pub moody_rating_above: String,
    // Filters out Contracts which Moody's rating is above this value.
    pub moody_rating_below: String,
    // Filters out Contracts with a S&P rating below this value.
    pub sp_rating_above: String,
    // Filters out Contracts with a S&P rating above this value.
    pub sp_rating_below: String,
    // Filter out Contracts with a maturity date earlier than this value.
    pub maturity_date_above: String,
    // Filter out Contracts with a maturity date older than this value.
    pub maturity_date_below: String,
    // Filter out Contracts with a coupon rate lower than this value.
    pub coupon_rate_above: f64,
    // Filter out Contracts with a coupon rate higher than this value.
    pub coupon_rate_below: f64,
    // Filters out Convertible bonds.
    pub exclude_convertible: String,
    // For example, a pairing "Annual, true" used on the "top Option Implied Vol % Gainers" scan would return annualized volatilities.
    pub scanner_setting_pairs: String,
    // CORP = Corporation ADR = American Depositary Receipt ETF = Exchange Traded Fund REIT = Real Estate Investment Trust CEF = Closed End Fund
    pub stock_type_filter: String,
}

const NO_ROW_NUMBER_SPECIFIED: i32 = -1;

impl Default for ScannerSubscription {
    fn default() -> ScannerSubscription {
        ScannerSubscription {
            number_of_rows: NO_ROW_NUMBER_SPECIFIED,
            instrument: "".to_string(),
            location_code: "".to_string(),
            scan_code: "".to_string(),
            above_price: f64::MAX,
            below_price: f64::MAX,
            above_volume: i32::MAX,
            average_option_volume_above: i32::MAX,
            market_cap_above: f64::MAX,
            market_cap_below: f64::MAX,
            moody_rating_above: "".to_string(),
            moody_rating_below: "".to_string(),
            sp_rating_above: "".to_string(),
            sp_rating_below: "".to_string(),
            maturity_date_above: "".to_string(),
            maturity_date_below: "".to_string(),
            coupon_rate_above: f64::MAX,
            coupon_rate_below: f64::MAX,
            exclude_convertible: "".to_string(),
            scanner_setting_pairs: "".to_string(),
            stock_type_filter: "".to_string(),
        }
    }
}
