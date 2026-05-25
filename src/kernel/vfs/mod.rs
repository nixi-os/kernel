//! The virtual file system

pub mod dentry;
pub mod error;
pub mod fd;
pub mod inode;
pub mod syscall;

use dentry::DEntryCache;
use error::VfsError;
use fd::{FileDescriptorCache, FileDescriptorId};
use fs::FileSystem;
use inode::{INode, INodeCache, INodeId};

use crate::kernel::device::block::BlockDevice;
use crate::kernel::fs;
use crate::kernel::fs::rootfs::RootFs;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;

use core::str::Split;

use spin::{Lazy, Mutex};

/// The global virtual file system handle
pub static VFS: Lazy<Mutex<VirtualFileSystem>> = Lazy::new(|| {
    let rootfs = RootFs::new();

    Mutex::new(VirtualFileSystem::new(INode::new(
        rootfs.root(),
        Arc::new(rootfs),
    )))
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

    vfs.mount(
        mount_point,
        MountSource::FileSystem {
            name: "proc",
            device: None,
        },
    )?;

    syscall::init();

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
    /// Create a OwnedPath from raw pointer
    pub unsafe fn from_raw_parts(ptr: *mut u8, len: usize) -> OwnedPath {
        OwnedPath {
            path: unsafe { String::from_raw_parts(ptr, len, len) },
        }
    }

    /// Get an iterator over the components of the path
    pub fn components<'a>(&'a self) -> Split<'a, char> {
        self.path.split('/')
    }

    /// Returns true if the path is absolute
    pub fn is_absolute(&self) -> bool {
        self.path.starts_with('/')
    }
}

/// A mount source can either be a bind mount or file system mount
pub enum MountSource<'a> {
    Bind(INodeId),
    FileSystem {
        name: &'a str,
        device: Option<Arc<dyn BlockDevice>>,
    },
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
    pub fn root(&self) -> INodeId {
        self.root
    }

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

    // NOTE: write and read are pretty much identical, however, its not worth it to generalize it
    // when its only used twice

    /// Read bytes into buf from file and return bytes read
    pub fn read(&mut self, fd_id: FileDescriptorId, buf: &mut [u8]) -> Result<u64, VfsError> {
        let fd = self.fd_cache.get_mut(fd_id).ok_or(VfsError::NoSuchFile)?;

        let read = self
            .inode_cache
            .get(fd.inode_id)
            .expect("inode owned by file descriptor should never be evicted")
            .read(fd.offset, buf)?;

        fd.offset += read;

        Ok(read)
    }

    /// Write bytes from buf into file and return bytes written
    pub fn write(&mut self, fd_id: FileDescriptorId, buf: &[u8]) -> Result<u64, VfsError> {
        let fd = self.fd_cache.get_mut(fd_id).ok_or(VfsError::NoSuchFile)?;

        let written = self
            .inode_cache
            .get(fd.inode_id)
            .expect("inode owned by file descriptor should never be evicted")
            .write(fd.offset, buf)?;

        fd.offset += written;

        Ok(written)
    }

    /// Create a subdirectory under parent
    pub fn create_dir(&mut self, parent: INodeId, name: &str) -> Result<(), VfsError> {
        self.inode_cache
            .get(parent)
            .ok_or(VfsError::NoSuchFile)?
            .create_dir(name)
    }

    /// Prepare a mount source
    fn prepare_mount_source(&mut self, source: MountSource) -> Result<INodeId, VfsError> {
        match source {
            MountSource::Bind(inode_id) => Ok(inode_id),
            MountSource::FileSystem { name, device } => {
                let fs = fs::prepare_fs(name, device)?;

                Ok(self.inode_cache.insert(INode::new(fs.root(), fs)))
            }
        }
    }

    /// Mount an inode at a mount point
    pub fn mount(&mut self, mount_point: INodeId, source: MountSource) -> Result<(), VfsError> {
        let inode_id = self.prepare_mount_source(source)?;

        self.inode_cache.set_pinned(mount_point, true);
        self.inode_cache.set_pinned(inode_id, true);

        self.mounts.insert(mount_point, inode_id);

        Ok(())
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
        let mut current = path
            .is_absolute()
            .then_some(self.root)
            .unwrap_or_else(|| todo!("get the current working directory from the process"));

        let mut components = path
            .components()
            .filter(|component| !component.is_empty())
            .peekable();

        while let Some(name) = components.peek() {
            if let Some(root) = self.mounts.get(&current) {
                current = *root;

                continue;
            } else if let Some(cached) = self.dentry_cache.get(current, name) {
                current = cached;
            } else {
                let inode = self
                    .inode_cache
                    .get(current)
                    .ok_or(VfsError::NoSuchFile)?
                    .lookup(name)?;

                let inode_id = self.inode_cache.insert(inode);

                self.dentry_cache
                    .insert(current, name.to_string(), inode_id);

                current = inode_id;
            }

            components.next();
        }

        Ok(current)
    }
}
