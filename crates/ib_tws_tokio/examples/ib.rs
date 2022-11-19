use std::net::SocketAddr;
use std::string::ToString;

use futures::StreamExt;
use ib_tws_tokio::TwsClientBuilder;
use ib_tws_core::domain;
use ib_tws_core::message::request::*;
use miette::IntoDiagnostic;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let port = std::env::args().nth(1).unwrap_or_default();
    let port = port.parse::<u32>().unwrap_or(7497);
    let addr = format!("{}:{}", "127.0.0.1", port);
    let addr = addr.parse::<SocketAddr>().unwrap();
    let builder = TwsClientBuilder::new(0);
    let apple = domain::contract::Contract::new_stock("AAPL", "SMART", "USD").unwrap();
    let eur_gbp = domain::contract::Contract::new_forex("EUR.GBP").unwrap();
    let stock_request = Request::ReqMktData(ReqMktData {
        req_id: 1000,
        contract: apple,
        generic_tick_list: "".to_string(),
        snapshot: false,
        regulatory_snapshot: false,
        mkt_data_options: Vec::new(),
    });

    let forex_request = Request::ReqMktData(ReqMktData {
        req_id: 1001,
        contract: eur_gbp,
        generic_tick_list: "".to_string(),
        snapshot: false,
        regulatory_snapshot: false,
        mkt_data_options: Vec::new(),
    });

    let client = builder
        .connect(addr, 0)
        .await.into_diagnostic()?;

    println!("version:{}", client.server_version);
    client.send_request(stock_request);
    client.send_request(forex_request);
    client.for_each(move |buf| async move {
        println!("buf: {:?}", buf);
    }).await;
    Ok(())
}
