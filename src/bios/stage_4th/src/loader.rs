pub mod config;
pub mod elf;

use crate::fs::{
    blkdev::BlockDevice,
    fat32::{dir::Dir, file::FileError},
};
use core::fmt::Debug;
use plankton::{
    layout::{CMD_LINE_ADDR, ELF_START, INITRD_START},
    mem::MemoryRegion,
};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidElfEndian,
    InvalidElfMagicNumber,
    InvalidEntryAddress,
    InvalidArchitecture,
    InvalidProgramHeaderSize,
    InvalidProgramHeaderOffset,
    InvalidProgramHeaderAddress,
}

pub enum GuestAddress {
    Addr32(u32),
    Addr64(u64),
}

pub fn load_elf(kernel_size: usize) -> Result<GuestAddress, ParseError> {
    let elf_image = MemoryRegion::new(ELF_START as u64, kernel_size as u64);
    let e_ident: &[u8] = elf_image.as_slice(0, 16);

    // checks
    if e_ident[elf::EI_MAG0] != elf::ELFMAG0
        || e_ident[elf::EI_MAG1] != elf::ELFMAG1
        || e_ident[elf::EI_MAG2] != elf::ELFMAG2
        || e_ident[elf::EI_MAG3] != elf::ELFMAG3
    {
        return Err(ParseError::InvalidElfMagicNumber);
    }
    if e_ident[elf::EI_DATA] != elf::ELFDATA2LSB {
        return Err(ParseError::InvalidElfEndian);
    }

    match e_ident[elf::EI_CLSS] {
        elf::ELF32CL => {
            println!("ELF32...");
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
            Ok(GuestAddress::Addr32(ehdr.e_entry))
        }
        elf::ELF64CL => {
            println!("ELF64...");
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
            Ok(GuestAddress::Addr64(ehdr.e_entry))
        }
        _ => Err(ParseError::InvalidArchitecture),
    }
}

// do loading kernel, initrd and command line and setting. return kernel size
pub fn load_items<T>(root: Dir<T>, config: config::Config) -> Result<usize, FileError>
where
    T: BlockDevice + Clone + Copy,
    <T as BlockDevice>::Error: Debug,
{
    let mut initrd_size = 0;

    // loading kernel img
    print!("  Loading {}...", config.kernel);
    let kernel_img = root.open_file(config.kernel).unwrap();
    let mut img_region = MemoryRegion::new(ELF_START as u64, kernel_img.len() as u64);
    let mut img = img_region.as_mut_slice::<u8>(0, img_region.len());
    kernel_img.read(&mut img)?;
    println!(" done!");

    // loading initrd if config exists
    if let Some(initrd) = config.initrd {
        print!("  Loading {}...", initrd);
        let initrd_img = root.open_file(initrd).unwrap();
        let mut rd_region = MemoryRegion::new(INITRD_START as u64, initrd_img.len() as u64);
        let mut rd = rd_region.as_mut_slice::<u8>(0, rd_region.len());
        initrd_img.read(&mut rd)?;
        initrd_size = initrd_img.len() as u32;
        println!(" done!")
    }

    // setting command line if config exists
    if let Some(cmdlin) = config.cmdlin {
        print!("  Setting command line...");
        setup_cmdline(cmdlin);
        println!(" done!");
    }

    // zero_page setup
    setup_zero_page(kernel_img.len() as u32, initrd_size);

    Ok(kernel_img.len())
}

fn setup_cmdline(cmdlin: &str) {
    let bytes = cmdlin.as_bytes();
    let mut cmdline_region = MemoryRegion::new(CMD_LINE_ADDR as u64, bytes.len() as u64 + 1);
    let cmdline = cmdline_region.as_mut_slice::<u8>(0, bytes.len() as u64);
    cmdline.copy_from_slice(bytes);
    cmdline_region.write_u8(bytes.len() as u64, b'\0');
}

fn setup_zero_page(kernel_size: u32, initrd_size: u32) {
    let zero_page = MemoryRegion::new(0x7C00, 4096);
    zero_page.write_u32(0x218, INITRD_START);
    zero_page.write_u32(0x21C, initrd_size);
    zero_page.write_u16(0x1fc, 0x0100);
    zero_page.write_u32(0x100, kernel_size);
    zero_page.write_u32(0x228, CMD_LINE_ADDR);
}
