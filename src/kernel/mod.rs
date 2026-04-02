pub mod drivers;
pub mod mem;
pub mod irq;
pub mod scheduler;
pub mod cpu;

use crate::helpers::*;

use drivers::tty;
use scheduler::context;

// TODO: decide whether we should move all error types into a separate crate, this would be useful
// because it would mean that we wouldnt have redefine error types for usermode programs

#[inline(never)]
fn example_fn(i: usize) -> bool {
    if i > 2048 {
        i % 32 == 0
    } else {
        i % 128 == 0
    }
}

fn task1() -> ! {
    loop {
        for i in 0..4096 {
            if example_fn(i) {
                log!("hello from task1");
            }
        }
    }
}

pub fn entry() -> ! {
    tty::init();

    context::enter_user();

    // TODO: when we have a file system we should load an init process from the file system here.
    //
    // the init process should preferably kill the kernel task once its ready, this is because
    // there is no reason for the kernel to sit and busy loop doing nothing, while eating cpu time
    // which could have been used by other tasks.
}


