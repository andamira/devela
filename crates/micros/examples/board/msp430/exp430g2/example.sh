#!/bin/sh
#
# MSP-EXP430G2 LaunchPad build and flashing configuration.

set -eu

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"
MSP430_CHIP="${MSP430_CHIP:-g2553}"

case "$MSP430_CHIP" in
    g2231|g2452|g2553) ;;
    *) echo "error: unsupported MSP430_CHIP: $MSP430_CHIP" >&2; exit 2 ;;
esac

# Keep flash images for different MCUs isolated.
TARGET_DIR="$DIR/target/$MSP430_CHIP"
TARGET="msp430-none-elf"
FLASHER_DRIVER="${FLASHER_DRIVER:-rf2500}"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME MSP430_CHIP TARGET_DIR TARGET FLASHER_DRIVER

exec "$DIR/../../../tools/msp430.sh" "$@"
