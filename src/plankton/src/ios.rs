// I/O deley
pub fn io_delay() {
    unsafe {
        asm!("inb $$0x80, %al
          inb $$0x80, %al"
         :::"eax"
        );
    }
}

pub fn inb(port: usize) -> u8 {
    let mut data: u8;
    unsafe {
        asm!("inb %dx, %al"
         : "={al}"(data)
         : "{dx}"(port)
        );
    }
    data
}

pub fn outb(data: u8, port: usize) {
    unsafe {
        asm!("outb %al, %dx"
         :
         : "{al}"(data), "{dx}"(port)
        );
    }
}
