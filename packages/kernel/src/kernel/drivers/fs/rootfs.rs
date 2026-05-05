//! The root file system

use crate::kernel::vfs::inode::{INode, INodeNumber, FileSystem};
use crate::kernel::vfs::error::VfsError;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;

use core::sync::atomic::{Ordering, AtomicUsize};

use spin::RwLock;


/// The root directory is a flat directory that can only contain subdirectories
pub struct Root {
    entries: RwLock<BTreeMap<String, INodeNumber>>,
    count: AtomicUsize,
}

impl Root {
    /// Create a new root directory
    pub fn new() -> Root {
        Root {
            entries: RwLock::new(BTreeMap::new()),
            count: AtomicUsize::new(0),
        }
    }
}

impl FileSystem for Root {
    fn lookup(self: Arc<Root>, _parent: INodeNumber, name: &str) -> Result<INode, VfsError> {
        let inode_num = self.entries.read().get(name).cloned().ok_or(VfsError::NoSuchFile)?;

        Ok(INode::new(inode_num, self as Arc<dyn FileSystem + Send + Sync>))
    }

    fn create_dir(self: Arc<Root>, _parent: INodeNumber, name: &str) -> Result<(), VfsError> {
        self.entries.write().insert(name.to_string(), INodeNumber::new(self.count.fetch_add(1, Ordering::Relaxed), false));

        Ok(())
    }

    fn read(&self, _inode_num: INodeNumber, _offset: u64, _buffer: &mut [u8]) -> Result<(), VfsError> {
        Err(VfsError::Unsupported)
    }
}


