use core::ffi::c_void;
use core::ptr;
use plankton::layout::{HEAP_END, HEAP_START};

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

pub fn decompress_kernel(source_len: u32) -> Result<(), &'static str> {
    use plankton::layout::{ELF_START, IMAGE_START, INITRD_START};
    let buf_size = INITRD_START - ELF_START;
    let ret = unsafe {
        BZ2_bzBuffToBuffDecompress(
            ELF_START as *const u8,
            &buf_size as *const u32,
            IMAGE_START as *const u8,
            source_len,
            0,
            0,
        )
    };
    match ret {
        0 => Ok(()),
        -1 => Err("Invaild data structures (buffers etc)"),
        -2 => {
            Err("A parameter to a function call is out of range or otherwise manifestly incorrect")
        }
        -3 => Err("A request to allocate memory failed"),
        -4 => Err("A data integrity error is detected"),
        -5 => Err("The compressed stream does not start with the correct magic bytes"),
        -7 => Err("The compressed file finishes before the logical end of stream is detected"),
        -8 => Err("The output data will not fit into the output buffer provided"),
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
