use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use std::io;

// IB Bulletins
// From time to time, IB sends out important News Bulletins
pub fn encode_req_news_bulletins(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqNewsBulletins,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(REQ_NEWS_BULLETINS);
    buf.push_int(VERSION);
    buf.push_bool(req.all_msgs);

    Ok(DispatchId::Global(OPCODE_REQ_NEWS_BULLETINS))
}

pub fn encode_cancel_news_bulletins(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &CancelNewsBulletins,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(CANCEL_NEWS_BULLETINS);
    buf.push_int(VERSION);

    Ok(DispatchId::Global(OPCODE_CANCEL_NEWS_BULLETINS))
}

pub fn decode_news_bulletins_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let msg_type = buf.read_int()?;
    let message = buf.read_string()?;
    let originating_exch = buf.read_string()?;

    Ok((
        Response::NewsBulletinsMsg(NewsBulletinsMsg {
            req_id,
            msg_type,
            message,
            originating_exch,
        }),
        req_id,
    ))
}
