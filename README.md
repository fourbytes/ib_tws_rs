# Interactive Brokers TWS API for Rust
[![LGPL 3.0](https://img.shields.io/crates/l/ib_tws_core?style=for-the-badge)](https://choosealicense.com/licenses/lgpl-3.0/)

## Usage
```rust
use ib_tws_tokio::Builder;
use ib_tws_core::message::request::*;

let client = Builder::new(0)
	.connect("127.0.0.1:4001".parse().unwrap(), 1)
	.await.into_diagnostic()?;

let (mut sink, stream) = client.split();
sink.send(Request::ReqMktData(ReqMktData {
	req_id: 1000,
	contract: apple,
	generic_tick_list: "".to_string(),
	snapshot: false,
	regulatory_snapshot: false,
	mkt_data_options: Vec::new(),
})).await?;
stream.for_each(move |buf| async move {
	match buf {
		Response::ErrMsgMsg(msg) => warn!("{:#?}", msg),
		buf => info!("buf: {:?}", buf),
	}
}).await;
```

## Crates
### `ib_tws_core`
[![crates.io](https://img.shields.io/crates/v/ib_tws_core?style=for-the-badge)](https://crates.io/crates/ib_tws_core) [![docs.rs](https://img.shields.io/badge/docs.rs-ib_tws_core-rs?style=for-the-badge)](https://docs.rs/ib_tws_core)

Contains core components including messages and encoding/decoding utilities.

### `ib_tws_tokio`
[![crates.io](https://img.shields.io/crates/v/ib_tws_tokio?style=for-the-badge)](https://crates.io/crates/ib_tws_core) [![docs.rs](https://img.shields.io/badge/docs.rs-ib_tws_tokio-rs?style=for-the-badge)](https://docs.rs/ib_tws_tokio)

A client implementation using Tokio.

## Credits
`ib_tws_core` is forked from [chrisdamba's ib_async](https://github.com/chrisdamba/ib_async).
