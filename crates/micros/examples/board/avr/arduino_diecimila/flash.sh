#!/bin/sh
#
# AVR flashing configuration for the Arduino Diecimila.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"

PART="${PART:-atmega168}"
PROGRAMMER="${PROGRAMMER:-arduino}"

PORT="${PORT:-/dev/ttyUSB0}"
UPLOAD_BAUD="${UPLOAD_BAUD:-19200}"
NO_ERASE="${NO_ERASE:-1}"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME PART PROGRAMMER
export PORT UPLOAD_BAUD NO_ERASE

exec "$DIR/../../../tools/avr-flash.sh" "$@"
