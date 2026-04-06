//! An implementation of the global descriptor table

use super::tss::TaskStateSegment;

/// Common flags for segment descriptors
pub struct DescriptorFlags;

// TODO: maybe descriptor flags should be shared between the gdt and the idt since alot of the
// layout of between descriptors in the idt and the gdt is similar and have the same offsets
impl DescriptorFlags {
    /// The LONG_MODE flag determines whether a code segment contains 64-bit code
    const LONG_MODE: u64 = 1 << 53;

    /// Indicates whether the segment is present in memory
    const PRESENT: u64 = 1 << 47;

    /// Specifies whether the segment descriptor is for a system segment or a code or data segment
    const CODE_DATA: u64 = 1 << 44;

    /// Indicates whether the descriptor is for a data segment or a code segment. EXECUTE corresponds to
    /// the high bit of the type field
    const EXECUTE: u64 = 1 << 43;
}

/// A builder for a segment descriptor as defined in figure 3-8
#[repr(C, packed)]
pub struct SegmentDescriptor {
    descriptor: u64,
}

impl SegmentDescriptor {
    /// Create a new empty segment descriptor
    pub const fn new() -> SegmentDescriptor {
        SegmentDescriptor {
            descriptor: 0,
        }
    }

    /// Create a new segment descriptor with default flags for code or data
    pub const fn code_or_data() -> SegmentDescriptor {
        let descriptor = SegmentDescriptor {
            descriptor: DescriptorFlags::LONG_MODE | DescriptorFlags::PRESENT | DescriptorFlags::CODE_DATA,
        };

        descriptor.set_type(0b010)
    }

    /// Set flags
    pub const fn set_flags(self, flags: u64) -> SegmentDescriptor {
        SegmentDescriptor {
            descriptor: self.descriptor | flags,
        }
    }

    /// Set descriptor privilege level (DPL)
    pub const fn set_privilege_level(self, level: u8) -> SegmentDescriptor {
        SegmentDescriptor {
            descriptor: self.descriptor | ((level as u64 & 0b11) << 45),
        }
    }

    /// Set the lowest 3 bits of type as defined in Table 3-1. Code- and Data-Segment Types
    pub const fn set_type(self, _type: u8) -> SegmentDescriptor {
        SegmentDescriptor {
            descriptor: self.descriptor | ((_type as u64 & 0b111) << 40),
        }
    }

    /// Turn the segment descriptor into a tss descriptor, its still required that the caller
    /// properly sets type field correctly and enables the present flag
    pub fn as_tss_descriptor(self, base: u128) -> TssDescriptor {
        let limit = (core::mem::size_of::<TaskStateSegment>() - 1) as u128;

        let low_limit = limit & 0xffff;
        let high_limit = limit & (0xf << 16);

        let low_base = base & 0xffff;
        let mid_low_base = base & (0xff << 16);
        let mid_high_base = base & (0xff << 24);
        let high_base = base & (0xffff_ffff << 32);

        TssDescriptor {
            descriptor: self.descriptor as u128
                | (low_base << 16)
                | (mid_low_base << 16)
                | (mid_high_base << 32)
                | (high_base << 32)
                | low_limit
                | (high_limit << 32),
        }
    }
}

/// A tss descriptor as defined in figure 10-4
#[repr(C, packed)]
pub struct TssDescriptor {
    descriptor: u128,
}

/// The global descriptor table
#[repr(C)]
pub struct GlobalDescriptorTable {
    null: SegmentDescriptor,
    kernel_code: SegmentDescriptor,
    kernel_data: SegmentDescriptor,
    user_data: SegmentDescriptor,
    user_code: SegmentDescriptor,
    tss: TssDescriptor,
}

impl GlobalDescriptorTable {
    /// Create a new global descriptor table with an uninitialized tss descriptor
    pub const fn uninit() -> GlobalDescriptorTable {
        GlobalDescriptorTable {
            null: SegmentDescriptor::code_or_data(),
            kernel_code: SegmentDescriptor::code_or_data().set_flags(DescriptorFlags::EXECUTE),
            kernel_data: SegmentDescriptor::code_or_data(),
            user_data: SegmentDescriptor::code_or_data().set_privilege_level(3),
            user_code: SegmentDescriptor::code_or_data().set_flags(DescriptorFlags::EXECUTE).set_privilege_level(3),
            tss: TssDescriptor {
                descriptor: 0,
            },
        }
    }

    /// Initialize the global descriptor table with a tss
    pub fn init_with_tss(self: *mut GlobalDescriptorTable, tss: u64) {
        unsafe {
            (*self).tss = SegmentDescriptor::new().set_flags(DescriptorFlags::PRESENT | DescriptorFlags::EXECUTE).set_type(0b001).as_tss_descriptor(tss as u128);
        }
    }
}


