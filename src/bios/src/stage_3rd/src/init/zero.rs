use plankton::mem::MemoryRegion;

pub fn clear_bss() {
    use core::ptr;
    extern "C" {
        static mut _data_end: u8;
        static mut _bss_end: u8;
    }
    unsafe {
        let count = &_bss_end as *const u8 as usize - &_data_end as *const u8 as usize;
        ptr::write_bytes(&mut _data_end as *mut u8, 0, count);
    }
}

pub enum Pages {
    FirstHalf,
    SecondHalf,
}

impl Pages {
    pub fn clear(&self) {
        let mut zero_page = MemoryRegion::new(0x000, 0x1000);
        match self {
            Self::FirstHalf => {
                for elem in zero_page.as_mut_slice::<u8>(0x000, 0x800).iter_mut() {
                    *elem = 0;
                }
            }
            Self::SecondHalf => {
                for elem in zero_page.as_mut_slice::<u8>(0x800, 0x800).iter_mut() {
                    *elem = 0;
                }
            }
        }
    }
}
