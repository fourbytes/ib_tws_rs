#[macro_use]
extern crate tracing;

pub mod client;
pub use client::Client;

pub mod builder;
pub use builder::{Builder, FramedStream};

pub mod task;

pub mod transport;
pub use transport::Transport;

// #[derive(Debug, thiserror::Error, miette::Diagnostic)]
// pub enum Error {
//     #[error("io error: {0}")]
//     Io(#[from] std::io::Error),
// }
