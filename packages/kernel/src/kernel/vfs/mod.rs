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
use alloc::collections::{BTreeMap, VecDeque};
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
    // TODO: in the future the file systems should all be mounted by userspace programs, eg. an init process
    //
    // this is only for testing

    let mut vfs = VFS.lock();
    let root = vfs.root();

    vfs.create_dir(root, "proc")?;

    let mount_point = vfs.resolve(OwnedPath::from("/proc"))?;

    let procfs = ProcFs::default();

    let inode_id = vfs.cache.insert_inode(INode::new(procfs.root(), Arc::new(procfs)));

    vfs.mount(mount_point, inode_id);

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

// TODO: maybe we should generalize the LRU cache so that we can have separate ones for inodes and dentries

/// A LRU cache for inodes and dentry
pub struct LruCache {
    inodes: BTreeMap<INodeId, INode>,
    dentry: BTreeMap<INodeId, BTreeMap<String, INodeId>>,
    order: VecDeque<INodeId>,
    inode_id: INodeId,
    capacity: usize,
}

impl LruCache {
    /// Create a new LRU cache for inodes and dentry
    pub fn new(root: INode, capacity: usize) -> LruCache {
        LruCache {
            inodes: BTreeMap::from_iter([(INodeId::new(0), root)]),
            dentry: BTreeMap::new(),
            order: VecDeque::new(),
            inode_id: INodeId::new(0),
            capacity,
        }
    }

    // TODO: we should evict in the insert function

    /// Insert an inode into the LRU cache and return the inode id
    pub fn insert_inode(&mut self, inode: INode) -> INodeId {
        let inode_id = self.inode_id.increment_next();

        self.inodes.insert(inode_id, inode);
        self.order.push_front(inode_id);

        inode_id
    }

    /// Return an inode if inode id exists
    pub fn get_inode(&self, inode_id: INodeId) -> Option<&INode> {
        self.inodes.get(&inode_id)
    }

    /// Insert an inode id under its parent and name
    pub fn insert_dentry(&mut self, parent: INodeId, name: String, inode: INodeId) {
        if let Some(parent) = self.dentry.get_mut(&parent) {
            parent.insert(name, inode);
        } else {
            self.dentry.insert(parent, BTreeMap::from([(name, inode)]));
        }
    }

    /// Get an inode id from its parent and name
    pub fn get_dentry(&self, parent: INodeId, name: &str) -> Option<INodeId> {
        self.dentry.get(&parent).and_then(|parent| parent.get(name)).copied()
    }
}

/// The virtual file system
pub struct VirtualFileSystem {
    descriptors: BTreeMap<FileDescriptorId, FileDescriptor>,
    mounts: BTreeMap<INodeId, INodeId>,
    cache: LruCache,
    fd_alloc: Bitmap<128>,
    root: INodeId,
}

impl VirtualFileSystem {
    /// Create a new virtual file system with a root inode
    pub fn new(root: INode) -> VirtualFileSystem {
        VirtualFileSystem {
            descriptors: BTreeMap::new(),
            mounts: BTreeMap::new(),
            cache: LruCache::new(root, 100),
            fd_alloc: Bitmap::new(),
            root: INodeId::new(0),
        }
    }

    /// Return the root inode id
    pub fn root(&self) -> INodeId { self.root }

    /// Open a file descriptor and return its id
    pub fn open(&mut self, path: OwnedPath) -> Result<FileDescriptorId, VfsError> {
        let inode_id = self.resolve(path)?;
        let fd_id = FileDescriptorId::new(self.fd_alloc.alloc().ok_or(VfsError::OutOfId)?);

        self.descriptors.insert(fd_id, FileDescriptor::new(inode_id, 0));

        Ok(fd_id)
    }

    /// Create a subdirectory under parent
    pub fn create_dir(&mut self, parent: INodeId, name: &str) -> Result<(), VfsError> {
        self.cache.get_inode(parent)
            .ok_or(VfsError::NoSuchFile)?
            .create_dir(name)
    }

    // TODO: mount should ping the mount point and root inodes, so that they wont get evicted
    // before they are unmounted.

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
            } else if let Some(cached) = self.cache.get_dentry(current, name) {
                current = cached;
            } else {
                let inode = self.cache.get_inode(current).ok_or(VfsError::NoSuchFile)?.lookup(name)?;

                let inode_id = self.cache.insert_inode(inode);

                self.cache.insert_dentry(current, name.to_string(), inode_id);

                current = inode_id;
            }
        }

        Ok(current)
    }
}


