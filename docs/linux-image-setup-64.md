# Creating Custom Linux Images and Booting
Create custom Linux images and boot it with KRaBs.
This example needs to be run on Linux.

1. Build a vmlinux
3. Create a initramfs
2. Create a Disk Image
4. Boot the custom linux image with KRaBs

## Build a minimal vmlinux
1. Get the Linux source code:
   ```shell
   wget https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.8.3.tar.xz
   tar xf linux-5.8.3.tar.xz
   cd linux-5.8.3
   ```
2. Configure your Linux build according the following:
   ```shell
   make allnoconfig
   make menuconfig
   ```
   ```
   64-bit kernel ---> yes
   General setup ---> Initial RAM filesystem and RAM disk (initramfs/initrd) support ---> yes
   General setup ---> Configure standard kernel features ---> Enable support for printk ---> yes
   Executable file formats / Emulations ---> Kernel support for ELF binaries ---> yes
   Executable file formats / Emulations ---> Kernel support for scripts starting with #! ---> yes
   Enable the block layer ---> yes
   Device Drivers ---> Generic Driver Options ---> Maintain a devtmpfs filesystem to mount at /dev ---> yes
   Device Drivers ---> Generic Driver Options ---> Automount devtmpfs at /dev, after the kernel mounted the rootfs ---> yes
   Device Drivers ---> Character devices ---> Enable TTY ---> yes
   Device Drivers ---> Character devices ---> Serial drivers ---> 8250/16550 and compatible serial support ---> yes
   Device Drivers ---> Character devices ---> Serial drivers ---> Console on 8250/16550 and compatible serial port ---> yes
   Device Drivers ---> Block devices ---> yes
   Device Drivers ---> PCI Support --> yes
   Device Drivers ---> Serial ATA and Parallel ATA drivers (libata) ---> yes
   Device Drivers ---> Serial ATA and Parallel ATA drivers (libata) ---> Intel ESB, ICH, PIIX3, PIIX4 PATA/SATA support ---> yes
   Device Drivers ---> Serial ATA and Parallel ATA drivers (libata) ---> Generic ATA support ---> yes   
   Device Drivers ---> SCSI device support ---> SCSI disk support
   File systems ---> The Extended 4 (ext4) filesystem ---> yes
   File systems ---> Pseudo filesystems ---> /proc file system support ---> yes
   File systems ---> Pseudo filesystems ---> sysfs file system support ---> yes
   File systems ---> DOS/FAT/EXFAT/NT Filesystems  ---> VFAT (Windows-95) fs support ---> yes
   File systems ---> DOS/FAT/EXFAT/NT Filesystems  ---> Enable FAT UTF-8 option by default ---> yes
   File systems ---> Native language support  ---> Codepage 437 (United States, Canada) ---> yes
   File systems ---> Native language support  ---> Codepage 437 (United States, Canada) ---> yes
   File systems ---> Native language support  ---> ASCII (United States) ---> yes
   File systems ---> Native language support  ---> NLS ISO 8859-1  (Latin 1; Western European Languages) ---> yes
   File systems ---> Native language support  ---> NLS UTF-8 ---> yes
   File systems ---> UTF-8 normalization and casefolding support --- yes
   ```
   Or Copy the [my recommended config](../resources/.config) to `.config`, then
   do the menuconfig for adjustments:
   ```shell
   wget https://raw.githubusercontent.com/ellbrid/krabs/master/resources/.config -O .config
   make menuconfig
   ```
3. Build the vmlinux:
   ```shell
   make vmlinux
   cp vmlinux vmlinux-5.8.3
   ```
4. You can find vmlinux under current directry as `./vmlinux-5.8.3`.

## Create a initramfs
1. Start off by creating basic `./src/initramfs` directory:
   ```shell
   cd ..
   mkdir --parents src/initramfs/{bin,dev,etc,lib,lib64,mnt/root,proc,root,sbin,sys}
   ```
2. Copy basic device nodes(null, console, tty, sda1, sda2 ...) from the root
filesystem to the initramfs example location:
   ```shell
   sudo cp --archive /dev/{null,console,tty,tty[0-4],sda,sda[1-8],mem,kmsg,random,urandom,zero} src/initramfs/dev/
   ```
3. Instead of using some core tools like sh and mount, we can get them from
busybox:
   ```shell
   curl -L 'https://www.busybox.net/downloads/binaries/1.31.0-defconfig-multiarch-musl/busybox-x86_64' > src/initramfs/bin/busybox
   sudo chmod +x src/initramfs/bin/busybox
   ./src/initramfs/bin/busybox --list | sed 's:^:src/initramfs/bin/:' | xargs -n 1 ln -s busybox
   ```
4. We'll also need an init script. example:
   ```shell
   cat >> src/initramfs/init << EOF
   #!/bin/sh

   mount -t devtmpfs  devtmpfs  /dev
   mount -t proc      proc      /proc
   mount -t sysfs     sysfs     /sys
   sleep 2
   cat <<END


   Boot took $(cut -d' ' -f1 /proc/uptime) seconds
                                                
   _____           _        __    _             
   |   __|___ ___ _| |_ _   |  |  |_|___ _ _ _ _ 
   |__   | .'|   | . | | |  |  |__| |   | | |_'_|
   |_____|__,|_|_|___|_  |  |_____|_|_|_|___|_,_|
                     |___|                       


   Welcome to Sandy Linux



   END
   exec sh
   EOF
   ```
   ```shell
   sudo chmod +x src/initramfs/init
   ```
5. Now to create the the initramfs:
   ```shell
   cd src/initramfs
   find . | cpio -o -H newc | gzip > ../../initramfs.cpio.gz
   ```

## Create a Disk Image
1. Create an image file with qemu:
   ```shell
   qemu-img create disk.img 512M
   ```
2. Partition with fdisk:
   ```shell
   gdisk disk.img 
   ...
   Creating new GPT entries in memory.

   Command (? for help): n
   Partition number (1-128, default 1): 
   First sector (34-1048542, default = 2048) or {+-}size{KMGTP}: 
   Last sector (2048-1048542, default = 1048542) or {+-}size{KMGTP}: +1M
   Current type is 8300 (Linux filesystem)
   Hex code or GUID (L to show codes, Enter = 8300): EF02
   Changed type of partition to 'BIOS boot partition'

   Command (? for help): n
   Partition number (2-128, default 2): 
   First sector (34-1048542, default = 4096) or {+-}size{KMGTP}: 
   Last sector (4096-1048542, default = 1048542) or {+-}size{KMGTP}: +50M
   Current type is 8300 (Linux filesystem)
   Hex code or GUID (L to show codes, Enter = 8300): EF00
   Changed type of partition to 'EFI system partition'

   Command (? for help): n
   Partition number (3-128, default 3): 
   First sector (34-1048542, default = 106496) or {+-}size{KMGTP}: 
   Last sector (106496-1048542, default = 1048542) or {+-}size{KMGTP}: 
   Current type is 8300 (Linux filesystem)
   Hex code or GUID (L to show codes, Enter = 8300): 
   Changed type of partition to 'Linux filesystem'

   Command (? for help): w
   ```
3. Create FAT32 on the second partition, ext4 on the 3rd partition.
   ```shell
   sudo kpartx -av disk.img 
   sudo mkfs.fat -F 32 /dev/mapper/loop0p2
   sudo mkfs.ext4 /dev/mapper/loop0p3
   ```

## Set FAT32 EFI System Partition
1. Copy vmlinux and initrd into EFI System Partition
   ```shell
   $ sudo mount /dev/mapper/loop0p2 /mnt
   $ sudo cp path/to/vmlinux-5.8.3 /mnt/
   $ sudo cp path/to/initramfs.cpio.gz /mnt/
   ```
2. Set the CONFIG.TXT. it is a simple matrix-oriented text file. It is described as follows:
   ```shell
   $ sudo vi /mnt/CONFIG.txt
   main.kernel vmlinux-5.8.3
   main.initrd initramfs.cpio.gz
   main.cmdlin clocksource=tsc
   ```
3. umount
   ```
   sudo umount /mnt
   sudo kpartx -d disk.img 
   ```

## Boot the custom linux image with KRaBs
Just run cargo; the -w option writes to disk and the -e option launches QEMU after the write.
```shell
$ pwd
path/to/krabs
$ cargo build
$ cargo run -- -w disk.img -e
```

## Working Example

![bootlin](images/2020demo.gif)

## DiskSpace layout
```
$ gdisk -l disk.img 
GPT fdisk (gdisk) version 1.0.5

Partition table scan:
  MBR: protective
  BSD: not present
  APM: not present
  GPT: present

Found valid GPT with protective MBR; using GPT.
Disk disk.img: 1048576 sectors, 512.0 MiB
Sector size (logical): 512 bytes
Disk identifier (GUID): D35B348E-F03D-4B3C-B934-BD9FA055B967
Partition table holds up to 128 entries
Main partition table begins at sector 2 and ends at sector 33
First usable sector is 34, last usable sector is 1048542
Partitions will be aligned on 2048-sector boundaries
Total free space is 2014 sectors (1007.0 KiB)

Number  Start (sector)    End (sector)  Size       Code  Name
   1            2048            4095   1024.0 KiB  EF02  BIOS boot partition
   2            4096          106495   50.0 MiB    EF00  EFI system partition
   3          106496         1048542   460.0 MiB   8300  Linux filesystem
```

