//! Error handling in the VFS


/// Virtual file system errors
#[derive(Debug)]
pub enum VfsError {
    OutOfBounds,
    NoSuchFile,
    UnMountable,
}

impl core::fmt::Display for VfsError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            VfsError::OutOfBounds => f.write_str("out of bounds"),
            VfsError::NoSuchFile => f.write_str("no such file"),
            VfsError::UnMountable => f.write_str("file system doesn't support mounting"),
        }
    }
}

impl core::error::Error for VfsError {}


