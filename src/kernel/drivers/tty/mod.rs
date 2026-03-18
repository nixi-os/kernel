pub mod pool;

use crate::kernel::drivers::serial::SerialTty;
use crate::helpers::*;

use alloc::boxed::Box;
use alloc::vec::Vec;


#[inline]
pub fn init() {
    log!("initializing tty with serial port as default");

    pool::init();

    let mut pool = pool::lock();

    let id = pool.create(TtyHandle::new(Box::new(SerialTty::new())));

    pool.switch(id);
}

/// A tty provider, this can be a serial or graphics device
pub trait TtyProvider: Send + Sync {
    fn write(&mut self, buf: &[u8]);
}

impl TtyProvider for Box<dyn TtyProvider> {
    fn write(&mut self, buf: &[u8]) {
        self.as_mut().write(buf);
    }
}

/// A handle to an active tty session
pub struct TtyHandle {
    provider: Box<dyn TtyProvider>,
    stdin: Vec<u8>,
}

impl TtyHandle {
    pub fn new(provider: Box<dyn TtyProvider>) -> TtyHandle {
        TtyHandle {
            provider,
            stdin: Vec::new(),
        }
    }

    /// Push a byte to the stdin queue
    fn push(&mut self, byte: u8) {
        self.stdin.push(byte);
    }
}

impl core::fmt::Write for TtyHandle {
    fn write_str(&mut self, buf: &str) -> Result<(), core::fmt::Error> {
        self.provider.write(buf.as_bytes());

        Ok(())
    }
}


