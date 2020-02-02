#![no_std]
#![no_main]

use plankton::{ELF_START, HEAP_END, HEAP_START, IMAGE_START, INIT_SEG, KERNEL_SIZE};
use stage_3r2::svm;
use stage_3r2::{clear_bss, loader::load_elf, loader::GuestAddress, print, println};

#[link_section = ".first"]
#[no_mangle]
fn stage4() -> ! {
    clear_bss();
    let buf_size = IMAGE_START - ELF_START;
    let ret: i32;
    let kernel_size = unsafe { *((KERNEL_SIZE + (INIT_SEG << 4)) as *const u32) };
    println!("  Decompressing kernel ...");

    ret = unsafe {
        BZ2_bzBuffToBuffDecompress(
            ELF_START as *const u8,
            &buf_size as *const u32,
            IMAGE_START as *const u8,
            kernel_size,
            0,
            0,
        )
    };
    if ret != 0 {
        println!("  ... failed!");
        loop {}
    }

    print!("  Relocating ");
    let entry_addr = load_elf(kernel_size)
        .map_err(|err| err.stringify())
        .unwrap();
    match entry_addr {
        GuestAddress::Addr32(entry_addr) => svm::pm::start_kernel(entry_addr),
        GuestAddress::Addr64(entry_addr) => svm::lm::start_kernel(entry_addr),
    }
}

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

use core::ffi::c_void;
use core::ptr;

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
