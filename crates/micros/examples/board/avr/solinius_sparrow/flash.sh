#!/bin/sh
#
# AVR flashing configuration for the Solinius Sparrow.

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"

PART="${PART:-atmega328p}"
PROGRAMMER="${PROGRAMMER:-arduino}"

PORT="${PORT:-/dev/ttyUSB0}"
UPLOAD_BAUD="${UPLOAD_BAUD:-115200}"
NO_ERASE="${NO_ERASE:-1}"

export EXAMPLE_DIR DEFAULT_NAME PART PROGRAMMER
export PORT UPLOAD_BAUD NO_ERASE

exec "$DIR/../../../tools/avr-flash.sh" "$@"
