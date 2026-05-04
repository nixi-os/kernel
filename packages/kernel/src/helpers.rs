use crate::kernel::drivers::serial::SERIAL;
use crate::kernel::drivers::tty::pool;

use core::fmt::{Write, Arguments, Error};


#[inline(always)]
pub fn __kprint_fmt(args: Arguments) -> Result<(), Error> {
    if let Some(mut pool) = pool::get() {
        pool.write_fmt(args)
    } else {
        SERIAL.lock().write_fmt(args)
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        {
            let _ = crate::helpers::__kprint_fmt(format_args!("\x1b[33m\x1b[1m[{}]\x1b[0m\x1b[37m {}\n", module_path!(), format_args!($($arg)*)));
        }
    };
}

pub(crate) use log;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            let _ = crate::helpers::__kprint_fmt(format_args!("\x1b[31m\x1b[1m[{}]\x1b[0m\x1b[37m {}\n", module_path!(), format_args!($($arg)*)));
        }
    };
}

pub(crate) use error;


