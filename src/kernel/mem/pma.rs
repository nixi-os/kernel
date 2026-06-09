///! The physical memory allocator handles allocation of physical frames, it does not concern
///! itself with virtual memory.
use crate::helpers::*;

use core::ops::Range;

use spin::Mutex;
use uefi::mem::memory_map::{MemoryMap, MemoryMapOwned, MemoryType};

static PMA: Mutex<PhysicalMemoryAllocator> = Mutex::new(PhysicalMemoryAllocator::new());

/// Initialize the physical memory allocator with a memory map
#[inline(always)]
pub fn init(mmap: &MemoryMapOwned) {
    PMA.lock().init(mmap)
}

/// Allocate a continuous set of physical frames within a 128 frame chunk
#[inline(always)]
pub fn alloc(frames: usize) -> *const () {
    PMA.lock().alloc(frames)
}

/// Allocate a continuous set of physical frames within a 128 frame chunk and overwrite it with all zeroes
#[inline(always)]
pub fn alloc_zeroed(frames: usize) -> *const () {
    let ptr = PMA.lock().alloc(frames);

    for index in 0..frames {
        unsafe {
            let frame = (ptr as *mut [u64; 512]).add(index);

            *frame = [0u64; 512];
        }
    }

    ptr
}

/// Free a continuous set of physical frames.
///
/// free is unsafe because the caller must ensure that the address and frame count is valid.
#[inline(always)]
pub unsafe fn free(address: *const (), frames: usize) {
    unsafe { PMA.lock().free(address, frames) }
}

pub struct PhysicalMemoryAllocator {
    bitmap: [u128; 2048],
}

impl PhysicalMemoryAllocator {
    /// Create a new physical memory allocator
    pub const fn new() -> PhysicalMemoryAllocator {
        PhysicalMemoryAllocator {
            bitmap: [u128::MAX; 2048],
        }
    }

    /// Initialize the physical memory allocator
    pub fn init(&mut self, mmap: &MemoryMapOwned) {
        log!("mmap entries: {}", mmap.len());

        for descriptor in mmap.entries() {
            if descriptor.ty == MemoryType::CONVENTIONAL && descriptor.phys_start > 0 {
                let base = descriptor.phys_start as usize / 4096;

                for frame in 0..descriptor.page_count {
                    let bit = base + frame as usize;

                    self.bitmap[bit / u128::BITS as usize] &=
                        !(1u128 << (bit % u128::BITS as usize));
                }
            }
        }
    }

    /// Find the lowest continuous set of physical frames and return its range
    fn find_free(&self, frames: usize) -> Range<usize> {
        for (index, window) in self.bitmap.windows((frames / 128) + 1).enumerate() {
            let first = window.first().expect("window size must be above 0");

            let offset = frames % 128;
            let mask = (offset > 0)
                .then(|| ((1u128 << (128 - offset)) - 1) << offset)
                .unwrap_or(u128::MAX);

            if first & mask == 0 && window.iter().skip(1).all(|chunk| *chunk == 0) {
                return ((index * u128::BITS as usize) + offset as usize)
                    ..((index + window.len()) * u128::BITS as usize);
            }
        }

        panic!("out of memory")
    }

    /// Allocate a continuous set of physical frames within a 128 frame chunk
    pub fn alloc(&mut self, frames: usize) -> *const () {
        assert_ne!(frames, 0);

        let free = self.find_free(frames);

        for bit in free.clone() {
            self.bitmap[bit / u128::BITS as usize] |= 1u128 << (bit % u128::BITS as usize);
        }

        (free.start * 4096) as *const ()
    }

    /// Free a continuous set of physical frames.
    ///
    /// free is unsafe because the caller must ensure that the address and frame count is valid.
    pub unsafe fn free(&mut self, address: *const (), frames: usize) {
        assert_ne!(frames, 0);

        let base = address as usize / 4096;

        for bit in base..base + frames {
            self.bitmap[bit / u128::BITS as usize] &= !(1u128 << (bit % u128::BITS as usize));
        }
    }
}
