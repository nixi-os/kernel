use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::Size4KiB;


#[derive(Debug)]
pub enum MemoryError {
    MapToError(MapToError<Size4KiB>),
}

impl core::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            MemoryError::MapToError(err) => f.write_fmt(format_args!("failed to map page: {:?}", err)),
        }
    }
}

impl core::error::Error for MemoryError {}


