pub mod a20;
pub mod cur;
pub mod ist;
pub mod kbd;
pub mod msz;
pub mod vid;
pub mod vrs;
pub mod zero;

pub fn setup() {
    zero::clear_bss();
    zero::Pages::SecondHalf.clear();
    zero::Pages::FirstHalf.clear();
    msz::set_mem_size();
    kbd::set_keyboard();
    ist::query_ist();
    vrs::set_version();
    a20::enable_a20();
    vid::set_screen_info();
}
