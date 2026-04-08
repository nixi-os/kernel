//! x86_64 specific code

pub mod tables;
pub mod interrupt;
pub mod registers;
pub mod io;

use crate::helpers::*;

use registers::{Cr4Flags, XCr0Flags};

use core::arch::x86_64;


/// Initialize required processor features
pub fn init() {
    if x86_64::__cpuid_count(7, 0).ebx & 1 == 1 && x86_64::__cpuid_count(1, 0).ecx & (1 << 26) != 0 {
        registers::cr4_mask(Cr4Flags::FSGSBASE | Cr4Flags::OSXSAVE);

        registers::xcr0_set(XCr0Flags::X87_FPU | XCr0Flags::SSE_STATE);

        log!("feature enabled: FSGSBASE and XSAVE");
    } else {
        panic!("processor must support FSGSBASE and XSAVE");
    }
}

/// The size (in bytes) required by the XSAVE instruction for an XSAVE area containing all the user state components supported by this processor
#[inline(always)]
pub fn required_xsave_size() -> u32 {
    x86_64::__cpuid_count(13, 0).ebx
}


