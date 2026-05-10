//! The virtual file system

pub mod error;
pub mod inode;
pub mod dentry;
pub mod fd;
pub mod fs;

use fd::{FileDescriptorCache, FileDescriptorId};
use inode::{INodeCache, INode, INodeId};
use dentry::DEntryCache;
use fs::FileSystem;
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
    // TODO: in the future the file systems should all be mounted by userspace programs, eg. an init process
    //
    // this is only for testing

    let mut vfs = VFS.lock();
    let root = vfs.root();

    vfs.create_dir(root, "proc")?;

    let mount_point = vfs.lookup(OwnedPath::from("/proc"))?;

    let procfs = ProcFs::default();

    let inode_id = vfs.inode_cache.insert(INode::new(procfs.root(), Arc::new(procfs)));

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

/// The virtual file system
pub struct VirtualFileSystem {
    fd_cache: FileDescriptorCache,
    inode_cache: INodeCache,
    dentry_cache: DEntryCache,
    mounts: BTreeMap<INodeId, INodeId>,
    root: INodeId,
}

impl VirtualFileSystem {
    /// Create a new virtual file system with a root inode
    pub fn new(root: INode) -> VirtualFileSystem {
        VirtualFileSystem {
            fd_cache: FileDescriptorCache::new(),
            inode_cache: INodeCache::new(root, 100),
            dentry_cache: DEntryCache::new(1000),
            mounts: BTreeMap::new(),
            root: INodeId::new(0),
        }
    }

    /// Return the root inode id
    pub fn root(&self) -> INodeId { self.root }

    /// Open a file descriptor and return its id
    pub fn open(&mut self, path: OwnedPath) -> Result<FileDescriptorId, VfsError> {
        let inode_id = self.lookup(path)?;

        self.inode_cache.update_rc(inode_id, |rc| rc + 1);

        Ok(self.fd_cache.open(inode_id))
    }

    /// Close a file descriptor
    pub fn close(&mut self, fd_id: FileDescriptorId) {
        if let Some(inode_id) = self.fd_cache.close(fd_id) {
            self.inode_cache.update_rc(inode_id, |rc| rc - 1);
        }
    }

    /// Create a subdirectory under parent
    pub fn create_dir(&mut self, parent: INodeId, name: &str) -> Result<(), VfsError> {
        self.inode_cache.get(parent)
            .ok_or(VfsError::NoSuchFile)?
            .create_dir(name)
    }

    /// Mount an inode at a mount point
    pub fn mount(&mut self, mount_point: INodeId, inode_id: INodeId) {
        self.inode_cache.set_pinned(mount_point, true);
        self.inode_cache.set_pinned(inode_id, true);

        self.mounts.insert(mount_point, inode_id);
    }

    /// Unmount the inode which is mounted at a mount point
    pub fn unmount(&mut self, mount_point: INodeId) {
        self.inode_cache.set_pinned(mount_point, false);

        if let Some(inode_id) = self.mounts.remove(&mount_point) {
            self.inode_cache.set_pinned(inode_id, false);
        }
    }

    /// Lookup a path and return corresponding inode id
    ///
    /// Each lookup is performed in the following order:
    ///  1. Check mount points
    ///  2. Check dentry cache
    ///  3. Query file system
    pub fn lookup(&mut self, path: OwnedPath) -> Result<INodeId, VfsError> {
        let mut current = path.is_absolute()
            .then_some(self.root)
            .unwrap_or_else(|| todo!("get the current working directory from the process"));

        for name in path.components().filter(|component| !component.is_empty()) {
            if let Some(root) = self.mounts.get(&current) {
                current = *root;
            } else if let Some(cached) = self.dentry_cache.get(current, name) {
                current = cached;
            } else {
                let inode = self.inode_cache.get(current).ok_or(VfsError::NoSuchFile)?.lookup(name)?;

                let inode_id = self.inode_cache.insert(inode);

                self.dentry_cache.insert(current, name.to_string(), inode_id);

                current = inode_id;
            }
        }

        Ok(current)
    }
}


