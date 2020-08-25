pub fn copy_block(src: u32, dst: u32, wcount: u16) -> Result<(), &'static str> {
    let ret: u16;
    let mut gdt: [u64; 6] = [0; 6];
    gdt[2] = 0x00CF9200 << 32
        | (src as u64 & 0xFF000000) << 32
        | (src as u64 & 0x00FFFFFF) << 16
        | 0xFFFF;
    gdt[3] = 0x00CF9200 << 32
        | (dst as u64 & 0xFF000000) << 32
        | (dst as u64 & 0x00FFFFFF) << 16
        | 0xFFFF;
    unsafe {
        llvm_asm!("int $$0x15"
         : "={ax}"(ret)
         : "{ax}"(0x8700),
           "{cx}"(wcount),
           "{si}"(&gdt as *const [u64;6])
        );
    }
    if ret & 0xff00 != 0 {
        Err("Memory transfer error!")
    } else {
        Ok(())
    }
}

pub struct MemoryRegion {
    base: u64,
    length: u64,
}

impl MemoryRegion {
    pub fn new(base: u64, length: u64) -> Self {
        Self { base, length }
    }
    
    pub fn len(&self) -> u64 {
        self.length
    }

    pub fn from_slice<T>(data: &[T]) -> Self {
        Self {
            base: data.as_ptr() as u64,
            length: (data.len() * core::mem::size_of::<T>()) as u64,
        }
    }

    pub fn as_mut_slice<T>(&mut self, offset: u64, length: u64) -> &mut [T] {
        assert!((offset + (length * core::mem::size_of::<T>() as u64)) <= self.length);
        unsafe { core::slice::from_raw_parts_mut((self.base + offset) as *mut T, length as usize) }
    }

    pub fn as_slice<T>(&self, offset: u64, length: u64) -> &[T] {
        assert!((offset + (length * core::mem::size_of::<T>() as u64)) <= self.length);
        unsafe { core::slice::from_raw_parts((self.base + offset) as *const T, length as usize) }
    }

    // Read a value from a given offset
    pub fn read<T: Copy>(&self, offset: u64) -> T {
        assert!((offset + (core::mem::size_of::<T>() - 1) as u64) < self.length);
        unsafe { *((self.base + offset) as *const T) }
    }

    pub fn read_u8(&self, offset: u64) -> u8 {
        self.read(offset)
    }

    pub fn read_u16(&self, offset: u64) -> u16 {
        self.read(offset)
    }

    pub fn read_u32(&self, offset: u64) -> u32 {
        self.read(offset)
    }

    pub fn read_u64(&self, offset: u64) -> u64 {
        self.read(offset)
    }

    pub fn write<T>(&self, offset: u64, value: T) {
        assert!((offset + (core::mem::size_of::<T>() - 1) as u64) < self.length);
        unsafe {
            *((self.base + offset) as *mut T) = value;
        }
    }

    pub fn write_u8(&self, offset: u64, value: u8) {
        self.write(offset, value)
    }

    pub fn write_u16(&self, offset: u64, value: u16) {
        self.write(offset, value)
    }

    pub fn write_u32(&self, offset: u64, value: u32) {
        self.write(offset, value)
    }

    pub fn write_u64(&self, offset: u64, value: u64) {
        self.write(offset, value)
    }
}
