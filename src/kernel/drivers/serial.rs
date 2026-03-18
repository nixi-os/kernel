//! A serial port driver

use crate::kernel::drivers::tty::TtyProvider;

use x86_64::instructions::interrupts;
use x86::io;

use spin::{Lazy, Mutex};

/// Global serial driver
pub static SERIAL: Lazy<Mutex<Serial>> = Lazy::new(|| Mutex::new(Serial::new(0x3f8)));

/// A serial port
pub struct Serial {
    port: u16,
}

impl Serial {
    pub fn new(port: u16) -> Serial {
        unsafe {
            // check if Recieved Data Available interrupt is set, if not then initialize
            if io::inb(port + 1) & 0x01 == 0 {
                io::outb(port + 1, 0x00);
                io::outb(port + 3, 0x80);
                io::outb(port, 0x03);
                io::outb(port + 1, 0x00);
                io::outb(port + 3, 0x03);
                io::outb(port + 2, 0xc7);
                io::outb(port + 4, 0x0b);
                io::outb(port + 4, 0x0f);

                // enable Recieved Data Available interrupt
                io::outb(port + 1, 0x01);
            }
        }

        Serial {
            port,
        }
    }

    pub fn write(&self, buf: &[u8]) {
        interrupts::without_interrupts(|| {
            for byte in buf {
                unsafe {
                    while io::inb(self.port + 5) & 0x20 == 0 {}

                    io::outb(self.port, *byte);
                }
            }
        });
    }
}

impl core::fmt::Write for Serial {
    fn write_str(&mut self, buf: &str) -> Result<(), core::fmt::Error> {
        self.write(buf.as_bytes());

        Ok(())
    }
}

/// A serial port tty provider. It acts as a passthrough layer, forwarding data directly to the serial port.
pub struct SerialTty;

impl SerialTty {
    pub fn new() -> SerialTty { SerialTty }
}

impl TtyProvider for SerialTty {
    fn write(&mut self, buf: &[u8]) {
        SERIAL.lock().write(buf);
    }
}


