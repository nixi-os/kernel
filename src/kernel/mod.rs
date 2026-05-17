pub mod drivers;
pub mod mem;
pub mod irq;
pub mod scheduler;
pub mod arch;
pub mod syscall;
pub mod vfs;
pub mod fs;

use scheduler::context;

#[inline(never)]
extern "C" fn task1() -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1,
            in("rdx") 67,
        );
    }

    loop {}
}

pub fn entry() -> ! {
    scheduler::with_scheduler(|scheduler| {
        let proc_id = scheduler.create_proc();

        scheduler.create_task(proc_id, task1 as *const () as u64, 3);
    });

    if let Err(err) = vfs::init() {
        panic!("failed to initialize: {:?}", err);
    }

    context::enter_usermode();
}


