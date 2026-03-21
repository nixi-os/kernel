//! cpu functionality

use crate::helpers::*;

use x86::cpuid::cpuid;
use x86_64::registers::control::Cr4;


/// Enable FSGSBASE to use efficent instructions to access FS/GS registers
#[inline]
pub fn enable_fsgsbase() {
    let cpuid = cpuid!(7, 0);

    if cpuid.ebx & 1 == 1 {
        unsafe {
            let mut cr4 = Cr4::read_raw();

            cr4 |= 1 << 16;

            Cr4::write_raw(cr4);
        }

        log!("enabled FSGSBASE flag in cr4");
    } else {
        panic!("cpu doesnt support FSGSBASE");
    }
}


