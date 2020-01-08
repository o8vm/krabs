#![no_std]
#![no_main]
use plankton::{con::inform, dev::Dap};
use stage_1st::entry;

#[link_section = ".size"]
#[no_mangle]
pub static mut STAGE2_SISE: u16 = 00;

entry!(main);
fn main() {
    let ptr = plankton::STAGE2_START as *const ();
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
        match Dap::new(STAGE2_SISE, plankton::STAGE2_LOAD | (0x07C0 << 16), 1).hd_read(0x80) {
            Err(err) => {
                inform(err);
                return;
            }
            Ok(_) => inform(b"load stage2"),
        }
    }
    stage2();
}
