//! The virtual file system

pub mod inode;

use inode::{INode, INodeId, INodeIdAllocator};

use crate::kernel::scheduler;

use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;

use core::str::Split;


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
    alloc: INodeIdAllocator,
    cache: Cache,
    root: INodeId,
}

impl VirtualFileSystem {
    /// Create a new virtual file system with a root inode
    pub fn new(root: INode) -> VirtualFileSystem {
        VirtualFileSystem {
            inodes: BTreeMap::from_iter([(0, root)]),
            alloc: INodeIdAllocator::new(0),
            cache: Cache::new(),
            root: 0,
        }
    }

    /// Resolve a path and return corresponding inode id
    pub fn resolve(&mut self, path: OwnedPath) -> Option<INodeId> {
        let mut current = path.is_absolute()
            .then_some(self.root)
            .unwrap_or_else(|| todo!("get the current working directory from the process"));

        for name in path.components().filter(|component| !component.is_empty()) {
            if let Some(cached) = self.cache.get(current, name) {
                current = cached;
            } else {
                let dir = self.inodes[&current].as_dir()?;

                let id = dir.resolve(name, &mut self.alloc)?;

                let inode = dir.create_inode(id)?;

                self.inodes.insert(id, inode);

                self.cache.insert(current, name.to_string(), id);

                current = id;
            }
        }

        Some(current)
    }
}


