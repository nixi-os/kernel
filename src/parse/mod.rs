//! Parseable formats

pub mod cpio;
pub mod elf;

/// Reinterpret bytes into a concrete type
fn decode<'a, T>(bytes: &'a [u8]) -> &'a T {
    assert_eq!(bytes.len(), core::mem::size_of::<T>());

    unsafe { &*(bytes.as_ptr() as *const T) }
}
