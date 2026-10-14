#!/bin/sh
#
# Atari 800XL example configuration.

SCRIPT_NAME="${0##*/}"
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

EXAMPLE_DIR="$DIR"
DEFAULT_NAME="atari800xl"

export EXAMPLE_DIR DEFAULT_NAME SCRIPT_NAME

exec "$DIR/../../../tools/atari8.sh" "$@"
