

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = crate::kernel::drivers::tty::TTY.lock().write_fmt(format_args!("\x1b[33m\x1b[1m[{}]\x1b[0m\x1b[37m {}\n", module_path!(), format_args!($($arg)*)));
        }
    };
}

pub(crate) use log;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = crate::kernel::drivers::tty::TTY.lock().write_fmt(format_args!("\x1b[31m\x1b[1m[{}]\x1b[0m\x1b[37m {}\n", module_path!(), format_args!($($arg)*)));
        }
    };
}

pub(crate) use error;


