#[macro_use]
extern crate tracing;

use std::time::Duration;

use futures::StreamExt;
use ib_tws_core::message::request::ReqAccountSummary;
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

    info!(version = client.server_version());
    for account in client.managed_accounts().await {
        let mut stream = Box::pin(client.request_account_summary(ReqAccountSummary::new("All".to_owned(), "$LEDGER:ALL".to_owned())).await?);
        while let Some(response) = stream.next().await {
            info!(?response);
        }
    }

    Ok(())
}
