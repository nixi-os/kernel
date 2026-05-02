//! Code for working with inodes

use super::error::VfsError;

use alloc::sync::Arc;

/// An inode id points to an inode globally in the inode cache
pub type INodeId = usize;

/// An inode number points to an inode within a specific file system
pub type INodeNumber = usize;

// TODO: the inodes should hold a last accessed timestamp in order to do LRU eviction to evict the
// last used inodes
//
// NOTE: i think the best combination for our virtual file system will be to have a combination of
// LRU eviction to remove old inodes, and generation numbers embedded in inode numbers to ensure
// that old inode numbers dont get reinterpreted as the wrong file if the old file is deleted

/// An inode is a node in the virtual file system. The inode can be backed by any file system implementation
#[derive(Clone)]
pub struct INode {
    pub fs: Arc<dyn FileSystem + Send + Sync>,
    pub inode_num: INodeNumber,
}

impl INode {
    /// Create a new inode
    pub fn new(inode_num: INodeNumber, fs: Arc<dyn FileSystem + Send + Sync>) -> INode {
        INode {
            fs,
            inode_num,
        }
    }

    /// Lookup an inode child
    pub fn lookup(&self, name: &str) -> Result<INode, VfsError> {
        Arc::clone(&self.fs).lookup(self.inode_num, name)
    }

    /// Mount an inode
    pub fn mount(&self, name: &str, inode: INode) -> Result<(), VfsError> {
        Arc::clone(&self.fs).mount(self.inode_num, name, inode)
    }
}

/// An underlying file system
pub trait FileSystem {
    /// Lookup an inode child from parent
    fn lookup(self: Arc<Self>, parent: INodeNumber, name: &str) -> Result<INode, VfsError>;

    /// Mount an inode at the given mount point
    fn mount(&self, parent: INodeNumber, name: &str, inode: INode) -> Result<(), VfsError>;

    /// Read from an inode
    fn read(&self, inode_num: INodeNumber, offset: u64, buffer: &mut [u8]) -> Result<(), VfsError>;

    /// Return the root inode number, the default implementation will always return inode number zero
    fn root(&self) -> INodeNumber { 0 }
}


