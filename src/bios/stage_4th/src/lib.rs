#![feature(llvm_asm)]
#![feature(alloc_error_handler)]
#![no_std]
use core::{alloc::GlobalAlloc, cell::UnsafeCell, ptr};
extern crate alloc;
use alloc::alloc::Layout;
use plankton::layout::{HEAP_END, HEAP_START};

#[macro_use]
pub mod text;
pub mod fs;
pub mod loader;
pub mod svm;

pub fn clear_bss() {
    extern "C" {
        static mut _data_end: u8;
        static mut _bss_end: u8;
    }
    unsafe {
        let count = &_bss_end as *const u8 as usize - &_data_end as *const u8 as usize;
        ptr::write_bytes(&mut _data_end as *mut u8, 0, count);
    }
}

struct Alloc {
    head: UnsafeCell<usize>,
    end: usize,
}

unsafe impl Sync for Alloc {}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let head = self.head.get();
        let align = layout.align();
        let res = *head % align;
        let start = if res == 0 { *head } else { *head + align - res };
        if start + align > self.end {
            ptr::null_mut()
        } else {
            *head = start + align;
            start as *mut u8
        }
    }
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // nothing to do
    }
}

#[global_allocator]
static HEAP: Alloc = Alloc {
    head: UnsafeCell::new(HEAP_START as usize),
    end: HEAP_END as usize,
};

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    panic!("Out of Memory");
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
