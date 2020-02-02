use core::ffi::c_void;
use core::ptr;
use plankton::{HEAP_END, HEAP_START};

#[link(name = "bz2", kind = "static")]
extern "C" {
    fn BZ2_bzBuffToBuffDecompress(
        dest: *const u8,
        dest_len: *const u32,
        source: *const u8,
        source_len: u32,
        small: i32,
        verbosity: i32,
    ) -> i32;
}

pub fn decompress_kernel(
    dest: *const u8,
    dest_len: *const u32,
    source: *const u8,
    source_len: u32,
    small: i32,
    verbosity: i32,
) -> Result<(), &'static str> {
    let ret =
        unsafe { BZ2_bzBuffToBuffDecompress(dest, dest_len, source, source_len, small, verbosity) };
    match ret {
        0 => Ok(()),
        _ => Err("failed to decompress the image"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    static mut CUR_PTR: usize = HEAP_START as usize;

    let ptr: *mut usize;

    if size + CUR_PTR > HEAP_END as usize {
        return ptr::null_mut();
    }

    ptr = CUR_PTR as *mut usize;
    CUR_PTR += size;
    ptr as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn free(_ptr: *mut c_void) {}

#[no_mangle]
pub unsafe extern "C" fn bz_internal_error(_error: i32) -> ! {
    loop {}
}
