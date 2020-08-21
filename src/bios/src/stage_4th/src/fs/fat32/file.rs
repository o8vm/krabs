use super::BIOSParameterBlock;
use crate::fs::blkdev::BlockDevice;

#[derive(Debug, Copy, Clone)]
pub enum FileError {
    BufTooSmall,
    IllegalName,
}

#[derive(Debug, Copy, Clone)]
pub struct File<T>
where
    T: BlockDevice + Clone + Copy,
    <T as BlockDevice>::Error: core::fmt::Debug,
{
    pub base: T,
    pub bpb: BIOSParameterBlock,
    pub dir_cluster: u32,
    pub offset: u32,
    pub file_name: [u8; 8],
    pub extension_name: [u8; 3],
    pub file_cluster: u32,
    pub length: usize,
}

impl<T> File<T>
where
    T: BlockDevice + Clone + Copy,
    <T as BlockDevice>::Error: core::fmt::Debug,
{
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, FileError> {
        if buf.len() < self.length as usize {
            return Err(FileError::BufTooSmall);
        }
        let bps = self.bpb.byte_per_sector as usize;
        let spc = self.bpb.sector_per_cluster as usize;

        // cluster pointer
        let mut loc = self.file_cluster;
        // buffer pointer
        let mut start_at = 0;

        // number of clusters
        let clusters = (self.length as usize / bps) / spc;
        // left sectors
        let sectors = (self.length as usize / bps) % spc;
        // left bytes
        let bytes = self.length as usize % bps;

        for _ in 0..clusters {
            self.base
                .read(
                    &mut buf[start_at..start_at + spc * bps],
                    self.bpb.offset(loc),
                )
                .unwrap();
            let entry = self.get_fat_entry(loc);
            loc = entry;
            start_at += spc * bps;
        }
        if sectors > 0 || bytes > 0 {
            self.base
                .read(
                    &mut buf[start_at..start_at + sectors * bps + bytes],
                    self.bpb.offset(loc),
                )
                .unwrap();
        }
        Ok(self.length as usize)
    }
    fn get_fat_entry(&self, loc: u32) -> u32 {
        let fat_addr = self.bpb.fat1();
        let offset = loc as usize * 4;
        let mut buf = [0; 4];

        self.base.read(&mut buf, fat_addr + offset).unwrap();
        u32::from_le_bytes(buf)
    }
}
