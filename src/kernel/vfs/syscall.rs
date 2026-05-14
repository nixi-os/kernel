//! Syscall implementation for the virtual file system

use super::{VFS, OwnedPath};
use super::fd::FileDescriptorId;

use crate::kernel::syscall::register::{self, SyscallHandler};
use crate::kernel::syscall::error::SyscallError;

use alloc::sync::Arc;
use alloc::boxed::Box;

/// Initialize the VFS syscalls
pub fn init() {
    register::register(0..4, Arc::new(VfsSyscallHandler));
}

pub struct VfsSyscallHandler;

impl VfsSyscallHandler {
    /// The open syscall opens a new file descriptor
    pub const OPEN: u64 = 0;

    /// The close syscall closes a file descriptor
    pub const CLOSE: u64 = 1;

    /// The read syscall reads a file descriptor
    pub const READ: u64 = 2;

    /// The write syscall writes a file descriptor
    pub const WRITE: u64 = 3;
}

impl SyscallHandler for VfsSyscallHandler {
    fn handle(&self, syscall: u64, args: [u64; 4]) -> Result<u64, SyscallError> {
        match syscall {
            VfsSyscallHandler::OPEN => {
                let path = unsafe { OwnedPath::from_raw_parts(args[0] as *mut u8, args[1] as usize) };

                VFS.lock()
                    .open(path)
                    .map_err(|err| SyscallError::HandlerError(Box::new(err)))
                    .map(|fd| fd.value())
            },
            VfsSyscallHandler::CLOSE => {
                VFS.lock().close(FileDescriptorId::new(args[0]));

                Ok(0)
            },
            // TODO: implement read and write
            _ => Err(SyscallError::NotFound),
        }
    }
}


