#[macro_use]
extern crate tracing;

use std::string::ToString;
use std::time::Duration;

use futures::StreamExt;
use ib_tws_core::domain;
use ib_tws_core::message::{request::*, Response};
use miette::IntoDiagnostic;

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let client = {
        let port = std::env::args()
            .nth(1)
            .and_then(|p| p.parse::<u32>().ok())
            .unwrap_or(4001);
        let transport = ib_tws_tokio::Transport::connect(
            format!("127.0.0.1:{port}").parse().unwrap(),
            Duration::from_secs(5),
        )
        .await
        .into_diagnostic()?;
        ib_tws_core::AsyncClient::setup(transport, 0).await?
    };
    info!(version = client.server_version(), "connected to client");

    let apple = domain::contract::Contract::new_stock("LKE", "ASX", "AUD").unwrap();
    let stock_request = Request::ReqMktData(ReqMktData {
        req_id: 1000,
        contract: apple,
        generic_tick_list: "".to_string(),
        snapshot: false,
        regulatory_snapshot: false,
        mkt_data_options: Vec::new(),
    });

    client.send(stock_request).await.into_diagnostic()?;
    client.response_rx().stream()
        .for_each(move |buf| async move {
            match buf {
                Response::ErrMsgMsg(msg) => warn!("{:#?}", msg),
                buf => info!("buf: {:?}", buf),
            }
        })
        .await;
    Ok(())
}
