//! Context switching

use super::SCHEDULER;

use crate::kernel::cpu::tables;
use crate::kernel::scheduler;
use crate::kernel::irq;
use crate::helpers::*;

use x86_64::instructions::interrupts;

use core::arch::asm;


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

/// Enter usermode, the scheduler will panic if it has no tasks when this is called
pub fn enter_usermode() -> ! {
    log!("enter usermode");

    let (stack_frame, kernel_stack) = scheduler::with_scheduler(|scheduler| {
        let (current, _) = scheduler.schedule_task();

        let task = scheduler.lookup_task(current);

        (task.ctx.stack_frame, unsafe { task.kernel_stack.as_ptr().add(task.kernel_stack.len()) })
    });

    interrupts::disable();

    irq::enable_timer();

    tables::set_kernel_stack(kernel_stack);

    unsafe {
        asm!(
            "push {ss}",
            "push {rsp}",
            "push {rflags}",
            "push {cs}",
            "push {rip}",
            "iretq",
            ss = in(reg) stack_frame.ss,
            rsp = in(reg) stack_frame.rsp,
            rflags = in(reg) stack_frame.rflags,
            cs = in(reg) stack_frame.cs,
            rip = in(reg) stack_frame.rip,
            options(noreturn),
        );
    }
}

/// Switch is called directly from assembly in the interrupt handler
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn switch(ctx: *mut Context) {
    let mut scheduler = SCHEDULER.lock();

    log!("switch context: {:x?}", unsafe { *ctx });

    unsafe {
        *ctx = scheduler.switch(*ctx);
    }
}


