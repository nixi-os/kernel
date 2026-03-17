pub mod drivers;
pub mod mem;
pub mod irq;

use crate::helpers::*;

use drivers::tty::pool;

pub fn entry() -> ! {
    pool::init();

    log!("test tty pool");

    loop {}
}


