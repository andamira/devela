#!/bin/sh
#
# CH32 flashing configuration for the CH32V003.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="minimal"
TARGET="${TARGET:-riscv32e-unknown-none-elf}"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME TARGET

exec "$DIR/../../../tools/ch32v003-flash.sh" "$@"
