#![warn(clippy::pedantic)]

#[macro_use]
extern crate tracing;

pub mod domain;
pub mod message;

#[cfg(feature = "async")]
pub mod async_client;
#[cfg(feature = "async")]
pub use async_client::AsyncClient;
