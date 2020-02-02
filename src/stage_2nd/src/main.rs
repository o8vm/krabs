#![no_std]
#![no_main]

use plankton::{dev::DiskRecord, print, println, STAGE31_START};
use stage_2nd::ParamTable;

#[link_section = ".table"]
#[no_mangle]
pub static mut PARAM: ParamTable = ParamTable {
    cmd_line: [0u8; 120],
    stage31_size: 0,
    stage32_size: 0,
    kernel_size: 0,
    initrd_size: 0,
};

#[link_section = ".startup"]
#[no_mangle]
fn stage2() {
    let kernel_size: u16;
    let stage31_size: u16;
    let stage32_size: u16;
    let initrd_size: u16;
    let cmd_line: &[u8];
    unsafe {
        kernel_size = PARAM.kernel_size;
        stage31_size = PARAM.stage31_size;
        stage32_size = PARAM.stage32_size;
        initrd_size = PARAM.initrd_size;
        cmd_line = &PARAM.cmd_line;
    }
    let ptr = STAGE31_START as *const ();
    let stage3: extern "C" fn(u16, u16, &[u8]) -> ! = unsafe { core::mem::transmute(ptr) };
    let mbr = 0x000usize as *const DiskRecord;
    let mbr: &DiskRecord = unsafe { &*mbr };

    print!("\r\nStage2: ");
    println!(
        "stage3+4_size = {:04X} : \
              kernel_size = {:04X} : \
              initrd_size = {:04X}",
        stage31_size + stage32_size,
        kernel_size,
        initrd_size
    );

    mbr.load_images(stage31_size, stage32_size, kernel_size, initrd_size)
        .unwrap();
    stage3(kernel_size, initrd_size, cmd_line);
}
