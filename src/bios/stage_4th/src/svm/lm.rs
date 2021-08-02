use core::hint::unreachable_unchecked;

#[repr(C, packed)]
struct DescoritorTable {
    limit: u16,
    base: u32,
}

#[no_mangle]
static GDT: [u64; 6] = [
    0x0000000000000000,
    0x0000000000000000,
    0x00AF9A000000FFFF, // __KERNEL_CS
    0x00CF92000000FFFF, // __KERNEL_DS
    0x0080890000000000, // TS descripter
    0x0000000000000000, // TS continued
];

#[no_mangle]
static mut GDTR: DescoritorTable = DescoritorTable {
    limit: 0x800,
    base: 0,
};

fn setup_gdt() {
    unsafe {
        llvm_asm!("movl  $$GDT, %eax
              movl  %eax, (GDTR+2)
              lgdt  GDTR"
         ::: "eax"
        );
    }
}

fn enable_pae() {
    unsafe {
        // Enable PAE
        llvm_asm!("movl  %cr4, %eax
              orl   $$0x20, %eax
              movl  %eax, %cr4"
         ::: "eax"
        );
    }
}

fn setup_page_tables() {
    use plankton::layout::PGTABLE_START;
    use plankton::mem::MemoryRegion;
    let mut pg_table = MemoryRegion::new(PGTABLE_START, 8 * 6 * 512);
    pg_table
        .as_mut_slice::<u64>(0x0000, 6 * 512)
        .copy_from_slice(&[0; 6 * 512]);

    // Build Level 4
    let level4 = pg_table.as_mut_slice::<u64>(0x0000, 512);
    for i in 0..1 {
        level4[i] = (PGTABLE_START + 0x1000) | 0x7;
    }

    // Build Level 3
    let level3 = pg_table.as_mut_slice::<u64>(0x1000, 512);
    for i in 0..4 {
        level3[i] = (PGTABLE_START + 0x2000 + 0x1000 * (i as u64)) | 0x7;
    }

    // Build Level 2
    let level2 = pg_table.as_mut_slice::<u64>(0x2000, 4 * 512);
    for i in 0..2048 {
        level2[i] = (0x00200000 * (i as u64)) | 0x00000183;
    }
}

fn enable_paging() {
    unsafe {
        // Enable the boot page tables
        llvm_asm!("movl  %eax, %cr3"
         :: "{eax}"(plankton::layout::PGTABLE_START)
        );

        // Enable Long mode in EFER (Extended Feature Enable Register)
        llvm_asm!("movl  $$0xC0000080, %ecx
              rdmsr
              btsl  $$8, %eax
              wrmsr"
         ::: "eax", "ecx"
        );
    }
}

fn jmp64(entry_addr: u64) -> ! {
    unsafe {
        llvm_asm!("pushl $$0x10
              pushl %eax
              movl  %ebx, %eax
              movl  %eax, %cr0
              lret"
         :
         : "{eax}"(entry_addr), "{ebx}"((1<<31)|(1<<0)) "{esi}"(0x7C00)
         :
        );
        unreachable_unchecked();
    }
}

pub fn start_kernel(entry_addr: u64) -> ! {
    setup_gdt();
    enable_pae();
    setup_page_tables();
    enable_paging();
    jmp64(entry_addr);
}
