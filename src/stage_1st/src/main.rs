#![no_std]
#![no_main]
use stage_1st::entry;
use plankton::{con::inform, dev::Dap};

#[link_section=".size"]
#[no_mangle]
static STAGE2_SISE: u16 = 50;

entry!(main);
fn main() {
    let ptr = plankton::STAGE2_START as *const ();
    let stage2: fn() = unsafe {
        core::mem::transmute(ptr)
    };
    inform(b"Stage1: ");
    match Dap::hd_reset(0x80) {
        Err(err) => { inform(err); return },
        Ok(_) => {},
    }
    match Dap::new(STAGE2_SISE, plankton::STAGE2_LOAD|(0x07C0<<16), 1)
        .hd_read(0x80) {
            Err(err) => {  inform(err); return },
            Ok(_) => inform(b"stage2 loaded\r\n"),
    }
    stage2();
}
