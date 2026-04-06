//! x86_64 interrupt structures

use core::arch::asm;


/// Clear interrupt flag in rflags and disable all maskable interrupts
pub fn clear_interrupt_flag() {
    unsafe {
        asm!("cli");
    }
}

/// The interrupt stack frame
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct StackFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
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

/// Bitflags pretty printing on a budget :)
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


