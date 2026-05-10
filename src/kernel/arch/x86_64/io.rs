//! I/O ports enable communication with peripheral devices

use core::arch::asm;


/// Write a byte to an I/O port
#[inline(always)]
pub unsafe fn outb(port: u16, byte: u8) {
    unsafe {
        asm!(
            "outb dx",
            in("al") byte,
            in("dx") port,
        );
    }
}

/// Read a byte from an I/O port
#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let mut byte = 0;

    unsafe {
        asm!(
            "inb dx",
            in("dx") port,
            out("al") byte,
        );
    }

    byte
}


