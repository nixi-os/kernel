//! The virtual file system

pub mod error;
pub mod inode;
pub mod fd;

use fd::{FileDescriptor, FileDescriptorId};
use inode::{INode, INodeId, FileSystem};
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

/// Initialize the virtual file system
pub fn init() -> Result<(), VfsError> {
    let vfs = VFS.lock();
    let root = vfs.root();

    let procfs = ProcFs::default();

    root.mount("proc", INode::new(procfs.root(), Arc::new(procfs)))?;

    Ok(())
}

/// The id allocator is a bitmap allocator used for both inodes and file descriptors
pub struct IdAllocator {
    bitmap: [u128; 512],
}

impl IdAllocator {
    /// Create a new id allocator
    pub fn new() -> IdAllocator {
        IdAllocator {
            bitmap: [0; 512],
        }
    }

    /// Allocate a new id
    pub fn alloc(&mut self) -> Result<usize, VfsError> {
        for (index, chunk) in self.bitmap.iter_mut().enumerate() {
            if chunk.leading_ones() < u128::BITS {
                *chunk |= 1u128 << chunk.leading_ones();

                return Ok((index * u128::BITS as usize) + (chunk.leading_ones() as usize - 1));
            }
        }

        Err(VfsError::OutOfId)
    }

    /// Free an id
    pub fn free(&mut self, id: usize) {
        self.bitmap[id / u128::BITS as usize] &= !(1u128 << (id % u128::BITS as usize));
    }
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

/// Allocators in the virtual file system
pub struct Allocators {
    inode: IdAllocator,
    fd: IdAllocator,
}

impl Allocators {
    /// Create new allocators for the virtual file system
    pub fn new() -> Allocators {
        Allocators {
            inode: IdAllocator::new(),
            fd: IdAllocator::new(),
        }
    }
}

/// The virtual file system
pub struct VirtualFileSystem {
    descriptors: BTreeMap<FileDescriptorId, FileDescriptor>,
    inodes: BTreeMap<INodeId, INode>,
    cache: Cache,
    alloc: Allocators,
    root: INodeId,
}

impl VirtualFileSystem {
    /// Create a new virtual file system with a root inode
    pub fn new(root: INode) -> VirtualFileSystem {
        VirtualFileSystem {
            descriptors: BTreeMap::new(),
            inodes: BTreeMap::from_iter([(0, root)]),
            cache: Cache::new(),
            alloc: Allocators::new(),
            root: 0,
        }
    }

    /// Return a reference to the root inode
    pub fn root(&self) -> &INode {
        self.inodes.get(&self.root).expect("root must always be in cache")
    }

    /// Open a file descriptor and return its id
    pub fn open(&mut self, path: OwnedPath) -> Result<FileDescriptorId, VfsError> {
        let inode_id = self.resolve(path)?;
        let fd_id = self.alloc.fd.alloc()?;

        self.descriptors.insert(fd_id, FileDescriptor::new(inode_id, 0));

        Ok(fd_id)
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

                let id = self.alloc.inode.alloc()?;

                self.inodes.insert(id, inode);

                self.cache.insert(current, name.to_string(), id);

                current = id;
            }
        }

        Ok(current)
    }
}


