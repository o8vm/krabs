#![feature(asm)]
#![no_std]

#[macro_use]
pub mod con;
pub mod mem;
pub mod dev;
pub mod ios;

// memlayout
pub const INIT_SEG:       u32 = 0x07C0;
pub const KERNEL_SIZE:    u32 = 0x100;
pub const STAGE2_LOAD:    u32 = 0x200;
pub const STAGE2_START:   u32 = 0x280;
pub const STAGE3_START:   u32 = 0x6000;
pub const TRACK_BUFFER:   u32 = 0xEE00;
pub const TRACK_BUF_SIZE: u32 = 0x1000;
pub const ELF_START:      u32 = 0x00100000;
pub const IMAGE_START:    u32 = 0x00380000;
pub const INITRD_START:   u32 = 0x00500000;
pub const HEAP_START:     u32 = 0x00600000;
pub const HEAP_END:       u32 = 0x00900000;