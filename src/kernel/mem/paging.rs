use crate::kernel::mem::pma::PhysicalMemoryAllocator;
use crate::kernel::mem::error::MemoryError;

use uefi::mem::memory_map::{MemoryMap, MemoryMapOwned};

use x86_64::structures::paging::mapper::{OffsetPageTable, MapToError};
use x86_64::structures::paging::{PhysFrame, PageTable, PageTableFlags, Mapper, Size4KiB};
use x86_64::registers::control::Cr3;
use x86_64::{VirtAddr, PhysAddr};


/// Initialize kernel page table by modifying the page table created by UEFI
pub fn init(mmap: &MemoryMapOwned, pma: &mut PhysicalMemoryAllocator) -> Result<OffsetPageTable<'static>, MemoryError> {
    let (frame, _) = Cr3::read();

    let ptr = frame.start_address().as_u64() as *mut PageTable;

    let mut table = unsafe { OffsetPageTable::new(ptr.as_mut_unchecked(), VirtAddr::zero()) };

    for descriptor in mmap.entries() {
        for page in 0..descriptor.page_count {
            let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(descriptor.phys_start + (page * 4096)));

            match unsafe { table.identity_map(frame, PageTableFlags::WRITABLE | PageTableFlags::PRESENT, pma) } {
                Ok(_) => {},
                Err(MapToError::PageAlreadyMapped(_)) => {
                    uefi::println!("warn: ignoring already mapped by uefi: {:?}", frame.start_address());
                },
                Err(err) => return Err(MemoryError::MapToError(err)),
            }
        }
    }

    Ok(table)
}


