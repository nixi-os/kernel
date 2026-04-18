//! Handling of syscalls

use core::arch::naked_asm;


// TODO: make the syscall handler do something useful, instead of just hanging

/// The syscall handler is called by the syscall instruction
#[unsafe(naked)]
pub fn syscall_handler() {
    naked_asm!(
        "label:",
        "jmp label",
    );
}


