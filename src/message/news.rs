use super::constants::*;
use super::context::{Context, DispatchId};
use super::error::EncodeError;
use super::request::*;
use super::response::*;
use super::util::*;
use super::util::*;
use super::wire::{TwsWireDecoder, TwsWireEncoder};
use bytes::{BufMut, BytesMut};
use domain::*;
use std::io;

// Checking Subscribed News Sources
pub fn encode_req_news_provider(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqNewsProvider,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_NEWS_PROVIDERS {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_REQ_NEWS_PROVIDERS,
        ));
    }

    buf.push_int(REQ_NEWS_PROVIDERS);

    Ok(DispatchId::Global(OPCODE_REQ_NEWS_PROVIDERS))
}

pub fn decode_news_providers_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let count = buf.read_int()?;
    let mut providers = Vec::new();

    for _ in 0..count {
        let code = buf.read_string()?;
        let name = buf.read_string()?;
        providers.push(NewsProvider { code, name });
    }
    Ok((
        Response::NewsProviderMsg(NewsProviderMsg { providers }),
        OPCODE_REQ_NEWS_PROVIDERS,
    ))
}

///////////////////////////////////////////////////////////////////////////
// Requesting News Articles
pub fn encode_req_news_article(
    ctx: &mut Context,
    buf: &mut BytesMut,
    req: &ReqNewsArticle,
) -> Result<DispatchId, EncodeError> {
    if ctx.server_version() < MIN_SERVER_VER_REQ_NEWS_ARTICLE {
        return Err(EncodeError::VersionLessError(
            MIN_SERVER_VER_REQ_NEWS_ARTICLE,
        ));
    }

    buf.push_int(REQ_NEWS_ARTICLE);
    buf.push_int(req.req_id);
    buf.push_string(&req.provider_code);
    buf.push_string(&req.article_id);

    if ctx.server_version() >= MIN_SERVER_VER_NEWS_QUERY_ORIGINS {
        encode_tagvalue_as_string(buf, &req.options);
    }

    Ok(DispatchId::Oneshot(req.req_id))
}

pub fn decode_news_article_msg(
    ctx: &mut Context,
    buf: &mut BytesMut,
) -> Result<(Response, i32), io::Error> {
    let req_id = buf.read_int()?;
    let article_type = buf.read_int()?;
    let article_text = buf.read_string()?;

    Ok((
        Response::NewsArticleMsg(NewsArticleMsg {
            req_id,
            article_type,
            article_text,
        }),
        req_id,
    ))
}
