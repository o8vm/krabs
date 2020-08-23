use crate::fs::blkdev::BlockDevice;
use core::convert::TryInto;

pub mod dir;
pub mod file;

const BUFFER_SIZE: usize = 512;

#[derive(Debug, Copy, Clone)]
pub struct BIOSParameterBlock {
    pub byte_per_sector: u16,
    pub sector_per_cluster: u8,
    pub reserved_sector: u16,
    pub num_fat: u8,
    pub total_sector: u32,
    pub sector_per_fat: u32,
    pub root_cluster: u32,
    pub id: u32,
    pub volume_label: [u8; 11],
    pub file_system: [u8; 8],
}

impl BIOSParameterBlock {
    // Get the first sector offset bytes of the cluster from the cluster number
    pub fn offset(&self, cluster: u32) -> usize {
        (((self.reserved_sector as u32)
            + (self.num_fat as u32) * self.sector_per_fat
            + (cluster - 2) * (self.sector_per_cluster as u32))
            * (self.byte_per_sector as u32)) as usize
    }
    // get fat1 start offset bytes
    pub fn fat1(&self) -> usize {
        ((self.reserved_sector as u32) * (self.byte_per_sector as u32)) as usize
    }
}

pub struct Volume<T>
where
    T: BlockDevice + Clone + Copy,
{
    base: T,
    bpb: BIOSParameterBlock,
}

impl<T> Volume<T>
where
    T: BlockDevice + Clone + Copy,
    <T as BlockDevice>::Error: core::fmt::Debug,
{
    // get volume
    pub fn new(base: T) -> Volume<T> {
        let mut buf = [0; BUFFER_SIZE];
        base.read(&mut buf, 0).unwrap();

        let mut volume_label = [0; 11];
        volume_label.copy_from_slice(&buf[0x47..0x52]);

        let mut file_system = [0; 8];
        file_system.copy_from_slice(&buf[0x52..0x5A]);

        let bps = read_le_u16(&buf[0x0B..0x0D]);
        if bps as usize != BUFFER_SIZE {
            panic!(
                "BUFFER_SIZE is {} Bytes, but byte_per_sector is {} Bytes",
                BUFFER_SIZE, bps
            );
        }

        Volume::<T> {
            base,
            bpb: BIOSParameterBlock {
                byte_per_sector: bps,
                sector_per_cluster: buf[0x0D],
                reserved_sector: ((buf[0x0F] as u16) << 8) | buf[0x0E] as u16,
                num_fat: buf[0x10],
                total_sector: read_le_u32(&buf[0x20..0x24]),
                sector_per_fat: read_le_u32(&buf[0x24..0x28]),
                root_cluster: read_le_u32(&buf[0x2C..0x30]),
                id: read_le_u32(&buf[0x43..0x47]),
                volume_label,
                file_system,
            },
        }
    }

    // into root_dir
    pub fn root_dir(&self) -> dir::Dir<T> {
        dir::Dir::<T> {
            base: self.base,
            bpb: self.bpb,
            dir_name: [0; 11],
            dir_cluster: self.bpb.root_cluster,
            length: 0,
        }
    }
}

pub fn read_le_u16(input: &[u8]) -> u16 {
    let (int_bytes, _) = input.split_at(core::mem::size_of::<u16>());
    u16::from_le_bytes(int_bytes.try_into().unwrap())
}

pub fn read_le_u32(input: &[u8]) -> u32 {
    let (int_bytes, _) = input.split_at(core::mem::size_of::<u32>());
    u32::from_le_bytes(int_bytes.try_into().unwrap())
}
