//! Code for working with Model Specific Registers (MSRs)
//!
//! ref: Intel® 64 and IA-32 Architectures Software Developer’s Manual Volume 4: Model-Specific Registers

use core::arch::asm;


/// A Model Specific Register
#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ModelSpecificRegister {
    IA32_EFER = 0xc0000080,
    IA32_STAR = 0xc0000081,
    IA32_LSTAR = 0xc0000082,
    IA32_FMASK = 0xc0000084,
}

/// Write to a Model Specific Register using the WRMSR instruction
pub fn write_msr(register: ModelSpecificRegister, value: u64) {
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") register as u32,
            in("edx") (value >> 32) as u32,
            in("eax") (value & 0xffff_ffff) as u32,
        );
    }
}

/// Read from Model Specific Register using the RDMSR instruction
pub fn read_msr(register: ModelSpecificRegister) -> u64 {
    let mut low = 0u32;
    let mut high = 0u32;

    unsafe {
        asm!(
            "rdmsr",
            in("ecx") register as u32,
            out("edx") high,
            out("eax") low,
        );
    }

    low as u64 & ((high as u64) << 32)
}

/// Convenience function for reading a MSR, updating it, and writing the new value
#[inline]
pub fn update_msr<F: FnOnce(u64) -> u64>(register: ModelSpecificRegister, f: F) {
    write_msr(register, f(read_msr(register)));
}


