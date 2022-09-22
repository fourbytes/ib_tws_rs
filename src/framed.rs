use std::fmt;
use std::io::{self, Read, Write};

use tokio_io::{AsyncRead, AsyncWrite};

use super::codec::{Decoder, Encoder};
use bytes::BytesMut;
use futures::{Async, AsyncSink, Poll, Sink, StartSend, Stream};
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

impl<S: AsyncRead, C: Decoder> Stream for Framed<S, C> {
    type Item = C::Item;
    type Error = C::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            // Repeatedly call `decode` or `decode_eof` as long as it is
            // "readable". Readable is defined as not having returned `None`. If
            // the upstream has returned EOF, and the decoder is no longer
            // readable, it can be assumed that the decoder will never become
            // readable again, at which point the stream is terminated.
            if self.is_readable {
                if self.eof {
                    let frame = self.codec.decode_eof(&mut self.read_buf)?;
                    return Ok(Async::Ready(frame));
                }

                if let Some(frame) = self.codec.decode(&mut self.read_buf)? {
                    return Ok(Async::Ready(Some(frame)));
                }

                self.is_readable = false;
            }

            assert!(!self.eof);

            // Otherwise, try to read more data and try again. Make sure we've
            // got room for at least one byte to read to ensure that we don't
            // get a spurious 0 that looks like EOF
            self.read_buf.reserve(1);
            if 0 == try_ready!(self.io.read_buf(&mut self.read_buf)) {
                self.eof = true;
            }

            self.is_readable = true;
        }
    }
}

impl<S: AsyncWrite, C: Encoder> Sink for Framed<S, C> {
    type SinkItem = C::Item;
    type SinkError = C::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        // If the buffer is already over 8KiB, then attempt to flush it. If after flushing it's
        // *still* over 8KiB, then apply backpressure (reject the send).
        if self.write_buf.len() >= BACKPRESSURE_BOUNDARY {
            self.poll_complete()?;

            if self.write_buf.len() >= BACKPRESSURE_BOUNDARY {
                return Ok(AsyncSink::NotReady(item));
            }
        }

        self.codec.encode(item, &mut self.write_buf)?;

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        //trace!("flushing framed transport");

        while !self.write_buf.is_empty() {
            //trace!("writing; remaining={}", self.buffer.len());

            let n = try_ready!(self.io.poll_write(&self.write_buf));

            if n == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to
                                          write frame to transport",
                ).into());
            }

            // TODO: Add a way to `bytes` to do this w/o returning the drained
            // data.
            let _ = self.write_buf.split_to(n);
        }

        // Try flushing the underlying IO
        try_ready!(self.io.poll_flush());

        //trace!("framed transport flushed");
        Ok(Async::Ready(()))
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        try_ready!(self.poll_complete());
        Ok(self.io.shutdown()?)
    }
}

impl<S: Read, C> Read for Framed<S, C> {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        self.io.read(dst)
    }
}

impl<S: AsyncRead, C> AsyncRead for Framed<S, C> {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.io.prepare_uninitialized_buffer(buf)
    }
}

impl<S: Write, C> Write for Framed<S, C> {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        self.io.write(src)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}

impl<S: AsyncWrite, C> AsyncWrite for Framed<S, C> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.io.shutdown()
    }
}
