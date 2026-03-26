//! cpu functionality

use crate::helpers::*;

use x86::cpuid::cpuid;
use x86_64::registers::control::{Cr4, Cr4Flags};
use x86_64::registers::xcontrol::{XCr0, XCr0Flags};


/// Enable FSGSBASE to use efficent instructions to access FS/GS registers
#[inline]
pub fn enable_fsgsbase() {
    let cpuid = cpuid!(7, 0);

    if cpuid.ebx & 1 == 1 {
        unsafe {
            Cr4::update(|flags| flags.insert(Cr4Flags::FSGSBASE));
        }

        log!("enabled FSGSBASE feature");
    } else {
        panic!("cpu doesnt support FSGSBASE");
    }
}

/// Enable XSAVE feature set for extended state management
#[inline]
pub fn enable_xsave() {
    let cpuid = cpuid!(1, 0);

    if cpuid.ecx & (1 << 26) != 0 {
        unsafe {
            Cr4::update(|flags| flags.insert(Cr4Flags::OSXSAVE));

            XCr0::update(|flags| flags.insert(XCr0Flags::X87 | XCr0Flags::SSE));
        }

        log!("enabled XSAVE with X87 fpu and SSE");
    } else {
        panic!("cpu doesnt support XSAVE");
    }
}

/// The size (in bytes) required by the XSAVE instruction for an XSAVE area containing all the user state components supported by this processor
#[inline(always)]
pub fn required_xsave_size() -> u32 {
    cpuid!(13, 0).ebx
}


