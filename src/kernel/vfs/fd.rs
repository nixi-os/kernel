//! File descriptors point to open files

use super::inode::INodeId;

use alloc::collections::BTreeMap;

/// The file descriptor id points to a file descriptor
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileDescriptorId(u64);

impl FileDescriptorId {
    /// Create a new file descriptor id
    pub fn new(id: u64) -> FileDescriptorId { FileDescriptorId(id) }

    /// Return the internal value
    pub fn value(&self) -> u64 { self.0 }
}

/// A file descriptor represents an open file. It holds the inode id and descriptor state
pub struct FileDescriptor {
    pub inode_id: INodeId,
    pub offset: u64,
}

/// The file descriptor cache manages open file descriptors and id's
pub struct FileDescriptorCache {
    descriptors: BTreeMap<FileDescriptorId, FileDescriptor>,
    next: u64,
}

impl FileDescriptorCache {
    /// Create a new file descriptor cache
    pub fn new() -> FileDescriptorCache {
        FileDescriptorCache {
            descriptors: BTreeMap::new(),
            next: 0,
        }
    }

    /// Return a mutable reference to a file descriptor
    pub fn get_mut(&mut self, fd_id: FileDescriptorId) -> Option<&mut FileDescriptor> {
        self.descriptors.get_mut(&fd_id)
    }

    /// Open a file descriptor for an inode id
    pub fn open(&mut self, inode_id: INodeId) -> FileDescriptorId {
        self.next += 1;

        let fd_id = FileDescriptorId::new(self.next);

        self.descriptors.insert(fd_id, FileDescriptor {
            inode_id,
            offset: 0,
        });

        fd_id
    }

    /// Close a file descriptor and return its inode id
    pub fn close(&mut self, fd_id: FileDescriptorId) -> Option<INodeId> {
        self.descriptors.remove(&fd_id)
            .map(|fd| fd.inode_id)
    }
}


