use std::io;

use bytes::BytesMut;

use super::context::Context;
use super::response::*;
use super::wire::TwsWireDecoder;

pub fn decode_reroute_mkt_depth_req(
    _ctx: &mut Context,
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
    _ctx: &mut Context,
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
