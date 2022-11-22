#![warn(clippy::pedantic)]

#[macro_use]
extern crate tracing;

pub mod domain;
pub mod message;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error("request channel closed")]
    RequestChannelClosed,
    #[error("response channel closed")]
    ResponseChannelClosed,
    #[error("transport io error: {0}")]
    TransportIo(#[from] std::io::Error),
    #[error("api error: {0:?}")]
    ApiError(message::response::ErrMsgMsg),
}

#[cfg(feature = "async")]
mod async_client;
#[cfg(feature = "async")]
pub use async_client::{SpawnTask, AsyncClient};

