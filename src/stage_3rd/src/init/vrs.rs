use plankton::mem::MemoryRegion;

pub fn set_version() {
    const KERNEL_BOOT_FLAG_MAGIC: u16 = 0xaa55;
    const KERNEL_HDR_MAGIC: u32 = 0x5372_6448;
    const KERNEL_LOADER_OTHER: u8 = 0xff;
    const KERNEL_MIN_ALIGNMENT_BYTES: u32 = 0x0100_0000;

    let zero_page = MemoryRegion::new(0x000, 4096);
    
    zero_page.write_u16(0x1FE, KERNEL_BOOT_FLAG_MAGIC);
    zero_page.write_u32(0x202, KERNEL_HDR_MAGIC);
    zero_page.write_u16(0x206, 0x020C);
    zero_page.write_u8(0x210, KERNEL_LOADER_OTHER);
    zero_page.write_u32(0x230, KERNEL_MIN_ALIGNMENT_BYTES);
}
