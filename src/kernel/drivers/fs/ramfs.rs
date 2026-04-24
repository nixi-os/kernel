//! ramfs is a ram-only file system which does not use an underlying block device

use crate::kernel::vfs::inode::{INode, INodeId, INodeIdAllocator, File, Directory};

pub struct RamFile {
}

impl File for RamFile {}


pub struct RamDirectory {
}

impl Directory for RamDirectory {
    fn resolve(&self, name: &str, alloc: &mut INodeIdAllocator) -> Option<INodeId> {
        todo!()
    }

    fn create_inode(&self, id: INodeId) -> Option<INode> {
        todo!()
    }
}


