use std::sync::{Arc, RwLock};
use std::io::{self, Write};
use std::net::Shutdown as ShutdownMode;

use Shutdown;

#[derive(Clone)]
pub struct SyncBuf {
    inner: Arc<RwLock<Vec<u8>>>,
}

impl SyncBuf {
    pub fn new() -> Self { SyncBuf { inner: Arc::new(RwLock::new(Vec::new())) } }

    pub fn bytes(&self) -> Vec<u8> {
        let inner = self.inner.read().unwrap();
        inner.clone()
    }
}

impl Write for SyncBuf {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut inner = self.inner.write().unwrap();
        inner.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        let mut inner = self.inner.write().unwrap();
        inner.flush()
    }
}

impl Shutdown for SyncBuf {
    fn shutdown(&self, _how: ShutdownMode) -> io::Result<()> {
        Ok(())
    }
}
