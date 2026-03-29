pub mod proc;

use crate::kernel::{cpu, irq};
use crate::helpers::*;

use proc::ProcId;

use spin::{Lazy, Mutex};

use alloc::alloc::Layout;
use alloc::vec::Vec;
use alloc::vec;


/// The global task table
static TASKS: Lazy<Mutex<Option<TaskTable>>> = Lazy::new(|| Mutex::new(None));

/// A task id
pub type TaskId = usize;

/// Initialize scheduler and turn current thread into a task
pub fn init() {
    log!("initializing scheduler");

    let task = Task::new(TaskOwner::Kernel, Context::default());

    TASKS.lock().replace(TaskTable::new(task));

    irq::enable_timer();
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn switch(ctx: *mut Context) {
    let mut guard = TASKS.lock();

    log!("switch context: {:x?}", unsafe { *ctx });

    if let Some(table) = &mut *guard {
        unsafe {
            *ctx = table.switch(*ctx);
        }
    } else {
        panic!("task table must be initialized before any context switches can occur");
    }
}

/// The task table
pub struct TaskTable {
    tasks: Vec<Task>,
    current: TaskId,
}

impl TaskTable {
    /// Create a new task table with init task
    pub fn new(init: Task) -> TaskTable {
        TaskTable {
            tasks: vec![init],
            current: 0,
        }
    }

    /// Perform a context switch from the current task to the next task
    unsafe fn switch(&mut self, ctx: Context) -> Context {
        let next = if self.current >= self.tasks.len() - 1 { 0 } else { self.current + 1 };

        self.tasks[self.current].ctx = ctx;

        unsafe {
            core::arch::x86_64::_xsave(self.tasks[self.current].xsave, u64::MAX);

            core::arch::x86_64::_xrstor(self.tasks[next].xsave, u64::MAX);
        }

        self.current = next;

        self.tasks[next].ctx
    }
}

/// A task can be owned by a userspace process or the kernel
pub enum TaskOwner {
    Proc(ProcId),
    Kernel,
}

/// A task which will be run by the scheduler on interval
pub struct Task {
    owner: TaskOwner,
    ctx: Context,
    xsave: *mut u8,
}

impl Task {
    pub fn new(owner: TaskOwner, ctx: Context) -> Task {
        let layout = Layout::from_size_align(cpu::required_xsave_size() as usize, 64).expect("xsave allocation shouldn't break alignment rules");
        let xsave = unsafe { alloc::alloc::alloc_zeroed(layout) };

        Task {
            owner,
            ctx,
            xsave,
        }
    }
}

unsafe impl Sync for Task {}
unsafe impl Send for Task {}

/// Segment registers
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Segments {
    pub fs: u64,
    pub gs: u64,
}

/// General purpose registers
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct GeneralPurpose {
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
}

/// The stack frame
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct StackFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

/// Saved context of a task
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Context {
    pub segments: Segments,
    pub general: GeneralPurpose,
    pub stack_frame: StackFrame,
}


