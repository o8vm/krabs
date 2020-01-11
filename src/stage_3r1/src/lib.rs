#![feature(asm)]
#![no_std]

pub mod init;
pub mod pm;

use core::panic::PanicInfo;
use plankton::{print, println};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
