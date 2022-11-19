use std::fmt;
use std::io::{self, Read, Write};
use std::pin::Pin;
use std::task::{Poll, Context};

use ib_tws_core::codec::{Decoder, Encoder};
use bytes::BytesMut;
use futures::{Sink, Stream, ready, TryFutureExt};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, AsyncReadExt};
use std::fmt::Debug;

/// A unified `Stream` and `Sink` interface to an underlying I/O object, using
/// the `Encoder` and `Decoder` traits to encode and decode frames.
///

const INITIAL_CAPACITY: usize = 8 * 1024;
const BACKPRESSURE_BOUNDARY: usize = INITIAL_CAPACITY;

pub struct Framed<S, C> {
    /// The inner transport used to read bytes to and write bytes to
    pub io: S,

    /// The codec
    pub codec: C,

    /// The buffer with read but unprocessed data.
    pub read_buf: BytesMut,

    pub(crate) eof: bool,

    pub(crate) is_readable: bool,

    /// A buffer with unprocessed data which are not written yet.
    pub write_buf: BytesMut,
}

impl<S, C> Framed<S, C> {
    pub fn new(io: S, codec: C) -> Self {
        Framed {
            io,
            codec,
            read_buf: BytesMut::with_capacity(INITIAL_CAPACITY),
            eof: false,
            is_readable: false,
            write_buf: BytesMut::with_capacity(INITIAL_CAPACITY),
        }
    }

    pub fn get_ref(&self) -> &S {
        &self.io
    }

    /// Returns a mutable reference to the underlying I/O stream wrapped by
    /// `Frame`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.io
    }

    /// Returns a reference to the underlying codec.
    pub fn get_codec(&self) -> &C {
        &self.codec
    }

    /// Returns a mutable reference to the underlying codec.
    pub fn get_codec_mut(&mut self) -> &mut C {
        &mut self.codec
    }

    /// Consumes the `Frame`, returning its underlying I/O stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn into_inner(self) -> S {
        self.io
    }
}

impl<S: Debug, C: Debug> fmt::Debug for Framed<S, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Transport")
            .field("inner", &self.io)
            .field("codec", &self.codec)
            .field("read_buf", &self.read_buf)
            .field("eof", &self.eof)
            .field("is_readable", &self.is_readable)
            .field("write_buf", &self.write_buf)
            .finish()
    }
}

impl<S: AsyncRead + Unpin, C: Decoder + Unpin> Stream for Framed<S, C> {
    type Item = Result<C::Item, C::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let s = unsafe { self.get_unchecked_mut() };
        loop {
            // Repeatedly call `decode` or `decode_eof` as long as it is
            // "readable". Readable is defined as not having returned `None`. If
            // the upstream has returned EOF, and the decoder is no longer
            // readable, it can be assumed that the decoder will never become
            // readable again, at which point the stream is terminated.
            if s.is_readable {
                if s.eof {
                    let frame = s.codec.decode_eof(&mut s.read_buf);
                    return Poll::Ready(Some(frame.map(|f| f.unwrap())));
                }

                {
                    if let Some(frame) = s.codec.decode(&mut s.read_buf)? {
                        return Poll::Ready(Some(Ok(frame)));
                    }
                }

                s.is_readable = false;
            }

            assert!(!s.eof);

            // Otherwise, try to read more data and try again. Make sure we've
            // got room for at least one byte to read to ensure that we don't
            // get a spurious 0 that looks like EOF
            s.read_buf.reserve(1);
            let mut fut = Box::pin(s.io.read_buf(&mut s.read_buf));
            match ready!(fut.try_poll_unpin(cx)) {
                Ok(b) => if 0 == b {
                    s.eof = true;
                },
                Err(_error) => return Poll::Ready(None)
            }

            s.is_readable = true;
        }
    }
}

impl<S: AsyncWrite + Unpin, C: Encoder + Unpin> Sink<C::Item> for Framed<S, C> {
    type Error = C::Error;

    fn start_send(self: Pin<&mut Self>, item: C::Item) -> Result<(), Self::Error> {
        // If the buffer is already over 8KiB, then attempt to flush it. If after flushing it's
        // *still* over 8KiB, then apply backpressure (reject the send).
        if self.write_buf.len() >= BACKPRESSURE_BOUNDARY {
            // self.poll_ready(cx)?;

            if self.write_buf.len() >= BACKPRESSURE_BOUNDARY {
                return Ok(());
            }
        }

        let s = self.get_mut();
        s.codec.encode(item, &mut s.write_buf)?;

        Ok(())
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // try_ready!(self.poll_complete());
        let s = self.get_mut();
        match Box::pin(s.io.shutdown()).try_poll_unpin(cx) {
            Poll::Ready(Err(e)) => Poll::Ready(Ok(())),
            Poll::Ready(Ok(o)) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending
        }
    }

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        //trace!("flushing framed transport");

        let s = self.get_mut();
        while !s.write_buf.is_empty() {
            //trace!("writing; remaining={}", self.buffer.len());

            let mut fut = Box::pin(s.io.write(&s.write_buf));
            let n = match ready!(fut.try_poll_unpin(cx)) {
                Ok(r) => r,
                Err(e) => return Poll::Ready(Ok(())),
            };

            if n == 0 {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to
                                          write frame to transport",
                ).into()))
            }

            // TODO: Add a way to `bytes` to do this w/o returning the drained
            // data.
            let _ = s.write_buf.split_to(n);
        }

        // Try flushing the underlying IO
        let mut fut = Box::pin(s.io.flush());
        ready!(fut.try_poll_unpin(cx));

        //trace!("framed transport flushed");
        Poll::Ready(Ok(()))

    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
}

impl<S: Read, C> Read for Framed<S, C> {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        self.io.read(dst)
    }
}

impl<S: AsyncRead + Unpin, C: Unpin> AsyncRead for Framed<S, C> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        <S as AsyncRead>::poll_read(Pin::new(&mut self.get_mut().io), cx, buf)
    }
    /* unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.io.prepare_uninitialized_buffer(buf)
    } */
}

impl<S: Write, C> Write for Framed<S, C> {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        self.io.write(src)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}

impl<S: AsyncWrite + Unpin, C: Unpin> AsyncWrite for Framed<S, C> {
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        <S as AsyncWrite>::poll_shutdown(Pin::new(&mut self.get_mut().io), cx)
    }

    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        <S as AsyncWrite>::poll_write(Pin::new(&mut self.get_mut().io), cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        <S as AsyncWrite>::poll_flush(Pin::new(&mut self.get_mut().io), cx)
    }
}
