//! The program loader

pub mod error;

use error::LoaderError;

use crate::parse::elf::ElfObject;
use crate::vfs::error::VfsError;
use crate::vfs::{self, OwnedPath};

use alloc::vec;
use alloc::vec::Vec;

/// Load a program from the file system
pub fn load_from_fs(path: &str) -> Result<(), LoaderError> {
    let program = vfs::with_vfs(|vfs| -> Result<Vec<u8>, VfsError> {
        let fd = vfs.open(OwnedPath::from(path))?;

        let metadata = vfs.metadata(fd)?;

        let mut buf = vec![0u8; metadata.length as usize];

        vfs.read(fd, &mut buf)?;

        Ok(buf)
    })?;

    let object = ElfObject::parse(&program).ok_or(LoaderError::InvalidElf)?;

    for header in object.program_headers() {
        crate::log!("header: {:x?}", header);
    }

    Ok(())
}
