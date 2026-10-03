#!/bin/sh
#
# ESP32 flashing configuration for the ESP32-C3 SuperMini OLED 0.42.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"

CHIP="${CHIP:-ESP32-C3}"
TARGET="${TARGET:-riscv32imc-unknown-none-elf}"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME CHIP TARGET

exec "$DIR/../../../tools/esp32.sh" "$@"
