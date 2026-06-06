//! Parseable formats

pub mod cpio;
pub mod elf;

/// Decode bytes into a concrete type
fn decode<T>(bytes: &[u8]) -> T {
    unsafe {
        assert_eq!(bytes.len(), core::mem::size_of::<T>());

        core::ptr::read(bytes.as_ptr() as *const T)
    }
}
