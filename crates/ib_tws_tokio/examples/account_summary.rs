#[macro_use]
extern crate tracing;

use std::net::SocketAddr;
use std::string::ToString;
use std::time::Duration;

use futures::{SinkExt, StreamExt};
use ib_tws_core::domain;
use ib_tws_core::domain::contract::Contract;
use ib_tws_core::message::{request::*, Response};
use ib_tws_tokio::Builder;
use miette::IntoDiagnostic;

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let client = {
        let port = std::env::args().nth(1).and_then(|p| p.parse::<u32>().ok()).unwrap_or(4001);
        let transport = ib_tws_tokio::Transport::connect(
            format!("127.0.0.1:{port}").parse().unwrap(), 
            Duration::from_secs(5)
        ).await.into_diagnostic()?;
        ib_tws_core::AsyncClient::setup(transport, 0).await?
    };

    // info!(version = client.server_version);
    // let response = client.req_account_summary(req).await;
    // info!(?response);

    Ok(())
}
