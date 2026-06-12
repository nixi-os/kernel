//! Syscall error

use crate::vfs::error::VfsError;

use thiserror::Error;

/// A syscall error is a generalized representation of internal error types
#[repr(u64)]
#[derive(Error, Debug)]
pub enum SyscallError {
    #[error("no handler is registered for the syscall number")]
    NotFound = 1,

    #[error("the argument is invalid")]
    InvalidArgument = 2,

    #[error("resource exhausted")]
    ResourceExhausted = 3,

    #[error("feature not implemented")]
    Unsupported = 4,
}

impl From<VfsError> for SyscallError {
    fn from(error: VfsError) -> SyscallError {
        match error {
            VfsError::OutOfBounds => SyscallError::InvalidArgument,
            VfsError::OutOfId => SyscallError::ResourceExhausted,
            VfsError::NoSuchFile => SyscallError::NotFound,
            VfsError::Unsupported => SyscallError::Unsupported,
        }
    }
}
