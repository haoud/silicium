export INSTALL_DIR = $(shell pwd)/iso/boot
export INITRD_DIR = $(shell pwd)/initrd
export TOOLCHAIN = $(shell pwd)/toolchain
export PATH := $(TOOLCHAIN)/bin:$(PATH)

# Cross compiler binaries
export TARGET = i686-elf
export OBJCOPY = ${TARGET}-objcopy
export STRIP = ${TARGET}-strip
export CC = ${TARGET}-gcc
export LD = ${TARGET}-gcc
export AS = ${TARGET}-as
export NM = ${TARGET}-nm

# GCC flags
export CFLAGS += -Os -flto -march=i686 -std=gnu2x -masm=intel -D CONFIG_DEBUG 

# LD flags
export LDFLAGS = -march=i686 -flto -nostdlib

# AS flags
export ASFLAGS = -march=i686

.PHONY: all kernel install-kernel initrd make-iso lauch clean

all: kernel install-kernel initrd make-iso

run: all lauch

kernel:
	make -C kernel all

initrd:
	cd initrd && \
	tar -cvf $(INSTALL_DIR)/initrd * && \
	cd ..

install-kernel:
	make -C kernel install

make-iso:
	mkdir -p bin
	grub-mkrescue -o bin/silicium.iso iso

lauch: lauch-bochs

lauch-bochs:
	bochs -f bochs.bxrc -q

lauch-bochs-debug:
	bin/bochs -f bochsd.bxrc -q

clean:
	make -C kernel clean
