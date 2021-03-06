#!/bin/sh
mkdir -p toolchain
mkdir -p toolchain/src

die() {
    echo "error: $@" >&2
    exit 1
}

[ -e ./LICENSE ]   \
    || die "you must run this script from the root of the repository"

cd toolchain/src
echo "Download gcc-12.1.0..."
wget -q https://mirror.ibcp.fr/pub/gnu/gcc/gcc-12.1.0/gcc-12.1.0.tar.xz
echo "Download binutils-2.38..."
wget -q https://mirror.ibcp.fr/pub/gnu/binutils/binutils-2.38.tar.xz
echo "Downlading musl-1.2.3..."
wget -q https://musl.libc.org/releases/musl-1.2.3.tar.gz

echo "Decompressing binutils-2.38.tar.xz..."
tar -xJf binutils-2.38.tar.xz
echo "Decompressing gcc-12.1.0.tar.xz..."
tar -xJf gcc-12.1.0.tar.xz
echo "Decompressing musl-1.2.3.tar.gz..."
tar -xzf musl-1.2.3.tar.gz

echo "Downloading gcc prerequisites..."
cd gcc-12.1.0
contrib/download_prerequisites

cd ..
echo "Cleaning..."
rm -f binutils-2.38.tar.xz gcc-12.1.0.tar.xz musl-1.2.3.tar.gz
