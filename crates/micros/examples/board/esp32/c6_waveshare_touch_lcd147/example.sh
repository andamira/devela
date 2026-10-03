#!/bin/sh
#
# ESP32 flashing configuration for the Waveshare ESP32-C6 Touch LCD 1.47.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"

CHIP="${CHIP:-ESP32-C6}"
TARGET="${TARGET:-riscv32imac-unknown-none-elf}"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME CHIP TARGET

exec "$DIR/../../../tools/esp32.sh" "$@"
