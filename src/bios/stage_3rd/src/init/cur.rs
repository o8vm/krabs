use plankton::mem::MemoryRegion;

pub fn set_cur() {
    let zero_page = MemoryRegion::new(0x000, 4096);

    let curs: u16;
    unsafe {
        llvm_asm!("int $$0x10"
         : "={dx}"(curs)//, "={dl}"(curs_x)
         : "{ax}"(0x0300), "{ebx}"(0)
        );
    }
    zero_page.write_u16(0x00, curs);
}

pub fn re_cur() {
    let zero_page = MemoryRegion::new(0x000, 4096);
    let curs = zero_page.read_u16(0x00);
    unsafe {
        llvm_asm!("int $$0x10"
         :: "{ax}"(0x0200u16), "{dx}"(curs), "{ebx}"(0)
        );
    }
}
