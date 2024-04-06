#!/bin/sh
set -e
die() {
    echo "error: $@" >&2
    exit 1
}

usage() {
    echo "Usage: $0 <package name>"
    # TODO: List available packages
    exit 1
}

# Verify that the script is being run from the root of the repository
[ -e ./README.md ]   \
    || die "You must run this script from the root of the repository"

# Verify that at least one argument was provided
[ $# -ge 1 ] || usage

# Install the package into the ISO
cp -v user/$1/target/x86_64/release/$1 iso/boot/$1.elf
