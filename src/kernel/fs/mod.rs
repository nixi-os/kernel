//! File system implementations

pub mod devfs;
pub mod initramfs;
pub mod procfs;
pub mod rootfs;

use devfs::DevFs;
use procfs::ProcFs;

use crate::kernel::device::block::BlockDevice;
use crate::kernel::fs::initramfs::InitramFs;
use crate::kernel::vfs::error::VfsError;
use crate::kernel::vfs::inode::INodeNumber;

use alloc::sync::Arc;

/// The interface between the virtual file system and file system implementations
pub trait FileSystem {
    /// Lookup an inode child from parent
    fn lookup(&self, parent: INodeNumber, name: &str) -> Result<INodeNumber, VfsError>;

    /// Create a subdirectory under parent
    fn create_dir(&self, parent: INodeNumber, name: &str) -> Result<(), VfsError>;

    /// Read from inode
    fn read(&self, inode_num: INodeNumber, offset: u64, buf: &mut [u8]) -> Result<u64, VfsError>;

    /// Write to inode
    fn write(&self, inode_num: INodeNumber, offset: u64, buf: &[u8]) -> Result<u64, VfsError>;

    /// Return the root inode number, the default implementation will always return inode number zero
    fn root(&self) -> INodeNumber {
        INodeNumber::new(0)
    }
}

/// Prepare a new file system
pub fn prepare_fs(
    name: &str,
    device: Option<Arc<dyn BlockDevice>>,
) -> Result<Arc<dyn FileSystem + Send + Sync>, VfsError> {
    match name {
        "proc" => Ok(Arc::new(ProcFs::default())),
        "dev" => Ok(Arc::new(DevFs::default())),
        "initramfs" => Ok(Arc::new(InitramFs::new())),
        _ => Err(VfsError::Unsupported),
    }
}
