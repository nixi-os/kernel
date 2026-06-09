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

use parse::elf::ElfObject;
use scheduler::context;
use vfs::OwnedPath;
use vfs::error::VfsError;

use alloc::vec;
use alloc::vec::Vec;

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

    // TODO: everything is shit, this is ugly subhuman code. Make it better, clean up, this is horrible

    if let Ok(init_program) = vfs::with_vfs(|vfs| -> Result<Vec<u8>, VfsError> {
        let fd = vfs.open(OwnedPath::from("/init"))?;

        let metadata = vfs.metadata(fd)?;

        let mut buf = vec![0u8; metadata.length as usize];

        vfs.read(fd, &mut buf)?;

        Ok(buf)
    }) {
        ElfObject::parse(&init_program);
    }

    scheduler::with_scheduler(|scheduler| {
        let proc_id = scheduler.create_proc();

        scheduler.create_task(proc_id, task1 as *const () as u64, 3);
    });

    context::enter_usermode();
}
