//! The Executable and Linking Format is a format for binary executables, object code and shared libraries

/// The ELF header is present at the start of all ELF binaries
#[repr(packed, C)]
struct ElfHeader {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

/// The program header describes a segment or other information the system needs to prepare the program for execution
#[derive(Debug)]
#[repr(packed, C)]
pub struct ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

/// An iterator over program headers
pub struct ProgramHeaderIterator<'a> {
    offset: u64,
    end: u64,
    bytes: &'a [u8],
}

impl<'a> Iterator for ProgramHeaderIterator<'a> {
    type Item = &'a ProgramHeader;

    fn next(&mut self) -> Option<&'a ProgramHeader> {
        if self.offset < self.end {
            let bytes = self.bytes.get(
                self.offset as usize..self.offset as usize + core::mem::size_of::<ProgramHeader>(),
            )?;

            self.offset += core::mem::size_of::<ProgramHeader>() as u64;

            Some(super::decode::<ProgramHeader>(bytes))
        } else {
            None
        }
    }
}

/// An ELF object file
pub struct ElfObject<'a> {
    elf_header: &'a ElfHeader,
    bytes: &'a [u8],
}

impl<'a> ElfObject<'a> {
    /// Parse an ELF object from a byte slice
    pub fn parse(bytes: &'a [u8]) -> Option<ElfObject<'a>> {
        let elf_header =
            super::decode::<ElfHeader>(bytes.get(..core::mem::size_of::<ElfHeader>())?);

        match elf_header.e_ident[0..4] {
            [0x7f, 0x45, 0x4c, 0x46] => Some(ElfObject { elf_header, bytes }),
            _ => None,
        }
    }

    /// Return an iterator of program headers
    pub fn program_headers(&'a self) -> ProgramHeaderIterator<'a> {
        ProgramHeaderIterator {
            offset: self.elf_header.e_phoff,
            end: self.elf_header.e_phoff
                + (self.elf_header.e_phnum as u64 * core::mem::size_of::<ProgramHeader>() as u64),
            bytes: self.bytes,
        }
    }
}
