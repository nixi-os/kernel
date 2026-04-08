use crate::kernel::irq;

use super::DescriptorTablePointer;

use core::arch::asm;

/// Load the interrupt descriptor table register
pub fn load(ptr: &DescriptorTablePointer) {
    unsafe {
        asm!(
            "lidt [{ptr}]",
            ptr = in(reg) ptr as *const DescriptorTablePointer,
        );
    }
}

/// An interrupt gate descriptor as defined in figure 7-8
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct InterruptGateDescriptor {
    descriptor: u128,
}

impl InterruptGateDescriptor {
    /// Create a new interrupt gate descriptor with an offset and segment selector, the segment
    /// selector must point to a valid 64-bit code segment
    pub fn new(offset: u64, cs: u16) -> InterruptGateDescriptor {
        let low_offset = offset as u128 & 0xffff;
        let mid_offset = offset as u128 & (0xffff << 16);
        let high_offset = offset as u128 & (0xffff_ffff << 32);

        InterruptGateDescriptor {
            descriptor: low_offset
                | (mid_offset << 32)
                | (high_offset << 32)
                | ((cs as u128) << 16)
                | (0xe << 40)
                | (1 << 47),
        }
    }
}

/// The interrupt descriptor table tells the processor where to jump on interrupts
#[repr(C, align(64))]
pub struct InterruptDescriptorTable {
    descriptors: [InterruptGateDescriptor; 256],
}

impl InterruptDescriptorTable {
    /// Create an uninitialized interrupt descriptor table
    pub const fn uninit() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            descriptors: [InterruptGateDescriptor { descriptor: 0 }; 256],
        }
    }

    /// Initialize the interrupt descriptor table
    pub fn init(self: *mut InterruptDescriptorTable) {
        unsafe {
            (*self).descriptors[8] = InterruptGateDescriptor::new(irq::double_fault as *const () as u64, 0x8);
            (*self).descriptors[13] = InterruptGateDescriptor::new(irq::gp_fault as *const () as u64, 0x8);
            (*self).descriptors[14] = InterruptGateDescriptor::new(irq::page_fault as *const () as u64, 0x8);
            (*self).descriptors[32] = InterruptGateDescriptor::new(irq::timer_interrupt as *const () as u64, 0x8);
            (*self).descriptors[36] = InterruptGateDescriptor::new(irq::com1_interrupt as *const () as u64, 0x8);
        }
    }
}


