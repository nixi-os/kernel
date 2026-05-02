//! x86_64 specific code

pub mod tables;
pub mod interrupt;
pub mod registers;
pub mod msr;
pub mod io;

use crate::helpers::*;
use crate::kernel::syscall;

use registers::{Cr4Flags, XCr0Flags};
use msr::ModelSpecificRegister;

use core::arch::x86_64;


/// Initialize required processor features and MSRs
pub fn init() {
    if x86_64::__cpuid_count(7, 0).ebx & 1 == 1 && x86_64::__cpuid_count(1, 0).ecx & (1 << 26) != 0 {
        registers::cr4_mask(Cr4Flags::FSGSBASE | Cr4Flags::OSXSAVE);

        registers::xcr0_set(XCr0Flags::X87_FPU | XCr0Flags::SSE_STATE);

        log!("enabled FSGSBASE and XSAVE");
    } else {
        panic!("processor must support FSGSBASE and XSAVE");
    }

    msr::write_msr(ModelSpecificRegister::IA32_LSTAR, syscall::syscall_handler as *const () as u64);

    msr::write_msr(ModelSpecificRegister::IA32_STAR, (0x18 << 48) | (0x08 << 32));

    msr::update_msr(ModelSpecificRegister::IA32_EFER, |efer| efer | 1);

    log!("initialized IA32_LSTAR, IA32_STAR and IA32_EFER.SCE");
}

/// The size (in bytes) required by the XSAVE instruction for an XSAVE area containing all the user state components supported by this processor
#[inline(always)]
pub fn required_xsave_size() -> u32 {
    x86_64::__cpuid_count(13, 0).ebx
}

/// Determine the physical address width supported by the processor also known as MAXPHYADDR as described in 5.1.4 Enumeration of Paging Features by CPUID
#[inline(always)]
pub fn physical_address_width() -> u32 {
    x86_64::__cpuid(0x80000008).eax & 0xff
}


