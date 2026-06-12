//! Loader errors

use crate::kernel::vfs::error::VfsError;

use thiserror::Error;

/// An error while loading a program
#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("invalid ELF binary")]
    InvalidElf,

    #[error(transparent)]
    Vfs(#[from] VfsError),
}
