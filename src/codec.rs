use bytes::BytesMut;
use std::io;
use tokio_io::{AsyncRead, AsyncWrite};

pub trait Encoder {
    /// The type of items consumed by the `Encoder`
    type Item;

    type Error: From<io::Error>;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error>;
}
pub trait Decoder {
    /// The type of decoded frames.
    type Item;

    type Error: From<io::Error>;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>;

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.decode(buf)? {
            Some(frame) => Ok(Some(frame)),
            None => {
                if buf.is_empty() {
                    Ok(None)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "bytes remaining on stream").into())
                }
            }
        }
    }
}
