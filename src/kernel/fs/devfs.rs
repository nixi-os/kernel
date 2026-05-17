//! The device file system allows devices to be interfaced with through the vfs

use super::FileSystem;

use crate::kernel::vfs::error::VfsError;
use crate::kernel::vfs::inode::INodeNumber;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::boxed::Box;

/// A device is a physical or virtual hardware component which interacts with the computer
pub trait Device {
    /// Read from device
    fn read(&self, offset: u64, buf: &mut [u8]) -> Result<u64, VfsError>;

    /// Write to device
    fn write(&self, offset: u64, buf: &[u8]) -> Result<u64, VfsError>;
}

/// The device file system
#[derive(Default)]
pub struct DevFs {
    devices: BTreeMap<String, Box<dyn Device + Send + Sync>>,
}

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


