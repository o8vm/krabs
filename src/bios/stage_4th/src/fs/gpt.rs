use crate::{print, println};
use crate::fs::blkdev::{read, BIOError, BlockDevice};
pub struct GPT {
    data: [u8; 1280], // 10 Partitions
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GPTEntry {
    ptype: [u8; 16],
    guid: [u8; 16],
    start_lba: u64,
    last_lba: u64,
    attr: u64,
    name: [u8; 72],
}

impl GPTEntry {
    pub fn is_efi_system_partition(&self) -> bool {
        let efisysprt: [u8; 16] = [
            0x28, 0x73, 0x2A, 0xC1, 0x1F, 0xF8, 0xD2, 0x11, 0xBA, 0x4B, 0x00, 0xA0, 0xC9, 0x3E,
            0xC9, 0x3B,
        ];
        if self.ptype == efisysprt {
            true
        } else {
            false
        }
    }
    pub fn starting_lba(&self) -> u32 {
        self.start_lba as u32
    }
    pub fn partition_length(&self) -> usize {
        (self.last_lba - self.start_lba + 1) as usize
    }
}

impl GPT {
    pub fn new() -> Self {
        let mut data: [u8; 1280] = [0u8; 1280];
        let offset = 2 * 512;
        read(&mut data, offset).unwrap();
        Self { data }
    }
    // pub fn as_mut_slice<T>(&mut self) -> &mut [T] {
    //     unsafe {
    //         core::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.data.len() / 128)
    //     }
    // }
    fn as_slice<T>(&self) -> &[T] {
        unsafe {
            core::slice::from_raw_parts(self.data.as_ptr() as *const T, self.data.len() / 128)
        }
    }
    pub fn get_efi_system_partition(&self) -> Option<Partition> {
        for entry in self.as_slice::<GPTEntry>().iter() {
            if entry.is_efi_system_partition() {
                return Some(Partition {
                    start_lba: entry.starting_lba(),
                    length: entry.partition_length(),
                });
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Partition {
    pub start_lba: u32,
    pub length: usize,
}

impl BlockDevice for Partition {
    type Error = BIOError;
    fn read(&self, buf: &mut [u8], offset: usize) -> Result<(), Self::Error> {
        let load_sectors = (buf.len() + offset + 512 - 1) / 512;    // sectors
        let real_offset = (self.start_lba as usize * 512) + offset; // bytes
        if load_sectors <= self.length {
            println!("offset = {}", offset);
            println!("load sectors = {}, length = {}", load_sectors, self.length);
            read(buf, real_offset)?;
            Ok(())
        } else {
            println!("offset = {}", offset);
            println!("load sectors = {}, length = {}", load_sectors, self.length);
            Err(BIOError::IOError)
        }
    }
    fn write(&self, _: &mut [u8], _: usize) -> Result<(), Self::Error> {
        Ok(())
    }
}
