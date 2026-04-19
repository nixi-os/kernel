//! Handling of syscalls

use crate::kernel::arch::x86_64::tables::{TABLES, Tables};
use crate::kernel::arch::x86_64::tables::tss::TaskStateSegment;
use crate::kernel::scheduler::context::Context;

use core::arch::naked_asm;

/// Used to save the stack pointer without globbering any additional registers
static STACK_POINTER_SAVE: u64 = 0;

/// Syscall is called from the syscall handler in assembly
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn syscall(ctx: *mut Context) {
    crate::log!("syscall: {:x?}", unsafe { *ctx });
}

/// The syscall handler is called by the syscall instruction. The syscall handler only globbers rcx and r11, as these are globbered by the syscall instruction itself
#[unsafe(naked)]
pub fn syscall_handler() {
    naked_asm!(
        // save rsp
        "mov [{stack_pointer_save}], rsp",

        // load kernel stack
        "mov rsp, [{tables} + {rsp0_offset}]",

        // save stack frame
        "sub rsp, 16",
        "push r11",
        "sub rsp, 8",
        "push rcx",

        // save general purpose registers
        "push rax",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rsi",
        "push rdi",
        "push rbp",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",

        // save FS/GS
        "rdgsbase rax",
        "push rax",
        "rdfsbase rax",
        "push rax",

        // call syscall
        "mov rcx, rsp",
        "call {syscall}",

        // restore FS/GS
        "pop rax",
        "wrfsbase rax",
        "pop rax",
        "wrgsbase rax",

        // restore general purpose registers
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rbp",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",

        // restore stack frame
        "mov rcx, [rsp]",
        "mov r11, [rsp + 16]",

        // restore stack
        "mov rsp, [{stack_pointer_save}]",

        "sysret",
        syscall = sym syscall,
        tables = sym TABLES,
        stack_pointer_save = sym STACK_POINTER_SAVE,
        rsp0_offset = const core::mem::offset_of!(Tables, tss) + core::mem::offset_of!(TaskStateSegment, rsp),
    );
}


