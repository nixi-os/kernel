use crate::kernel::drivers::tty::TtyHandle;
use crate::helpers::*;

use spin::{Mutex, MutexGuard, Once};

use alloc::vec::Vec;

/// Global tty pool
static POOL: Once<Mutex<TtyPool>> = Once::new();

#[inline]
pub fn init() {
    log!("initializing tty pool");

    POOL.call_once(|| Mutex::new(TtyPool::new()));
}

#[inline]
pub fn lock() -> MutexGuard<'static, TtyPool> {
    POOL.wait().lock()
}

#[inline]
pub fn get() -> Option<MutexGuard<'static, TtyPool>> {
    POOL.get().map(|mutex| mutex.lock())
}

/// A pool of active tty sessions
pub struct TtyPool {
    handles: Vec<TtyHandle>,
    current: Option<usize>,
}

impl TtyPool {
    pub fn new() -> TtyPool {
        TtyPool {
            handles: Vec::new(),
            current: None,
        }
    }

    pub fn create(&mut self, tty: TtyHandle) -> usize {
        self.handles.push(tty);

        self.handles.len() - 1
    }

    pub fn switch(&mut self, index: usize) {
        self.current = Some(index);
    }

    /// Push a byte to the stdin queue of the current tty
    pub fn push(&mut self, byte: u8) {
        if let Some(current) = self.current {
            self.handles[current].push(byte);
        }
    }
}

impl core::fmt::Write for TtyPool {
    fn write_str(&mut self, buf: &str) -> Result<(), core::fmt::Error> {
        if let Some(current) = self.current {
            self.handles[current].write_str(buf)?;
        }

        Ok(())
    }
}


