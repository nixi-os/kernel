pub mod drivers;
pub mod mem;
pub mod irq;
pub mod scheduler;
pub mod cpu;

use crate::helpers::*;

use drivers::tty;
use scheduler::{TaskDescriptor, context};

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

// TODO: when we enter usermode we get a page fault with the protection violation flag because the
// task1 function is inside the kernel memory which is mapped in pages with Supervisor-only set,
// if we want to execute code in usermode then we must allocate pages and mark them as accesible by
// usermode(ring 3).
#[inline(never)]
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

    scheduler::with_scheduler(|scheduler| {
        let pid = scheduler.create_proc().expect("unable to create init process");
        let tid = scheduler.create_task(pid, task1 as *const () as u64, 3).expect("unable to create init task");

        scheduler.current = Some(TaskDescriptor::new(pid, tid));
    });

    context::enter_usermode();
}


