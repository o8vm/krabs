use plankton::mem::MemoryRegion;

pub fn set_screen_info() {
    let zero_page = MemoryRegion::new(0x000, 4096);

    let curs_y: u8;
    // let curs_x: u8;
    unsafe {
        asm!("int $$0x10"
         : "={dh}"(curs_y)//, "={dl}"(curs_x)
         : "{ax}"(0x0300), "{ebx}"(0)
        );
    }
    zero_page.write_u8(0x01, curs_y);
    // zero_page.write_u8(0x00, curs_x);

    let mode: u8;
    let page: u8;
    unsafe {
        asm!("int $$0x10"
         : "={al}"(mode), "={bh}"(page)
         : "{ax}"(0x0f00), "{ebx}"(0)
        );
    }
    zero_page.write_u8(0x004, page);
    zero_page.write_u8(0x006, mode & 0x7f);

    zero_page.write_u8(0x007, 80);
    zero_page.write_u8(0x00E, 25);
    zero_page.write_u8(0x00F, 1);

    let font: u16;
    unsafe {
        asm!("movw %ax, %gs
          movw %gs:(0x485), %ax"
         : "={ax}"(font)
         : "{ax}"(0)
        );
    }
    zero_page.write_u16(0x010, font);

    zero_page.write_u16(0x1FA, 0xFFFF);
}
