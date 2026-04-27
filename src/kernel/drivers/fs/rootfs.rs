//! The root file system

use crate::kernel::vfs::inode::{INode, INodeNumber, FileSystem};

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

use spin::RwLock;


/// A root directory for mounting other file systems
pub struct Root {
    mounts: RwLock<BTreeMap<String, INode>>,
}

impl Root {
    /// Create a new root directory
    pub fn new() -> Root {
        Root {
            mounts: RwLock::new(BTreeMap::new()),
        }
    }
}

impl FileSystem for Root {
    fn lookup(&self, _parent: INodeNumber, name: &str) -> Option<INode> {
        self.mounts.read().get(name).cloned()
    }

    fn mount(&self, _parent: INodeNumber, name: &str, inode: INode) {
        self.mounts.write().insert(name.to_string(), inode);
    }
}


