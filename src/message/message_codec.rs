use super::context::Context;
use super::request::Request;
use super::response::Response;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::{error, fmt, io, io::Cursor};
use {Decoder, Encoder};
use super::constants::MAX_MSG_LENGTH;

const FRAME_HEAD_LEN: usize = 4;

#[derive(Debug, Clone, Copy)]
enum FrameState {
    Head,
    Data(usize),
}

#[derive(Debug)]
pub struct MessageCodec {
    state: FrameState,
    ctx: Context,
}

impl MessageCodec {
    pub(crate) fn new() -> Self {
        MessageCodec {
            state: FrameState::Head,
            ctx: Context::new(),
        }
    }

    pub fn context(&self) -> &Context {
        &self.ctx
    }

    pub fn context_mut(&mut self) -> &mut Context { &mut self.ctx }

    fn decode_head(&mut self, src: &mut BytesMut) -> io::Result<Option<usize>> {
        if src.len() < FRAME_HEAD_LEN {
            return Ok(None);
        }

        let n = Cursor::new(&*src).get_u32_be() as usize;

        if n > MAX_MSG_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                FrameTooBigError { current_size: n },
            ));
        }

        src.split_to(FRAME_HEAD_LEN);

        src.reserve(n);

        Ok(Some(n as usize))
    }

    fn decode_data(&self, n: usize, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        if src.len() < n {
            return Ok(None);
        }

        Ok(Some(src.split_to(n)))
    }

    fn decode_message(&mut self, data: &mut BytesMut) -> Result<Option<Response>, io::Error> {
         Ok(Some(self.ctx.decode_message(data)? ))
    }
}

impl Decoder for MessageCodec {
    type Item = Response;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        {
            let n = match self.state {
                FrameState::Head => match self.decode_head(src)? {
                    Some(n) => {
                        self.state = FrameState::Data(n);
                        n
                    }
                    None => return Ok(None),
                },
                FrameState::Data(n) => n,
            };

            match self.decode_data(n, src)? {
                Some(mut data) => {
                    let response = self.decode_message(&mut data)?;

                    self.state = FrameState::Head;

                    src.reserve(FRAME_HEAD_LEN);

                    Ok(response)
                }
                None => Ok(None),
            }
        }
    }
}

impl Encoder for MessageCodec {
    type Item = Request;
    type Error = io::Error;

    fn encode(&mut self, request: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let req = self.ctx.encode_message(&request)?;
        buf.reserve(req.len() + 4);
        buf.put_u32_be(req.len() as u32);
        buf.put(&req);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FrameTooBigError {
    pub current_size: usize,
}

impl fmt::Display for FrameTooBigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "frame size:{} large than max:{}",
            self.current_size, MAX_MSG_LENGTH
        )
    }
}

impl error::Error for FrameTooBigError {
    fn description(&self) -> &str {
        "frame size too large than max size"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
