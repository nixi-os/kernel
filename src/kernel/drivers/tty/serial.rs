use crate::kernel::drivers::tty::Tty;
use crate::kernel::drivers::serial::SERIAL;

use alloc::vec::Vec;

/// A serial port tty. It acts as a passthrough layer, forwarding data directly to the serial port.
pub struct SerialTty {
    stdin: Vec<u8>,
}

impl SerialTty {
    pub fn new() -> SerialTty {
        SerialTty {
            stdin: Vec::new(),
        }
    }
}

impl Tty for SerialTty {
    fn write(&mut self, buf: &[u8]) {
        SERIAL.lock().write(buf);
    }

    fn read(&mut self) -> Vec<u8> {
        self.stdin.drain(..).collect()
    }
}


