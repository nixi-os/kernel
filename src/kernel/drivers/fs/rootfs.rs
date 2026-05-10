//! The root file system

use crate::kernel::vfs::inode::INodeNumber;
use crate::kernel::vfs::fs::FileSystem;
use crate::kernel::vfs::error::VfsError;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

use spin::{Mutex, RwLock};


/// The root file system is a flat directory that can only contain subdirectories
pub struct Root {
    entries: RwLock<BTreeMap<String, INodeNumber>>,
    next: Mutex<u128>,
}

impl Root {
    /// Create a new root directory
    pub fn new() -> Root {
        Root {
            entries: RwLock::new(BTreeMap::new()),
            next: Mutex::new(0),
        }
    }
}

impl FileSystem for Root {
    fn lookup(&self, _parent: INodeNumber, name: &str) -> Result<INodeNumber, VfsError> {
        let inode_num = self.entries.read().get(name).cloned().ok_or(VfsError::NoSuchFile)?;

        Ok(inode_num)
    }

    fn create_dir(&self, _parent: INodeNumber, name: &str) -> Result<(), VfsError> {
        let mut next = self.next.lock();

        *next += 1;

        self.entries.write().insert(name.to_string(), INodeNumber::new(*next));

        Ok(())
    }

    fn read(&self, _inode_num: INodeNumber, _offset: u64, _buffer: &mut [u8]) -> Result<(), VfsError> {
        Err(VfsError::Unsupported)
    }
}


