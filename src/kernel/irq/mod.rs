mod pic8259;

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

// TODO: we must design an interface so that we can pass this input to the tty,
// preferably we want the interface to be flexible so that it can also be implemented for other
// input types later on
extern "x86-interrupt" fn com1_interrupt(_stack_frame: InterruptStackFrame) {
    unsafe {
        let byte = x86::io::inb(0x3f8);

        log!("got keyboard press: {}", byte as char);

        pic8259::end_of_interrupt(36);
    }
}


