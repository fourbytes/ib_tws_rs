use super::context::Context;
use super::request::Request;
use super::response::Response;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::{error, fmt, io, io::Cursor};
use tokio_codec::{Decoder, Encoder};

const FRAME_HEAD_LEN: usize = 4;
const MAX_FRAME_LENGTH: usize = 0xff_ffff;

#[derive(Debug, Clone, Copy)]
enum FrameState {
    Head,
    Data(usize),
}

#[derive(Debug)]
pub struct LengthCodec {
    state: FrameState,
}

impl LengthCodec {
    pub(crate) fn new() -> Self {
        LengthCodec {
            state: FrameState::Head,
        }
    }

    fn decode_head(&mut self, src: &mut BytesMut) -> io::Result<Option<usize>> {
        if src.len() < FRAME_HEAD_LEN {
            return Ok(None);
        }

        let n = Cursor::new(&*src).get_u32_be() as usize;

        if n > MAX_FRAME_LENGTH {
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
}

impl Encoder for LengthCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn encode(&mut self, req: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        buf.reserve(req.len() + 4);
        buf.put_u32_be(req.len() as u32);
        buf.put(&req);
        Ok(())
    }
}

impl Decoder for LengthCodec {
    type Item = BytesMut;
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
                Some(data) => {
                    self.state = FrameState::Head;

                    src.reserve(FRAME_HEAD_LEN);

                    Ok(Some(data))
                }
                None => Ok(None),
            }
        }
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
            self.current_size, MAX_FRAME_LENGTH
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
