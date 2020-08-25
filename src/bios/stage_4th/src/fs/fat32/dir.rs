use super::{file::File, read_le_u32, BIOSParameterBlock, BUFFER_SIZE};
use crate::fs::blkdev::BlockDevice;
use core::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub enum DirError {
    NoMatch,
    NoMatchDir,
    NoMatchFile,
    InvalidName,
}

#[derive(Debug, Copy, Clone)]
enum NameType {
    Short,
    Long,
}

#[derive(Debug, Copy, Clone)]
pub struct Dir<T>
where
    T: BlockDevice + Clone + Copy,
    <T as BlockDevice>::Error: Debug,
{
    pub partition: T,
    pub bpb: BIOSParameterBlock,
    pub dir_name: [u8; 11],
    pub dir_cluster: u32,
    pub length: u32,
}

impl<T> Dir<T>
where
    T: BlockDevice + Clone + Copy,
    <T as BlockDevice>::Error: core::fmt::Debug,
{
    pub fn cd(&self, dir: &str) -> Result<Dir<T>, DirError> {
        match self.exists(dir) {
            Ok(buf) => Ok(self.get_dir(&buf.0)),
            Err(_) => Err(DirError::NoMatchDir),
        }
    }
    pub fn open_file(&self, file: &str) -> Result<File<T>, DirError> {
        match self.exists(file) {
            Ok(buf) => Ok(self.get_file(&buf.0, buf.1)),
            Err(_) => Err(DirError::NoMatchFile),
        }
    }

    pub fn exists(&self, name: &str) -> Result<([u8; 32], u32), DirError> {
        let invalid_char = "\\/:*?\"<>|";
        for ch in invalid_char.chars() {
            if name.contains(ch) {
                return Err(DirError::InvalidName);
            }
        }

        let bps = self.bpb.byte_per_sector as usize;
        let name_type = self.short_or_long(name);
        let op = |buf: &[u8]| -> [u8; 32] {
            let mut temp = [0; 32];
            for (i, elem) in temp.iter_mut().enumerate() {
                *elem = buf[i];
            }
            temp
        };

        let mut buf = [0; BUFFER_SIZE];
        let mut offset_count = 0;
        let mut step_count = 0;
        let mut copy_name = name;
        let mut long_cmp_done = false;

        for i in (0..).step_by(32) {
            if i % BUFFER_SIZE == 0 {
                self.partition
                    .read(
                        &mut buf,
                        self.bpb.offset(self.dir_cluster) as usize + offset_count * bps,
                    )
                    .unwrap();
                offset_count += 1;
            }

            if step_count != 0 {
                step_count -= 1;
                continue;
            }

            let offset = i - (offset_count - 1) * bps;
            if buf[0x00 + offset] == 0x00 {
                break;
            }
            if long_cmp_done {
                return Ok((op(&buf[offset..offset + 32]), i as u32));
            }

            if buf[0x00 + offset] == 0xE5 {
                continue;
            }
            if buf[0x0B + offset] == 0x0F {
                match name_type {
                    NameType::Short => {
                        step_count = buf[0x00 + offset] & 0x1F;
                        continue;
                    }
                    NameType::Long => {
                        let len = copy_name.chars().count();
                        let count = buf[0x00 + offset] & 0x1F;
                        let info = self.get_long_name(&buf[offset..offset + 32]);
                        let part_name = core::str::from_utf8(&info.0[0..info.1]).unwrap();
                        let multi = if len % 13 == 0 {
                            len / 13 - 1
                        } else {
                            len / 13
                        };
                        let start_at = if len <= 13 {
                            0
                        } else {
                            self.get_slice_index(copy_name, 13 * multi)
                        };
                        if !&copy_name[start_at..].eq(part_name) {
                            copy_name = name;
                            step_count = count;
                            continue;
                        } else if start_at == 0 && count == 1 {
                            long_cmp_done = true;
                        } else {
                            copy_name = &copy_name[0..start_at];
                        }
                    }
                }
            } else {
                if let NameType::Short = name_type {
                    let info = self.get_short_name(&buf[offset..offset + 32]);
                    let file_name = core::str::from_utf8(&info.0[0..info.1]).unwrap();
                    if name.eq_ignore_ascii_case(file_name) {
                        return Ok((op(&buf[offset..offset + 32]), i as u32));
                    }
                }
            }
        }
        Err(DirError::NoMatch)
    }

    fn short_or_long(&self, name: &str) -> NameType {
        let part = match name.find('.') {
            Some(i) => (&name[0..i], &name[i + 1..]),
            None => (&name[..], ""),
        };
        if name.is_ascii() && !name.contains(' ') && part.0.len() <= 8 && part.1.len() <= 3 {
            NameType::Short
        } else {
            NameType::Long
        }
    }

    fn get_slice_index(&self, name: &str, end: usize) -> usize {
        let mut len = 0;
        for ch in name.chars().enumerate() {
            if (0..end).contains(&ch.0) {
                len += ch.1.len_utf8();
            }
        }
        len
    }

    fn get_dir(&self, buf: &[u8]) -> Dir<T> {
        let mut dir_name = [0; 11];
        dir_name.copy_from_slice(&buf[0x00..0x0B]);

        Dir::<T> {
            partition: self.partition,
            bpb: self.bpb,
            dir_name,
            dir_cluster: ((buf[0x15] as u32) << 24)
                | ((buf[0x14] as u32) << 16)
                | ((buf[0x1B] as u32) << 8)
                | (buf[0x1A] as u32),
            length: read_le_u32(&buf[0x1C..0x20]),
        }
    }
    fn get_file(&self, buf: &[u8], offset: u32) -> File<T> {
        let mut file_name = [0; 8];
        let mut extension_name = [0; 3];

        let mut index = 0;
        for i in 0x00..0x08 {
            if buf[i] != 0x20 {
                file_name[index] = buf[i];
                index += 1;
            } else {
                break;
            }
        }

        index = 0;
        for i in 0x08..0x0B {
            if buf[i] != 0x20 {
                extension_name[index] = buf[i];
                index += 1;
            } else {
                break;
            }
        }

        let file_cluster = ((buf[0x15] as u32) << 24)
            | ((buf[0x14] as u32) << 16)
            | ((buf[0x1B] as u32) << 8)
            | (buf[0x1A] as u32);
        let mut len = [0; 4];
        len.copy_from_slice(&buf[0x1C..0x20]);

        File::<T> {
            partition: self.partition,
            bpb: self.bpb,
            dir_cluster: self.dir_cluster,
            offset,
            file_name,
            extension_name,
            file_cluster,
            length: read_le_u32(&buf[0x1C..0x20]) as usize,
        }
    }
    fn get_short_name(&self, buf: &[u8]) -> ([u8; 13], usize) {
        let mut file_name = [0; 13];
        let mut index = 0;

        for i in 0x00..=0x0A {
            if buf[i] != 0x20 {
                if i == 0x08 {
                    file_name[index] = b'.';
                    index += 1;
                }
                file_name[index] = buf[i];
                index += 1;
            }
        }
        (file_name, index)
    }
    fn get_long_name(&self, buf: &[u8]) -> ([u8; 13 * 3], usize) {
        let mut res = ([0; 13 * 3], 0);

        let op = |res: &mut ([u8; 13 * 3], usize), start: usize, end: usize| {
            for i in (start..end).step_by(2) {
                if buf[i] == 0x00 && buf[i + 1] == 0x00 {
                    break;
                }
                let unicode = ((buf[i + 1] as u16) << 8) | buf[i] as u16;
                if unicode <= 0x007F {
                    res.0[res.1] = unicode as u8;
                    res.1 += 1;
                } else if unicode >= 0x0080 && unicode <= 0x07FF {
                    let part1 = (0b11000000 | (0b00011111 & (unicode >> 12))) as u8;
                    let part2 = (0b10000000 | 0b00111111 & unicode) as u8;
                    res.0[res.1] = part1;
                    res.0[res.1 + 1] = part2;
                    res.1 += 2;
                } else if unicode >= 0x0800 {
                    let part1 = (0b11100000 | (0b00011111 & (unicode >> 12))) as u8;
                    let part2 = (0b10000000 | (0b00111111 & (unicode >> 6))) as u8;
                    let part3 = (0b10000000 | 0b00111111 & unicode) as u8;
                    res.0[res.1] = part1;
                    res.0[res.1 + 1] = part2;
                    res.0[res.1 + 2] = part3;
                    res.1 += 3;
                }
            }
        };

        if buf[0x01] != 0xFF {
            op(&mut res, 0x01, 0x0A);
        }
        if buf[0x0E] != 0xFF {
            op(&mut res, 0x0E, 0x19);
        }
        if buf[0x1C] != 0xFF {
            op(&mut res, 0x1C, 0x1F);
        }
        res
    }
}
