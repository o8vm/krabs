pub mod elf;

use crate::loader::GuestAddress::Addr32;
use crate::loader::GuestAddress::Addr64;
use plankton::{layout::ELF_START, mem::MemoryRegion};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    BigEndianElfOnLittle,
    InvalidElfMagicNumber,
    InvalidEntryAddress,
    InvalidArchitecture,
    InvalidProgramHeaderSize,
    InvalidProgramHeaderOffset,
    InvalidProgramHeaderAddress,
}

impl ParseError {
    pub fn stringify(&self) -> &'static str {
        match self {
            ParseError::BigEndianElfOnLittle => "Unsupported ELF File byte order",
            ParseError::InvalidElfMagicNumber => "Invalid ELF magic number",
            ParseError::InvalidEntryAddress => "Invalid entry address found in ELF header",
            ParseError::InvalidArchitecture => "Unsupported target acrhitecture",
            ParseError::InvalidProgramHeaderSize => "Invalid ELF program header size",
            ParseError::InvalidProgramHeaderOffset => "Invalid ELF program header offset",
            ParseError::InvalidProgramHeaderAddress => "Invalid ELF program header address",
        }
    }
}

pub type Result<T> = core::result::Result<T, ParseError>;

pub enum GuestAddress {
    Addr32(u32),
    Addr64(u64),
}

pub fn load_elf(kernel_size: u32) -> Result<GuestAddress> {
    let elf_image = MemoryRegion::new(ELF_START as u64, (kernel_size * 512) as u64);
    let e_ident: &[u8] = elf_image.as_slice(0, 16);

    // Sanity checks
    if e_ident[elf::EI_MAG0] != elf::ELFMAG0
        || e_ident[elf::EI_MAG1] != elf::ELFMAG1
        || e_ident[elf::EI_MAG2] != elf::ELFMAG2
        || e_ident[elf::EI_MAG3] != elf::ELFMAG3
    {
        return Err(ParseError::InvalidElfMagicNumber);
    }
    if e_ident[elf::EI_DATA] != elf::ELFDATA2LSB {
        return Err(ParseError::BigEndianElfOnLittle);
    }

    match e_ident[elf::EI_CLSS] {
        elf::ELF32CL => {
            println!("ELF32 ...");
            let ehdr = elf_image.read::<elf::Elf32Header>(0);
            let phdrs = elf_image
                .as_slice::<elf::Elf32ProgramHeader>(ehdr.e_phoff as u64, ehdr.e_phnum as u64);

            if ehdr.e_phsize as usize != core::mem::size_of::<elf::Elf32ProgramHeader>() {
                return Err(ParseError::InvalidProgramHeaderSize);
            }
            if (ehdr.e_phoff as usize) < core::mem::size_of::<elf::Elf32Header>() {
                return Err(ParseError::InvalidProgramHeaderOffset);
            }
            if ehdr.e_entry < 0x100000 {
                return Err(ParseError::InvalidEntryAddress);
            }

            for &phdr in phdrs {
                if phdr.p_type != elf::PT_LOAD || phdr.p_fsize == 0 {
                    continue;
                }
                if phdr.p_paddr < 0x100000 {
                    return Err(ParseError::InvalidProgramHeaderAddress);
                }
                let mut dst_region =
                    plankton::mem::MemoryRegion::new(phdr.p_paddr as u64, phdr.p_fsize as u64);
                let dst = dst_region.as_mut_slice::<u8>(0, phdr.p_fsize as u64);
                let src = elf_image.as_slice::<u8>(phdr.p_offset as u64, phdr.p_fsize as u64);
                dst.copy_from_slice(src);
            }
            Ok(Addr32(ehdr.e_entry))
        }
        elf::ELF64CL => {
            println!("ELF64 ...");
            let ehdr = elf_image.read::<elf::Elf64Header>(0);
            let phdrs = elf_image
                .as_slice::<elf::Elf64ProgramHeader>(ehdr.e_phoff as u64, ehdr.e_phnum as u64);

            if ehdr.e_phsize as usize != core::mem::size_of::<elf::Elf64ProgramHeader>() {
                return Err(ParseError::InvalidProgramHeaderSize);
            }
            if (ehdr.e_phoff as usize) < core::mem::size_of::<elf::Elf64Header>() {
                return Err(ParseError::InvalidProgramHeaderOffset);
            }
            if ehdr.e_entry < 0x100000 {
                return Err(ParseError::InvalidEntryAddress);
            }

            for &phdr in phdrs.iter() {
                if phdr.p_type != elf::PT_LOAD || phdr.p_fsize == 0 {
                    continue;
                }
                if phdr.p_paddr < 0x100000 {
                    return Err(ParseError::InvalidProgramHeaderAddress);
                }
                let mut dst_region =
                    plankton::mem::MemoryRegion::new(phdr.p_paddr as u64, phdr.p_fsize as u64);
                let dst = dst_region.as_mut_slice::<u8>(0, phdr.p_fsize as u64);
                let src = elf_image.as_slice::<u8>(phdr.p_offset as u64, phdr.p_fsize as u64);
                dst.copy_from_slice(src);
            }
            Ok(Addr64(ehdr.e_entry))
        }
        _ => Err(ParseError::InvalidArchitecture),
    }
}
