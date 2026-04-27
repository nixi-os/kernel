//! Code for working with inodes

use alloc::sync::Arc;

/// An inode id points to an inode globally in the inode cache
pub type INodeId = usize;

/// An inode number points to an inode within a specific file system
pub type INodeNumber = usize;

/// An allocator for inode id's
pub struct INodeIdAllocator {
    id: INodeId,
}

impl INodeIdAllocator {
    /// Create a new inode id allocator with a starting id
    pub fn new(id: INodeId) -> INodeIdAllocator {
        INodeIdAllocator {
            id,
        }
    }

    /// Allocate a new inode id
    pub fn alloc_inode(&mut self) -> INodeId {
        self.id += 1;

        self.id
    }
}

/// An inode is a node in the virtual file system. The inode can be backed by any file system implementation
#[derive(Clone)]
pub struct INode {
    pub fs: Arc<dyn FileSystem + Send + Sync>,
    pub inode_num: INodeNumber,
}

impl INode {
    /// Create a new inode
    pub fn new(fs: Arc<dyn FileSystem + Send + Sync>, inode_num: INodeNumber) -> INode {
        INode {
            fs,
            inode_num,
        }
    }

    /// Lookup an inode child
    pub fn lookup(&self, name: &str) -> Option<INode> {
        self.fs.lookup(self.inode_num, name)
    }
}

/// An underlying file system
pub trait FileSystem {
    /// Lookup an inode child from parent
    fn lookup(&self, parent: INodeNumber, name: &str) -> Option<INode>;

    /// Mount an inode at the given mount point
    fn mount(&self, parent: INodeNumber, name: &str, inode: INode);
}


