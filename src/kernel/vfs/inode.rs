//! Code for working with inodes

use super::error::VfsError;
use super::interface::{FileSystem, Metadata};

use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;

/// An inode id points to an inode globally in the inode cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct INodeId(u128);

impl INodeId {
    /// Create a new inode id
    pub fn new(id: u128) -> INodeId {
        INodeId(id)
    }
}

/// An inode number points to an inode within a specific file system
#[derive(Clone, Copy)]
pub struct INodeNumber(u128);

impl INodeNumber {
    /// Create a new inode number
    pub fn new(num: u128) -> INodeNumber {
        INodeNumber(num)
    }

    /// Return the inner value
    pub fn value(&self) -> u128 {
        self.0
    }
}

/// An inode is a node in the virtual file system. The inode can be backed by any file system implementation
#[derive(Clone)]
pub struct INode {
    fs: Arc<dyn FileSystem + Send + Sync>,
    inode_num: INodeNumber,
    pinned: bool,
    rc: usize,
}

impl INode {
    /// Create a new inode
    pub fn new(inode_num: INodeNumber, fs: Arc<dyn FileSystem + Send + Sync>) -> INode {
        INode {
            fs,
            inode_num,
            pinned: false,
            rc: 0,
        }
    }

    /// Lookup an inode child
    pub fn lookup(&self, name: &str) -> Result<INode, VfsError> {
        let inode_num = self.fs.lookup(self.inode_num, name)?;

        Ok(INode::new(inode_num, Arc::clone(&self.fs)))
    }

    /// Create a subdirectory
    pub fn create_dir(&self, name: &str) -> Result<(), VfsError> {
        self.fs.create_dir(self.inode_num, name)
    }

    /// Get length of an inode
    pub fn length(&self) -> Result<u64, VfsError> {
        self.fs.length(self.inode_num)
    }

    /// Read from inode
    pub fn read(&self, offset: u64, buf: &mut [u8]) -> Result<u64, VfsError> {
        self.fs.read(self.inode_num, offset, buf)
    }

    /// Write to inode
    pub fn write(&self, offset: u64, buf: &[u8]) -> Result<u64, VfsError> {
        self.fs.write(self.inode_num, offset, buf)
    }
}

/// The inode cache is an LRU cache of inodes. The capacity is more a suggestion then a rule, the
/// cache may use more if needed
pub struct INodeCache {
    inodes: BTreeMap<INodeId, INode>,
    order: VecDeque<INodeId>,
    next: u128,
    capacity: usize,
}

impl INodeCache {
    /// Create a new inode cache
    pub fn new(root: INode, capacity: usize) -> INodeCache {
        INodeCache {
            inodes: BTreeMap::from_iter([(INodeId::new(0), root)]),
            order: VecDeque::new(),
            next: 0,
            capacity,
        }
    }

    /// Update the reference count of an inode
    pub fn update_rc<F: FnOnce(usize) -> usize>(&mut self, inode_id: INodeId, f: F) {
        if let Some(inode) = self.inodes.get_mut(&inode_id) {
            inode.rc = f(inode.rc);
        }
    }

    /// Set the pinned flag of an inode, if pinned is true then it will prevent eviction
    pub fn set_pinned(&mut self, inode_id: INodeId, pinned: bool) {
        if let Some(inode) = self.inodes.get_mut(&inode_id) {
            inode.pinned = pinned;
        }
    }

    /// Mark an inode as recently used
    pub fn touch(&mut self, inode_id: INodeId) {
        if let Some(index) = self.order.iter().position(|id| *id == inode_id) {
            self.order.remove(index);

            self.order.push_back(inode_id);
        }
    }

    /// Get an inode from its inode id
    pub fn get(&mut self, inode_id: INodeId) -> &INode {
        self.touch(inode_id);

        &self.inodes[&inode_id]
    }

    /// Insert an inode and return its inode id
    pub fn insert(&mut self, inode: INode) -> INodeId {
        self.evict_lru();

        self.next += 1;
        let inode_id = INodeId::new(self.next);

        self.inodes.insert(inode_id, inode);
        self.order.push_back(inode_id);

        inode_id
    }

    /// Try to evict as many least recently used inodes needed to obey capacity
    pub fn evict_lru(&mut self) {
        let mut count = 0;
        let len = self.order.len();

        while count < len && self.order.len() > self.capacity {
            let Some(inode_id) = self.order.pop_front() else {
                break;
            };

            if self
                .inodes
                .get(&inode_id)
                .map(|inode| inode.pinned || inode.rc > 0)
                .unwrap_or(false)
            {
                self.order.push_back(inode_id);
            } else {
                self.inodes.remove(&inode_id);
            }

            count += 1;
        }
    }
}
