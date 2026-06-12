//! The interface between the virtual file system and file system implementations

use super::error::VfsError;
use super::inode::INodeNumber;

/// File system metadata
pub trait Metadata {
    /// Get the length of an inode
    fn length(&self, inode_num: INodeNumber) -> Result<u64, VfsError>;
}

/// A file system backend
pub trait FileSystem: Metadata {
    /// Lookup an inode child from parent
    fn lookup(&self, parent: INodeNumber, name: &str) -> Result<INodeNumber, VfsError>;

    /// Create a subdirectory under parent
    fn create_dir(&self, parent: INodeNumber, name: &str) -> Result<(), VfsError>;

    /// Read from inode
    fn read(&self, inode_num: INodeNumber, offset: u64, buf: &mut [u8]) -> Result<u64, VfsError>;

    /// Write to inode
    fn write(&self, inode_num: INodeNumber, offset: u64, buf: &[u8]) -> Result<u64, VfsError>;

    /// Return the root inode number, the default implementation will always return inode number zero
    fn root(&self) -> INodeNumber {
        INodeNumber::new(0)
    }
}
