// memlayout
pub const INIT_SEG: u32 = 0x07C0;

pub const STAGE2_LOAD: u32 = 0x200;     // + 7C00
pub const STAGE2_START: u32 = 0x206;    // + 7C00
pub const STAGE3_START: u32 = 0x6000;   // + 7C00
pub const TRACK_BUFFER: u32 = 0xD000;   // + 7C00 = 0x8900
pub const TRACK_BUF_SIZE: u32 = 0x2000; // 0x8900 + 0x2000 = 0xA900

pub const CMD_LINE_ADDR: u32 = 0x0002_0000; // prot
pub const STAGE4_START: u32 = 0x0003_0000;  // ptot
pub const PROT_STACK: u32 = 0x9a000;        // prot
pub const PGTABLE_START: u64 = 0x1000;      // prot
pub const ELF_START: u32 = 0x0010_0000;     // prot
pub const IMAGE_START: u32 = 0x0350_0000;   // prot
pub const INITRD_START: u32 = 0x0560_0000;  // prot
pub const HEAP_START: u32 = 0x0760_0000;    // prot
pub const HEAP_END: u32 = 0x07FF_FFFF;      // prot
