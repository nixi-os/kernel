//! Code for working with file descriptors

use super::inode::INodeId;

/// The file descriptor id is an integer which points to a file descriptor
pub type FileDescriptorId = usize;

/// A file descriptor represents an open file. It holds the inode id and descriptor state
pub struct FileDescriptor {
    inode_id: INodeId,
    offset: u64,
}

impl FileDescriptor {
    /// Create a new file descriptor
    pub fn new(inode_id: INodeId, offset: u64) -> FileDescriptor {
        FileDescriptor {
            inode_id,
            offset,
        }
    }
}


