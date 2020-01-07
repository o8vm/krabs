#![no_std]
#![no_main]

use stage_2nd::ParamTable;
use plankton::{dev::DiskRecord, print, println, STAGE3_START};

#[link_section=".table"]
#[no_mangle]
pub static mut PARAM: ParamTable = ParamTable { 
    cmd_line: [0u8; 122], 
    stage3_size: 0,
    kernel_size: 0,
    initrd_size: 0,
};

#[link_section=".startup"]
#[no_mangle]
fn stage2() {
    let kernel_size: u16; let stage3_size: u16; let initrd_size: u16;
    let cmd_line: &[u8];
    unsafe {
        kernel_size = PARAM.kernel_size; 
        stage3_size = PARAM.stage3_size; 
        initrd_size = PARAM.initrd_size;
        cmd_line = &PARAM.cmd_line;
    }
    let ptr = STAGE3_START as *const ();
    let stage3: extern "C" fn(u16, u16, &[u8]) -> ! = unsafe {
        core::mem::transmute(ptr)
    };
    let mbr = 0x000usize as *const DiskRecord;
    let mbr: &DiskRecord = unsafe { &*mbr };

    print!("\r\nStage2: ");
    println!("stage3_size = {:04X} : \
              kernel_size = {:04X} : \
              initrd_size = {:04X}", 
            stage3_size, kernel_size, initrd_size);

    mbr.load_images(stage3_size, kernel_size, initrd_size).unwrap();
    stage3(kernel_size, initrd_size, cmd_line);
}