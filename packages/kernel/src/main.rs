#![feature(ptr_as_ref_unchecked)]
#![feature(iter_map_windows)]
#![feature(naked_functions_rustic_abi)]
#![feature(abi_x86_interrupt)]
#![feature(arbitrary_self_types_pointers)]

#![no_main]
#![no_std]

extern crate alloc;

mod boot;
mod kernel;
mod helpers;

use uefi::prelude::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!("panic in '{}' at line {}: {}", location.file(), location.line(), info.message());
    } else {
        error!("panic: {}", info.message());
    }

    loop {}
}

#[entry]
fn main() -> Status {
    if let Err(err) = boot::boot() {
        error!("fatal boot error: {}", err);
    }

    loop {}
}


