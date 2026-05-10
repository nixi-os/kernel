//! The interface between the virtual file system and file system implementations

use super::error::VfsError;
use super::inode::INodeNumber;

/// A file system
pub trait FileSystem {
    /// Lookup an inode child from parent
    fn lookup(&self, parent: INodeNumber, name: &str) -> Result<INodeNumber, VfsError>;

    /// Create a subdirectory under parent
    fn create_dir(&self, parent: INodeNumber, name: &str) -> Result<(), VfsError>;

    /// Read from an inode
    fn read(&self, inode_num: INodeNumber, offset: u64, buffer: &mut [u8]) -> Result<(), VfsError>;

    /// Return the root inode number, the default implementation will always return inode number zero
    fn root(&self) -> INodeNumber { INodeNumber::new(0) }
}


