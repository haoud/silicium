#!/bin/sh
mkdir -p toolchain/hosted
mkdir -p toolchain/hosted/build
mkdir -p sysroot

TARGET=i686-silicium
PREFIX=$PWD/toolchain/
SYSROOT=$PWD/sysroot/
export PATH="$PREFIX/bin:$PATH"

die() {
    echo "error: $@" >&2
    exit 1
}

[ -e ./LICENSE ]   \
    || die "you must run this script from the root of the repository"

cd toolchain/hosted/build

# Compile musl
echo "Compiling musl..."
mkdir -p musl-1.2.3
cd musl-1.2.3
../../../src/musl-1.2.3/configure \
    --prefix=/usr \
    --disable-shared \
    --target=i686-elf \
    --with-sysroot=$SYSROOT > /dev/null
make -j $(nproc) > /dev/null
make install DESTDIR=$SYSROOT > /dev/null

# Compile binutils
echo "Compiling binutils..."
cd ..
mkdir -p binutils-2.38
cd binutils-2.38
../../../src/binutils-2.38/configure \
    --target=$TARGET \
    --prefix=$PREFIX \
    --disable-werror \
    --with-sysroot=$SYSROOT > /dev/null
make -j $(nproc) > /dev/null
make install-strip > /dev/null

# Compile gcc and libstdc++
echo "Compiling gcc and libstdc++..."
cd ..
mkdir -p gcc-12.1.0
cd gcc-12.1.0
../../../src/gcc-12.1.0/configure \
    --target=$TARGET \
    --prefix=$PREFIX \
    --with-sysroot=$SYSROOT \
    --enable-languages=c,c++ > /dev/null
make all-gcc -j $(nproc) > /dev/null
make all-target-libgcc -j $(nproc) > /dev/null
make all-target-libstdc++-v3 -j $(nproc) > /dev/null
make install-strip-gcc > /dev/null
make install-target-libgcc > /dev/null
make install-strip-target-libstdc++-v3 > /dev/null

# Cleaning build directory
cd ..
rm -rf gcc-12.1.0
rm -rf binutils-2.38
rm -rf musl-1.2.3
