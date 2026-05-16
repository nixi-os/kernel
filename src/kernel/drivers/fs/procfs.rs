//! The procfs provides an interface for managing processes through the virtual file system

use crate::kernel::vfs::inode::INodeNumber;
use crate::kernel::vfs::fs::FileSystem;
use crate::kernel::vfs::error::VfsError;


/// The ProcPathFlags allows a procfs path to be encoded as an inode number
struct ProcPathFlags;

impl ProcPathFlags {
    /// The process bit, this corresponds to '/proc/{pid}'
    pub const PROC: u128 = 1;

    /// The file bits are bits 1-31, these bits encode a file, this means there can exist up to 0x7fffffff files under each '/proc/{pid}/'
    pub const FILE: u128 = 0x7fffffff << 1;
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
    fn lookup(&self, parent: INodeNumber, name: &str) -> Result<INodeNumber, VfsError> {
        if parent.value() & ProcPathFlags::PROC == 0 {
            let pid = name.parse::<u32>().ok().filter(|pid| *pid > 0).ok_or(VfsError::NoSuchFile)? as u128;
            let inode_num = INodeNumber::new((pid << 32) | ProcPathFlags::PROC);

            Ok(inode_num)
        } else if parent.value() & ProcPathFlags::FILE == 0 {
            let proc_file = ProcFile::new(name)?;
            let inode_num = INodeNumber::new(parent.value() | ((proc_file as u128) << 1));

            Ok(inode_num)
        } else {
            Err(VfsError::NoSuchFile)
        }
    }

    fn create_dir(&self, _parent: INodeNumber, _name: &str) -> Result<(), VfsError> {
        Err(VfsError::Unsupported)
    }

    fn read(&self, inode_num: INodeNumber, offset: u64, buf: &mut [u8]) -> Result<u64, VfsError> {
        match ProcFile::try_from(((inode_num.value() & ProcPathFlags::FILE) >> 1) as u32)? {
            ProcFile::Cwd => {
                // TODO: read current working directory into buffer

                Ok(0)
            },
        }
    }
}


