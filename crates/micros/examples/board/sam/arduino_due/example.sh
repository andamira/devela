#!/bin/sh
#
# SAM flashing configuration for the Arduino Due.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"

TARGET="${TARGET:-thumbv7m-none-eabi}"
PORT="${PORT:-/dev/ttyACM0}"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME TARGET PORT

exec "$DIR/../../../tools/sam.sh" "$@"
