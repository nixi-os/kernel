pub mod drivers;
pub mod mem;
pub mod irq;
pub mod scheduler;
pub mod cpu;

use crate::helpers::*;

use drivers::tty;

pub fn entry() -> ! {
    tty::init();

    log!("test tty");

    loop {}
}


