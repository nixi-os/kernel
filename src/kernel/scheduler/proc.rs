use super::TaskId;

use spin::{Lazy, Mutex};

use alloc::vec::Vec;

/// The global process table
static PROCS: Lazy<Mutex<ProcTable>> = Lazy::new(|| Mutex::new(ProcTable::default()));

/// A process id
pub type ProcId = usize;

/// The process table holds all active processes
#[derive(Default)]
pub struct ProcTable {
    procs: Vec<Proc>,
}

// TODO: the process must also store the page table
/// A process, there can be multiple tasks under the same process
pub struct Proc {
    tasks: Vec<TaskId>,
}


