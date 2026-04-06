//! An implementation of task state segments

// TODO: we will have to do a unique kernel stack for each task, this will simply mean that we
// switch out the rsp0 whenever we do a context switch such that if a context switch happens inside
// kernel mode then we know that the stack wont be corrupted


/// The task state segment contains RSP0-2, IST1-7 and the I/O Map Base Address
#[repr(C, packed)]
pub struct TaskStateSegment {
    _reserved1: u32,
    rsp: [u64; 3],
    _reserved2: u64,
    ist: [u64; 7],
    _reserved3: u64,
    _reserved4: u16,
    io_map: u16,
}

impl TaskStateSegment {
    /// Create a new task state segment with an uninitialized rsp0
    pub const fn uninit() -> TaskStateSegment {
        TaskStateSegment {
            _reserved1: 0,
            rsp: [0; 3],
            _reserved2: 0,
            ist: [0; 7],
            _reserved3: 0,
            _reserved4: 0,
            io_map: core::mem::size_of::<TaskStateSegment>() as u16,
        }
    }

    pub fn set_rsp0(self: *mut TaskStateSegment, rsp0: u64) {
        unsafe {
            (*self).rsp[0] = rsp0;
        }
    }
}


