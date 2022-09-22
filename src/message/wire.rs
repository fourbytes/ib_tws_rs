use bytes::BytesMut;
use memchr;
use std::{f64, i32};
use std::io;
use std::str;
use std::str::FromStr;
use std::vec::Vec;

pub(crate) trait TwsWireEncoder {
    fn push_slice(&mut self, extend: &[u8]);

    #[inline(always)]
    fn push_u8(&mut self, v: u8) {
        self.push_slice(&[v]);
    }

    #[inline(always)]
    fn push_string(&mut self, v: &str) {
        self.push_slice(v.as_bytes());
        self.push_u8(0);
    }

    #[inline(always)]
    fn push_bool(&mut self, v: bool) {
        if v {
            self.push_string("1");
        } else {
            self.push_string("0");
        }
    }

    #[inline(always)]
    fn push_int(&mut self, v: i32) {
        self.push_string(&v.to_string());
    }

    #[inline(always)]
    fn push_long(&mut self, v: i64) {
        self.push_string(&v.to_string())
    }

    #[inline(always)]
    fn push_int_max(&mut self, v: i32) {
        if v == i32::MAX {
            self.push_u8(0);
        } else {
            self.push_int(v);
        }
    }

    #[inline(always)]
    fn push_double(&mut self, v: f64) {
        self.push_string(&v.to_string());
    }

    #[inline(always)]
    fn push_double_max(&mut self, v: f64) {
        if v == f64::MAX {
            self.push_u8(0);
        } else {
            self.push_double(v);
        }
    }
}

impl TwsWireEncoder for BytesMut {
    #[inline(always)]
    fn push_slice(&mut self, slice: &[u8]) {
        self.extend_from_slice(slice)
    }
}

impl TwsWireEncoder for Vec<u8> {
    #[inline(always)]
    fn push_slice(&mut self, slice: &[u8]) {
        self.extend_from_slice(slice)
    }
}

pub(crate) trait TwsWireDecoder {
    fn split(&mut self) -> Result<BytesMut, io::Error>;

    fn read_int(&mut self) -> Result<i32, io::Error> {
        let s = self.split()?;
        let s = unsafe { str::from_utf8_unchecked(&s) };
        if s.is_empty() {
            Ok(0i32)
        } else {
            i32::from_str_radix(s, 10).map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                "cannot read i32 from stream",
            ))
        }
    }

    fn read_int_max(&mut self) -> Result<i32, io::Error> {
        let s = self.split()?;
        let s = unsafe { str::from_utf8_unchecked(&s) };
        if s.is_empty() {
            Ok(i32::MAX)
        } else {
            i32::from_str_radix(s, 10).map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                "cannot read i32 from stream",
            ))
        }
    }

    fn read_bool(&mut self) -> Result<bool, io::Error> {
        let v = self.read_int()?;
        if v > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn read_bool_from_str(&mut self) -> Result<bool, io::Error> {
        let s = self.split()?;
        let s = unsafe { str::from_utf8_unchecked(&s) };
        if s == "true" {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn read_long(&mut self) -> Result<i64, io::Error> {
        let s = self.split()?;
        let s = unsafe { str::from_utf8_unchecked(&s) };
        if s.is_empty() {
            Ok(0i64)
        } else {
            i64::from_str_radix(s, 10).map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                "cannot read i64 from stream",
            ))
        }
    }

    fn read_double(&mut self) -> Result<f64, io::Error> {
        let s = self.split()?;
        let s = unsafe { str::from_utf8_unchecked(&s) };
        if s.is_empty() {
            Ok(0.0)
        } else {
            f64::from_str(s).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "cannot read f64 from stream",
                )
            })
        }
    }

    fn read_double_max(&mut self) -> Result<f64, io::Error> {
        let s = self.split()?;
        let s = unsafe { str::from_utf8_unchecked(&s) };
        if s.is_empty() {
            Ok(f64::MAX)
        } else {
            f64::from_str(s).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "cannot read f64 max from stream",
                )
            })
        }
    }

    fn read_string(&mut self) -> Result<String, io::Error> {
        let s = self.split()?;
        str::from_utf8(&s)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, "utf8 error"))
            .map(|s| s.to_string())
        //Ok(s.to_string())
    }
}

impl TwsWireDecoder for BytesMut {
    fn split(&mut self) -> Result<BytesMut, io::Error> {
        let n = memchr::memchr(b'\0', &self);
        match n {
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "cannot find '\0'",
            )),
            Some(index) => {
                let res = self.split_to(index);
                if !self.is_empty() {
                    self.advance(1)
                }; // skip '\0'
                Ok(res)
            }
        }
    }
}
