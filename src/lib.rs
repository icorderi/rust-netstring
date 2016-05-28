use ::std::io::{Read, Write, Result, BufWriter};
use ::std::io::{Error, ErrorKind};

// TODO: get rid of this
const DIGIT_LIMIT: usize = 64;

pub trait ReadNetstring {
    fn read_netstring(&mut self) -> Result<String>;
}

pub trait WriteNetstring {
    fn write_netstring<S: AsRef<str>>(&mut self, value: S) -> Result<()>;
}

impl<R: Read> ReadNetstring for R {
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
        if t[0] != b","[0] {
            return Err(Error::new(ErrorKind::InvalidData, "Expected `,` delimiter."));
        }

        // return utf8 string
        match String::from_utf8(data) {
            Ok(s) => Ok(s),
            Err(err) => Err(Error::new(ErrorKind::InvalidData, err)),
        }
    }
}

impl<W: Write> WriteNetstring for W {
    fn write_netstring<S: AsRef<str>>(&mut self, value: S) -> Result<()> {
        let mut w = BufWriter::new(self);

        let value = value.as_ref();
        let s = format!("{}:", value.len());
        try!(w.write(s.as_bytes()));
        try!(w.write(value.as_bytes()));
        try!(w.write(b","));

        try!(w.flush());

        Ok(())
    }
}

fn read_length<R: Read>(r: &mut R) -> Result<usize> {
    let mut t = [0u8; DIGIT_LIMIT];
    let mut current = 0usize;
    let mut done = false;
    while !done {
        let r = try!(r.read(t[current..current+1].as_mut()));
        if r == 0 {
            return Err(Error::new(ErrorKind::ConnectionAborted, "Connection closed by target."));
        }
        if t[current] == 0x3A {
            done = true;
        } else {
            current += 1;
        }
    }

    let s = match String::from_utf8(t[..current].to_vec()) {
                Ok(s)  => s,
                Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
            };

    let ln = match s.parse::<u64>() {
                Ok(x)  => x,
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
}
