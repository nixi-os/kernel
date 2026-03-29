//! An implementation of the global descriptor table


/// Common flags for segment descriptors
pub struct DescriptorFlags;

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

/// A builder for a segment descriptor as defined in figure 3-8. of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 3
#[repr(C)]
pub struct SegmentDescriptor {
    descriptor: u64,
}

impl SegmentDescriptor {
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
}

/// The global descriptor table
#[repr(C)]
pub struct GlobalDescriptorTable {
    null: SegmentDescriptor,
    kernel_code: SegmentDescriptor,
    kernel_data: SegmentDescriptor,
    user_data: SegmentDescriptor,
    user_code: SegmentDescriptor,
    tss: u128,
}

impl GlobalDescriptorTable {
    /// Create a new global descriptor table
    pub const fn new() -> GlobalDescriptorTable {
        GlobalDescriptorTable {
            null: SegmentDescriptor::code_or_data(),
            kernel_code: SegmentDescriptor::code_or_data().set_flags(DescriptorFlags::EXECUTE),
            kernel_data: SegmentDescriptor::code_or_data(),
            user_data: SegmentDescriptor::code_or_data().set_privilege_level(3),
            user_code: SegmentDescriptor::code_or_data().set_flags(DescriptorFlags::EXECUTE).set_privilege_level(3),
            tss: 0,
        }
    }
}


