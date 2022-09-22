use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::*;
use std::{f64, i32};
use std::io;

pub fn encode_req_mkt_depth_exchanges(
    ctx: &mut Context,
    buf: &mut BytesMut,
    _req: &ReqMktDepthExchanges,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_MKT_DEPTH_EXCHANGES {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_REQ_MKT_DEPTH_EXCHANGES,
        ));
    }

    buf.push_int(REQ_MKT_DEPTH_EXCHANGES);

    Ok(DispatchId::Global(OPCODE_REQ_MKT_DEPTH_EXCHANGES))
}

pub fn decode_mkt_depth_exchanges_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let count = buf.read_int()?;
    let mut depth_mkt_data_descriptions = Vec::new();
    for _ in 0..count {
        let desc = if ctx.server_version() >= MIN_SERVER_VER_SERVICE_DATA_TYPE {
            let exchange = buf.read_string()?;
            let sec_type = buf.read_string()?;
            let listing_exch = buf.read_string()?;
            let service_data_type = buf.read_string()?;
            let agg_group = buf.read_int_max()?;
            DepthMktDataDescription {
                exchange,
                sec_type,
                listing_exch,
                service_data_type,
                agg_group,
            }
        } else {
            let exchange = buf.read_string()?;
            let sec_type = buf.read_string()?;
            let listing_exch = "".to_string();
            let deep_bool = buf.read_bool()?;
            let service_data_type = if deep_bool {
                "Deep2".to_string()
            } else {
                "Deep".to_string()
            };
            let agg_group = i32::MAX;
            DepthMktDataDescription {
                exchange,
                sec_type,
                listing_exch,
                service_data_type,
                agg_group,
            }
        };

        depth_mkt_data_descriptions.push(desc);
    }

    Ok((
        Response::MktDepthExchangesMsg(MktDepthExchangesMsg {
            depth_mkt_data_descriptions,
        }),
        OPCODE_REQ_MKT_DEPTH_EXCHANGES,
    ))
}
