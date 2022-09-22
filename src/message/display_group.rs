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

// Query Display Groups
pub fn encode_query_display_groups(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &QueryDisplayGroups,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(QUERY_DISPLAY_GROUPS);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

// Group ID separated by the "|" character,
//          Example: "4|1|2|5|3|6|7"
pub fn decode_display_group_list_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let groups = buf.read_string()?;

    Ok((
        Response::DisplayGroupListMsg(DisplayGroupListMsg { req_id, groups }),
        req_id,
    ))
}
////////////////////////////////////////////

// Subscribe To Group Events
pub fn encode_subscribe_to_group_event(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &SubscribeToGroupEvent,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(SUBSCRIBE_TO_GROUP_EVENTS);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);
    buf.push_int(req.group_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn encode_unsubscribe_from_group_events(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &UbsubscribeFromGroupEvents,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(UNSUBSCRIBE_FROM_GROUP_EVENTS);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);

    Ok(DispatchId::Oneshot(req.req_id))
}

// contractInfo = The encoded value that uniquely represents the contract in IB. Possible values include:
//     none = empty selection
//     contractID@exchange“ any non-combination contract.
//                          Examples: 8314@SMART for IBM SMART; 8314@ARCA for IBM @ARCA.
//     combo = if any combo is selected.
pub fn decode_display_group_updated_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let _version = buf.read_int()?;
    let req_id = buf.read_int()?;
    let contract_info = buf.read_string()?;

    Ok((
        Response::DisplayGroupUpdatedMsg(DisplayGroupUpdatedMsg {
            req_id,
            contract_info,
        }),
        OPCODE_DISPLAY_GROUP_UPDATED_MSG,
    ))
}

///////////////////////////////////////////////////////////////

// Once you have subscribed to a specific Group,
// you can then have the Group Window in TWS to display a certain contract by invoking update_display_group
pub fn encode_update_display_group(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &UpdateDisplayGroup,
) -> Result<DispatchId, EncodeError> {
    const VERSION: i32 = 1;

    buf.push_int(UPDATE_DISPLAY_GROUP);
    buf.push_int(VERSION);
    buf.push_int(req.req_id);
    buf.push_string(&req.contract_info);

    Ok(DispatchId::Oneshot(req.req_id))
}
