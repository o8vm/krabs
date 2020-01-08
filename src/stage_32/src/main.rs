#![feature(asm)]
#![no_std]
#![no_main]

use plankton::{ELF_START, HEAP_END, HEAP_START, IMAGE_START, INIT_SEG, KERNEL_SIZE};
use stage_32::{clear_bss, load_elf};

#[link_section = ".fisrt"]
#[no_mangle]
fn start_32() -> ! {
    clear_bss();
    let buf_size = IMAGE_START - ELF_START;
    let ret: i32;
    let kernel_size = unsafe { *(KERNEL_SIZE as *const u32) };

    ret = unsafe {
        BZ2_bzBuffToBuffDecompress(
            (ELF_START - (INIT_SEG << 4)) as *const u8,
            &buf_size as *const u32,
            (IMAGE_START - (INIT_SEG << 4)) as *const u8,
            *(KERNEL_SIZE as *const u32),
            0,
            0,
        )
    };
    if ret != 0 {
        loop {}
    }
    let entry_addr = load_elf(kernel_size);
    unsafe {
        asm!("movl  %eax, (jmp_offset)
              cld
              movl  %ebx, %ds
              movl  %ebx, %es
              movl  %ebx, %fs
              movl  %ebx, %gs
              movl  %ebx, %ss
              .byte 0xEA
             jmp_offset: .long 0
              .word 0x10"
         :
         : "{eax}"(entry_addr), "{ebx}"(0x18), "{esi}"(0x7C00)
         : "ebx", "eax"
        );
    }
    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
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
