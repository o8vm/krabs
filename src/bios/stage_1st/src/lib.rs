#![feature(llvm_asm)]
#![no_std]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub fn __main() -> () {
            // type check the given path
            let f: fn() -> () = $path;

            f()
        }
    };
}

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    extern "Rust" {
        fn main() -> ();
    }
    unsafe {
        llvm_asm!("ljmp $$0x07C0, $$reentry
             reentry:
              cli
              movw $$0x07C0, %ax
              movw %ax, %ds
              movw %ax, %es
              movw %ax, %ss
              movl $$0xFFF0, %esp
              sti"
         :::
        );
        main();
    }
    loop {}
}
