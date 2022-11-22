use std::io;

use bytes::{BufMut, BytesMut};
use ib_tws_core::message::{
    context::Context,
    message_codec::{decode_data, decode_head, FrameState, FRAME_HEAD_LEN},
    Request, Response,
};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub struct Codec {
    state: FrameState,
    ctx: Context,
}

impl Codec {
    #[must_use]
    pub fn context(&self) -> &Context {
        &self.ctx
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    fn decode_message(&mut self, data: &mut BytesMut) -> Result<Option<Response>, io::Error> {
        Ok(Some(self.ctx.decode_message(data)?))
    }
}

impl Default for Codec {
    fn default() -> Self {
        Codec {
            state: FrameState::Head,
            ctx: Context::new(),
        }
    }
}

impl Decoder for Codec {
    type Item = Response;
    type Error = io::Error;

    #[instrument(err)]
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        {
            let n = match self.state {
                FrameState::Head => {
                    if let Some(n) = decode_head(src)? {
                        self.state = FrameState::Data(n);
                        n
                    } else {
                        return Ok(None);
                    }
                }
                FrameState::Data(n) => n,
            };

            if let Some(mut data) = decode_data(n, src)? {
                let response = self.decode_message(&mut data)?;
                // trace!(?response, "decoded response");

                self.state = FrameState::Head;

                src.reserve(FRAME_HEAD_LEN);

                Ok(response)
            } else {
                Ok(None)
            }
        }
    }
}

impl Encoder<Request> for Codec {
    type Error = io::Error;

    fn encode(&mut self, request: Request, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let mut req = self.ctx.encode_message(&request)?;
        buf.reserve(req.len() + 4);
        buf.put_u32(req.len().try_into().expect("tried to encode message longer than maximum"));
        buf.put(&mut req);
        trace!(?request, ?buf, "encoded request");
        Ok(())
    }
}
