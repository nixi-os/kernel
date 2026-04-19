pub mod drivers;
pub mod mem;
pub mod irq;
pub mod scheduler;
pub mod arch;
pub mod syscall;

use crate::helpers::*;

use drivers::tty;
use scheduler::{TaskDescriptor, context};


#[inline(never)]
extern "C" fn task1() -> ! {
    unsafe {
        // TODO: after the syscall, we can see that the CS(code segment) and SS(stack segment) are
        // wrong, they should be cs: 0x1b, ss: 0x23, instead they are cs: 0xb, ss: 0x13
        core::arch::asm!(
            "syscall",
            in("rax") 0x123,
        );
    }

    loop {}
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


