#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[macro_use]
extern crate approx;
extern crate bit;
extern crate bytes;
extern crate chrono;
//#![feature(uniform_paths)]
//#![feature(await_macro, async_await, futures_api)]
#[macro_use]
extern crate failure;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate memchr;
extern crate ordered_float;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate slog;
#[cfg(test)]
extern crate slog_async;
#[cfg(test)]
extern crate slog_term;
#[macro_use]
extern crate tokio;
//extern crate tokio_codec;
extern crate tokio_io;
extern crate tokio_timer;
extern crate tokio_uds;
#[macro_use]
extern crate diesel;

use builder::FramedStream;
use framed::Framed;
use codec::{Decoder, Encoder};
pub use builder::TwsClientBuilder;
use channel::{CommandChannel, TransportChannel};
pub use client::TwsClient;

//pub mod builder;
pub mod builder;
mod channel;
pub mod client;
pub mod domain;
pub mod message;
mod task;
pub mod framed;
pub mod codec;

