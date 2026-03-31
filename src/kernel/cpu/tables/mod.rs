//! Common tables on x86_64. Currently this module has implementations for the global descriptor
//! table, the interrupt descriptor table and task state segments

pub mod gdt;
pub mod idt;
pub mod tss;

use crate::helpers::*;

use gdt::GlobalDescriptorTable;
use tss::TaskStateSegment;

use x86_64::structures::DescriptorTablePointer;
use x86_64::VirtAddr;

// NOTE: having a single global gdt and tss without a mutex is safe as long as we dont do SMP
static mut GDT_TSS: GdtTss = GdtTss {
    gdt: GlobalDescriptorTable::uninit(),
    tss: TaskStateSegment::uninit(),
};

/// A shared structure for the global descriptor table and the task state segment
#[repr(C)]
struct GdtTss {
    gdt: GlobalDescriptorTable,
    tss: TaskStateSegment,
}

/// Initialize the global descriptor table
pub fn init_gdt() {
    unsafe {
        log!("initializing global descriptor table at: {:#x?}", &raw const GDT_TSS.gdt);

        (&raw mut GDT_TSS.gdt).init_with_tss(&raw const GDT_TSS.tss as u64);

        x86_64::instructions::tables::lgdt(&DescriptorTablePointer {
            base: VirtAddr::from_ptr(&raw const GDT_TSS.gdt),
            limit: core::mem::size_of::<GlobalDescriptorTable>() as u16,
        });

        // load CS using far return
        core::arch::asm!(
            "push {sel}",
            "lea {tmp}, [2f + rip]",
            "push {tmp}",
            "retfq",
            "2:",
            sel = in(reg) 0x08u64,
            tmp = lateout(reg) _,
            options(preserves_flags),
        );

        // load all other segment registers
        core::arch::asm!(
            "mov ds, {sel}",
            "mov es, {sel}",
            "mov fs, {sel}",
            "mov gs, {sel}",
            "mov ss, {sel}",
            sel = in(reg) 0x10u64,
        );
    }
}

/// Set the kernel stack (rsp0) in the task state segment
pub fn set_kernel_stack(stack: *const u8) {
    unsafe {
        (&raw mut GDT_TSS.tss).set_rsp0(stack as u64);
    }
}


