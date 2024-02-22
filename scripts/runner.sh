#!/bin/sh
set -e
die() {
    echo "error: $@" >&2
    exit 1
}

# Verify that the ARCH environment variable is set
if [ -z "$ARCH" ]; then
    die "This script should not be run directly. Use the make run command instead."
fi

# Used exclusively by cargo, not intended to be run manually
# This script allow us to run our kernel with cargo run as if it was a normal binary
./scripts/build_iso.sh $ARCH

# Check the return code of the previous command. If it's 0, then the ISO was
# successfully built and we can run it. Otherwise, we exit with the same return
# code as the previous command.
if [ $? -ne 0 ]; then
    exit $?
fi

./scripts/run.sh $ARCH
