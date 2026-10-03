#!/bin/sh
#
# ESP32-S3 flashing configuration for the LilyGO T-Deck.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"

TARGET="${TARGET:-xtensa-esp32s3-none-elf}"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME TARGET

exec "$DIR/../../../tools/esp32-s3.sh" "$@"
