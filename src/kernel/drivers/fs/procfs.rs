//! The procfs provides an interface for processes through the virtual file system

use crate::kernel::vfs::inode::{INode, INodeNumber, FileSystem};
use crate::kernel::vfs::error::VfsError;

use alloc::sync::Arc;

/// A proc offset specifies what to read from a procfs entry
pub enum ProcOffset {
    Tasks,
    Cwd,
}

impl ProcOffset {
    pub fn new(offset: u64) -> Result<ProcOffset, VfsError> {
        match offset {
            0 => Ok(ProcOffset::Tasks),
            1 => Ok(ProcOffset::Cwd),
            _ => Err(VfsError::OutOfBounds),
        }
    }
}

/// A procfs implementation
#[derive(Default)]
pub struct ProcFs;

impl FileSystem for ProcFs {
    fn lookup(self: Arc<ProcFs>, _parent: INodeNumber, name: &str) -> Option<INode> {
        Some(INode::new(self as Arc<dyn FileSystem + Send + Sync>, name.parse().ok()?))
    }

    fn mount(&self, _parent: INodeNumber, _name: &str, _inode: INode) -> Result<(), VfsError> {
        Err(VfsError::UnMountable)
    }

    fn read(&self, inode_num: INodeNumber, offset: u64, buffer: &mut [u8]) -> Result<(), VfsError> {
        match ProcOffset::new(offset)? {
            ProcOffset::Tasks => {
                // TODO: here we should write into the buffer whatever data the user is requesting
            },
            ProcOffset::Cwd => {
            },
        }

        Ok(())
    }
}


