#![feature(llvm_asm)]
#![no_std]
#![no_main]
extern crate rlibc;
use plankton::{
    layout::{PROT_STACK, STAGE4_START},
    print, println,
};
use stage_3rd::{init, mpm};
pub mod rfn;
#[link_section = ".first"]
#[no_mangle]
fn stage3() -> ! {
    println!("STG3:");
    println!("  Initializing system.");
    init::setup();
    println!("  Moving to protected mode.");

    init::cur::set_cur();
    mpm::move_to_protect(STAGE4_START, PROT_STACK)
}