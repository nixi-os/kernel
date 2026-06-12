//! Boot error

use thiserror::Error;

/// An error during boot
#[derive(Error, Debug)]
pub enum BootError {
    #[error("ACPI not found in config table")]
    AcpiNotFound,

    #[error(transparent)]
    UefiError(#[from] uefi::Error<()>),
}
