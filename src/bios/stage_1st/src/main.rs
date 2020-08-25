#![no_std]
#![no_main]
extern crate rlibc;
use plankton::layout::{STAGE2_LOAD, STAGE2_START};
use plankton::{con::inform, dev::Dap};
use stage_1st::entry;

#[link_section = ".size"]
#[no_mangle]
pub static mut STAGE2_SIZE: u16 = 00;
#[link_section = ".slba"]
#[no_mangle]
pub static mut STAGE2_SLBA: u16 = 00;

entry!(main);
fn main() {
    let ptr = STAGE2_START as *const ();
    let stage2: fn() = unsafe { core::mem::transmute(ptr) };
    inform(b"Stage1: ");
    match Dap::hd_reset(0x80) {
        Err(err) => {
            inform(err);
            return;
        }
        Ok(_) => {}
    }
    unsafe {
        match Dap::new(
            STAGE2_SIZE as u16,
            STAGE2_LOAD | (0x07C0 << 16),
            STAGE2_SLBA as u64,
        )
        .hd_read(0x80)
        {
            Err(err) => {
                inform(err);
                return;
            }
            Ok(_) => inform(b"Ok"),
        }
    }
    stage2();
}
