//! The syscall register

use super::error::SyscallError;

use crate::kernel::scheduler::context::Context;

use alloc::sync::Arc;

use core::ops::Range;

use spin::Mutex;

static REGISTER: Mutex<SyscallRegister> = Mutex::new(SyscallRegister::new());

/// Register a syscall handler
#[inline(always)]
pub fn register(range: Range<usize>, handler: Arc<dyn SyscallHandler + Send + Sync>) {
    REGISTER.lock().register(range, handler);
}

/// A syscall handler can be implemented by any subsystem to support syscalls
pub trait SyscallHandler {
    fn handle(&self, syscall: u64, args: [u64; 4]) -> Result<u64, SyscallError>;
}

/// The syscall register keeps a record of all syscall handlers and handles dispatching
pub struct SyscallRegister {
    handlers: [Option<Arc<dyn SyscallHandler + Send + Sync>>; 255],
}

impl SyscallRegister {
    /// Create a new syscall register
    pub const fn new() -> SyscallRegister {
        SyscallRegister {
            handlers: [const { None }; 255],
        }
    }

    /// Register a syscall handler for a range of syscalls. This will overwrite any previously registered syscall at the same range
    pub fn register(
        &mut self,
        range: Range<usize>,
        handler: Arc<dyn SyscallHandler + Send + Sync>,
    ) {
        for index in range {
            self.handlers[index] = Some(Arc::clone(&handler));
        }
    }

    /// Dispatch the correct syscall handler
    pub fn dispatch(&self, ctx: &Context) -> Result<u64, SyscallError> {
        if let Some(handler) = self.handlers.get(ctx.general.rax as usize).flatten_ref() {
            handler.handle(
                ctx.general.rax,
                [
                    ctx.general.rdx,
                    ctx.general.rcx,
                    ctx.general.rdi,
                    ctx.general.rsi,
                ],
            )
        } else {
            Err(SyscallError::NotFound)
        }
    }
}

/// Syscall is called from the syscall handler in assembly
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn syscall(ctx: *mut Context) {
    unsafe {
        let result = REGISTER.lock().dispatch(ctx.as_ref_unchecked());

        (*ctx).general.rax = result.is_err() as u64;
        (*ctx).general.rbx = result.map_or_else(|err| err.error_code(), |value| value);
    }
}
