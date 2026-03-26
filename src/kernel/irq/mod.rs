pub mod pic8259;

use crate::kernel::drivers::tty::pool;
use crate::kernel::scheduler;

use crate::helpers::*;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::instructions::interrupts;
use x86_64::VirtAddr;
use lazy_static::lazy_static;

use core::arch::naked_asm;


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.double_fault.set_handler_fn(double_fault);
        idt.page_fault.set_handler_fn(page_fault);

        unsafe {
            idt[32].set_handler_addr(VirtAddr::new(timer_interrupt as *const () as u64));
        }

        idt[36].set_handler_fn(com1_interrupt);

        idt
    };
}

pub fn init() {
    log!("enabling interrupts");

    IDT.load();

    pic8259::init(32);

    pic8259::mask(0b1111_1111_1110_1111);

    interrupts::enable();
}

#[inline(always)]
pub fn enable_timer() {
    pic8259::mask(0b1111_1111_1110_1110);
}

extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    error!("double fault:\n{:#x?}\nerror code: {}", stack_frame, error_code);

    loop {}
}

extern "x86-interrupt" fn page_fault(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    error!("page fault:\n{:#x?}\nerror code: {:?}", stack_frame, error_code);

    loop {}
}

#[unsafe(naked)]
fn timer_interrupt() {
    naked_asm!(
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

        // call switch
        "mov rcx, rsp",
        "call {}",

        // call end_of_interrupt
        "mov cl, 32",
        "call {}",

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
        "iretq",
        sym scheduler::switch,
        sym pic8259::end_of_interrupt,
    );
}

extern "x86-interrupt" fn com1_interrupt(_stack_frame: InterruptStackFrame) {
    unsafe {
        let byte = x86::io::inb(0x3f8);

        pool::lock().push(byte);

        pic8259::end_of_interrupt(36);
    }
}


