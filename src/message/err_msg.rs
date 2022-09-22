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

// 	id: the request identifier which generated the error. Note: -1 will indicate a notification and not true error condition.
// error_code: 	the code identifying the error.
// error_message: error's description.
pub fn decode_err_msg(ctx: &mut Context, buf: &mut BytesMut) -> Result<(Response, i32), io::Error> {
    let version = buf.read_int()?;
    if version < 2 {
        let msg = buf.read_string()?;
        Ok((
            Response::ErrMsgMsg(ErrMsgMsg {
                id: -1,
                error_code: -1,
                error_message: msg,
            }),
            -1,
        ))
    } else {
        let id = buf.read_int()?;
        let error_code = buf.read_int()?;
        let error_message = buf.read_string()?;
        Ok((
            Response::ErrMsgMsg(ErrMsgMsg {
                id,
                error_code,
                error_message,
            }),
            OPCODE_ERR,
        ))
    }
}
