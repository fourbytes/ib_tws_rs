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

pub fn encode_req_family_codes(
    ctx: &mut Context,
    buf: &mut BytesMut,
    _req: &ReqFamilyCodes,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_FAMILY_CODES {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_REQ_FAMILY_CODES,
        ));
    }
    buf.push_int(REQ_FAMILY_CODES);

    Ok(DispatchId::Global(OPCODE_REQ_FAMILY_CODES))
}

pub fn decode_family_codes_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let count = buf.read_int()?;
    let mut family_codes = Vec::new();
    for _ in 0..count {
        let account_id = buf.read_string()?;
        let family_code = buf.read_string()?;
        let family = FamilyCode {
            account_id,
            family_code,
        };
        family_codes.push(family);
    }

    Ok((
        Response::FamilyCodesMsg(FamilyCodesMsg { family_codes }),
        OPCODE_REQ_FAMILY_CODES,
    ))
}
