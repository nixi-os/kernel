//! The scheduler implements multitasking on a single cpu

pub mod error;
pub mod context;
pub mod task;
pub mod proc;

use error::SchedulerError;
use context::Context;

use task::{TaskManager, Task, TaskId};
use proc::{ProcManager, ProcId};

use spin::{Lazy, Mutex};

/// The global scheduler
static SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| Mutex::new(Scheduler::new()));

/// Call a closure with scheduler
pub fn with_scheduler<F: FnOnce(&mut Scheduler) -> R, R>(f: F) -> R {
    let mut scheduler = SCHEDULER.lock();

    f(&mut scheduler)
}

/// The scheduler manages tasks and processes
pub struct Scheduler {
    task_manager: TaskManager,
    proc_manager: ProcManager,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Scheduler {
        Scheduler {
            task_manager: TaskManager::new(),
            proc_manager: ProcManager::new(),
        }
    }

    /// Perform a context switch
    pub fn switch(&mut self, ctx: Context) -> Context {
        let (proc_id, ctx) = self.task_manager.switch(ctx);

        self.proc_manager.load_pt(proc_id);

        ctx
    }

    /// Create a new process and return its process id
    pub fn create_proc(&mut self) -> ProcId {
        self.proc_manager.create()
    }

    /// Create a new task for a process and return its task id
    pub fn create_task(&mut self, proc_id: ProcId, entry: u64, privilege_level: u8) -> TaskId {
        let task_id = self.task_manager.create(Task::new(proc_id, entry, privilege_level));

        self.proc_manager.adopt_task(proc_id, task_id);

        task_id
    }
}


