use core::hint::unreachable_unchecked;

pub fn start_kernel(entry_addr: u32) -> ! {
    unsafe {
        llvm_asm!("xorl  %ebx, %ebx
              movl  %ebx, %ebp
              movl  %ebx, %edi
              jmp *%eax"
         :
         : "{eax}"(entry_addr), "{esi}"(0x7C00)
        );
        unreachable_unchecked();
    }
}
