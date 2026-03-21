pub mod proc;

use crate::helpers::*;

use proc::ProcId;

use alloc::vec::Vec;

use spin::{Lazy, Mutex};

/// The global task table
static TASKS: Lazy<Mutex<TaskTable>> = Lazy::new(|| Mutex::new(TaskTable::default()));

/// A task id
pub type TaskId = usize;

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn switch(ctx: *const Context) {
    log!("switch context: {:?}", unsafe { *ctx });
}

/// The task table
#[derive(Default)]
pub struct TaskTable {
    tasks: Vec<Task>,
    next: Option<TaskId>,
}

/// A task which will be run by the scheduler on interval
pub struct Task {
    proc: ProcId,
    ctx: Context,
}

/// Saved context of a task
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    // floating point unit, fs and gs
    // pub fpu: [u8; 512],
    pub fs: u64,
    pub gs: u64,

    // general purpose registers
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,

    // stack frame which is pushed by the cpu on interrupt
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}


