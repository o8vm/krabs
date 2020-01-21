pub mod a20;
pub mod cmd;
pub mod img;
pub mod ist;
pub mod kbd;
pub mod msz;
pub mod vid;
pub mod zero;

pub fn setup(kernel_size: u16, initrd_size: u16, cmd_line: &[u8]) {
    zero::clear_bss();
    zero::Pages::FirstHalf.clear();
    cmd::set_cmdline(cmd_line);
    zero::Pages::SecondHalf.clear();
    msz::set_mem_size();
    kbd::set_keyboard();
    ist::query_ist();
    img::set_image(kernel_size as u32 * 512, initrd_size as u32 * 512);
    a20::enable_a20();
    vid::set_screen_info();
}
