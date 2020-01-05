use plankton::{print, println};
use plankton::ios::{inb, io_delay, outb};

fn flush_8042() {
    let mut stat: u8;
    
    loop {
        io_delay();
        stat = inb(0x64);
        if stat & 0x01 != 0 {
            io_delay(); inb(0x60);
            continue;
        }
        if stat & 0x02 != 0 {
            continue;
        }
        break;
    }
}

fn check_a20() -> Result<(), ()> {
    let ret: u32;
    unsafe {
        asm!("movl $$1, %ebx
              xorw %ax, %ax
              movw %ax, %fs
              notw %ax
              movw %ax, %gs
              movw %fs:0, %ax
              cmpw %gs:16, %ax
              jnz  1f
              cli
              movw %ax, %dx
              notw %ax
              movw %ax, %fs:0
              cmpw %gs:16, %ax
              movw %dx, %fs:0
              sti
              jnz  1f
              xorl %ebx, %ebx
             1:
              movl %ebx, %eax"
             : "={eax}"(ret)
             :
             : "eax" "ebx" "edx"
            );
    }
    if ret != 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn enable_a20() {
    let data: u8;
    if check_a20().is_ok() {
        println!("  A20 line is already activate.");
        return;
    }
    // Classical AT type
    flush_8042();
    outb(0x64, 0xD1);
    flush_8042();
    outb(0x60, 0xDF);
    flush_8042();
    if check_a20().is_ok() {
        println!("  A20 line is activated by classical method.");
        return;
    }
    // PS/2 type (Fast A20 version)
    data = inb(0x92);
    io_delay();
    outb(data | 2, 0x92);
    while check_a20().is_err() {}
    println!("  A20 line is activated by fast method.");
}