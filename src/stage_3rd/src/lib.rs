#![feature(asm)]
#![no_std]

pub mod init;
use init::*;

use plankton::{print, println};
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

pub fn setup(kernel_size: u16, initrd_size: u16, cmd_line: &[u8]) {
    zero::clear_bss();
    zero::Pages::FirstHalf.clear();
    cmd::set_cmdline(cmd_line);
    zero::Pages::SecondHalf.clear();
    vid::set_screen_info();
    msz::set_mem_size();
    kbd::set_keyboard();
    img::set_image(kernel_size as u32 * 512, initrd_size as u32 * 512);
    a20::enable_a20();
}

// 
use plankton::ios::*;

#[repr(C, packed)]
struct DescoritorTable {
    limit: u16,
    base:  u32,
}

#[no_mangle]
static GDT: [u64; 6] = [
    0x0000000000000000,                 
    0x0000000000000000,                 
    0x00CF9A000000FFFF,
    0x00CF92000000FFFF,                 
    0x00CF9A007C00FFFF,                 
    0x00CF92007C00FFFF,
];

#[no_mangle]
static IDTR: DescoritorTable = DescoritorTable { limit: 0, base: 0 };
#[no_mangle]
static GDTR: DescoritorTable = DescoritorTable { limit: 0x800, base: 0 };

pub fn move_to_protect() {
    extern "C" {
        fn _start_32();
    }
    outb(0, 0xF0);
    outb(0, 0xF1);
    io_delay();

    unsafe { asm!("cli":::); }
    outb(0x80, 0x70);
    outb(0xFF, 0xA1);
    io_delay();
    outb(0xFB, 0x21);

    unsafe { asm!("lidt IDTR":::); }

    unsafe {
        asm!("xorl  %eax, %eax
              movw  %ds, %ax
              shll  $$4, %eax
              addl  $$GDT, %eax
              movl  %eax, (GDTR+2)
              lgdt  GDTR
              movl  %cr0, %eax
              orl   $$1, %eax
              movl  %eax, %cr0
              jmp   flushing
             flushing:
              movl  $$_start_32, %eax
              movl  %eax, (jmp_offset)
              movw  $$0x28, %ax
              movw  %ax, %ds
              movw  %ax, %es
              movw  %ax, %fs
              movw  %ax, %gs
              movw  %ax, %ss"
             ::
             : "eax"
            );
    }

    unsafe {
        asm!(".byte 0x66
              .byte 0xEA
             jmp_offset:   .long 0
              .word 0x20":::);
    }
}