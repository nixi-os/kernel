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

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!("panic in '{}' at line {}: {}", location.file(), location.line(), info.message());
    } else {
        error!("panic: {}", info.message());
    }

    loop {}
}

// TODO: implement paging so that we can run usermode processes without causing page fault as a
// result of processor trying to fetch instruction when its located inside memory marked as supervisor-only

#[entry]
fn main() -> Status {
    if let Err(err) = boot::boot() {
        error!("error: {}", err);
    }

    loop {}
}


