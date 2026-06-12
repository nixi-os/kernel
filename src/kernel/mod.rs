//! The kernel

pub mod arch;
pub mod device;
pub mod drivers;
pub mod fs;
pub mod irq;
pub mod loader;
pub mod mem;
pub mod parse;
pub mod scheduler;
pub mod syscall;
pub mod vfs;

use scheduler::context;

/// Load the init process
pub fn load_init() -> ! {
    if let Err(err) = vfs::init() {
        panic!("failed to initialize: {:?}", err);
    }

    if let Err(err) = loader::load_from_fs("/init") {
        panic!("failed to load init process: {:?}", err);
    }

    /*
    scheduler::with_scheduler(|scheduler| {
        let proc_id = scheduler.create_proc();

        scheduler.create_task(proc_id, task1 as *const () as u64, 3);
    });
    */

    context::enter_usermode();
}
