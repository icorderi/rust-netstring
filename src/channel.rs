use ::std::io::ErrorKind as IOErrorKind;
use ::std::sync::Arc;
use ::std::sync::mpsc::{sync_channel, SyncSender, SendError, RecvError};
use ::std::sync::atomic::{AtomicBool, Ordering};
use ::std::thread;
use ::std::error::Error;
use ::std::net::Shutdown;

use {ReadNetstring, WriteNetstring};

#[derive(Clone)]
pub struct Channel {
    outgoing: SyncSender<Op>,
    stop: Arc<AtomicBool>,
}

#[derive(Debug)]
pub enum ChannelError {
    ChannelClosed(Option<String>),
}

enum Op {
    Flush(SyncSender<()>),
    Message(String),
    Last(String, SyncSender<()>),
}

impl Channel {
    pub fn map<R, W, F, U>(mut reader: R,
                           mut writer: W,
                           incoming: SyncSender<U>,
                           outgoing_capacity: usize,
                           f: F)
                           -> Self
        where R: ReadNetstring + Send + 'static,
              W: WriteNetstring + Send + 'static,
              F: Fn(String) -> Option<U> + Send + 'static,
              U: Send + 'static
    {
        let (out_tx, out_rx) = sync_channel(outgoing_capacity);

        let stop = Arc::new(AtomicBool::new(false));

        // Reader
        {
            let stop = stop.clone();
            thread::spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    trace!("Waiting for result...");
                    match reader.read_netstring() {
                        Ok(msg) => {
                            if let Some(x) = f(msg) {
                                if let Err(_) = incoming.send(x) {
                                    // This can happen when *incoming* is dropped
                                    debug!("Received message but nobody is listening");
                                    reader.shutdown(Shutdown::Both).ok();
                                    break;
                                }
                            } // else - we were told to ignore the msg
                        }
                        Err(ref err) if err.kind() == IOErrorKind::ConnectionAborted => {
                            debug!("Connection aborted, closing reader");
                            reader.shutdown(Shutdown::Both).ok();
                            break;
                        }
                        Err(err) => {
                            error!("Error reading netstring from socket. {}", err);
                            reader.shutdown(Shutdown::Both).ok();
                            break;
                        }
                    }
                }
                trace!("Reader loop ended");
            });
        }

        // Writer
        {
            let stop = stop.clone();
            thread::spawn(move || {
                loop {
                    match out_rx.recv() {
                        Ok(Op::Message(msg)) => {
                            trace!("Writing message...");
                            writer.write_netstring(msg).expect("Failed to write netstring");
                        }
                        Ok(Op::Flush(c)) => {
                            trace!("Flushed");
                            if let Ok(_) = writer.flush() {
                                c.send(()).ok();
                            }
                        }
                        Ok(Op::Last(msg, c)) => {
                            // Signal stop to reader
                            stop.store(true, Ordering::Relaxed);
                            trace!("Writing *last* message...");
                            writer.write_netstring(msg).expect("Failed to write netstring");
                            writer.flush().expect("Failed to flush on last message");
                            if let Ok(_) = writer.flush() {
                                c.send(()).ok();
                            }
                            // We are leaving without shutting down
                            break;
                        }
                        Err(_) => {
                            trace!("Channel closed");
                            writer.shutdown(Shutdown::Both).ok();
                            break;
                        }
                    }
                }
                trace!("Writer loop ended");
            });
        }

        Channel { outgoing: out_tx, stop: stop }
    }

    pub fn new<R, W>(reader: R, writer: W, incoming: SyncSender<String>, outgoing_capacity: usize) -> Self
        where R: ReadNetstring + Send + 'static,
              W: WriteNetstring + Send + 'static
    {
        Channel::map(reader, writer, incoming, outgoing_capacity, |x| Some(x))
    }

    pub fn send<S: Into<String>>(&self, msg: S) -> Result<(), ChannelError> {
        try!(self.outgoing.send(Op::Message(msg.into())));
        Ok(())
    }

    /// Flushes all pending operations
    pub fn flush(&self) -> Result<(), ChannelError> {
        let (tx, rx) = sync_channel(1);
        try!(self.outgoing.send(Op::Flush(tx)));
        try!(rx.recv());
        Ok(())
    }

    /// Sends a last message and consumes the channel
    ///
    /// The writer will be flushed before the method returns.
    ///
    /// The following example ilustrates how to exit _cleanly_, this requires the
    /// last message to elicit a response from the remote side:
    ///
    /// ```
    /// use netstring::channel::{Channel, ChannelError};
    /// fn terminate(channel: Channel) -> Result<(), ChannelError> {
    ///     // send some messages
    ///     try!(channel.send("hello"));
    ///     try!(channel.send("world"));
    ///     // ..drain responses
    ///     try!(channel.send_last("good bye"));
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// You must make sure all pending responses have been received or you risk them being
    /// dropped.
    pub fn send_last<S: Into<String>>(self, msg: S) -> Result<(), ChannelError> {
        let (tx, rx) = sync_channel(1);
        try!(self.outgoing.send(Op::Last(msg.into(), tx)));
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
            Op::Last(msg, _) => ChannelError::ChannelClosed(Some(msg)),
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
    fn reader() {
        let reader = "5:hello,".as_bytes();
        let (tx, rx) = sync_channel(10);
        let _c = Channel::new(reader, io::sink(), tx, 10);
        let x = rx.recv().unwrap();
        assert_eq!("hello", x);
    }

    #[test]
    fn send() {
        let writer = SyncBuf::new();
        let reader = "".as_bytes();
        let (tx, _) = sync_channel(10);

        let c = Channel::new(reader, writer.clone(), tx, 10);
        c.send("hello").unwrap();
        c.flush().unwrap();

        assert_eq!(writer.bytes(), b"5:hello,");
    }

    #[test]
    fn send_last() {
        let writer = SyncBuf::new();
        let reader = "".as_bytes();
        let (tx, _) = sync_channel(10);

        let c = Channel::new(reader, writer.clone(), tx, 10);
        c.send_last("hello").unwrap();

        assert_eq!(writer.bytes(), b"5:hello,");
    }
}
