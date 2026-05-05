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

use kernel_utils::bitmap::Bitmap;
use spin::{Mutex, Lazy};

/// The global virtual file system handle
static VFS: Lazy<Mutex<VirtualFileSystem>> = Lazy::new(|| {
    let rootfs = Root::new();

    Mutex::new(VirtualFileSystem::new(INode::new(rootfs.root(), Arc::new(rootfs))))
});

/// Initialize the virtual file system
pub fn init() -> Result<(), VfsError> {
    let vfs = VFS.lock();

    // TODO: in the future the file systems should all be mounted by userspace programs, eg. an init process
    //
    // this is only for testing

    /*
    let root_fd = vfs.open(OwnedPath::from("/"))?;

    vfs.create_dir(root_fd, "proc");

    let proc_fd = vfs.open

    vfs.mount();

    // TODO: we must create the /proc directory before we can mount to it

    let procfs = ProcFs::default();

    root.mount("proc", INode::new(procfs.root(), Arc::new(procfs)))?;
    */

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

/// Allocators in the virtual file system.
///
/// There are 65536 inode id's and 16384 file descriptors.
pub struct Allocators {
    inode: Bitmap<512>,
    fd: Bitmap<128>,
}

impl Allocators {
    /// Create new allocators for the virtual file system
    pub fn new() -> Allocators {
        Allocators {
            inode: Bitmap::new(),
            fd: Bitmap::new(),
        }
    }
}

/// The virtual file system
pub struct VirtualFileSystem {
    descriptors: BTreeMap<FileDescriptorId, FileDescriptor>,
    mounts: BTreeMap<INodeId, INodeId>,
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
            mounts: BTreeMap::new(),
            inodes: BTreeMap::from_iter([(0, root)]),
            cache: Cache::new(),
            alloc: Allocators::new(),
            root: 0,
        }
    }

    /// Return the root inode id
    pub fn root(&self) -> INodeId { self.root }

    /// Open a file descriptor and return its id
    pub fn open(&mut self, path: OwnedPath) -> Result<FileDescriptorId, VfsError> {
        let inode_id = self.resolve(path)?;
        let fd_id = self.alloc.fd.alloc().ok_or(VfsError::OutOfId)?;

        self.descriptors.insert(fd_id, FileDescriptor::new(inode_id, 0));

        Ok(fd_id)
    }

    /// Create a subdirectory under parent
    pub fn create_dir(&mut self, parent: FileDescriptorId, name: &str) -> Result<(), VfsError> {
        let descriptor = self.descriptors.get(&parent).ok_or(VfsError::NoSuchFile)?;

        self.inodes.get(&descriptor.inode_id)
            .ok_or(VfsError::NoSuchFile)?
            .create_dir(name)
    }

    // TODO: we should turn file descriptor id and inode id into separate structs instead of just
    // renaming them with types. this is because there can be confusion between them

    /// Mount a root inode at a mount point
    pub fn mount(&mut self, mount_point: INodeId, root: INodeId) {
        self.mounts.insert(mount_point, root);
    }

    /// Resolve a path and return corresponding inode id
    pub fn resolve(&mut self, path: OwnedPath) -> Result<INodeId, VfsError> {
        let mut current = path.is_absolute()
            .then_some(self.root)
            .unwrap_or_else(|| todo!("get the current working directory from the process"));

        for name in path.components().filter(|component| !component.is_empty()) {
            if let Some(root) = self.mounts.get(&current) {
                current = *root;
            } else if let Some(cached) = self.cache.get(current, name) {
                current = cached;
            } else {
                let inode = self.inodes.get(&current).ok_or(VfsError::NoSuchFile)?.lookup(name)?;

                let id = self.alloc.inode.alloc().ok_or(VfsError::OutOfId)?;

                self.inodes.insert(id, inode);

                self.cache.insert(current, name.to_string(), id);

                current = id;
            }
        }

        Ok(current)
    }
}


