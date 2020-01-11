#![no_std]
pub mod elf;
use plankton::{mem::MemoryRegion, ELF_START};

pub fn load_elf(kernel_size: u32) -> u32 {
    let elf_image = MemoryRegion::new(ELF_START as u64, (kernel_size * 512) as u64);
    let e_ident: &[u8] = elf_image.as_slice(0, 16);

    let ehdr = elf_image.read::<elf::Elf32Header>(0);
    let phdrs =
        elf_image.as_slice::<elf::Elf32ProgramHeader>(ehdr.e_phoff as u64, ehdr.e_phnum as u64);
    for &phdr in phdrs.iter() {
        match phdr.p_type {
            1 => {
                let mut dst_region = MemoryRegion::new(phdr.p_paddr as u64, phdr.p_fsize as u64);
                let dst = dst_region.as_mut_slice::<u8>(0, phdr.p_fsize as u64);
                let src = elf_image.as_slice::<u8>(phdr.p_offset as u64, phdr.p_fsize as u64);
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
