use crate::kernel::mem::pma;

use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::mem;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

/// Every block has a block header, the block header will always be between the padding and the block, free blocks must have zero padding
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct BlockHeader {
    next: Option<*mut BlockHeader>,
    size: usize,
    padding: usize,
}

impl BlockHeader {
    pub const fn new(next: Option<*mut BlockHeader>, size: usize, padding: usize) -> BlockHeader {
        BlockHeader {
            next,
            size,
            padding,
        }
    }

    /// Find the first block with atleast size bytes and remove it from free list
    pub fn find(&mut self, size: usize) -> Option<*mut BlockHeader> {
        self.next.and_then(|next| unsafe {
            if (*next).size >= size {
                self.next = (*next).next;

                Some(next)
            } else {
                (*next).find(size)
            }
        })
    }
}

/// The global allocator handles memory allocation in the kernel
pub struct Allocator {
    free: RefCell<BlockHeader>,
}

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator {
            free: RefCell::new(BlockHeader::new(None, 0, 0)),
        }
    }


    /// Find a block with size bytes and remove it from free list
    /// or allocate a new block with size bytes
    fn prepare_block(&self, size: usize) -> *mut BlockHeader {
        // TODO: we should split the block if its too big
        match self.free.borrow_mut().find(size) {
            Some(header) => header,
            None => {
                let header = pma::alloc(size.div_ceil(4096)) as *mut BlockHeader;

                unsafe {
                    *header = BlockHeader::new(None, size, 0);
                }

                header
            },
        }
    }

    /// Push a block to the free list
    fn free(&self, block: *mut BlockHeader) {
        // TODO: we should do defragmentation, and free complete frames back to the pma
        let mut first = self.free.borrow_mut();

        unsafe {
            (*block).next = first.next;

            first.next = Some(block);
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    /// Allocate memory based on layout. The allocation will have atleast size bytes
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let old_header = self.prepare_block((layout.align() - 1) + mem::size_of::<BlockHeader>() + layout.size());

        unsafe {
            let padding = (old_header.byte_add(mem::size_of::<BlockHeader>()) as *mut u8).align_offset(layout.align());

            let new_header = old_header.byte_add(padding);

            *new_header = BlockHeader::new((*old_header).next, (*old_header).size - padding, padding);

            new_header.byte_add(mem::size_of::<BlockHeader>()) as *mut u8
        }
    }

    /// Deallocate memory. dealloc does not use the layout, rather it uses the block header
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            let old_header = ptr.byte_sub(mem::size_of::<BlockHeader>()) as *mut BlockHeader;

            let new_header = old_header.byte_sub((*old_header).padding);

            *new_header = BlockHeader::new(None, (*old_header).size + (*old_header).padding, 0);

            self.free(new_header);
        }
    }
}

unsafe impl Sync for Allocator {}


