# KRaBs Design

## Scope

### What is KRaBs
KRaBs is working on booting vmlinux and other kernels formatted in ELF on
32-bit/64-bit PCs and is under the development. Krabs also aims to support only the minimal Linux x86/x86_64 boot protocol. 
This allows you to specify the kernel command line and initrd/initramfs.  

### Features
1. Supports legacy BIOS.
2. Supported media are HDD and SSD which have GPT.
3. GPT must have BIOS Boot Partition and EFI System Partition.
4. Supports 32bit protected mode and 64bit long mode. 
5. Supports OS kernel formatted in ELF32/ELF64.
6. Supports minimal
[x86/x86_64 linux boot protocol](https://www.kernel.org/doc/html/latest/x86/boot.html). 
7. KRaBs interprets the FAT32 file system and is set by CONFIG.TXT on the that file system.
8. KRaBs can load modules such as initramsfs/initrd according to linux boot protocol.
9. KRaBs can transmit kernel command line to the kernel according to linux boot protocol.

### Specifications
KRaBs's technical specifications are available in
[the Specifications document](specifications.md).
KRaBs supports only the minimal
[x86 Linux boot protocol](https://www.kernel.org/doc/html/latest/x86/boot.html).
So your OS needs to use this as well.  
Read more about it in [the Specification document](specifications.md).

### CONFIG.TXT formats
simple matrix-oriented text file like this:
```
main.kernel sample-kernel
main.initrd sample-initrd
main.cmdlin sample command line clocksource=tsc net.ifnames=0
```

## How KRaBs works
The minimum requirement for booting an ELF-formatted kernel is that the kernel
image must be parsed and loaded to the address specified in the program header.
In this project, the following four types of initialization processing are
performed.

**Hardware initialization:**
* Setting the keyboard repeat rate.
* Disable interrupts and mask all interrupt levels.
* Setting Interrupt descriptor (IDT) and segment descriptor (GDT). As a result,
all selectors (CS, DS, ES, FS, GS) refer to the 4 Gbyte flat linear address
space.
* Change the address bus to 32 bits (Enable A20 line).
* Transition to protected mode.
* If the target is ELF64, set the 4G boot pagetable and transition to long mode.

**Software initialization:**
* Get system memory by BIOS call.

**Information transmission to the kernel:**
* KRaBs mount the FAT32 EFI System Partition and Reading the CONFIG.TXT.
* Setting [Zero Page](https://www.kernel.org/doc/html/latest/x86/zero-page.html)
of kernel parameters and transmit it to the OS.

**Load items and Relocate the kernel:**
* Load kernel, initrd and command line according to CONFIG.TXT.
* The target is an ELF file, KRaBs do the ELF relocation.

## Structure and Overview
1. stage1  
A 446 byte program written to the boot sector. The segment registers(CS, DS, ES, SS) are set to `0x07C0`, and the stack pointer (ESP) is initialized to `0xFFF0`. After that, stage2 is loaded to address `0x07C0:0x0200`, and jumps to address `0x07C0:0x0206`. In the latter half of stage1, there is an area for storing the sector position and length (in units of 512 bytes) of the stage2 program.
2. stage2  
Load stage3 and stage4, then jump to stage3. The stage3 program is loaded at address `0x07C0:0x6000`, the stage4 is loaded at address `0x0003_0000` in the extended memory area. The file is read from the disk using a 2K byte track buffer from address `0x07C0:0xEE00`, and further transferred to an appropriate address using `INT 15h` BIOS Function `0x87h`. A mechanism similar to this function is used in stage 4. When the loading of stage3 and stage4 is completed, jump to address `0x07C0:0x6000`. 
3. stage3  
Do hardware and software initialization which need BIOS calls. After a series of initialization, empty_zero_page information is prepared in `0x07C0:0x0000` to `0x07C0:0x0FFF`. Enable the A20 line, change the address bus to 32 bits, and shift to the protect mode. Then, jump to the Stage4.
4. stage4  
Mount the FAT32 EFI System Partition. Then, read and parse the CONFIG.TXT on that partition. Load ELF kernel image, initrd, and kernel command line according to CONFIG.TXT. Drop to real mode when executing I/O. Set Command line and image informations in empty_zero_page. ELF kernel image is stored to the extended memory address `0x100000` or later, and then the ELF32/ELF64 file is parsed and loaded. If the target is ELF64, set the 4G boot pagetable and transition to long mode. Finally, jump to the entry point to launch the kernel. At this time, put the physical address (`0x00007C00`) of the empty_zero_page information prepared in the low-order memory into the `ESI` or `RSI` register.

5. plankton🦠  
library common to stage1 ~ stage4.