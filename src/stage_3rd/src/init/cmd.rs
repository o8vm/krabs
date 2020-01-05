use plankton::{INIT_SEG, mem::MemoryRegion};

pub fn set_cmdline(cmd_line: &[u8]) {
    let mut cmdline_region = MemoryRegion::new(0x800, 2048 as u64);

    let zero_page = MemoryRegion::new(0x000, 4096);

    let cmdline = cmdline_region.as_mut_slice::<u8>(0, 2048 as u64);
    cmdline[..cmd_line.len()].copy_from_slice(cmd_line);
    cmdline[cmd_line.len()] = 0;

    zero_page.write_u32(0x228, (INIT_SEG<<4) + 0x800);
    zero_page.write_u32(0x238, cmd_line.len() as u32);
}