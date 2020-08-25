# Specifications
KRaBs supports only the minimal
[x86 Linux boot protocol](https://www.kernel.org/doc/html/latest/x86/boot.html).  
Before attempting to use KRaBs for loading and booting your OS, take action as
needed to meet the following requirements:

## 32bit boot protocol 
* At entry, the CPU is in 32-bit protected mode with paging disabled.
* A GDT is loaded with the descriptors for selectors `__BOOT_CS(0x10)` and
`__BOOT_DS(0x18)`. Both descriptors is 4G flat segment. `__BOOT_CS` has
execute/read permission, and `__BOOT_DS` has read/write permission.
* `CS` is `__BOOT_CS` and `DS`, `ES`, `SS` is `__BOOT_DS`.
* Interrupt is disabled.
* `%ebp`, `%edi` and `%ebx` is zero.
* `%esi` holds the base physical address(`0x7C00`) of the
[struct boot_params](https://github.com/torvalds/linux/blob/master/arch/x86/include/uapi/asm/bootparam.h#L175). 

## 64bit boot protocol
* At entry, the CPU is in 64-bit mode with paging enabled. 
* A GDT is loaded with the descriptors for selectors `__BOOT_CS(0x10)` and
`__BOOT_DS(0x18)`. Both descriptors is 4G flat segment. `__BOOT_CS` has
execute/read permission, and `__BOOT_DS` has read/write permission.
* `CS` is `__BOOT_CS` and `DS`, `ES`, `SS` is  `__BOOT_DS`.
* Interrupt is disabled.
* `%rsi` holds the base physical address(`0x7C00`) of the
[struct boot_params](https://github.com/torvalds/linux/blob/master/arch/x86/include/uapi/asm/bootparam.h#L175). 
