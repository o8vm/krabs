use core::fmt;
use plankton::ios::{io_delay, outb};

pub const CURSOR_START: u8 = 13;
pub const CURSOR_END: u8 = 14;
pub const VGA_CRT_ADDR: usize = 0x3D4;
pub const VGA_CRT_DATA: usize = 0x3D5;
pub const VGA_CRT_CURSTART: u8 = 10;
pub const VGA_CRT_CUREND: u8 = 11;
pub const VGA_CRT_CURADDH: u8 = 14;
pub const VGA_CRT_CURADDL: u8 = 15;

pub const TEXT_MODE_START: usize = 0xB8000;
pub const TEXT_DEF_ATTRIBUTE: u8 = 0x07;

macro_rules! vram_data {
    ($moji:expr, $iro:expr) => {
        (($iro as u16) << 8) + $moji as u16
    };
}

pub struct Screen {
    buffer: usize,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    attr: u8,
}

static mut SCRN: [Screen; 8] = [
    Screen {
        buffer: TEXT_MODE_START + 0x0000,
        width: 80,
        height: 25,
        x: 0,
        y: 0,
        attr: TEXT_DEF_ATTRIBUTE,
    },
    Screen {
        buffer: TEXT_MODE_START + 0x0800,
        width: 80,
        height: 25,
        x: 0,
        y: 0,
        attr: TEXT_DEF_ATTRIBUTE,
    },
    Screen {
        buffer: TEXT_MODE_START + 0x1000,
        width: 80,
        height: 25,
        x: 0,
        y: 0,
        attr: TEXT_DEF_ATTRIBUTE,
    },
    Screen {
        buffer: TEXT_MODE_START + 0x1800,
        width: 80,
        height: 25,
        x: 0,
        y: 0,
        attr: TEXT_DEF_ATTRIBUTE,
    },
    Screen {
        buffer: TEXT_MODE_START + 0x2000,
        width: 80,
        height: 25,
        x: 0,
        y: 0,
        attr: TEXT_DEF_ATTRIBUTE,
    },
    Screen {
        buffer: TEXT_MODE_START + 0x2800,
        width: 80,
        height: 25,
        x: 0,
        y: 0,
        attr: TEXT_DEF_ATTRIBUTE,
    },
    Screen {
        buffer: TEXT_MODE_START + 0x3000,
        width: 80,
        height: 25,
        x: 0,
        y: 0,
        attr: TEXT_DEF_ATTRIBUTE,
    },
    Screen {
        buffer: TEXT_MODE_START + 0x3800,
        width: 80,
        height: 25,
        x: 0,
        y: 0,
        attr: TEXT_DEF_ATTRIBUTE,
    },
];

static mut SCRN_PAGE: usize = 0;

pub struct Writer {
    page: &'static mut usize,
    scrn: &'static mut [Screen; 8],
}

impl Writer {
    pub fn new() -> Self {
        unsafe {
            Writer {
                page: &mut SCRN_PAGE,
                scrn: &mut SCRN,
            }
        }
    }

    fn cursor_size(start: u8, end: u8) {
        outb(VGA_CRT_CURSTART, VGA_CRT_ADDR);
        io_delay();
        outb(start, VGA_CRT_DATA);
        io_delay();
        outb(VGA_CRT_CUREND, VGA_CRT_ADDR);
        io_delay();
        outb(end, VGA_CRT_DATA);
    }

    pub fn cursor_pos(&self) {
        let page = *(self.page);
        let addr = self.scrn[page].y * self.scrn[page].width + self.scrn[page].x;
        Self::cursor_size(1, 0);
        io_delay();
        outb(VGA_CRT_CURADDH, VGA_CRT_ADDR);
        io_delay();
        outb(((addr >> 8) & 0xFF) as u8, VGA_CRT_DATA);
        io_delay();
        outb(VGA_CRT_CURADDL, VGA_CRT_ADDR);
        io_delay();
        outb((addr & 0xFF) as u8, VGA_CRT_DATA);
        Self::cursor_size(CURSOR_START, CURSOR_END);
    }

    pub fn select_page(&mut self, num: usize) {
        if num > 8 {
            *(self.page) = 7;
        } else {
            *(self.page) = num;
        }
    }

    pub fn color(&mut self, attr: u8) {
        self.scrn[*(self.page)].attr = attr;
    }

    pub fn locate(&mut self, xx: usize, yy: usize) {
        let page = *(self.page);
        if xx > self.scrn[page].width {
            self.scrn[page].x = self.scrn[page].width - 1;
        } else {
            self.scrn[page].x = xx;
        }

        if yy > self.scrn[page].height {
            self.scrn[page].y = self.scrn[page].height - 1;
        } else {
            self.scrn[page].y = yy;
        }
    }

    pub fn restore(&mut self) {
        let zero_page = plankton::mem::MemoryRegion::new(0x7C00, 4096);
        let xx = zero_page.read_u8(0x00);
        let yy = zero_page.read_u8(0x01);
        self.locate(xx as usize, yy as usize);
    }

    pub fn store(&self) {
        let zero_page = plankton::mem::MemoryRegion::new(0x7C00, 4096);
        let page = *(self.page);
        let xx: u8 = self.scrn[page].x as u8;
        let yy: u8 = self.scrn[page].y as u8;
        zero_page.write_u8(0x00, xx);
        zero_page.write_u8(0x01, yy);
    }

    fn xfer_line(src: usize, dst: usize, len: usize) {
        let src = unsafe { core::slice::from_raw_parts(src as *mut u16, len) };
        let dst = unsafe { core::slice::from_raw_parts_mut(dst as *mut u16, len) };
        dst.copy_from_slice(src);
    }

    fn fill_line(dst: usize, len: usize, data: u16) {
        let dst = unsafe { core::slice::from_raw_parts_mut(dst as *mut u16, len) };
        for v in dst {
            *v = data;
        }
    }

    fn fill_char(dst: usize, data: u16) {
        let dst = dst as *mut u16;
        unsafe {
            *dst = data;
        }
    }

    fn scrollup(&self) {
        let mut cnt: usize = 0;
        let h = self.scrn[*(self.page)].height;
        let w = self.scrn[*(self.page)].width;
        let mut dst = self.scrn[*(self.page)].buffer;
        let mut src = dst + w;
        while cnt < h {
            Self::xfer_line(src, dst, w);
            src += w;
            dst += w;
            cnt += 1;
        }
        Self::fill_line(dst, w, vram_data!(b' ', self.scrn[*(self.page)].attr));
    }

    pub fn clear(&self) {
        let h = self.scrn[*(self.page)].height;
        let w = self.scrn[*(self.page)].width;
        let mut dst = self.scrn[*(self.page)].buffer;
        let mut cnt = 0;
        while cnt < h {
            Self::fill_line(dst, w, vram_data!(b' ', self.scrn[*(self.page)].attr));
            dst += w;
            cnt += 1;
        }
    }

    pub fn putchar(&mut self, ch: u8) {
        let page = *(self.page);
        let h = self.scrn[page].height;
        let w = self.scrn[page].width;
        match ch {
            b'\n' => {
                self.scrn[page].x = 0;
                if self.scrn[page].y < (h - 1) {
                    self.scrn[page].y += 1;
                } else {
                    self.scrollup();
                }
            }
            0x08 => {
                if self.scrn[page].x > 0 {
                    self.scrn[page].x -= 1;
                    let dst =
                        self.scrn[page].buffer + self.scrn[page].y * w * 2 + self.scrn[page].x * 2;
                    Self::fill_char(dst, vram_data!(b' ', self.scrn[page].attr));
                }
            }
            0x20..=0x7e => {
                let ch = ch & 0xff;
                let dst =
                    self.scrn[page].buffer + self.scrn[page].y * w * 2 + self.scrn[page].x * 2;
                Self::fill_char(dst, vram_data!(ch, self.scrn[page].attr));
                if self.scrn[page].x < (w - 1) {
                    self.scrn[page].x += 1;
                } else {
                    self.scrn[page].x = 0;
                    if self.scrn[page].y < (h - 1) {
                        self.scrn[page].y += 1;
                    } else {
                        self.scrollup();
                    }
                }
            }
            _ => {}
        }
        self.cursor_pos();
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.putchar(c);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = Writer::new();
    writer.restore();
    writer.write_fmt(args).unwrap();
    writer.store();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::text::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\n"), $($arg)*)
    };
}
