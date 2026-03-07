use crate::helpers::*;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::instructions::interrupts;
use lazy_static::lazy_static;


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.double_fault.set_handler_fn(double_fault);
        idt.page_fault.set_handler_fn(page_fault);

        idt
    };
}

pub fn init() {
    log!("enabling interrupts");

    IDT.load();

    interrupts::enable();
}

extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    error!("double fault:\n{:#x?}\nerror code: {}", stack_frame, error_code);

    loop {}
}

extern "x86-interrupt" fn page_fault(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    error!("page fault:\n{:#x?}\nerror code: {:?}", stack_frame, error_code);

    loop {}
}


