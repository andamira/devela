#!/bin/sh
#
# Adafruit UF2 flashing configuration for the nice!nano nRF52840.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"

TARGET="thumbv7em-none-eabihf"
FLASH_BASE="0x26000" # Adafruit UF2 + S140 v6
FLASH_END="0xEC000"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME TARGET FLASH_BASE FLASH_END

exec "$DIR/../../../tools/nrf.sh" "$@"
