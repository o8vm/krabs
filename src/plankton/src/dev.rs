use super::mem::copy_block;
use super::{TRACK_BUFFER, INIT_SEG, STAGE3_START, IMAGE_START, INITRD_START};
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
    const DISK_ERR: &'static [u8] = b"I/O error";
    pub fn new(sectors: u16, buffer: u32, startlba: u64) -> Self {
        Dap { size: 0x10, sectors, buffer, startlba, 
                .. Default::default() }
    }

    pub fn hd_read(&self, drv: u8) -> Result<(), &'static [u8]> {
        let ret: u16;
        let address: *const Dap = self; 
        unsafe {
            asm!("int $$0x13"
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
            asm!("int $$0x13"
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

#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
struct CHS(u8, u8, u8);

#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
struct PartitionTable {
    boot:      u8,
    start_chs: CHS,
    ptype:     u8,
    end_chs:   CHS,
    start_lba: u32,
    size:      u32,
}

#[repr(C, packed)]
pub struct DiskRecord {
    bootstrap: [u8; 446],
    table: [PartitionTable; 4],
    sign: u16,
}

impl DiskRecord {
    const MAX_RECORD:  u16 = 33;
    const MAX_SECTOR:  u16 = 8; // 18
    const SECTOR_SIZE: u16 = 512;

    pub fn get_lba(&self) -> Result<u32, &'static str> {
        let mut dr_offset:   u32;
        let mut epbr_offset: u32 = 0;
        let mut recordcnt:   u16 = 0;
        let mut dr: &Self = self;
        let tmp: Self = DiskRecord {
            bootstrap: [0;446], 
            table: [Default::default();4], 
            sign: 0
        };
            
        for s in dr.table.iter() {
            if 0 != s.boot {
                return Ok(s.start_lba)
            }
        }

        recordcnt += 1;
        loop {
            let mut extndx = 0;
            for (i, s) in dr.table.iter().enumerate() {
                match s.ptype {
                    0x5 | 0x85 | 0x0f => extndx = i,
                    _ => {},
                }
            }

            if recordcnt > Self::MAX_RECORD { 
                break Err("Too many disk record") 
            }

            if extndx != 0 {
                dr_offset = dr.table.get(extndx).unwrap().start_lba;
                if recordcnt < 2 { 
                    epbr_offset = dr_offset 
                } else { 
                    dr_offset += epbr_offset 
                }

                let address: *const Self = &tmp;
                match Dap::new(1, address as u32|(0x07C0<<16), dr_offset as u64)
                    .hd_read(0x80) {
                    Err(err) => break Err(str::from_utf8(err).unwrap()),
                    Ok(_) => {
                        dr = &tmp;
                        for s in dr.table.iter() {
                            if 0 != s.boot { return Ok(dr_offset + s.start_lba) }
                        }
                    },
                }
                recordcnt += 1;
            } else {
                break Err("Cannot find boot partition")
            }
        }
    }

    fn read_image(image_size: u16, dst: u32, start_lba: u32) -> Result<(), &'static str> {
        let mut start_lba  = start_lba;
        let mut image_size = image_size;
        let mut dst = dst;
        let mut load_sectors: u16 = Self::MAX_SECTOR;
        while image_size > 0 {
            if load_sectors > image_size { load_sectors = image_size }
            Dap::new(load_sectors, TRACK_BUFFER|(0x07C0<<16), start_lba as u64)
                .hd_read(0x80).map_err(|err| str::from_utf8(err).unwrap())?;

            copy_block((INIT_SEG<<4)+TRACK_BUFFER, dst, load_sectors * 256)?;
            dst+= (Self::SECTOR_SIZE * load_sectors) as u32;
            print!(".");
            
            image_size -= load_sectors;
            start_lba += load_sectors as u32;
        }
        println!(".");
        Ok(())
    }

    pub fn load_images(&self, stage3_size: u16, kernel_size: u16, initrd_size: u16) -> Result<(), &'static str> {
        let mut slba = self.get_lba()?;

        if stage3_size > 0 {
            print!("  Loading stage3 ");
            Self::read_image(stage3_size, (INIT_SEG<<4)+STAGE3_START, slba)?;
        }

        if kernel_size > 0 {
            print!("  Loading compressed kernel image ");
            slba += stage3_size as u32;
            Self::read_image(kernel_size, IMAGE_START, slba)?;
        }

        if initrd_size > 0 {
            print!("  Loading initrd ");
            slba += kernel_size as u32;
            Self::read_image(initrd_size, INITRD_START, slba)?;
        }
        Ok(())
    }
}