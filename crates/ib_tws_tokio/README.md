<h1 align="center">Interactive Brokers TWS API - Tokio</h1>
<p align="center">
	<a href="https://crates.io/crates/ib_tws_tokio">
		<img src="https://img.shields.io/crates/v/ib_tws_tokio" />
    </a>
	<a href="https://docs.rs/ib_tws_tokio">
		<img src="https://img.shields.io/badge/docs.rs-ib_tws_tokio-rs" />
    </a>
	<img src="https://img.shields.io/crates/l/ib_tws_tokio" />
</p>
<p align="center">
	<a href="https://github.com/fourbytes/ib_tws_rs/tree/main/crates/ib_tws_tokio/examples">Examples</a>
    <!-- &nbsp;&bull;&nbsp; --!>
</p>

A transport implementation using Tokio, intended to be used with the `ib_tws_core::AsyncClient`.

# Usage
```rust
let client = {
	let transport = ib_tws_tokio::Transport::connect(
		"127.0.0.1:4001".parse().unwrap(),
		Duration::from_secs(5),
	)
	.await?;
	ib_tws_core::AsyncClient::setup(transport, 0).await?
};
info!(version = client.server_version(), "connected to client");

```
