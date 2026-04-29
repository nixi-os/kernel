//! The virtual file system

pub mod error;
pub mod inode;

use inode::{INode, INodeId, INodeIdAllocator, FileSystem};
use error::VfsError;

use crate::kernel::drivers::fs::rootfs::Root;
use crate::kernel::drivers::fs::procfs::ProcFs;

use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;
use alloc::sync::Arc;

use core::str::Split;

use spin::{Mutex, Lazy};

/// The global virtual file system handle
static VFS: Lazy<Mutex<VirtualFileSystem>> = Lazy::new(|| {
    let rootfs = Root::new();

    Mutex::new(VirtualFileSystem::new(INode::new(rootfs.root(), Arc::new(rootfs))))
});

pub fn init() -> Result<(), VfsError> {
    let mut vfs = VFS.lock();
    let root = vfs.root();

    let procfs = ProcFs::default();

    root.mount("proc", INode::new(procfs.root(), Arc::new(procfs)))?;

    crate::log!("resolved '/proc/0': {:?}", vfs.resolve(OwnedPath::from("/proc/0")));

    Ok(())
}

/// A path identifies an inode in the file system
pub struct OwnedPath {
    path: String,
}

impl From<&str> for OwnedPath {
    fn from(path: &str) -> OwnedPath {
        OwnedPath {
            path: path.to_string(),
        }
    }
}

impl OwnedPath {
    /// Get an iterator over the components of the path
    pub fn components<'a>(&'a self) -> Split<'a, char> {
        self.path.split('/')
    }

    /// Returns true if the path is absolute
    pub fn is_absolute(&self) -> bool {
        self.path.starts_with('/')
    }
}

/// A cache of parent and name pairs to inode id's
pub struct Cache {
    cache: BTreeMap<INodeId, BTreeMap<String, INodeId>>,
}

impl Cache {
    /// Create a new cache
    pub fn new() -> Cache {
        Cache {
            cache: BTreeMap::new(),
        }
    }

    /// Get an inode id from its parent and name
    pub fn get(&self, parent: INodeId, name: &str) -> Option<INodeId> {
        self.cache.get(&parent).and_then(|parent| parent.get(name)).copied()
    }

    /// Insert an inode id under its parent and name
    pub fn insert(&mut self, parent: INodeId, name: String, inode: INodeId) {
        if let Some(parent) = self.cache.get_mut(&parent) {
            parent.insert(name, inode);
        } else {
            self.cache.insert(parent, BTreeMap::from([(name, inode)]));
        }
    }
}

/// The virtual file system
pub struct VirtualFileSystem {
    inodes: BTreeMap<INodeId, INode>,
    cache: Cache,
    alloc: INodeIdAllocator,
    root: INodeId,
}

impl VirtualFileSystem {
    /// Create a new virtual file system with a root inode
    pub fn new(root: INode) -> VirtualFileSystem {
        VirtualFileSystem {
            inodes: BTreeMap::from_iter([(0, root)]),
            cache: Cache::new(),
            alloc: INodeIdAllocator::new(0),
            root: 0,
        }
    }

    /// Return a reference to the root inode
    pub fn root(&self) -> &INode {
        self.inodes.get(&self.root).expect("root must always be in cache")
    }

    /// Resolve a path and return corresponding inode id
    pub fn resolve(&mut self, path: OwnedPath) -> Result<INodeId, VfsError> {
        let mut current = path.is_absolute()
            .then_some(self.root)
            .unwrap_or_else(|| todo!("get the current working directory from the process"));

        for name in path.components().filter(|component| !component.is_empty()) {
            if let Some(cached) = self.cache.get(current, name) {
                current = cached;
            } else {
                let inode = self.inodes.get(&current).ok_or(VfsError::NoSuchFile)?.lookup(name)?;

                let id = self.alloc.alloc_inode();

                self.inodes.insert(id, inode);

                self.cache.insert(current, name.to_string(), id);

                current = id;
            }
        }

        Ok(current)
    }
}


