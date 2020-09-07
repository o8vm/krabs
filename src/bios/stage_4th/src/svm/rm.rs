use crate::fs::blkdev::BIOError;
use plankton::{layout::INIT_SEG, mem::MemoryRegion};

macro_rules! scratch_push {
    () => {
        llvm_asm!("
            pushl %eax
            pushl %ecx
            pushl %edx
            pushl %edi
            pushl %esi
        ":::);
    };
}

macro_rules! scratch_pop {
    () => {
        llvm_asm!("
            popl %esi
            popl %edi
            popl %edx
            popl %ecx
            popl %eax
        ":::);
    };
}

macro_rules! preserved_push {
    () => {
        llvm_asm!("
            pushl %ebx
            pushl %ebp
        ":::);
    };
}

macro_rules! preserved_pop {
    () => {
        llvm_asm!("
            popl %ebp
            popl %ebx
        ":::);
    };
}

// real mode gdt
#[repr(C, packed)]
struct Descriptortable {
    limit: u16,
    base: u32,
}

#[no_mangle]
static GDT16: [u64; 4] = [
    0x0000000000000000,
    0x0000000000000000,
    0x00009E000000FFFF, // 16 bit real mode CS
    0x000092000000FFFF, // 16 bit real mode DS
];

#[no_mangle]
static IDTR16: Descriptortable = Descriptortable {
    limit: 0x400,
    base: 0,
};
#[no_mangle]
static GDTR16: Descriptortable = Descriptortable {
    limit: 0x1f,
    base: 0,
};

#[inline(never)]
#[no_mangle]
pub fn diskread(start_lba: u32, sectors: u32) -> Result<(), BIOError> {
    let mut param_region = MemoryRegion::new(((INIT_SEG << 4) + 0x6100) as u64, 16);
    let param = param_region.as_mut_slice::<u32>(0, 4);
    // save start_lba and sectors
    param[0] = start_lba;
    param[1] = sectors;
//loop {}
    unsafe {
        llvm_asm!("start:");
        scratch_push!();
        preserved_push!();
        
        llvm_asm!("cli");

        // save return address and stack pointer
        llvm_asm!("movl $$continue, %eax":"={eax}"(param[2]));
        llvm_asm!("movl %esp, %eax":"={eax}"(param[3]));
        
        // setup gdt & idt
        llvm_asm!("
            movl  $$GDT16, %eax
            movl  %eax, (GDTR16+2)
            lgdt  GDTR16
            lidt IDTR16"
         :::
        );
        // set up new stack for real mode 0x07C0:0xFFF0
        llvm_asm!("
            movl $$0xFFF0, %eax
            movl %eax, %esp
            movl %eax, %ebp"
         :::"eax"
        );
        // setup segment
        llvm_asm!("
            movw  $$0x18, %ax
            movw  %ax, %ds
            movw  %ax, %es
            movw  %ax, %fs
            movw  %ax, %gs
            movw  %ax, %ss"
        );

        // goto real mode
        llvm_asm!("jmp  $$0x10, $$0xDD10");

        // return point
        llvm_asm!("
          continue:
            sti"
        );
        preserved_pop!();
        scratch_pop!();
        llvm_asm!("end:")
    }
    //loop {}
    if param[0] == 1 {
        Ok(())
    } else {
        Err(BIOError::IOError)
    }
}
