use plankton::ios::{io_delay, outb};
use plankton::{print, println};

#[repr(C, packed)]
struct DescoritorTable {
    limit: u16,
    base: u32,
}

#[no_mangle]
static GDT: [u64; 4] = [
    0x0000000000000000,
    0x0000000000000000,
    0x00CF9A000000FFFF,
    0x00CF92000000FFFF,
];

#[no_mangle]
static IDTR: DescoritorTable = DescoritorTable { limit: 0, base: 0 };
#[no_mangle]
static GDTR: DescoritorTable = DescoritorTable {
    limit: 0x800,
    base: 0,
};

pub fn move_to_protect(entry_addr: u32, prot_stack: u32) -> ! {
    reset_coprocessor();
    mask_all_interrupts();
    setup_idt();
    setup_gdt();
    protected_mode_jump(entry_addr, prot_stack)
}

fn mask_all_interrupts() {
    outb(0xFF, 0xA1);
    io_delay();
    outb(0xFB, 0x21);
    io_delay();
}

fn reset_coprocessor() {
    outb(0, 0xF0);
    io_delay();
    outb(0, 0xF1);
    io_delay();
}

pub fn setup_gdt() {
    unsafe {
        llvm_asm!("xorl  %eax, %eax
              movw  %ds, %ax
              shll  $$4, %eax
              addl  $$GDT, %eax
              movl  %eax, (GDTR+2)
              lgdt  GDTR"
         :::"eax"
        );
    }
}

pub fn setup_idt() {
    unsafe {
        llvm_asm!("lidt IDTR":::);
    }
}

fn protected_mode_jump(entry_addr: u32, prot_stack: u32) -> ! {
    unsafe {
        llvm_asm!("
            movl  %eax, %esp
            movl %eax, %ebp"
         :
         :"{eax}"(prot_stack)
         :
        );
        llvm_asm!("
            movl  %cr0, %eax
            orl   $$1, %eax
            movl  %eax, %cr0
            jmp   flushing
         flushing:
            movl  %ebx, (jmp_offset)
            movw  $$0x18, %ax
            movw  %ax, %ds
            movw  %ax, %es
            movw  %ax, %fs
            movw  %ax, %gs
            movw  %ax, %ss"
         :
         : "{ebx}"(entry_addr)
         : "eax"
        );
        llvm_asm!(".byte 0x66
              .byte 0xEA
             jmp_offset:   .long 0
              .word 0x10"
         :::
        );
    }
    loop {}
}

