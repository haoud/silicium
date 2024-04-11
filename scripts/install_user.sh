#!/bin/sh
set -e
die() {
    echo "error: $@" >&2
    exit 1
}

# Create missing folder if needed
[ -d iso ] || mkdir iso
[ -d iso/boot ] || mkdir iso/boot

# Install user server and programs
./scripts/install_package.sh init
