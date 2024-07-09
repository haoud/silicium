#!/bin/sh
die() {
    echo "error: $@" >&2
    exit 1
}

[ -e ./README.md ]   \
    || die "you must run this script from the root of the repository"

# Depending on the architecture, we shoud use different QEMU
# options. For example, the ARM architecture requires to specify
# the machine type and the CPU model.
case $1 in
  i686)
    qemu-system-i386              \
      -device isa-debug-exit      \
      -cdrom bin/silicium.iso     \
      -rtc base=localtime         \
      -serial stdio               \
      -cpu max                    \
      -smp 2                      \
      -m 128                      \
    ;;
  x86_64)
    qemu-system-x86_64            \
      -display gtk,gl=on          \
      -cdrom bin/silicium.iso     \
      -rtc base=localtime         \
      -serial stdio               \
      -vga virtio                 \
      -cpu max                    \
      -smp 2                      \
      -m 128                      \
    ;;
  aarch64)
    qemu-system-aarch64           \
      -cdrom bin/silicium.iso     \
      -device ramfb               \
      -serial stdio               \
      -cpu cortex-a53             \
      -M virt
    ;;
  *)
    die "unsupported architecture: $1"
    ;;
esac

code=$?
if [ $code -eq 3 ]; then
    exit 0
else
    exit $code
fi

