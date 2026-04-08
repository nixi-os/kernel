use crate::helpers::*;

use crate::kernel::arch::x86_64::io;

const MASTER: u16 = 0x20;
const SLAVE: u16 = 0xa0;
const WAIT: u16 = 0x80;

/// Initialize 8259A interrupt controller
pub fn init(offset: u8) {
    assert_eq!(offset % 8, 0);

    log!("initializing 8259A interrupt controller with irq offset: {}", offset);

    unsafe {
        // ICW1: identify ICW1 and enable ICW4
        io::outb(MASTER, 0b0001_0001);
        io::outb(WAIT, 0);
        io::outb(SLAVE, 0b0001_0001);
        io::outb(WAIT, 0);

        // ICW2: set base vector offset, master and slave have contiguous offsets
        io::outb(MASTER + 1, offset);
        io::outb(WAIT, 0);
        io::outb(SLAVE + 1, offset + 8);
        io::outb(WAIT, 0);

        // ICW3: notify master that IRQ2 has slave attached
        io::outb(MASTER + 1, 0b0000_0100);
        io::outb(WAIT, 0);

        // ICW3: attach slave to IRQ2
        io::outb(SLAVE + 1, 2);
        io::outb(WAIT, 0);

        // ICW4: enable 8086 mode
        io::outb(MASTER + 1, 0b0000_0001);
        io::outb(WAIT, 0);
        io::outb(SLAVE + 1, 0b0000_0001);
        io::outb(WAIT, 0);
    }
}

/// Set the interrupt mask
pub fn mask(mask: u16) {
    unsafe {
        io::outb(MASTER + 1, (mask & 0xff) as u8);
        io::outb(SLAVE + 1, ((mask >> 8) & 0xff) as u8);
    }
}

/// Notify end of interrupt
pub extern "C" fn end_of_interrupt(irq: u8) {
    unsafe {
        if irq > 8 {
            io::outb(SLAVE, 0b0010_0000);
        }

        io::outb(MASTER, 0b0010_0000);
    }
}


