#!/bin/sh
#
# MSP430 flashing configuration for the MSP430G2553.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"

TARGET="${TARGET:-msp430-none-elf}"
FLASHER_DRIVER="${FLASHER_DRIVER:-rf2500}"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME TARGET FLASHER_DRIVER

exec "$DIR/../../../tools/msp430.sh" "$@"
