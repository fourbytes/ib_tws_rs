use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use std::io;

pub fn encode_verify_request(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &VerifyRequest,
) -> Result<DispatchId, EncodeError> {
    if !ctx.extra_auth() {
        return Err(EncodeError::NeedExtraAuth);
    }

    const VERSION: i32 = 1;

    buf.push_int(VERIFY_REQUEST);
    buf.push_int(VERSION);
    buf.push_string(&req.api_name);
    buf.push_string(&req.api_version);

    Ok(DispatchId::Global(OPCODE_VERIFY_REQUEST))
}

pub fn encode_verify_message(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &VerifyMessage,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(VERIFY_MESSAGE);
    buf.push_int(VERSION);
    buf.push_string(&req.api_data);

    Ok(DispatchId::Global(OPCODE_VERIFY_MESSAGE))
}

pub fn encode_verify_and_auth_request(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &VerfyAndAuthRequest,
) -> Result<DispatchId, EncodeError> {
    if !ctx.extra_auth() {
        return Err(EncodeError::NeedExtraAuth);
    }

    const VERSION: i32 = 1;

    buf.push_int(VERIFY_AND_AUTH_REQUEST);
    buf.push_int(VERSION);
    buf.push_string(&req.api_name);
    buf.push_string(&req.api_version);
    buf.push_string(&req.opaque_is_vkey);

    Ok(DispatchId::Global(OPCODE_VERIFY_AND_AUTH_REQUEST))
}

pub fn encode_verify_and_auth_message(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &VerifyAndAuthMessage,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(VERIFY_AND_AUTH_MESSAGE);
    buf.push_int(VERSION);
    buf.push_string(&req.api_data);
    buf.push_string(&req.xyz_response);

    Ok(DispatchId::Global(OPCODE_VERIFY_AND_AUTH_MESSAGE))
}

pub fn decode_verify_and_auth_completed_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let is_successful = buf.read_string()?.to_lowercase() == "true";
    let error_text = buf.read_string()?;

    Ok((
        Response::VerifyAndAuthCompletedMsg(VerifyAndAuthCompletedMsg {
            is_successful,
            error_text,
        }),
        OPCODE_VERIFY_AND_AUTH_REQUEST,
    ))
}

// [NO REQ_ID]
pub fn decode_verify_and_auth_message_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let api_data = buf.read_string()?;
    let xyz_challenge = buf.read_string()?;

    Ok((
        Response::VerifyAndAuthMessageMsg(VerifyAndAuthMessageMsg {
            api_data,
            xyz_challenge,
        }),
        OPCODE_VERIFY_AND_AUTH_MESSAGE,
    ))
}

pub fn decode_verify_message_api_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let api_data = buf.read_string()?;

    Ok((
        Response::VerifyMessageApiMsg(VerifyMessageApiMsg { api_data }),
        OPCODE_VERIFY_MESSAGE,
    ))
}

pub fn decode_verify_completed_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let is_successful = buf.read_string()?.to_lowercase() == "true";
    let error_text = buf.read_string()?;

    Ok((
        Response::VerifyCompletedMsg(VerifyCompletedMsg {
            is_successful,
            error_text,
        }),
        OPCODE_VERIFY_REQUEST,
    ))
}
