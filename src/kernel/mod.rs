pub mod drivers;
pub mod mem;
pub mod irq;
pub mod scheduler;
pub mod arch;
pub mod syscall;
pub mod vfs;

use drivers::tty;
use scheduler::{TaskDescriptor, context};


#[inline(never)]
extern "C" fn task1() -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 0x123,
        );
    }

    loop {}
}

pub fn entry() -> ! {
    tty::init();

    // TODO: work we should do before we continue with more file systems:
    // - Make the virtual file system have reference counting for all inodes, so that once an inode
    // no longer has any file handles attached to it, we evict it
    // - Rewrite the scheduler, specifically process and task handling

    scheduler::with_scheduler(|scheduler| {
        let pid = scheduler.create_proc().expect("unable to create init process");
        let tid = scheduler.create_task(pid, task1 as *const () as u64, 3).expect("unable to create init task");

        scheduler.current = Some(TaskDescriptor::new(pid, tid));
    });

    let _ = vfs::init();

    context::enter_usermode();
}


