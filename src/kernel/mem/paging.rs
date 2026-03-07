/*
use crate::kernel::mem::pma::PhysicalMemoryAllocator;
use crate::kernel::mem::error::MemoryError;
use crate::helpers::*;

use uefi::mem::memory_map::{MemoryMap, MemoryMapOwned};
use uefi::boot::MemoryType;

use x86_64::structures::paging::mapper::{OffsetPageTable, MapToError};
use x86_64::structures::paging::{PhysFrame, PageTable, PageTableFlags, Mapper, Size4KiB};
use x86_64::registers::control::Cr3;
use x86_64::{VirtAddr, PhysAddr};
*/


// NOTE: after a little research, turns out UEFI maps all conventional memory for us, so there is
// no reason to identity map the kernel memory because its already mapped for us
/*
/// Initialize kernel page table by modifying the page table created by UEFI
pub fn init(mmap: &MemoryMapOwned, pma: &mut PhysicalMemoryAllocator) -> Result<OffsetPageTable<'static>, MemoryError> {
    let (frame, _) = Cr3::read();

    let ptr = frame.start_address().as_u64() as *mut PageTable;

    log!("initializing kernel page table at {:x?}", ptr);

    let mut table = unsafe { OffsetPageTable::new(ptr.as_mut_unchecked(), VirtAddr::zero()) };

    for descriptor in mmap.entries() {
        if descriptor.ty == MemoryType::CONVENTIONAL && descriptor.phys_start != 0 {
            log!("descriptor: {:x?}", descriptor);

            for page in 0..descriptor.page_count {
                log!("addr: {:x?}", descriptor.phys_start + (page * 4096));

                let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(descriptor.phys_start + (page * 4096)));

                log!("mapping frame: {:x?}", frame);

                // TODO: we should setup interrupts first, this is most likely a fault but we dont
                // see it since we dont handle interrupts
                match unsafe { table.identity_map(frame, PageTableFlags::WRITABLE | PageTableFlags::PRESENT, pma) } {
                    Ok(_) | Err(MapToError::PageAlreadyMapped(_)) => {},
                    Err(err) => return Err(MemoryError::MapToError(err)),
                }

                log!("done mapping: {:x?}", frame);
            }
        }
    }

    log!("done");

    Ok(table)
}
*/


