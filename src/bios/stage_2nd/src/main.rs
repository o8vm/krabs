#![no_std]
#![no_main]
extern crate rlibc;
use plankton::layout::STAGE3_START;
use plankton::{print, println};
use stage_2nd::{ParamTable, load_images};

#[link_section = ".table"]
#[no_mangle]
pub static mut PARAM: ParamTable = ParamTable {
    stage3_poss: 00,
    stage3_size: 00,
    stage4_size: 00,
};

#[link_section = ".startup"]
#[no_mangle]
fn stage2() {
    let stage3_poss: u16;
    let stage3_size: u16;
    let stage4_size: u16;
    unsafe {
        stage3_poss = PARAM.stage3_poss;
        stage3_size = PARAM.stage3_size;
        stage4_size = PARAM.stage4_size;
    }
    let ptr = STAGE3_START as *const ();
    let stage3: fn() -> ! = unsafe { core::mem::transmute(ptr) };

    print!("\r\nSTG2: ");
    println!(
        "stage3_size = {:04X} : \
              stage4_size = {:04X}",
        stage3_size, stage4_size,
    );
    load_images(stage3_poss, stage3_size, stage4_size).unwrap();
    stage3();
}
