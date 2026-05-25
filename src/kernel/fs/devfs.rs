//! The device file system is an interface for the device manager trough the virtual file system

use super::FileSystem;

use crate::kernel::vfs::error::VfsError;
use crate::kernel::vfs::inode::INodeNumber;

// TODO: we should maybe have a generic implementation for state embedded in inode numbers, the
// same way we do it in procfs, except its generic
//
// hmm, we actually dont want deterministic inode numbers, we want dynamic one-time-use inode numbers

/// The device file system
#[derive(Default)]
pub struct DevFs;

impl FileSystem for DevFs {
    fn lookup(&self, parent: INodeNumber, name: &str) -> Result<INodeNumber, VfsError> {
        Err(VfsError::Unsupported)
    }

    fn create_dir(&self, parent: INodeNumber, name: &str) -> Result<(), VfsError> {
        Err(VfsError::Unsupported)
    }

    fn read(&self, inode_num: INodeNumber, offset: u64, buf: &mut [u8]) -> Result<u64, VfsError> {
        Err(VfsError::Unsupported)
    }

    fn write(&self, inode_num: INodeNumber, offset: u64, buf: &[u8]) -> Result<u64, VfsError> {
        Err(VfsError::Unsupported)
    }
}
