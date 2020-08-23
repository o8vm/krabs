#![feature(llvm_asm)]
#![no_std]

pub mod init;
pub mod mpm;
//pub mod rfn;

use core::panic::PanicInfo;
use plankton::{print, println};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[repr(C, packed)]
pub struct ParamTable {
    pub start_lba: u32,
    pub sectors: u32,
    pub ret_addr: u32,
    pub prot_stack: u32,
}