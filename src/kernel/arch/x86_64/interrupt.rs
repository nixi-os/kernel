//! Code for working with x86_64 interrupts


/// The interrupt stack frame
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct StackFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: RFlags,
    pub rsp: u64,
    pub ss: u64,
}

/// The system flags in the RFLAGS register as defined in figure 2-5
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct RFlags {
    pub flags: u64,
}

impl RFlags {
    pub fn new(flags: u64) -> RFlags {
        RFlags {
            flags,
        }
    }
}

impl RFlags {
    const FLAGS: [(&'static str, u64); 9] = [
        ("TRAP", 1 << 8),
        ("INTERRUPT_ENABLE", 1 << 9),
        ("NESTED_TASK", 1 << 14),
        ("RESUME", 1 << 16),
        ("VIRTUAL_8086", 1 << 17),
        ("ALIGNMENT_CHECK", 1 << 18),
        ("VIRTUAL_INTERRUPT", 1 << 19),
        ("VIRTUAL_INTERRUPT_PENDING", 1 << 20),
        ("IDENTIFICATION", 1 << 21),
    ];
}

impl core::fmt::Debug for RFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        f.write_str("RFlags { ")?;

        let mut flags = RFlags::FLAGS.iter().filter(|(_, flag)| self.flags & flag != 0);

        f.write_fmt(format_args!("iopl: {}, flags: {}", (self.flags >> 12) & 3, flags.next().map(|(name, _)| *name).unwrap_or("None")))?;

        while let Some((name, _)) = flags.next() {
            f.write_fmt(format_args!(" | {}", name))?;
        }

        f.write_str(" }")?;

        Ok(())
    }
}

/// A page fault error code as defined in figure 7-11
#[repr(transparent)]
pub struct PageFaultErrorCode {
    flags: u64,
}

impl PageFaultErrorCode {
    /// Page fault error code flags
    const FLAGS: [(&'static str, u64); 9] = [
        ("PRESENT", 1),
        ("WRITE", 1 << 1),
        ("USER_MODE", 1 << 2),
        ("RESERVED", 1 << 3),
        ("INSTRUCTION_FETCH", 1 << 4),
        ("PROTECTION", 1 << 5),
        ("SHADOW_STACK", 1 << 6),
        ("HLAT", 1 << 7),
        ("SGX", 1 << 15),
    ];
}

impl core::fmt::Display for PageFaultErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        f.write_str("PageFaultErrorCode(")?;

        let mut flags = PageFaultErrorCode::FLAGS.iter().filter(|(_, flag)| self.flags & flag != 0);

        f.write_fmt(format_args!("{}", flags.next().map(|(name, _)| *name).unwrap_or("None")))?;

        while let Some((name, _)) = flags.next() {
            f.write_fmt(format_args!(" | {}", name))?;
        }

        f.write_str(")")?;

        Ok(())
    }
}


