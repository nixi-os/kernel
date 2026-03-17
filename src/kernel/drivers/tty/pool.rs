use crate::kernel::drivers::tty::serial::SerialTty;
use crate::kernel::drivers::tty::TtyHandle;

use crate::helpers::*;

use spin::{Mutex, Once};

use alloc::boxed::Box;
use alloc::vec::Vec;

/// Global tty pool
pub static POOL: Once<Mutex<TtyPool>> = Once::new();

#[inline]
pub fn init() {
    log!("switching to tty");

    let mutex = POOL.call_once(|| Mutex::new(TtyPool::new()));

    let mut pool = mutex.lock();

    let id = pool.create(TtyHandle::new(Box::new(SerialTty::new())));

    pool.switch(id);
}

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
}

impl core::fmt::Write for TtyPool {
    fn write_str(&mut self, buf: &str) -> Result<(), core::fmt::Error> {
        if let Some(current) = self.current {
            self.handles[current].write_str(buf)?;
        }

        Ok(())
    }
}


