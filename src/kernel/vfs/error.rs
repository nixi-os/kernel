//! Error handling in the VFS

use thiserror::Error;

/// Virtual file system errors
#[derive(Error, Debug, Clone, Copy)]
pub enum VfsError {
    #[error("the offset is beyond EOF")]
    OutOfBounds,

    #[error("the requested file does not exist")]
    NoSuchFile,

    #[error("failed to allocate id")]
    OutOfId,

    #[error("unsupported feature")]
    Unsupported,
}
