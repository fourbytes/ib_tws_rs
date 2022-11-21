use std::{error, fmt, io, io::Cursor};

use bytes::{Buf, BytesMut};

use super::constants::MAX_MSG_LENGTH;

pub const FRAME_HEAD_LEN: usize = 4;

#[derive(Debug, Clone, Copy)]
pub enum FrameState {
    Head,
    Data(usize),
}

pub fn decode_head(src: &mut BytesMut) -> io::Result<Option<usize>> {
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

pub fn decode_data(n: usize, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
    if src.len() < n {
        return Ok(None);
    }

    Ok(Some(src.split_to(n)))
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
