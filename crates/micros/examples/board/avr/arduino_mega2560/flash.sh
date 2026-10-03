#!/bin/sh
#
# AVR flashing configuration for the Arduino Mega 2560.

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="blink"

PART="${PART:-atmega2560}"
PROGRAMMER="${PROGRAMMER:-wiring}"

PORT="${PORT:-/dev/ttyACM0}"
UPLOAD_BAUD="${UPLOAD_BAUD:-115200}"
NO_ERASE="${NO_ERASE:-1}"

export EXAMPLE_DIR DEFAULT_NAME PART PROGRAMMER
export PORT UPLOAD_BAUD NO_ERASE

exec "$DIR/../../../tools/avr-flash.sh" "$@"
