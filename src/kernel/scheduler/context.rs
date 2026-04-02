//! Context switching

use super::SCHEDULER;

use crate::kernel::irq;
use crate::helpers::*;

use x86_64::instructions::interrupts;


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

/// Initialize scheduler and enter usermode with init process
pub fn enter_user() -> ! {
    log!("initializing scheduler");

    // TODO: in the future the will be no Task::new method, tasks should only be possible to be
    // made through a process
    // let task = Task::new(0, Context::default());

    // TASKS.lock().replace(TaskTable::new(task));

    interrupts::disable();

    irq::enable_timer();

    loop {}
}

/// Switch is called directly from assembly in the interrupt handler
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn switch(ctx: *mut Context) {
    let mut guard = SCHEDULER.lock();

    log!("switch context: {:x?}", unsafe { *ctx });

    if let Some(scheduler) = &mut *guard {
        unsafe {
            *ctx = scheduler.switch(*ctx);
        }
    } else {
        panic!("task table must be initialized before any context switches can occur");
    }
}


