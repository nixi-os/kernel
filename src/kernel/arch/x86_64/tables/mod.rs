//! Common tables on x86_64. Currently this module has implementations for the global descriptor
//! table, the interrupt descriptor table and task state segments

pub mod gdt;
pub mod idt;
pub mod tss;

use crate::helpers::*;

use idt::InterruptDescriptorTable;
use gdt::GlobalDescriptorTable;
use tss::TaskStateSegment;

use core::arch::asm;

// NOTE: having a single global gdt and tss without a mutex is safe as long as we dont do SMP
static mut TABLES: Tables = Tables {
    idt: InterruptDescriptorTable::uninit(),
    gdt: GlobalDescriptorTable::uninit(),
    tss: TaskStateSegment::uninit(),
};

/// A shared structure for the global descriptor table, the task state segment and the interrupt
/// descriptor table
#[repr(C)]
struct Tables {
    idt: InterruptDescriptorTable,
    gdt: GlobalDescriptorTable,
    tss: TaskStateSegment,
}

/// The descriptor pointer is used to specify the address and size of a descriptor table
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    pub limit: u16,
    pub base: u64,
}

/// Initialize the tables
pub fn init() {
    unsafe {
        log!("idt: {:#x?}", &raw const TABLES.idt);
        log!("gdt: {:#x?}", &raw const TABLES.gdt);
        log!("tss: {:#x?}", &raw const TABLES.tss);

        (&raw mut TABLES.gdt).init_with_tss(&raw const TABLES.tss as u64);

        gdt::load(&DescriptorTablePointer {
            limit: core::mem::size_of::<GlobalDescriptorTable>() as u16,
            base: &raw const TABLES.gdt as u64,
        });

        // load code segment (CS) using far return
        asm!(
            "push {sel}",
            "lea {tmp}, [2f + rip]",
            "push {tmp}",
            "retfq",
            "2:",
            sel = in(reg) 0x08u64,
            tmp = lateout(reg) _,
        );

        // load data segments
        asm!(
            "mov ds, {sel}",
            "mov es, {sel}",
            "mov fs, {sel}",
            "mov gs, {sel}",
            "mov ss, {sel}",
            sel = in(reg) 0x10u64,
        );

        // load tss
        asm!(
            "ltr {sel:x}",
            sel = in(reg) 0x28u64,
        );

        (&raw mut TABLES.idt).init();

        idt::load(&DescriptorTablePointer {
            limit: core::mem::size_of::<InterruptDescriptorTable>() as u16,
            base: &raw const TABLES.idt as u64,
        });
    }
}

/// Set the kernel stack (rsp0) in the task state segment
pub fn set_kernel_stack(stack: *const u8) {
    log!("set_kernel_stack: {:#x?}", stack);

    unsafe {
        (&raw mut TABLES.tss).set_rsp0(stack as u64);
    }
}


