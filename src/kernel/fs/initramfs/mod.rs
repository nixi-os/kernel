pub mod cpio;

use super::FileSystem;

use crate::kernel::vfs::error::VfsError;
use crate::kernel::vfs::inode::INodeNumber;

use cpio::{CpioEntry, CpioParser};

use alloc::vec::Vec;

/// The initramfs is built into the kernel at compile-time and mounted as root at boot
pub struct InitramFs {
    entries: Vec<CpioEntry<'static>>,
}

impl InitramFs {
    pub fn new() -> InitramFs {
        let parser = CpioParser::new(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/initramfs.cpio"
        )));

        InitramFs {
            entries: parser
                .filter(|entry| entry.path != "TRAILER!!!")
                .collect::<Vec<CpioEntry<'static>>>(),
        }
    }
}

impl FileSystem for InitramFs {
    fn lookup(&self, _parent: INodeNumber, name: &str) -> Result<INodeNumber, VfsError> {
        // TODO: support nested file structures

        self.entries
            .iter()
            .find(|entry| entry.path.trim_matches('/') == name)
            .map(|entry| INodeNumber::new(entry.inode as u128))
            .ok_or(VfsError::NoSuchFile)
    }

    fn create_dir(&self, _parent: INodeNumber, _name: &str) -> Result<(), VfsError> {
        Err(VfsError::Unsupported)
    }

    fn read(&self, inode_num: INodeNumber, offset: u64, buf: &mut [u8]) -> Result<u64, VfsError> {
        let entry = self
            .entries
            .iter()
            .find(|entry| entry.inode as u128 == inode_num.value())
            .ok_or(VfsError::NoSuchFile)?;

        if entry.data.len() > offset as usize {
            buf[..entry.data.len() - offset as usize]
                .copy_from_slice(&entry.data[offset as usize..]);

            Ok(entry.data.len() as u64 - offset)
        } else {
            Err(VfsError::OutOfBounds)
        }
    }

    fn write(&self, _inode_num: INodeNumber, _offset: u64, _buf: &[u8]) -> Result<u64, VfsError> {
        Err(VfsError::Unsupported)
    }
}
