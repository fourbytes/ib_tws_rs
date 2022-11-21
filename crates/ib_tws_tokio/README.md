# IB TWS Tokio
[![crates.io](https://img.shields.io/crates/v/ib_tws_tokio?style=for-the-badge)](https://crates.io/crates/ib_tws_core) [![docs.rs](https://img.shields.io/badge/docs.rs-ib_tws_tokio-rs?style=for-the-badge)](https://docs.rs/ib_tws_tokio) [![LGPL 3.0](https://img.shields.io/crates/l/ib_tws_core?style=for-the-badge)](https://choosealicense.com/licenses/lgpl-3.0/)

An IB TWS API client implementation using the Tokio runtime.

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
