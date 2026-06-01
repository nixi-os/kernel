//! CPIO is a format for file archives. There are three variants of the CPIO format: Binary CPIO, portable ASCII CPIO, and new ASCII CPIO

use core::iter::Iterator;
use core::slice::Iter;

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

    /// Return the next chunk of N bytes and pad with (pad) bytes
    fn next_chunk<const N: usize>(&mut self, pad: usize) -> Option<[u8; N]> {
        let chunk = self.bytes.next_chunk::<N>().ok()?.map(|byte| *byte);

        if pad > 0 {
            self.bytes.nth(pad - 1);
        }

        Some(chunk)
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
        assert_eq!(u16::from_le_bytes(self.next_chunk::<2>(2)?), 0x71c7);

        let path_size = u16::from_le_bytes(self.next_chunk::<2>(0)?);

        let data_size_high = u16::from_le_bytes(self.next_chunk::<2>(0)?) as u32;
        let data_size_low = u16::from_le_bytes(self.next_chunk::<2>(0)?) as u32;
        let data_size = (data_size_high << 16) | data_size_low;

        let path = str::from_utf8(self.next_bytes(path_size as usize - 1, 2)?).ok()?;

        if path != "TRAILER!!!" {
            Some(CpioEntry {
                path,
                data: self.next_bytes(data_size as usize, 2)?,
            })
        } else {
            None
        }
    }
}
