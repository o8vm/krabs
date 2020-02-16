# Krabs: x86 bootloader
Krabs is an experimental x86/x86_64 bootloader written in Rust.  
Krabs can boot the ELF formatted kernel which compressed with bzip2. Krabs
decompresses the bz2 image and relocate the ELF image, then boot the kernel.

Some of the source code uses libbzip2 C library for decompressing, but the rest
is completely Rust only.

## What is Krabs?
Krabs is working on booting vmlinux and other kernels formatted in ELF on
32-bit/64-bit PCs and is under the development.  
Krabs also aims to support only the minimal Linux boot protocol. This allows you
to specify the kernel command line and manipulate the behavior of the kernel at
boot time.
Another feature is that in order to save space, the ELF format kernel is
compressed using bzip2 before use and uses libbzip2 library for decompressing.  

## News
2020/02/03:
* Krabs now supports long mode. You can boot the 64bit kernel formatted in
ELF64. 
* Krabs still supports ELF32 on 32bit PCs. 
* It does not need to rebuild Krabs every time, because it can tell executable
file format. Once Krabs is built, it supports both 32-bit and 64-bit.

2020/02/09:
* Krabs can now let vmlinux uses initrd.
* But, no testing has been done.

## Getting Started
To get started with Krabs, build it from source.

### Requirements
You need a nightly Rust compiler and binutils. First you need to install the
[cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) and
[cargo-binutils](https://github.com/rust-embedded/cargo-binutils):

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-xbuild 
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup component add rust-src
```

If you are using 64-bit Linux, you need to create a 32-bit multilib environment:

```
RHEL/CentOS:
$ sudo yum install -y glibc.i686 glibc-devel.i686 libgcc.i686
Ubuntu:
$ sudo apt install gcc-multilib -y
```

For testing, you also need the qemu and MBR disk image. Disk image should have a
bootflaged partition.
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
The `-k`, `-i`, and `-c` options are not required.

### Run
You can test it using QEMU:  

```shell
qemu-system-x86_64 --hda disk.img
```

## Examples 
Simple examples are described in [the example document](docs/example.md).  
This is also a quickstart guide and should be read.

Examples for x86-64 Linux is described in
[the docs of 'Creating Custom Linux Images and Booting'](docs/linux-image-setup-64.md).

## Contributing
Krabs welcomes all contributions.

To contribute to Krabs, check out the [getting started guide](#getting-started)
and then the Krabs [contribution guidelines](CONTRIBUTING.md).

## Design
Krabs's overall architecture is described in
[the design document](docs/design.md) and
[the specification document](docs/specifications.md).

## Features
1. Supports legacy BIOS.
2. Supported media are HDD and SSD which have MBR.
3. Supports 32bit protected mode and 64bit long mode. 
4. Supports minimal
[x86/x86_64 linux boot protocol](https://www.kernel.org/doc/html/latest/x86/boot.html).
5. Supports OS kernel formatted in ELF32/ELF64.
6. To save space, OS kernels is compressd with bzip2 before use. When loading, Krabs
unpacks it.
7. An area of ​​122 bytes is reserved for the kernel command line.
Using this area, Krabs can transmit parameters to the OS, and can manipulate the
behavior of the kernel at startup.
8. Krabs can load modules such as initramsfs/initrd according to 
[x86/x86_64 linux boot protocol](https://www.kernel.org/doc/html/latest/x86/boot.html).

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

* Contact me on my matrix chat: @ellbrid:matrix.org. 
* Contact me on my [twitter](https://twitter.com/ellbrid).
* Open a GitHub issue in this repository.
* Email me at [ell@exoskeleton.dev](mailto:ell@exoskeleton.dev).

_Note: I'm on a Japan time zone._  

When communicating within the Krabs community, please mind our
[code of conduct](CODE_OF_CONDUCT.md).