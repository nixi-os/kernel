//! The dentry cache ensures fast lookup of directory entries

use super::inode::INodeId;

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;

// TODO: make capacity actually do something

/// The dentry cache stores dentries in an LRU cache
pub struct DEntryCache {
    dentry: BTreeMap<INodeId, BTreeMap<String, INodeId>>,
    order: VecDeque<INodeId>,
    capacity: usize,
}

impl DEntryCache {
    /// Create a new dentry cache
    pub fn new(capacity: usize) -> DEntryCache {
        DEntryCache {
            dentry: BTreeMap::new(),
            order: VecDeque::new(),
            capacity,
        }
    }

    /// Mark a dentry as recently used
    pub fn touch(&mut self, parent: INodeId) {
        if let Some(index) = self.order.iter().position(|id| *id == parent) {
            self.order.remove(index);

            self.order.push_back(parent);
        }
    }

    /// Get an inode id from its parent and name
    pub fn get(&mut self, parent: INodeId, name: &str) -> Option<INodeId> {
        self.touch(parent);

        self.dentry.get(&parent).and_then(|parent| parent.get(name)).copied()
    }

    /// Insert an inode id under its parent and name
    pub fn insert(&mut self, parent: INodeId, name: String, inode: INodeId) {
        if let Some(parent) = self.dentry.get_mut(&parent) {
            parent.insert(name, inode);
        } else {
            self.order.push_back(parent);

            self.dentry.insert(parent, BTreeMap::from([(name, inode)]));
        }
    }
}



