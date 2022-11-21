use std::{error, fmt, io, io::Cursor};

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use super::constants::MAX_MSG_LENGTH;
use super::context::Context;
use super::request::Request;
use super::response::Response;

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
    pub fn new() -> Self {
        MessageCodec {
            state: FrameState::Head,
            ctx: Context::new(),
        }
    }

    pub fn context(&self) -> &Context {
        &self.ctx
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    fn decode_head(&mut self, src: &mut BytesMut) -> io::Result<Option<usize>> {
        if src.len() < FRAME_HEAD_LEN {
            return Ok(None);
        }

        let n = Cursor::new(&*src).get_u32() as usize;

        if n > MAX_MSG_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                FrameTooBigError { current_size: n },
            ));
        }

        let _ = src.split_to(FRAME_HEAD_LEN);

        src.reserve(n);

        Ok(Some(n))
    }

    fn decode_data(&self, n: usize, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        if src.len() < n {
            return Ok(None);
        }

        Ok(Some(src.split_to(n)))
    }

    fn decode_message(&mut self, data: &mut BytesMut) -> Result<Option<Response>, io::Error> {
        Ok(Some(self.ctx.decode_message(data)?))
    }
}

impl Decoder for MessageCodec {
    type Item = Response;
    type Error = io::Error;

    #[instrument(err)]
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            //trace!("decoding empty message");
            return Ok(None);
        }
        // info!("decoding");
        {
            // trace!("decode head");
            let n = match self.state {
                FrameState::Head => match self.decode_head(src)? {
                    Some(n) => {
                        self.state = FrameState::Data(n);
                        n
                    }
                    None => {
                        warn!("head none");
                        return Ok(None);
                    }
                },
                FrameState::Data(n) => n,
            };

            // trace!("decode data");
            match self.decode_data(n, src)? {
                Some(mut data) => {
                    let response = self.decode_message(&mut data)?;
                    // trace!(?response, "decoded response");

                    self.state = FrameState::Head;

                    src.reserve(FRAME_HEAD_LEN);

                    Ok(response)
                }
                None => {
                    warn!("data none");
                    Ok(None)
                }
            }
        }
    }
}

impl Encoder<Request> for MessageCodec {
    type Error = io::Error;

    fn encode(&mut self, request: Request, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let mut req = self.ctx.encode_message(&request)?;
        buf.reserve(req.len() + 4);
        buf.put_u32(req.len() as u32);
        buf.put(&mut req);
        trace!(?request, ?buf, "encoded request");
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
