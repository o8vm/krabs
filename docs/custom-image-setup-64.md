# Creating Custom Linux Images
1. Build a vmlinux
3. Create a initramfs
2. Create a Disk Image

## Build a vmlinux
1. Get the Linux source code:
   ```
   wget https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.5.2.tar.gz
   tar xf linux-5.5.2.tar.gz 
   cd linux-5.5.2
   ```
2. Configure your Linux build:  
   First, copy the [my recommended config](../resources/.config) to `.config`, Then do the menuconfig for adjustments:
   ```
   wget https://raw.githubusercontent.com/ellbrid/krabs/master/resources/.config -O .config
   make menuconfig
   ```
3. Build the vmlinux:
   ```
   make vmlinux
   ```
4. You can find vmlinux under current directry as `./vmlinux`.

## Create a initramfs
WIP

## Create a Disk Image
Create MBR disk image with qemu & fdisk.

qemu:
```
$ qemu-img create disk.img 512M
```

fdisk:
```
$ fdisk disk.img 
Welcome to fdisk (util-linux 2.23.2).
...
Command (m for help): n
Partition type:
   p   primary (0 primary, 0 extended, 4 free)
   e   extended
Select (default p): p
Partition number (1-4, default 1): 1
First sector (2048-1048575, default 2048): 2048
Last sector, +sectors or +size{K,M,G} (2048-1048575, default 1048575): 206848
Partition 1 of type Linux and of size 100 MiB is set

Command (m for help): a
Selected partition 1

Command (m for help): n
Partition type:
   p   primary (1 primary, 0 extended, 3 free)
   e   extended
Select (default p): p
Partition number (2-4, default 2): 
First sector (206849-1048575, default 208896): 
Using default value 208896
Last sector, +sectors or +size{K,M,G} (208896-1048575, default 1048575): 
Using default value 1048575
Partition 2 of type Linux and of size 410 MiB is set

Command (m for help): p

Disk disk.img: 536 MB, 536870912 bytes, 1048576 sectors
Units = sectors of 1 * 512 = 512 bytes
Sector size (logical/physical): 512 bytes / 512 bytes
I/O size (minimum/optimal): 512 bytes / 512 bytes
Disk label type: dos
Disk identifier: 0x53e169ce

   Device Boot      Start         End      Blocks   Id  System
disk.img1   *        2048      206848      102400+  83  Linux
disk.img2          208896     1048575      419840   83  Linux

Command (m for help): w
The partition table has been altered!

Syncing disks.
```

WIP