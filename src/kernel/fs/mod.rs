//! File system implementations

pub mod initramfs;
pub mod procfs;
pub mod rootfs;

use procfs::ProcFs;

use crate::kernel::device::block::BlockDevice;
use crate::kernel::fs::initramfs::InitramFs;
use crate::kernel::vfs::error::VfsError;
use crate::kernel::vfs::interface::FileSystem;

use alloc::sync::Arc;

/// Prepare a new file system
pub fn prepare_fs(
    name: &str,
    device: Option<Arc<dyn BlockDevice>>,
) -> Result<Arc<dyn FileSystem + Send + Sync>, VfsError> {
    match name {
        "proc" => Ok(Arc::new(ProcFs::default())),
        "initramfs" => Ok(Arc::new(InitramFs::new())),
        _ => Err(VfsError::Unsupported),
    }
}
