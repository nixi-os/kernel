pub mod drivers;
pub mod mem;
pub mod irq;
pub mod scheduler;
pub mod arch;

use crate::helpers::*;

use drivers::tty;
use scheduler::{TaskDescriptor, context};

use mem::paging::*;

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
                // TODO: the log function will deadlock if we run it inside a task
                // log!("hello from task1");
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

    // TODO: during the boot phase we will have to create a kernel page table, this table will
    // simply map high memory to the address which is marked as code in the memory map from uefi
    //
    // we will make it a simple 1gig mapping, we will just copy paste it into all userspace page
    // tables too so that we dont have to switch when we are in kernel mode

    // TODO: in order to make the transition to paging easier, we should start by making it so that
    // both the kernel and the userspace is identity mapped

    context::enter_usermode();
}


