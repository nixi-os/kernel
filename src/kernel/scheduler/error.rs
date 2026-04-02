

/// A scheduler error
#[derive(Debug)]
pub enum SchedulerError {
    OutOfPid,
}

impl core::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            SchedulerError::OutOfPid => f.write_str("out of process identifiers"),
        }
    }
}

impl core::error::Error for SchedulerError {}


