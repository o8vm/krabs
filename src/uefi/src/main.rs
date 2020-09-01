#![no_std]
#![no_main]
extern crate rlibc;

use core::ffi::c_void;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(C)]
pub enum EfiStatus {
    SUCCESS,
}
// UEFI System table
#[repr(C)]
pub struct EfiSystemTable {
    _buf: [u8; 60],
    pub con_out: *mut EfiSimpleTextOutputProtocol,
}

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol, extended: bool) -> EfiStatus,
    pub output_string: unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    _buf2: [usize; 4],
    pub clear_screen: unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol) -> EfiStatus,
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct EfiHandle(*mut c_void);

#[no_mangle]
pub extern "C" fn efi_main(_handle: EfiHandle, st: EfiSystemTable) -> EfiStatus {
    let stdout: &mut EfiSimpleTextOutputProtocol = unsafe { &mut *(st.con_out) };
    let string = "hello UEFI".as_bytes();
    let mut buf = [0u16; 32];

    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }

    unsafe {
        (stdout.reset)(stdout, false);
        (stdout.clear_screen)(stdout);
        (stdout.output_string)(stdout, buf.as_ptr());
    }
    loop {}
    EfiStatus::SUCCESS
}