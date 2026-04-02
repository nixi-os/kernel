pub mod error;
pub mod context;

use error::SchedulerError;
use context::Context;

use crate::kernel::cpu;

use spin::{Lazy, Mutex};

use alloc::alloc::Layout;
use alloc::collections::BTreeMap;
use alloc::boxed::Box;
use alloc::vec::Vec;


/// The global scheduler
static SCHEDULER: Lazy<Mutex<Option<Scheduler>>> = Lazy::new(|| Mutex::new(None));


/// The scheduler manages processes and tasks
pub struct Scheduler {
    procs: BTreeMap<usize, Proc>,
    current: TaskDescriptor,
}

impl Scheduler {
    pub fn new(init: Proc) -> Scheduler {
        Scheduler {
            procs: BTreeMap::from_iter([(0, init)]),
            current: TaskDescriptor::new(0, 0),
        }
    }

    /// Allocate a new process id
    fn alloc_pid(&self) -> Result<usize, SchedulerError> {
        let pid = self.procs.keys()
            .map_windows(|[pid1, pid2]| (**pid1, **pid2))
            .find_map(|(pid1, pid2)| (pid2 - pid1 > 1).then_some(pid1 + 1));

        match pid {
            Some(pid) => Ok(pid),
            None => self.procs.keys().max().map(|max| max + 1).ok_or(SchedulerError::OutOfPid),
        }
    }

    /// Get the next task
    fn next_task(&self) -> TaskDescriptor {
        assert!(!self.procs.is_empty());

        if self.current.tid >= self.procs[&self.current.pid].tasks.len() - 1 {
            let next = self.procs.range(self.current.pid+1..).next();
            let (pid, _) = next.unwrap_or_else(|| self.procs.first_key_value().expect("procs shouldn't be empty"));

            TaskDescriptor::new(*pid, 0)
        } else {
            TaskDescriptor::new(self.current.pid, self.current.tid + 1)
        }
    }

    /// Perform a context switch from the current task to the next task
    unsafe fn switch(&mut self, ctx: Context) -> Context {
        let next = self.next_task();

        self.procs.get_mut(&self.current.pid).expect("current task descriptor should never be invalid").tasks[self.current.tid].ctx = ctx;

        // TODO: we must set the kernel stack on each context switch

        unsafe {
            core::arch::x86_64::_xsave(self.procs[&self.current.pid].tasks[self.current.tid].xsave, u64::MAX);

            core::arch::x86_64::_xrstor(self.procs[&next.pid].tasks[next.tid].xsave, u64::MAX);
        }

        self.current = next;

        self.procs[&next.pid].tasks[next.tid].ctx
    }
}

/// A process, a single process can have multiple tasks
#[derive(Default)]
pub struct Proc {
    tasks: Vec<Task>,
}

/// A task which will be run by the scheduler on interval
pub struct Task {
    ctx: Context,
    user_stack: Box<[u8; 4096 * 4]>,
    kernel_stack: Box<[u8; 4096 * 4]>,
    xsave: *mut u8,
}

impl Task {
    /// Create a new task
    pub fn new() -> Task {
        let layout = Layout::from_size_align(cpu::required_xsave_size() as usize, 64).expect("xsave allocation shouldn't break alignment rules");
        let xsave = unsafe { alloc::alloc::alloc_zeroed(layout) };

        Task {
            ctx: Context::default(),
            user_stack: Box::new([0; 4096 * 4]),
            kernel_stack: Box::new([0; 4096 * 4]),
            xsave,
        }
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        unsafe {
            // NOTE: this uses a bogus layout since our allocator doesnt make use of the layout when deallocating
            alloc::alloc::dealloc(self.xsave, Layout::from_size_align_unchecked(8, 8));
        }
    }
}

unsafe impl Sync for Task {}
unsafe impl Send for Task {}

/// A task descriptor has the process id and the task id
#[derive(Clone, Copy)]
pub struct TaskDescriptor {
    pid: usize,
    tid: usize,
}

impl TaskDescriptor {
    pub fn new(pid: usize, tid: usize) -> TaskDescriptor {
        TaskDescriptor {
            pid,
            tid,
        }
    }
}


