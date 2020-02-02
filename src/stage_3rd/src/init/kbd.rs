use plankton::mem::MemoryRegion;
pub fn set_keyboard() {
    let zero_page = MemoryRegion::new(0x000, 4096);
    let ret: u8;
    unsafe {
        asm!("int $$0x16"
         : "={al}"(ret)
         : "{ax}"(0x0200)
        );
        asm!("int $$0x16"
        :
        : "{ax}"(0x0305), "{ebx}"(0)
        );
    }
    zero_page.write_u8(0x1eb, ret);
}
