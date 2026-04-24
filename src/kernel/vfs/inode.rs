//! Code for working with inodes

use alloc::boxed::Box;


/// An inode id points to an inode in the inode vector
pub type INodeId = usize;

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
    pub fn alloc(&mut self) -> INodeId {
        self.id += 1;

        self.id
    }
}

/// An inode is a node in the virtual file system
pub struct INode {
    kind: INodeKind,
}

impl INode {
    /// Returns a directory if the inode is a directory
    pub fn as_dir<'a>(&'a self) -> Option<&'a dyn Directory> {
        match &self.kind {
            INodeKind::File(_) => None,
            INodeKind::Directory(directory) => Some(directory.as_ref()),
        }
    }
}

/// An inode can either be a file or a directory
pub enum INodeKind {
    File(Box<dyn File>),
    Directory(Box<dyn Directory>),
}

/// A file is an I/O interface which can be written and read
pub trait File {
    // TODO: i/o functions
}

/// A directory contains directory entries, inodes are created on-demand
pub trait Directory {
    /// Resolve a relative path and return the corresponding inode id
    fn resolve(&self, name: &str, alloc: &mut INodeIdAllocator) -> Option<INodeId>;

    /// Create an inode from its inode id if it exists in the directory
    fn create_inode(&self, id: INodeId) -> Option<INode>;
}


