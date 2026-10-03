#!/bin/sh
#
# AVR flashing configuration for the ATtiny4.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="minimal"

PART="${PART:-t4}"
PROGRAMMER="${PROGRAMMER:-usbasp}"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME PART PROGRAMMER

exec "$DIR/../../../tools/avr-flash.sh" "$@"
