//! The initramfs

use crate::kernel::parse::cpio::{CpioEntry, CpioParser};
use crate::kernel::vfs::error::VfsError;
use crate::kernel::vfs::inode::INodeNumber;
use crate::kernel::vfs::interface::{FileSystem, Metadata};

use alloc::vec;
use alloc::vec::Vec;

/// The initramfs is built into the kernel at compile-time and mounted as root at boot
pub struct InitramFs {
    entries: Vec<CpioEntry<'static>>,
}

impl InitramFs {
    /// Create a new initramfs
    pub fn new() -> InitramFs {
        let mut entries = vec![CpioEntry {
            path: "",
            data: &[],
        }];

        entries.extend(CpioParser::new(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/initramfs.cpio"
        ))));

        InitramFs { entries }
    }
}

impl Metadata for InitramFs {
    fn length(&self, inode_num: INodeNumber) -> Result<u64, VfsError> {
        let entry = &self.entries[inode_num.value() as usize];

        Ok(entry.data.len() as u64)
    }
}

impl FileSystem for InitramFs {
    fn lookup(&self, parent: INodeNumber, name: &str) -> Result<INodeNumber, VfsError> {
        let parent_entry = &self.entries[parent.value() as usize];

        self.entries
            .iter()
            .enumerate()
            .find(|(_, entry)| {
                entry.path.starts_with(&parent_entry.path)
                    && entry.path.ends_with(name)
                    && if !parent_entry.path.is_empty() {
                        entry.path.split('/').count() == parent_entry.path.split('/').count() + 1
                    } else {
                        true
                    }
            })
            .map(|(index, _)| INodeNumber::new(index as u128))
            .ok_or(VfsError::NoSuchFile)
    }

    fn create_dir(&self, _parent: INodeNumber, _name: &str) -> Result<(), VfsError> {
        Err(VfsError::Unsupported)
    }

    fn read(&self, inode_num: INodeNumber, offset: u64, buf: &mut [u8]) -> Result<u64, VfsError> {
        let entry = &self.entries[inode_num.value() as usize];

        if entry.data.len() > offset as usize {
            let length = (entry.data.len() - offset as usize).min(buf.len());

            buf[..length].copy_from_slice(&entry.data[offset as usize..offset as usize + length]);

            Ok(length as u64)
        } else {
            Err(VfsError::OutOfBounds)
        }
    }

    fn write(&self, _inode_num: INodeNumber, _offset: u64, _buf: &[u8]) -> Result<u64, VfsError> {
        Err(VfsError::Unsupported)
    }
}
