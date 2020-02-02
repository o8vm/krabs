#![no_std]
#![no_main]

use plankton::{ELF_START, IMAGE_START, INIT_SEG, KERNEL_SIZE};
use stage_4th::bz2d::decompress_kernel;
use stage_4th::svm;
use stage_4th::{clear_bss, loader::load_elf, loader::GuestAddress, print, println};

#[link_section = ".first"]
#[no_mangle]
fn stage4() -> ! {
    clear_bss();
    let buf_size = IMAGE_START - ELF_START;
    let kernel_size = unsafe { *((KERNEL_SIZE + (INIT_SEG << 4)) as *const u32) };
    println!("  Decompressing kernel ...");
    decompress_kernel(
        ELF_START as *const u8,
        &buf_size as *const u32,
        IMAGE_START as *const u8,
        kernel_size,
        0,
        0,
    )
    .unwrap();

    print!("  Relocating ");
    let entry_addr = load_elf(kernel_size)
        .map_err(|err| err.stringify())
        .unwrap();
    match entry_addr {
        GuestAddress::Addr32(entry_addr) => svm::pm::start_kernel(entry_addr),
        GuestAddress::Addr64(entry_addr) => svm::lm::start_kernel(entry_addr),
    }
}
