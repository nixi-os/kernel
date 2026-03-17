pub mod pool;
mod serial;

use alloc::boxed::Box;
use alloc::vec::Vec;


/// A tty is a terminal interface
pub trait Tty: Send + Sync {
    fn write(&mut self, buf: &[u8]);

    fn read(&mut self) -> Vec<u8>;
}

impl Tty for Box<dyn Tty> {
    fn write(&mut self, buf: &[u8]) {
        self.as_mut().write(buf);
    }

    fn read(&mut self) -> Vec<u8> {
        self.as_mut().read()
    }
}

/// Wrapper type for tty to satisfy orphan rules
pub struct TtyHandle(Box<dyn Tty>);

impl TtyHandle {
    pub fn new(tty: Box<dyn Tty>) -> TtyHandle {
        TtyHandle(tty)
    }
}

impl core::fmt::Write for TtyHandle {
    fn write_str(&mut self, buf: &str) -> Result<(), core::fmt::Error> {
        self.0.write(buf.as_bytes());

        Ok(())
    }
}



