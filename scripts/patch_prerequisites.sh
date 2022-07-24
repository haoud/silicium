#!/bin/sh
die() {
    echo "error: $@" >&2
    exit 1
}

[ -e ./LICENSE ]   \
    || die "you must run this script from the root of the repository"

cp patch/binutils-2.38.patch toolchain/src/binutils-2.38.patch
cp patch/gcc-12.1.0.patch toolchain/src/gcc-12.1.0.patch
cp patch/musl-1.2.3.patch toolchain/src/musl-1.2.3.patch

cd toolchain/src
# Patch binutils, gcc and musl
patch -p0 < binutils-2.38.patch
patch -p0 < gcc-12.1.0.patch
patch -p0 < musl-1.2.3.patch

# Run autoconf where needed
cd gcc-12.1.0/libstdc++-v3
autoconf
