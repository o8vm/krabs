#![no_std]
#![no_main]

use plankton::{INIT_SEG, KERNEL_SIZE};
use stage_4th::{
    bz2d::decompress_kernel, clear_bss, cmdl, loader::load_elf, loader::GuestAddress, print,
    println, svm,
};

#[link_section = ".first"]
#[no_mangle]
fn stage4() -> ! {
    clear_bss();
    cmdl::setup_cmdline();
    let kernel_size = unsafe { *((KERNEL_SIZE + (INIT_SEG << 4)) as *const u32) };

    println!("  Decompressing kernel ...");
    decompress_kernel(kernel_size).unwrap();

    print!("  Relocating ");
    let entry_addr = load_elf(kernel_size)
        .map_err(|err| err.stringify())
        .unwrap();
    match entry_addr {
        GuestAddress::Addr32(entry_addr) => svm::pm::start_kernel(entry_addr),
        GuestAddress::Addr64(entry_addr) => svm::lm::start_kernel(entry_addr),
    }
}
