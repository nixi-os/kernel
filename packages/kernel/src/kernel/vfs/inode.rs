//! Code for working with inodes

use super::error::VfsError;

use alloc::sync::Arc;

/// An inode id points to an inode globally in the inode cache
pub type INodeId = usize;

/// An inode number points to an inode within a specific file system and stores a generation number
/// which is incremented on each reuse to invalidate stale cache entries
#[derive(Clone, Copy)]
pub struct INodeNumber {
    pub num: usize,
    pub generation: Option<u64>,
}

impl INodeNumber {
    /// Create a new inode number, if generation is true then a generation number will be used
    ///
    /// SAFETY: inode numbers MUST be deterministic OR one time use if you don't use a generation number
    pub fn new(num: usize, generation: bool) -> INodeNumber {
        INodeNumber {
            num,
            generation: generation.then(|| 1),
        }
    }

    /// Reuse a inode number. The generation will be incremented if the inode number has generation
    /// numbers enabled
    pub fn reuse(self) -> INodeNumber {
        INodeNumber {
            num: self.num,
            generation: self.generation.map(|generation| generation + 1),
        }
    }
}

// TODO: the inodes should hold a last accessed timestamp in order to do LRU eviction to evict the
// last used inodes
//
// NOTE: i think the best combination for our virtual file system will be to have a combination of
// LRU eviction to remove old inodes, and generation numbers embedded in inode numbers to ensure
// that old inode numbers dont get reinterpreted as the wrong file if the old file is deleted
//
// TODO: we should have a reference count on the inode, so that its evicted from the inode cache
// once there is no references left. Currently this would only evict files which are part of
// some file systems, this is due to the fact that some file systems only act as a "root" for mount
// points, so it will store the inodes and therefore stop the inode reference count from reaching
// zero. This is not an issue.
//
// For on-disk file systems this is a non-issue, eg. with fat32, ext4 or anything similar we do not
// need to store the inodes, so the reference count would be able to reach zero. This applies
// to virtually all file systems which do not support mount points. For file systems which DO
// support mount points, the mount points will by design stay in memory and therefore never reach a
// reference count of zero.
//
// the reference count should be incremented on each clone, specifically we should create our own
// Clone implementation to have this work.

/// An inode is a node in the virtual file system. The inode can be backed by any file system implementation
#[derive(Clone)]
pub struct INode {
    pub fs: Arc<dyn FileSystem + Send + Sync>,
    pub inode_num: INodeNumber,
}

impl INode {
    /// Create a new inode
    pub fn new(inode_num: INodeNumber, fs: Arc<dyn FileSystem + Send + Sync>) -> INode {
        INode {
            fs,
            inode_num,
        }
    }

    /// Lookup an inode child
    pub fn lookup(&self, name: &str) -> Result<INode, VfsError> {
        Arc::clone(&self.fs).lookup(self.inode_num, name)
    }

    /// Create a subdirectory
    pub fn create_dir(&self, name: &str) -> Result<(), VfsError> {
        Arc::clone(&self.fs).create_dir(self.inode_num, name)
    }
}

/// An underlying file system
pub trait FileSystem {
    /// Lookup an inode child from parent
    fn lookup(self: Arc<Self>, parent: INodeNumber, name: &str) -> Result<INode, VfsError>;

    /// Create a subdirectory under parent
    fn create_dir(self: Arc<Self>, parent: INodeNumber, name: &str) -> Result<(), VfsError>;

    /// Read from an inode
    fn read(&self, inode_num: INodeNumber, offset: u64, buffer: &mut [u8]) -> Result<(), VfsError>;

    /// Return the root inode number, the default implementation will always return inode number zero
    fn root(&self) -> INodeNumber { INodeNumber::new(0, false) }
}


