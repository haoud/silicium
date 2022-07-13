#!/bin/sh
TARGET=i686-elf
PREFIX=$PWD/toolchain/
export PATH="$PREFIX/bin:$PATH"

die() {
    echo "error: $@" >&2
    exit 1
}

[ -e ./LICENSE ]   \
    || die "you must run this script from the root of the repository"

echo "Building binutils..."
mkdir -p toolchain/build/binutils-2.38
cd toolchain/build/binutils-2.38
../../src/binutils-2.38/configure   \
    --target=$TARGET                \
    --prefix="$PREFIX"              \
    --with-sysroot                  \
    --disable-werror                \
    > /dev/null
make -j $(nproc) > /dev/null
make install > /dev/null

cd ../../..

echo "Building gcc..."
mkdir -p toolchain/build/gcc-12.1.0
cd toolchain/build/gcc-12.1.0
../../src/gcc-12.1.0/configure   \
    --target=$TARGET            \
    --prefix="$PREFIX"          \
    --enable-languages=c,c++    \
    --without-headers           \
    > /dev/null
make all-gcc all-target-libgcc -j $(nproc) > /dev/null
make install-gcc install-target-libgcc > /dev/null
