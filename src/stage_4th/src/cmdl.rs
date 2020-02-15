use plankton::{mem::MemoryRegion, CMD_LINE_ADDR, PRE_CMDLINEAD};

pub fn setup_cmdline() {
    let pre_cmdline = unsafe { *(PRE_CMDLINEAD as *const [u8; 120]) };
    let mut cmdline_region = MemoryRegion::new(CMD_LINE_ADDR as u64, 120 as u64);

    let cmdline = cmdline_region.as_mut_slice::<u8>(0, 120 as u64);
    cmdline[..pre_cmdline.len()].copy_from_slice(&pre_cmdline);

    let zero_page = MemoryRegion::new(0x7C00, 4096);
    zero_page.write_u32(0x228, CMD_LINE_ADDR);
}
