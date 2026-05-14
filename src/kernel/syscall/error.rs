//! Syscall error

use alloc::boxed::Box;

/// An error inside a syscall handler
pub trait HandlerError {
    fn error_code(&self) -> u64;
}

impl HandlerError for () {
    fn error_code(&self) -> u64 { 0 }
}

/// A syscall error
pub enum SyscallError {
    /// No handler is registered for the syscall number
    NotFound,

    /// An error occured inside the syscall handler
    HandlerError(Box<dyn HandlerError + Send + Sync>),
}

impl SyscallError {
    /// Return the error code. Handler errors are always error_code + 0x1000
    pub fn error_code(&self) -> u64 {
        match self {
            SyscallError::NotFound => 1,
            SyscallError::HandlerError(err) => err.error_code() + 0x1000,
        }
    }
}


