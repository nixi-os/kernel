//! The procfs provides an interface for managing processes through the virtual file system

use crate::kernel::vfs::inode::{INode, INodeNumber, FileSystem};
use crate::kernel::vfs::error::VfsError;

use alloc::sync::Arc;


/// The ProcPathFlags allows a procfs path to be encoded as an inode number
struct ProcPathFlags;

impl ProcPathFlags {
    /// The process bit, this corresponds to '/proc/{pid}'
    pub const PROC: usize = 1;

    /// The file bits are bits 1-31, these bits encode a file, this means there can exist up to 0x7fffffff files under each '/proc/{pid}/'
    pub const FILE: usize = 0x7fffffff << 1;
}

/// Each file under '/proc/{pid}/' has its own file number. This is the number which is encoded in
/// the [file bits](ProcPathFlags::FILE)
#[repr(u32)]
enum ProcFile {
    Cwd = 1,
}

impl TryFrom<u32> for ProcFile {
    type Error = VfsError;

    fn try_from(file: u32) -> Result<ProcFile, VfsError> {
        match file {
            1 => Ok(ProcFile::Cwd),
            _ => Err(VfsError::NoSuchFile),
        }
    }
}

impl ProcFile {
    /// Create a new ProcFile from a filename
    pub fn new(name: &str) -> Result<ProcFile, VfsError> {
        match name {
            "cwd" => Ok(ProcFile::Cwd),
            _ => Err(VfsError::NoSuchFile),
        }
    }
}

/// A procfs implementation, it uses deterministic inode numbers meaning each inode number is the
/// encoding of a path
#[derive(Default)]
pub struct ProcFs;

impl FileSystem for ProcFs {
    fn lookup(self: Arc<ProcFs>, parent: INodeNumber, name: &str) -> Result<INode, VfsError> {
        if parent.num & ProcPathFlags::PROC == 0 {
            let pid = name.parse::<u32>().ok().filter(|pid| *pid > 0).ok_or(VfsError::NoSuchFile)? as usize;
            let inode_num = INodeNumber::new((pid << 32) | ProcPathFlags::PROC);

            Ok(INode::new(inode_num, self as Arc<dyn FileSystem + Send + Sync>))
        } else if parent.num & ProcPathFlags::FILE == 0 {
            let proc_file = ProcFile::new(name)?;
            let inode_num = INodeNumber::new(parent.num | ((proc_file as usize) << 1));

            Ok(INode::new(inode_num, self as Arc<dyn FileSystem + Send + Sync>))
        } else {
            Err(VfsError::NoSuchFile)
        }
    }

    fn mount(&self, _parent: INodeNumber, _name: &str, _inode: INode) -> Result<(), VfsError> {
        Err(VfsError::UnMountable)
    }

    fn read(&self, inode_num: INodeNumber, offset: u64, buffer: &mut [u8]) -> Result<(), VfsError> {
        match ProcFile::try_from(((inode_num.num & ProcPathFlags::FILE) >> 1) as u32)? {
            ProcFile::Cwd => {
                // TODO: read current working directory into buffer
            },
        }

        Ok(())
    }
}


