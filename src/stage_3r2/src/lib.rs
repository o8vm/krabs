#![no_std]

use plankton::{mem::MemoryRegion, ELF_START};

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ElfHeader {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_cpu: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phsize: u16,
    pub e_phnum: u16,
    pub e_shsize: u16,
    pub e_shnum: u16,
    pub e_shname: u16,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_fsize: u32,
    pub p_msize: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

pub fn load_elf(kernel_size: u32) -> u32 {
    let elf = MemoryRegion::new(ELF_START as u64, (kernel_size * 512) as u64);
    let ehdr: ElfHeader = elf.read::<ElfHeader>(0);
    let phdrs = elf.as_slice::<ProgramHeader>(ehdr.e_phoff as u64, ehdr.e_phnum as u64);
    for &phdr in phdrs.iter() {
        match phdr.p_type {
            1 => {
                let mut dst_region = MemoryRegion::new(phdr.p_paddr as u64, phdr.p_fsize as u64);
                let dst = dst_region.as_mut_slice::<u8>(0, phdr.p_fsize as u64);
                let src = elf.as_slice::<u8>(phdr.p_offset as u64, phdr.p_fsize as u64);
                dst.copy_from_slice(src);
            }
            _ => {}
        }
    }
    ehdr.e_entry
}

pub fn clear_bss() {
    use core::ptr;
    extern "C" {
        static mut _data_end: u8;
        static mut _bss_end: u8;
    }
    unsafe {
        let count = &_bss_end as *const u8 as usize - &_data_end as *const u8 as usize;
        ptr::write_bytes(&mut _data_end as *mut u8, 0, count);
    }
}

pub fn printc(data: u8) {
    let vga_buffer = 0xb8000 as *mut u8;
    static mut POS: isize = 0;
    unsafe {
        *vga_buffer.offset(POS as isize * 2) = data;
        *vga_buffer.offset(POS as isize * 2 + 1) = 0xb;
        POS += 1;
    }
}

pub fn dump_byte(data: u8) {
    let hex: [u8; 16] = *b"0123456789ABCDEF";
    printc(hex[((data & 0xF0) >> 4) as usize]);
    printc(hex[(data & 0x0F) as usize]);
}

pub fn dump_word(data: u16) {
    dump_byte(((data & 0xFF00) >> 8) as u8);
    dump_byte((data & 0xFF) as u8);
}
pub fn dump_quad(data: u32) {
    dump_word(((data & 0xFFFF0000) >> 16) as u16);
    dump_word((data & 0xFFFF) as u16);
}
