#![no_std]
use core::panic::PanicInfo;
use plankton::*;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[repr(C, packed)]
pub struct ParamTable {
    pub cmd_line: [u8; 120],
    pub stage3_size: u16,
    pub stage4_size: u16,
    pub kernel_size: u16,
    pub initrd_size: u16,
}
