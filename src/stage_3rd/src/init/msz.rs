use plankton::mem::MemoryRegion;

const E820_MAX_ENTRIES_ZEROPAGE: usize = 128;
const SMAP: u32 = 0x534d4150;

#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
struct E820Entry {
    addr: u64,
    size: u64,
    entry_type: u32,
}

fn detect_memory_e820() {
    let mut zero_page = MemoryRegion::new(0x000, 4096);
    let e820_table = zero_page.as_mut_slice::<E820Entry>(0x2d0, E820_MAX_ENTRIES_ZEROPAGE as u64);
    let size: u32 = core::mem::size_of::<E820Entry>() as u32; // ecx
    let buf: E820Entry = Default::default();
    let refb: u16 = (&buf as *const E820Entry) as u16; // di
    let mut smap: u32 = SMAP; // edx
    let mut flag: u32;
    let mut count: usize = 0;
    let mut addr: u32 = 0x00; // ebx

    loop {
        unsafe {
            asm!("int $$0x15
                  pushfl
                  popl %ecx"
             : "={ecx}"(flag), "={eax}"(smap), "={ebx}"(addr)
             : "{ax}"(0xe820), "{ebx}"(addr), "{di}"(refb), "{ecx}"(size), "{edx}"(smap)
             : "ecx"
            );
        }
        if flag & 1 != 0 {
            break;
        }
        if smap != SMAP {
            count = 0;
            break;
        }
        e820_table[count] = buf;
        count += 1;

        if (addr != 0) && (count < E820_MAX_ENTRIES_ZEROPAGE) {
            continue;
        } else {
            break;
        }
    }
    zero_page.write_u8(0x1e8, count as u8);
}

fn detect_memory_e801() {
    let zero_page = MemoryRegion::new(0x000, 4096);
    let kb01size: u16; // AX
    let kb64size: u16; // BX
    let flag: u32; // eflag

    unsafe {
        asm!("int $$0x15
              pushfl
              popl %ecx"
         : "={ax}"(kb01size), "={bx}"(kb64size), "={ecx}"(flag)
         : "{ax}"(0xe801)
        );
    }
    if flag & 1 != 0 {
        return;
    }
    if kb01size > 15 * 1024 {
        return;
    } else if kb01size == 15 * 1024 {
        zero_page.write_u32(0x1E0, ((kb64size << 6) + kb01size) as u32);
    } else {
        zero_page.write_u32(0x1E0, kb01size as u32);
    }
}

fn detect_memory_88() {
    let zero_page = MemoryRegion::new(0x000, 4096);
    let size: u16;
    unsafe {
        asm!("int $$0x15"
         : "={ax}"(size)
         : "{ax}"(0x8800)
        );
    }
    zero_page.write_u16(0x002, size & 0xFFFF);
}

pub fn set_mem_size() {
    detect_memory_e820();

    detect_memory_e801();

    detect_memory_88();
}
