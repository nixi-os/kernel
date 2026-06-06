//! CPIO is a format for file archives. There are three variants of the CPIO format: Binary CPIO, portable ASCII CPIO, and new ASCII CPIO

use core::iter::Iterator;
use core::slice::Iter;

/// Size of a raw CPIO entry in bytes
const RAW_ENTRY_SIZE: usize = core::mem::size_of::<RawEntry>();

/// Raw byte representation of a CPIO entry
#[repr(packed, C)]
#[derive(Debug)]
struct RawEntry {
    signature: u16,
    device: u16,
    inode: u16,
    mode: u16,
    user_id: u16,
    group_id: u16,
    links: u16,
    block_device: u16,
    modified: [u8; 4],
    path_size: u16,
    data_size: [u8; 4],
}

/// A CPIO entry. The CPIO entry is stripped of all unused values
#[derive(Debug)]
pub struct CpioEntry<'a> {
    pub path: &'a str,
    pub data: &'a [u8],
}

/// Parser of the Binary CPIO format
pub struct CpioParser<'a> {
    bytes: Iter<'a, u8>,
}

impl<'a> CpioParser<'a> {
    /// Create a new CPIO parser
    pub fn new(bytes: &'a [u8]) -> CpioParser<'a> {
        CpioParser {
            bytes: bytes.iter(),
        }
    }

    /// Return the next n bytes and pad with (n % pad) bytes
    fn next_bytes(&mut self, n: usize, pad: usize) -> Option<&'a [u8]> {
        let slice = self.bytes.as_slice();

        if slice.len() > n {
            let (bytes, rest) = slice.split_at(n);

            self.bytes = rest.iter();

            if n % pad > 0 {
                self.bytes.nth((n % pad) - 1);
            }

            Some(bytes)
        } else {
            None
        }
    }
}

impl<'a> Iterator for CpioParser<'a> {
    type Item = CpioEntry<'a>;

    fn next(&mut self) -> Option<CpioEntry<'a>> {
        let bytes = self
            .bytes
            .next_chunk::<RAW_ENTRY_SIZE>()
            .ok()?
            .map(|byte| *byte);

        let entry = super::decode::<RawEntry>(&bytes);

        let path_bytes = self.next_bytes(entry.path_size as usize, 2)?;
        let path = str::from_utf8(&path_bytes[..path_bytes.len() - 1]).ok()?;

        if path != "TRAILER!!!" {
            let data_size = u32::from_le_bytes([
                entry.data_size[2],
                entry.data_size[3],
                entry.data_size[0],
                entry.data_size[1],
            ]);

            Some(CpioEntry {
                path,
                data: self.next_bytes(data_size as usize, 2)?,
            })
        } else {
            None
        }
    }
}
