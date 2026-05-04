//! Error handling in the VFS


/// Virtual file system errors
#[derive(Debug)]
pub enum VfsError {
    /// The offset is beyond EOF
    OutOfBounds,

    /// The requested file does not exist
    NoSuchFile,

    /// File system doesn't support mounting inodes
    UnMountable,

    /// The bitmap allocator for file descriptor and inode id's failed to find a free id
    OutOfId,
}

impl core::fmt::Display for VfsError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            VfsError::OutOfBounds => f.write_str("out of bounds"),
            VfsError::NoSuchFile => f.write_str("no such file"),
            VfsError::UnMountable => f.write_str("file system doesn't support mounting"),
            VfsError::OutOfId => f.write_str("out of id"),
        }
    }
}

impl core::error::Error for VfsError {}


