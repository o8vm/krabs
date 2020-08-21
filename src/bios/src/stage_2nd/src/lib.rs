#![no_std]
use plankton::{dev::read_image, layout::{INIT_SEG, STAGE3_START, STAGE4_START}, print, println};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[repr(C, packed)]
pub struct ParamTable {
    pub stage3_poss: u16,
    pub stage3_size: u16,
    pub stage4_size: u16,
}

pub fn load_images(mut slba: u16, stage3_size: u16, stage4_size: u16) -> Result<(), &'static str> {
    if stage3_size > 0 {
        print!("  Loading stage3 ");
        read_image(stage3_size, (INIT_SEG << 4) + STAGE3_START, slba)?;
        println!("");
    } else {
        return Err("stage3 is not found");
    }

    if stage4_size > 0 {
        print!("  Loading stage4 ");
        slba += stage3_size;
        read_image(stage4_size, STAGE4_START, slba)?;
        println!(""); 
    } else {
        // return Err("stage4 is not found");
    }
    Ok(())
}