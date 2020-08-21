//use plankton::{print, println};
use stage_3rd::{init::cur::{re_cur, set_cur}, ParamTable};
use plankton::dev::read_to_trackbuf;


#[link_section = ".param"]
#[no_mangle]
pub static mut PARAM: ParamTable = ParamTable {
    start_lba: 0, 
    sectors: 0,
    ret_addr: 0,
    prot_stack: 0,
};

#[link_section = ".second"]
#[no_mangle]
fn read() -> ! {
    unsafe {
        llvm_asm!(
            "
            movl %cr0, %eax
            andl $$0x7FFFFFFE, %eax
            movl %eax, %cr0
            ljmp $$0x07C0, $$reentry
           reentry:
            cli
            movw $$0x07C0, %ax
            movw %ax, %ds
            movw %ax, %es
            movw %ax, %fs
            movw %ax, %gs
            movw %ax, %ss
            movl $$0xFFE0, %esp
            movl $$0xFFE0, %ebp
            sti"
        :::"eax");
    }
    let start_lba: u32;
    let sectors: u32;
    let ret_addr: u32;
    let prot_stack: u32;
    unsafe {
        start_lba = PARAM.start_lba;
        sectors = PARAM.sectors;
        ret_addr = PARAM.ret_addr;
        prot_stack = PARAM.prot_stack;
    }
    re_cur();
    //println!("  {:X} {:X} {:X} {:X}", start_lba, sectors, ret_addr, prot_stack);
    match read_to_trackbuf(sectors as u16, start_lba as u64) {
        Ok(_) => unsafe { PARAM.start_lba = true as u32 },
        Err(_) => unsafe { PARAM.start_lba = false as u32 },
    }
    //println!("done!");
    set_cur();
    stage_3rd::mpm::move_to_protect(ret_addr, prot_stack)
}
