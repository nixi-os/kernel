//! Syscall implementation for the virtual file system

use super::{VFS, OwnedPath};
use super::fd::FileDescriptorId;

use crate::kernel::syscall::register::{self, SyscallHandler};
use crate::kernel::syscall::error::SyscallError;

use alloc::sync::Arc;
use alloc::boxed::Box;

use core::slice;

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
            VfsSyscallHandler::READ => {
                let buf = unsafe { slice::from_raw_parts_mut(args[1] as *mut u8, args[2] as usize) };

                VFS.lock()
                    .read(FileDescriptorId::new(args[0]), buf)
                    .map_err(|err| SyscallError::HandlerError(Box::new(err)))
            },
            VfsSyscallHandler::WRITE => {
                let buf = unsafe { slice::from_raw_parts(args[1] as *const u8, args[2] as usize) };

                VFS.lock()
                    .write(FileDescriptorId::new(args[0]), buf)
                    .map_err(|err| SyscallError::HandlerError(Box::new(err)))
            },
            _ => unreachable!(),
        }
    }
}


