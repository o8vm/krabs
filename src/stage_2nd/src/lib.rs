#![no_std]
use plankton::*;
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[repr(C, packed)]
pub struct ParamTable {
    pub cmd_line:    [u8; 122],
    pub stage3_size: u16,
    pub kernel_size: u16,
    pub initrd_size: u16,
}