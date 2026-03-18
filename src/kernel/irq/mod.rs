mod pic8259;

use crate::kernel::drivers::tty::pool;

use crate::helpers::*;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::instructions::interrupts;
use lazy_static::lazy_static;


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.double_fault.set_handler_fn(double_fault);
        idt.page_fault.set_handler_fn(page_fault);
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

extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    error!("double fault:\n{:#x?}\nerror code: {}", stack_frame, error_code);

    loop {}
}

extern "x86-interrupt" fn page_fault(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    error!("page fault:\n{:#x?}\nerror code: {:?}", stack_frame, error_code);

    loop {}
}

extern "x86-interrupt" fn com1_interrupt(_stack_frame: InterruptStackFrame) {
    unsafe {
        let byte = x86::io::inb(0x3f8);

        pool::lock().push(byte);

        pic8259::end_of_interrupt(36);
    }
}


