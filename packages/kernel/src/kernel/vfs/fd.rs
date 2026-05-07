//! Code for working with file descriptors

use super::inode::INodeId;

/// The file descriptor id points to a file descriptor
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileDescriptorId(usize);

impl FileDescriptorId {
    /// Create a new file descriptor id
    pub fn new(id: usize) -> FileDescriptorId { FileDescriptorId(id) }
}

/// A file descriptor represents an open file. It holds the inode id and descriptor state
pub struct FileDescriptor {
    pub inode_id: INodeId,
    pub offset: u64,
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


