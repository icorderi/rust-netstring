use ::std::io::ErrorKind as IOErrorKind;
use ::std::sync::mpsc::{sync_channel, SyncSender, SendError, RecvError};
use ::std::thread;
use ::std::error::Error;

use {ReadNetstring, WriteNetstring};

#[derive(Clone)]
pub struct Channel {
    outgoing: SyncSender<Op>,
}

#[derive(Debug)]
pub enum ChannelError {
    ChannelClosed(Option<String>),
}

enum Op {
    Flush(SyncSender<()>),
    Message(String),
}

impl Channel {
    pub fn new<R, W>(mut reader: R,
                     mut writer: W,
                     incoming: SyncSender<String>,
                     outgoing_capacity: usize)
                     -> Self
        where R: ReadNetstring + Send + 'static,
              W: WriteNetstring + Send + 'static
    {
        let (out_tx, out_rx) = sync_channel(outgoing_capacity);

        let c = Channel { outgoing: out_tx };

        // Reader
        thread::spawn(move || {
            loop {
                trace!("Waiting for result...");
                match reader.read_netstring() {
                    Ok(encoded) => {
                        if let Err(_) = incoming.send(encoded) {
                            // This can happen when *incoming* is dropped
                            debug!("Received message but nobody is listening");
                            // TODO: try to send error to caller
                            break;
                        }
                    }
                    Err(ref err) if err.kind() == IOErrorKind::ConnectionAborted => {
                        debug!("Connection aborted, closing reader");
                        break;
                    }
                    Err(err) => {
                        error!("Error reading netstring from socket. {}", err);
                        break;
                    }
                }
            }
            trace!("Reader loop ended");
            reader
        });

        // Writer
        thread::spawn(move || {
            loop {
                match out_rx.recv() {
                    Ok(Op::Message(msg)) => {
                        trace!("Writing request...");
                        writer.write_netstring(msg).expect("Failed to write netstring");
                    }
                    Ok(Op::Flush(c)) => {
                        trace!("Flushed");
                        c.send(()).ok();
                    }
                    Err(_) => {
                        trace!("Channel closed");
                        break;
                    }
                }
            }
            trace!("Writer loop ended");
            writer
        });

        c
    }

    pub fn send<S: Into<String>>(&self, msg: S) -> Result<(), ChannelError> {
        try!(self.outgoing.send(Op::Message(msg.into())));
        Ok(())
    }

    pub fn flush(&self) -> Result<(), ChannelError> {
        let (tx, rx) = sync_channel(1);
        try!(self.outgoing.send(Op::Flush(tx)));
        try!(rx.recv());
        Ok(())
    }
}

impl ::std::fmt::Display for ChannelError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ref err => {
                match err.cause() {
                    Some(inner) => write!(f, "{} - {}", self.description(), inner),
                    None => write!(f, "{}", self.description()),
                }
            }
        }
    }
}

impl Error for ChannelError {
    fn description(&self) -> &str {
        match *self {
            ChannelError::ChannelClosed(_) => "Channel is closed",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            _ => None,
        }
    }
}

impl From<SendError<Op>> for ChannelError {
    fn from(x: SendError<Op>) -> Self {
        match x.0 {
            Op::Message(msg) => ChannelError::ChannelClosed(Some(msg)),
            Op::Flush(_) => ChannelError::ChannelClosed(None),
        }
    }
}

impl From<RecvError> for ChannelError {
    fn from(_: RecvError) -> Self { ChannelError::ChannelClosed(None) }
}

#[cfg(test)]
mod tests {
    use ::std::sync::mpsc::sync_channel;
    use ::std::io;

    use syncbuf::SyncBuf;

    use super::*;

    #[test]
    fn read() {
        let reader = "5:hello,".as_bytes();
        let (tx, rx) = sync_channel(10);
        let _c = Channel::new(reader, io::sink(), tx, 10);
        let x = rx.recv().unwrap();
        assert_eq!("hello", x);
    }

    #[test]
    fn write() {
        let writer = SyncBuf::new();
        let reader = "".as_bytes();
        let (tx, _) = sync_channel(10);

        let c = Channel::new(reader, writer.clone(), tx, 10);
        c.send("hello").unwrap();
        c.flush().unwrap();

        assert_eq!(writer.bytes(), b"5:hello,");
    }
}
