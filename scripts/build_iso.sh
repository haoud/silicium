#!/bin/sh
set -e
die() {
    echo "error: $@" >&2
    exit 1
}

[ -e ./README.md ]   \
    || die "You must run this script from the root of the repository"

# Verify that at least one argument was passed
[ $# -eq 1 ] || die "Usage: $0 <arch>"

# Check that limine is installed and build it if necessary
if [ ! -e bin/src/limine/limine-uefi-cd.bin ] || 
   [ ! -e bin/src/limine/limine-bios-cd.bin ] ||
   [ ! -e bin/src/limine/limine-bios.sys ]; then
    echo "Limine is not installed. Downloading and building it..."
    ./scripts/build_limine.sh
fi

mkdir -p iso/boot
mkdir -p iso/EFI/BOOT

# Copy the right limine.cfg file depending on the archirecture
case $1 in
  i686)
    cp -v boot/limine-i686.cfg iso/boot/limine.cfg
    ;;
  x86_64)
    cp -v boot/limine-x86_64.cfg iso/boot/limine.cfg
    ;;
  aarch64)
    cp -v boot/limine-aarch64.cfg iso/boot/limine.cfg
    ;;
  *)
    die "Invalid architecture: $1"
    ;;
esac

# Copy the limine bootloader inside the ISO directory
cp -v                                   \
    bin/src/limine/limine-uefi-cd.bin   \
    bin/src/limine/limine-bios-cd.bin   \
    bin/src/limine/limine-bios.sys      \
    iso/boot/
cp -v                                   \
  bin/src/limine/BOOTAA64.EFI           \
  bin/src/limine/BOOTIA32.EFI           \
  bin/src/limine/BOOTX64.EFI            \
  iso/EFI/BOOT/

# Install the kernel
cp -v kernel/target/$1/release/kernel iso/boot/silicium.elf

# Create the ISO
xorriso -as mkisofs -b boot/limine-bios-cd.bin			  \
		-no-emul-boot -boot-load-size 4 -boot-info-table 	\
		--efi-boot boot/limine-uefi-cd.bin 					      \
		-efi-boot-part --efi-boot-image  					        \
		--protective-msdos-label iso -o bin/silicium.iso

# Deploy Limine to the ISO
./bin/src/limine/limine bios-install bin/silicium.iso
