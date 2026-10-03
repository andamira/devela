#!/bin/sh
#
# ZX Spectrum 48K example configuration.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="screen"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME

exec "$DIR/../../../tools/spectrum48.sh" "$@"
