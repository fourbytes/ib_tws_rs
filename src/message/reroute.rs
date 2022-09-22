use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::*;
use std::io;

pub fn decode_reroute_mkt_depth_req(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let con_id = buf.read_int()?;
    let exchange = buf.read_string()?;

    Ok((
        Response::RerouteMktDepthReq(RerouteMktDepthReq {
            req_id,
            con_id,
            exchange,
        }),
        req_id,
    ))
}

pub fn decode_reroute_mkt_data_req(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let con_id = buf.read_int()?;
    let exchange = buf.read_string()?;

    Ok((
        Response::RerouteMktDataReq(RerouteMktDataReq {
            req_id,
            con_id,
            exchange,
        }),
        req_id,
    ))
}
