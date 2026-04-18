pub mod error;
pub mod context;

use error::SchedulerError;
use context::{Context, Segments, GeneralPurpose};

use crate::kernel::arch::x86_64::interrupt::{StackFrame, RFlags};
use crate::kernel::arch::x86_64::{self, tables};
use crate::kernel::mem::paging::{PageTable, PageSize, PageTableEntryFlags};

use spin::{Lazy, Mutex};

use alloc::alloc::Layout;
use alloc::collections::BTreeMap;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// The global scheduler
static SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| Mutex::new(Scheduler::new()));

/// Call a closure with scheduler
pub fn with_scheduler<F: FnOnce(&mut Scheduler) -> R, R>(f: F) -> R {
    let mut scheduler = SCHEDULER.lock();

    f(&mut scheduler)
}

/// The scheduler manages processes and tasks
pub struct Scheduler {
    pub procs: BTreeMap<usize, Proc>,
    pub current: Option<TaskDescriptor>,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Scheduler {
        Scheduler {
            procs: BTreeMap::new(),
            current: None,
        }
    }

    /// Create a new process and return process id
    pub fn create_proc(&mut self) -> Result<usize, SchedulerError> {
        let pid = self.alloc_pid()?;

        self.procs.insert(pid, Proc::new());

        Ok(pid)
    }

    /// Create a new task for a process with an entry point and return task id
    pub fn create_task(&mut self, pid: usize, entry: u64, privilege_level: u8) -> Result<usize, SchedulerError> {
        let proc = self.procs.get_mut(&pid).ok_or(SchedulerError::NoSuchPid)?;

        Ok(proc.create_task(entry, privilege_level))
    }

    /// Lookup a task and process from its task descriptor
    pub fn lookup<'a>(&'a self, task: TaskDescriptor) -> (&'a Task, &'a Proc) {
        (&self.procs[&task.pid].tasks[task.tid], &self.procs[&task.pid])
    }

    /// Allocate a new process id
    fn alloc_pid(&self) -> Result<usize, SchedulerError> {
        let pid = self.procs.keys()
            .map_windows(|[pid1, pid2]| (**pid1, **pid2))
            .find_map(|(pid1, pid2)| (pid2 - pid1 > 1).then_some(pid1 + 1));

        match pid {
            Some(pid) => Ok(pid),
            None => {
                if let Some(max) = self.procs.keys().max() {
                    max.checked_add(1).ok_or(SchedulerError::OutOfPid)
                } else {
                    Ok(0)
                }
            },
        }
    }

    /// Get the current and next task
    fn schedule_task(&self) -> (TaskDescriptor, TaskDescriptor) {
        assert!(!self.procs.is_empty());

        let current = self.current.expect("scheduling can never occur before the current task is initialized");

        if current.tid >= self.procs[&current.pid].tasks.len() - 1 {
            let next = self.procs.range(current.pid+1..).next();
            let (pid, _) = next.unwrap_or_else(|| self.procs.first_key_value().expect("procs shouldn't be empty"));

            (current, TaskDescriptor::new(*pid, 0))
        } else {
            (current, TaskDescriptor::new(current.pid, current.tid + 1))
        }
    }

    /// Set the kernel stack to a task from its descriptor
    unsafe fn set_kernel_stack(&self, descriptor: TaskDescriptor) {
        let kernel_stack = &self.procs[&descriptor.pid].tasks[descriptor.tid].kernel_stack;

        tables::set_kernel_stack(unsafe { kernel_stack.as_ptr().add(kernel_stack.len()) });
    }

    /// Perform a context switch from the current task to the next task
    unsafe fn switch(&mut self, ctx: Context) -> Context {
        let (current, next) = self.schedule_task();

        self.procs.get_mut(&current.pid).expect("current task descriptor should never be invalid").tasks[current.tid].ctx = ctx;

        unsafe {
            core::arch::x86_64::_xsave(self.procs[&current.pid].tasks[current.tid].xsave, u64::MAX);

            core::arch::x86_64::_xrstor(self.procs[&next.pid].tasks[next.tid].xsave, u64::MAX);

            self.set_kernel_stack(next);
        }

        self.procs[&next.pid].page_table.load();

        self.current = Some(next);

        self.procs[&next.pid].tasks[next.tid].ctx
    }
}

/// A process, a single process can have multiple tasks
pub struct Proc {
    tasks: Vec<Task>,
    page_table: PageTable,
}

impl Proc {
    /// Create a new process
    pub fn new() -> Proc {
        let mut page_table = PageTable::new();

        page_table.identity_map(0, 1, PageTableEntryFlags::USER | PageTableEntryFlags::WRITE, PageSize::Page1GiB);

        Proc {
            tasks: Vec::new(),
            page_table,
        }
    }

    /// Create a new task and return the task id
    pub fn create_task(&mut self, entry: u64, privilege_level: u8) -> usize {
        self.tasks.push(Task::new(entry, privilege_level));

        self.tasks.len() - 1
    }
}

/// A task which will be run by the scheduler on interval
pub struct Task {
    ctx: Context,
    user_stack: Box<[u8; 4096 * 4]>,
    kernel_stack: Box<[u8; 4096 * 4]>,
    xsave: *mut u8,
}

impl Task {
    /// Create a new task with an entry point and privilege level
    pub fn new(entry: u64, privilege_level: u8) -> Task {
        let layout = Layout::from_size_align(x86_64::required_xsave_size() as usize, 64).expect("xsave allocation shouldn't break alignment rules");
        let xsave = unsafe { alloc::alloc::alloc_zeroed(layout) };

        let user_stack = Box::new([0; 4096 * 4]);

        Task {
            ctx: Context {
                segments: Segments::default(),
                general: GeneralPurpose::default(),
                stack_frame: StackFrame {
                    rip: entry,
                    cs: 0x18 | (privilege_level as u64 & 3),
                    rflags: RFlags::new(1 | (1 << 9)),
                    rsp: user_stack.as_ptr() as u64 + user_stack.len() as u64,
                    ss: 0x20 | (privilege_level as u64 & 3),
                },
            },
            user_stack,
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


