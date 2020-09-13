#![feature(llvm_asm)]
#![no_std]
#![no_main]
extern crate alloc;
extern crate rlibc;

#[macro_use]
use alloc::vec;
use stage_4th::fs::{fat32::FileSystem, gpt::GPT};
use stage_4th::{
    clear_bss,
    loader::{config::Config, load_elf, load_items, GuestAddress},
    print, println, svm,
};

#[link_section = ".first"]
#[no_mangle]
fn stage4() -> ! {
    clear_bss();
    println!("STG4: ");
    let file_name = "CONFIG.TXT";

    // Mount FileSystem.
    print!("  Mounting FAT32 EFI System Partition...");
    let partition = match GPT::new().get_efi_system_partition() {
        Some(partition) => partition,
        None => panic!("None"),
    };
    let fs = FileSystem::new(partition);
    let root = fs.root_dir();
    println!(" done!");

    // Read CONFIG File
    print!("  Reading {}...", file_name);
    let config_txt = root.open_file(file_name).unwrap();
    //loop {}
    //println!("{}", config_txt.len());
    let mut buf = vec![0u8; config_txt.len()];
    config_txt.read(&mut buf).unwrap();
    let config = Config::new(core::str::from_utf8(&buf).unwrap()).unwrap();
    println!(" done!");
    //loop {}
    // Load items
    let kernel_size = load_items(root, config).unwrap();

    // Relocating ELF formatted Kernel
    print!("  Relocating kernel ");
    let entry_addr = load_elf(kernel_size).unwrap();

    // Excute
    match entry_addr {
        GuestAddress::Addr32(entry_addr) => svm::pm::start_kernel(entry_addr),
        GuestAddress::Addr64(entry_addr) => svm::lm::start_kernel(entry_addr),
    }
}
