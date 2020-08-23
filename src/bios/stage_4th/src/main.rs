#![feature(llvm_asm)]
#![no_std]
#![no_main]
extern crate alloc;
extern crate rlibc;

#[macro_use]
use alloc::vec;
use stage_4th::fs::{fat32::Volume, gpt::GPT};
use stage_4th::{
    clear_bss, loader::config::Config, loader::load_elf, loader::GuestAddress, print, println, svm,
};

#[link_section = ".first"]
#[no_mangle]
fn stage4() -> ! {
    clear_bss();
    println!("Stage4: ");
    print!("  Searching EFI System Partition... ");
    let table = GPT::new();
    let partition = match table.get_efi_system_partition() {
        Some(partition) => {
            println!("found!");
            partition
        }
        None => panic!("None"),
    };
    let fs = Volume::new(partition);
    let root = fs.root_dir();

    println!("  Reading CONFIG.TXT");
    let file = root.load_file("CONFIG.TXT").unwrap();
    let mut buf = vec![0u8; file.length];
    file.read(&mut buf).unwrap();
    //print!("{}", core::str::from_utf8(&buf).unwrap());
    let config = Config::new(core::str::from_utf8(&buf).unwrap()).unwrap();
    print!("{:?}", config);
    //println!("  Decompressing kernel ...");
    //decompress_kernel(kernel_size).unwrap();
    loop {}
    /*
    let entry_addr = load_elf(0)
        .map_err(|err| err.stringify())
        .unwrap();
    match entry_addr {
        GuestAddress::Addr32(entry_addr) => svm::pm::start_kernel(entry_addr),
        GuestAddress::Addr64(entry_addr) => svm::lm::start_kernel(entry_addr),
    }*/
}
