use super::layout::{INIT_SEG, TRACK_BUFFER};
use super::mem::copy_block;
use core::str;

#[derive(Default)]
#[repr(C, packed)]
pub struct Dap {
    size: u8,
    zero: u8,
    pub sectors: u16,
    pub buffer: u32,
    pub startlba: u64,
}

impl Dap {
    const DISK_ERR: &'static [u8] = b"X";
    pub fn new(sectors: u16, buffer: u32, startlba: u64) -> Self {
        Dap {
            size: 0x10,
            sectors,
            buffer,
            startlba,
            ..Default::default()
        }
    }

    pub fn hd_read(&self, drv: u8) -> Result<(), &'static [u8]> {
        let ret: u16;
        let address: *const Dap = self;
        unsafe {
            llvm_asm!("int $$0x13"
             : "={ax}"(ret)
             : "{ax}"(0x4200),
               "{si}"(address),
               "{dl}"(drv)
            );
        }
        if ret & 0xff00 != 0 {
            Err(Self::DISK_ERR)
        } else {
            Ok(())
        }
    }

    pub fn hd_reset(drv: u16) -> Result<(), &'static [u8]> {
        let ret: u16;
        unsafe {
            llvm_asm!("int $$0x13"
             : "={eax}"(ret)
             : "{eax}"(0),
               "{edx}"(drv)
            );
        }
        if ret & 0xff00 != 0 {
            Err(Self::DISK_ERR)
        } else {
            Ok(())
        }
    }
}

pub const MAX_SECTOR: u16 = 16;  // 8192B = 16 secs, 8192*2B = 32
pub const SECTOR_SIZE: u16 = 512;

pub fn read_image(image_size: u16, dst: u32, start_lba: u16) -> Result<(), &'static str> {
    let mut start_lba = start_lba;
    let mut image_size = image_size;
    let mut dst = dst;
    let mut load_sectors: u16 = MAX_SECTOR;
    while image_size > 0 {
        if load_sectors > image_size {
            load_sectors = image_size
        }
        Dap::new(
            load_sectors,
            TRACK_BUFFER | (0x07C0 << 16),
            start_lba as u64,
        )
        .hd_read(0x80)
        .map_err(|err| str::from_utf8(err).unwrap())?;

        copy_block((INIT_SEG << 4) + TRACK_BUFFER, dst, load_sectors * 256)?;
        dst += (SECTOR_SIZE * load_sectors) as u32;
        print!(".");

        image_size -= load_sectors;
        start_lba += load_sectors;
    }
    Ok(())
}

pub fn read_to_trackbuf(secs: u16, start_lba: u64) -> Result<(), &'static str> {
    Dap::new(
        secs,
        TRACK_BUFFER | (0x07C0 << 16),
        start_lba,
    )
    .hd_read(0x80)
    .map_err(|err| str::from_utf8(err).unwrap())?;
    Ok(())
}