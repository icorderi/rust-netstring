#[macro_use]
extern crate log;

pub mod channel;
#[cfg(test)]
mod syncbuf;

use std::io::{Read, Write, Result};
use std::io::{Error, ErrorKind};
use std::net::Shutdown as ShutdownMode;
use std::ops::Deref;

// TODO: get rid of this
const DIGIT_LIMIT: usize = 64;

pub trait ReadNetstring: Shutdown {
    fn read_netstring(&mut self) -> Result<String>;
}

pub trait WriteNetstring: Shutdown {
    fn write_netstring<S: AsRef<str>>(&mut self, value: S) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
}

pub trait Shutdown {
    fn shutdown(&self, how: ShutdownMode) -> Result<()>;
}

impl<R: Read + Shutdown> ReadNetstring for R {
    fn read_netstring(&mut self) -> Result<String> {
        let ln = try!(read_length(self));
        let mut data = vec![0u8;ln];

        let mut offset = 0usize;
        let mut done = false;

        while !done {
            let r = try!(self.read(data[offset..].as_mut()));
            offset = offset + r;
            if r == 0 || offset == ln {
                done = true;
            }
        }

        // TODO: there has to be a a cleaner way to do this...
        // read delimiter ","
        let mut t = vec![0u8].into_boxed_slice();
        try!(self.read(t[..].as_mut()));

        // Verify delimiter
        if t[0] != b',' {
            return Err(Error::new(ErrorKind::InvalidData, "Expected `,` delimiter."));
        }

        // return utf8 string
        match String::from_utf8(data) {
            Ok(s) => Ok(s),
            Err(err) => Err(Error::new(ErrorKind::InvalidData, err)),
        }
    }
}

impl<W: Write + Shutdown> WriteNetstring for W {
    fn write_netstring<S: AsRef<str>>(&mut self, value: S) -> Result<()> {
        let value = value.as_ref();
        let s = format!("{}:{},", value.len(), value);
        try!(self.write_all(s.as_bytes()));
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        Write::flush(self)
    }
}

impl Shutdown for ::std::os::unix::net::UnixStream {
    fn shutdown(&self, how: ShutdownMode) -> Result<()> {
        ::std::os::unix::net::UnixStream::shutdown(self, how)
    }
}

impl Shutdown for ::std::net::TcpStream {
    fn shutdown(&self, how: ShutdownMode) -> Result<()> {
        ::std::net::TcpStream::shutdown(self, how)
    }
}

impl<T: Shutdown> Shutdown for Box<T> {
    fn shutdown(&self, how: ShutdownMode) -> Result<()> {
        self.deref().shutdown(how)
    }
}

impl<'a> Shutdown for &'a [u8] {
     fn shutdown(&self, _how: ShutdownMode) -> Result<()> {
         Ok(())
     }
}

impl<T> Shutdown for Vec<T> {
     fn shutdown(&self, _how: ShutdownMode) -> Result<()> {
         Ok(())
     }
}

impl Shutdown for ::std::io::Sink {
    fn shutdown(&self, _how: ShutdownMode) -> Result<()> {
        Ok(())
    }
}

fn read_length<R: Read>(r: &mut R) -> Result<usize> {
    let mut t = [0u8; DIGIT_LIMIT];
    let mut current = 0usize;
    let mut done = false;
    while !done {
        let r = try!(r.read(t[current..current + 1].as_mut()));
        if r == 0 {
            return Err(Error::new(ErrorKind::ConnectionAborted, "Connection closed by target."));
        }
        // Reached ":" signaling the end of the length
        if t[current] == b':' {
            done = true;
        } else {
            current += 1;
        }
    }

    let s = match String::from_utf8(t[..current].to_vec()) {
        Ok(s) => s,
        Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
    };

    let ln = match s.parse::<u64>() {
        Ok(x) => x,
        Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
    };


    Ok(ln as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_read() {
        let mut raw = "5:hello,".as_bytes();
        let x = raw.read_netstring().unwrap();
        assert_eq!("hello", x);
    }

    #[test]
    fn basic_write() {
        let mut raw = vec![];
        let _ = raw.write_netstring("hello").unwrap();
        assert_eq!(raw, b"5:hello,");
    }

    #[test]
    #[should_panic(expected="Expected `,` delimiter.")]
    fn invalid_delimiter() {
        let mut raw = "5:hello?".as_bytes();
        raw.read_netstring().unwrap();
    }

    #[test]
    #[should_panic(expected="Expected `,` delimiter.")]
    fn longer() {
        let mut raw = "10:hello,".as_bytes();
        raw.read_netstring().unwrap();
    }

    #[test]
    #[should_panic(expected="Expected `,` delimiter.")]
    fn shorter() {
        let mut raw = "2:hello,".as_bytes();
        raw.read_netstring().unwrap();
    }

    #[test]
    fn multiple() {
        let mut raw = "5:hello,5:world,10:xxxxxxxxxx,".as_bytes();
        let x1 = raw.read_netstring().unwrap();
        let x2 = raw.read_netstring().unwrap();
        let x3 = raw.read_netstring().unwrap();
        assert_eq!(x1, "hello");
        assert_eq!(x2, "world");
        assert_eq!(x3, "xxxxxxxxxx");
    }
}
