#![feature(asm)]
#![no_std]

#[macro_use]
pub mod con;
pub mod dev;
pub mod ios;
pub mod mem;

// memlayout
pub const INIT_SEG: u32 = 0x07C0;
pub const KERNEL_SIZE: u32 = 0x100;
pub const STAGE2_LOAD: u32 = 0x200;
pub const STAGE2_START: u32 = 0x280;
pub const STAGE31_START: u32 = 0x6000;
pub const STAGE32_START: u32 = 0x20000;
pub const TRACK_BUFFER: u32 = 0xB000;
pub const TRACK_BUF_SIZE: u32 = 0x4000;
pub const PGTABLE_START: u64 = 0x1000;
pub const ELF_START: u32    = 0x00100000;
pub const IMAGE_START: u32  = 0x00600000;
pub const INITRD_START: u32 = 0x00900000;
pub const HEAP_START: u32   = 0x01600000;
pub const HEAP_END: u32     = 0x01900000;
pub const STACK_SIZE: i32 = 1024;
