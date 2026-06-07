pub mod arch;
pub mod device;
pub mod drivers;
pub mod fs;
pub mod irq;
pub mod mem;
pub mod parse;
pub mod scheduler;
pub mod syscall;
pub mod vfs;

use scheduler::context;
use vfs::OwnedPath;

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

// TODO: at this point our goal is to create an init process and for this init process to be able
// to mount all of the needed file systems. after we have successfully managed to create an init
// process, the next step will be implementing a block device, and a file system for that block
// device eg. FAT32, ext3 or something
//
// we will do an initramfs type approach, where we have a read only file system at some predefined
// location, then the init process which is contained by it will load the actual block device which
// will be used
//
// we will also have to implement an ELF loader
//
// TODO: we will have to do device discovery

/// Load the init process
pub fn load_init() -> ! {
    if let Err(err) = vfs::init() {
        panic!("failed to initialize: {:?}", err);
    }

    vfs::with_vfs(|vfs| {
        let fd = vfs.open(OwnedPath::from("/init"));
    });

    scheduler::with_scheduler(|scheduler| {
        let proc_id = scheduler.create_proc();

        scheduler.create_task(proc_id, task1 as *const () as u64, 3);
    });

    context::enter_usermode();
}
