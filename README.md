# Krabs: x86 bootloader
Krabs is an experimental x86 bootloader written in Rust.  
Krabs can boot the ELF formatted kernel which compressed with bzip2. Krabs decompresses the bz2 image and relocate the ELF image, then boot the kernel.

Some of the source code uses libbzip2 C library for decompressing, but the rest is completely Rust only.

## What is Krabs?
Krabs is working on booting vmlinux and other kernels formatted in ELF on 32-bit/64-bit PCs and is under the development.  
Krabs also aims to support only the minimal Linux boot protocol. This allows you to specify the kernel command line and manipulate the behavior of the kernel at boot time.
Another feature is that in order to save space, the ELF format kernel is compressed using bzip2 before writing and uses libbzip2 library for decompressing.  

## News
2020/02/03:
* Krabs now supports long mode. You can boot the 64bit kernel formatted in ELF64. 
* Krabs still supports ELF32 on 32bit PCs. 
* It does not need to rebuild Krabs every time, because it can tell executable file format. Once Krabs is built, it supports both 32-bit and 64-bit.

2020/02/09:
* Krabs can now let vmlinux uses initrd.
* But, no testing has been done.

## Getting Started
To get started with Krabs, build it from source.

### Requirements
You need a nightly Rust compiler and binutils. First you need to install the [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) and [cargo-binutils](https://github.com/rust-embedded/cargo-binutils):

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-xbuild 
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup component add rust-src
```

For testing, you also need the qemu and MBR disk image.  
The following is an example on macOS.

```shell
brew install qemu
qemu-img create disk.img 100M
```

```shell
$ fdisk -e disk.img
The signature for this MBR is invalid.
Would you like to initialize the partition table? [y] y
fdisk:*1> edit 1   
Partition id ('0' to disable)  [0 - FF]: [0] (? for help) 83
Do you wish to edit in CHS mode? [n] n
Partition offset [0 - 204800]: [63] 
Partition size [1 - 204737]: [204737] 10000
fdisk:*1> flag 1
Partition 1 marked active.
fdisk:*1> p
Disk: disk.img geometry: 812/4/63 [204800 sectors]
Offset: 0       Signature: 0xAA55
         Starting       Ending
 #: id  cyl  hd sec -  cyl  hd sec [     start -       size]
------------------------------------------------------------------------
*1: 83    0   1   1 - 1023 254  63 [        63 -      10000] Linux files*
 2: 00    0   0   0 -    0   0   0 [         0 -          0] unused      
 3: 00    0   0   0 -    0   0   0 [         0 -          0] unused      
 4: 00    0   0   0 -    0   0   0 [         0 -          0] unused      
fdisk:*1> quit
Writing current MBR to disk.
```

### Build
You can build Krabs as follows:

```shell
git clone https://github.com/ellbrid/krabs.git
cd krabs
./tools/build.sh -k [ELF_kernel_file] -i [initrd_file] -c "kernel command line" disk.img
```

krabs will be installed into disk.img.   
The -k, -i, and -c options are not required.

### Run
You can test it using QEMU:  

```shell
qemu-system-i386 disk.img -boot c
```

## Examples 
Build and launch a simple kernel that only displays Hello world.

### ELF32 example (protect mode)

```shell
$ pwd
path/to/krabs
$ cd eg-kernel
$ cargo xbuild --release
$ cd ..
$ ./tools/build.sh -k eg-kernel/target/i586-example_os/release/eg-kernel eg-kernel/test.img 
$ qemu-system-i386 eg-kernel/test.img -boot c
```

screenshot:

![eg-kernel](docs/images/eg-kernel.png)

### ELF64 example (long mode)

```shell
$ pwd
path/to/krabs
$ cd eg-kernel64
$ cargo xbuild --release
$ cd ..
$ ./tools/build.sh -k eg-kernel64/target/x64-example_os/release/eg-kernel eg-kernel/test.img 
$ qemu-system-x86_64 eg-kernel/test.img -boot c
```

screenshot:

![eg-kernel64](docs/images/eg-kernel64.png)

## Examples for Linux
* 64bit version: wip
* 32bit version: wip

## Contributing
Krabs welcomes all contributions.

To contribute to Krabs, check out the [getting started guide](#getting-started) and then the Krabs [contribution guidelines](CONTRIBUTING.md).

## Design
The minimum requirement for booting an ELF-format OS kernel is that the ELF-format image file must be parsed and loaded to the address specified in the program header.
In this project, the following four types of initialization processing are performed.

**Hardware initialization:**
* Setting the keyboard repeat rate.
* Disable interrupts and mask all interrupt levels.
* Setting Interrupt descriptor (IDT) and segment descriptor (GDT). As a result, all selectors (CS, DS, ES, FS, GS) refer to the 4 Gbyte flat linear address space.
* Change the address bus to 32 bits (Enable A20 line).
* Transition to protected mode.
* If the target is ELF64, set the 4G boot pagetable and transition to long mode.

**Software initialization:**
* Get system memory by BIOS call.

**Information transmission to the kernel:**
* Setting kernel parameters.(mem=, root=, etc. For details, see [kernel-parameters.txt](https://github.com/torvalds/linux/blob/master/Documentation/admin-guide/kernel-parameters.txt))

**Relocate the kernel:**
* The target is an ELF file, but Krabs uses it after bzip2 compression. Therefore, two-stage relocation is needed. One is bzip2 decompression and the other is ELF relocation.

### Structure and Overview
1. stage1  
A 446 byte program written to the boot sector. The segment registers (CS, DS, ES, SS) are set to 0x07C0, and the stack pointer (ESP) is initialized to 0xFFF0. After that, stage2 is loaded to address 0x07C0:0x0200, and jumps to address 0x07C0:0x0280. In the latter half of stage1, there is an area for storing the sector length (in units of 512 bytes) of the stage2 program.
2. stage2  
The stage3 program is loaded at address 0x07C0:0x6000, the compressed kernel image is loaded at address 0x380000 in the extended memory area, and the initrd file is loaded at 0x500000. The file is read from the disk using a 4K byte track buffer from address 0x07C0:0xEE00, and further transferred to an appropriate address using INT 15h BIOS Function 0x87h. When the loading of stage3, initrd and compressed kernel image is completed, jump to address 0x07C0:0x6000.
The kernel command line is held in the area of 122 bytes from address 0x280.
3. stage3 + stage4  
Stage3 + Stage4 is linked with the libbzip2 decompression routine. Since an external C library is used, it is necessary to support zero clear of the .bss section. After a series of hardware and software initialization, empty_zero_page information is prepared in 0x07C0:0x0000 to 0x07C0:0x0FFF together with the information written in stage2. Enable the A20 line, change the address bus to 32 bits, and shift to the protect mode. The decompression function is called, the bzip2 compressed ELF kernel image is restored to the extended memory address 0x100000 or later, and then the ELF32/ELF64 file is parsed and loaded. If the target is ELF64, set the 4G boot pagetable and transition to long mode. Finally, jump to the entry point to launch the kernel. At this time, it is necessary to set the physical address (0x00007C00) of the empty_zero_page information prepared in the low-order memory in the ESI or RSI register.
4. plankton🦠  
library common to stage1 ~ stage4.

### Disk Space Layout

![layout](docs/images/layout.png)

## How to use Krabs for your original OS
Krabs supports only the minimal [x86 Linux boot protocol](https://www.kernel.org/doc/html/latest/x86/boot.html).  
An OS that uses krabs must be developed under the following assumptions:

### 32bit boot protocol 
* At entry, the CPU is in 32-bit protected mode with paging disabled.
* A GDT is loaded with the descriptors for selectors `__BOOT_CS(0x10)` and `__BOOT_DS(0x18)`. Both descriptors is 4G flat segment. `__BOOT_CS` has execute/read permission, and `__BOOT_DS` has read/write permission.
* `CS` is `__BOOT_CS` and `DS`, `ES`, `SS` is `__BOOT_DS`.
* Interrupt is disabled.
* `%ebp`, `%edi` and `%ebx` is zero.
* `%esi` holds the base physical address(0x7C00) of the [struct boot_params](https://github.com/torvalds/linux/blob/master/arch/x86/include/uapi/asm/bootparam.h#L175). 

### 64bit boot protocol
* At entry, the CPU is in 64-bit mode with paging enabled. 
* A GDT is loaded with the descriptors for selectors `__BOOT_CS(0x10)` and `__BOOT_DS(0x18)`. Both descriptors is 4G flat segment. `__BOOT_CS` has execute/read permission, and `__BOOT_DS` has read/write permission.
* `CS` is `__BOOT_CS` and `DS`, `ES`, `SS` is  `__BOOT_DS`.
* Interrupt is disabled.
* `%rsi` holds the base physical address(0x7C00) of the [struct boot_params](https://github.com/torvalds/linux/blob/master/arch/x86/include/uapi/asm/bootparam.h#L175). 

### Constraints

* The size of vmlinux must be 52MiB or less.
* The size of initrd/initramfs must be 32MiB or less.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Contanct

You can get in touch with me in the following ways:

* Contact me on my [twitter](https://twitter.com/ellbrid). _Note: I'm on a Japan time zone._
* Open a GitHub issue in this repository.
* Email me at [ell@exoskeleton.dev](mailto:ell@exoskeleton.dev).

When communicating within the Krabs community, please mind our [code of conduct](CODE_OF_CONDUCT.md).