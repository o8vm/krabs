use plankton::mem::MemoryRegion;

pub fn set_cmdline(cmd_line: &[u8]) {
    let mut cmdline_region = MemoryRegion::new(0x1000, 120 as u64);
    let cmdline = cmdline_region.as_mut_slice::<u8>(0, 120 as u64);
    cmdline[..cmd_line.len()].copy_from_slice(cmd_line);
}
