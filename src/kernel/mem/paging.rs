//! Code for working with 64-bit paging

use crate::kernel::arch::x86_64;


/// Represents the size of a page, modern processors support up to 1GiB pages
pub enum PageSize {
    Page4KiB,
    Page2MiB,
    Page1GiB,
}

/// Represents the entire recursive page table
pub struct PageTable {
    pml4: *mut PageTableEntry,
}

impl PageTable {
    /// Map a virtual address to a physical address, both addresses must be aligned to the page size
    pub fn map(vaddr: u64, paddr: u64, size: PageSize) {
    }
}

/// Page table entry flags, only flags which are common between PML4E, PDPTE, PDE and PTE are represented
pub struct PageTableEntryFlags;

impl PageTableEntryFlags {
    /// Must be 1 on all pages which you wish to use
    pub const PRESENT: u64 = 1;

    /// Indicates the ability to write to memory inside this page
    pub const WRITE: u64 = 1 << 1;

    /// Indicates the ability for usermode (ring 3) access to this page
    pub const USER: u64 = 1 << 2;

    /// If 1 then this will map the entry as a page, if 0 then the entry will reference a page table
    pub const PAGE_SIZE: u64 = 1 << 7;
}

/// A generic page table entry, this can either be the PML4E, PDPTE, PDE or the PTE
#[repr(C)]
pub struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    pub fn new(paddr: u64, flags: u64) -> PageTableEntry {
        // NOTE: if its mapped as a page then the offset of the physical address depends on the
        // page size, however this is not something we need to worry about since physical addresses
        // will automatically fit this offset because of alignment guarantees that these low bits are zero

        PageTableEntry {
            entry: (flags & 0xff) | ((paddr & ((1u64 << x86_64::physical_address_width()) - 1)) << 12),
        }
    }
}


