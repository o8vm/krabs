pub fn inform(s: &[u8]) {
    for &c in s {
        printc(c);
    }
}

fn printc(ch: u8) {
    unsafe {
        asm!("int $$0x10"
         :
         : "{ax}"(0x0e00 | (ch as u16 & 0xffu16)),
           "{ebx}"(7)
        );
    }
}

use core::fmt::{self, Write};
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::con::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\r\n"), $($arg)*)
    };
}

pub fn _print(args: fmt::Arguments) {
    let mut writer = BiosWriter {};
    writer.write_fmt(args).unwrap();
}

struct BiosWriter;

impl Write for BiosWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            printc(c);
        }
        Ok(())
    }
}
