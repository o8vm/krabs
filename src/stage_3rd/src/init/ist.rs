use plankton::mem::MemoryRegion;
pub fn query_ist() {
    let zero_page = MemoryRegion::new(0x000, 4096);
    let signature: u32;
    let command: u32;
    let event: u32;
    let perf_level: u32;

    unsafe {
        asm!("int $$0x15"
         : "={eax}"(signature), "={ebx}"(command), "={ecx}"(event), "={edx}"(perf_level)
         : "{ax}"(0xe980), "{edx}"(0x47534943)
        );
    }
    // 0x60, 0x64, 0x68, 0x6c
    zero_page.write_u32(0x60, signature);
    zero_page.write_u32(0x64, command);
    zero_page.write_u32(0x68, event);
    zero_page.write_u32(0x6c, perf_level);
}
