#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub fn __main() -> () {
            // type check the given path
            let f: fn() -> () = $path;

            f()
        }
    }
}

#[link_section=".startup"]
#[no_mangle]
fn _start() -> ! {
    extern "Rust" {
        fn main() -> ();
    }
    unsafe {
        main();
    }
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

entry!(main);
fn main() {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}