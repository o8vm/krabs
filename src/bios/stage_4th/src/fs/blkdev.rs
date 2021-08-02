use plankton::{
    dev::MAX_SECTOR,
    layout::{INIT_SEG, TRACK_BUFFER},
    mem::MemoryRegion,
};
use crate::{print, println};

pub trait BlockDevice {
    type Error;
    fn read(&self, buf: &mut [u8], offset: usize) -> Result<(), Self::Error>;
    fn write(&self, buf: &mut [u8], offset: usize) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, Copy)]
pub enum BIOError {
    IOError,
}

#[inline(never)]
#[no_mangle]
pub fn copy_bytes(
    buf: &mut [u8],
    stored_bytes: usize,
    buff_offset: usize,
    sector_offset: usize,
) -> usize {
    let track_buffer = MemoryRegion::new(
        (TRACK_BUFFER + (INIT_SEG << 4)) as u64,
        (512 * MAX_SECTOR) as u64,
    );

    let buf_cap = buf.len() - buff_offset;
    let avail_bytes = stored_bytes - sector_offset;

    let n_bytes = if buf_cap < avail_bytes {
        buf_cap
    } else {
        avail_bytes
    };
    println!("sector_offset = {}, n_bytes = {}, buffer_offset = {}", sector_offset, n_bytes, buff_offset);
    let slice = track_buffer.as_slice::<u8>(sector_offset as u64, n_bytes as u64);
    unsafe {
        core::ptr::copy_nonoverlapping(slice.as_ptr(), buf[buff_offset..(buff_offset + n_bytes)].as_mut_ptr(), slice.len());
    }
    //buf[buff_offset..(buff_offset + n_bytes)].clone_from_slice(&slice);
    println!("slice = {:?}", slice);
    println!("buf = {:?}", buf);
    n_bytes
}

pub fn read(buf: &mut [u8], offset: usize) -> Result<(), BIOError> {
    let start_lba = offset / 512;
    let end_lba = (offset + buf.len() - 1) / 512;
    let mut num_sectors = end_lba - start_lba + 1;

    let num_invokes = (num_sectors + MAX_SECTOR as usize - 1) / MAX_SECTOR as usize;
    let mut buff_offset = 0;
    for i in 0..num_invokes {
        let load_sectors = if num_sectors > MAX_SECTOR as usize {
            MAX_SECTOR as u32
        } else {
            num_sectors as u32
        };
        let sector_offset = if i == 0 { offset % 512 } else { 0 };
        crate::svm::rm::diskread(
            start_lba as u32 + (i as u32 * MAX_SECTOR as u32),
            load_sectors,
        )?;
        
        buff_offset += copy_bytes(buf, load_sectors as usize * 512, buff_offset, sector_offset);
        num_sectors -= load_sectors as usize;
    }
    Ok(())
}
