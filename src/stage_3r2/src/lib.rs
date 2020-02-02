#![feature(asm)]
#![no_std]
#[macro_use]
pub mod text;
pub mod loader;
pub mod svm;

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

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
