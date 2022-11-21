#![feature(trait_alias)]

#[macro_use]
extern crate tracing;

pub mod domain;
pub mod message;
pub mod channel;
// pub use codec::{Decoder, Encoder};
pub use channel::{TransportChannel, CommandChannel, channel4};

pub mod async_client;
pub use async_client::AsyncClient;
