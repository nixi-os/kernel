//! The Executable and Linking Format is a format for binary executables, object code and shared libraries

use alloc::vec::Vec;

/// The ELF header is present at the start of all ELF binaries
#[repr(packed, C)]
struct ElfHeader {
    e_ident: [char; 16],
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

/// An ELF object file
pub struct ElfObject<'a> {
    elf_header: &'a ElfHeader,
    program_headers: &'a [ProgramHeader],
}

impl<'a> ElfObject<'a> {
    /// Parse an ELF object from a byte slice
    pub fn parse(bytes: &'a [u8]) -> Option<ElfObject<'a>> {
        let header = super::decode::<ElfHeader>(bytes.get(..core::mem::size_of::<ElfHeader>())?);

        let e_ident = header.e_ident;
        crate::log!("header.e_ident: {:?}", e_ident);

        None
    }
}
