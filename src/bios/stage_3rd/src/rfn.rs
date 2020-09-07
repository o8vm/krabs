use stage_3rd::{init::cur::{re_cur, set_cur}, ParamTable};
use plankton::dev::read_to_trackbuf;
use plankton::{print, println};


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
            movw $$0x07C0, %ax
            movw %ax, %ds
            movw %ax, %es
            movw %ax, %fs
            movw %ax, %gs
            movw %ax, %ss
            sti"
        :::"eax");
    }
    
    let start_lba: u32;
    let sectors: u32;
    unsafe {
        start_lba = PARAM.start_lba;
        sectors = PARAM.sectors;
    }
    re_cur();
    println!("slba = {}, secs = {}", start_lba, sectors);
    match read_to_trackbuf(sectors as u16, start_lba as u64) {
        Ok(_) => unsafe { PARAM.start_lba = true as u32 },
        Err(_) => unsafe { PARAM.start_lba = false as u32 },
    }
    set_cur();



    // back to protected mode
    unsafe {
        llvm_asm!("cli");
        stage_3rd::mpm::setup_gdt();
        stage_3rd::mpm::setup_idt();
        llvm_asm!("
            movl %eax, (return_addr)
            movl %ebx, %esp"
         :
         : "{eax}"(PARAM.ret_addr), "{ebx}"(PARAM.prot_stack)
        );
        llvm_asm!("
            movl  %cr0, %eax
            orl   $$1,  %eax
            movl  %eax, %cr0"
         :
         :
         :"eax"
        );
        llvm_asm!("
            jmp   flush
         flush:
            movw  %ax, %ds
            movw  %ax, %es
            movw  %ax, %fs
            movw  %ax, %gs
            movw  %ax, %ss"
         :
         : "{eax}"(0x18)
        );
        
        llvm_asm!("
            .byte 0x66
            .byte 0xEA
         return_addr:   .long 0
            .word 0x10"
         :::
        );
    }
    loop {}
}