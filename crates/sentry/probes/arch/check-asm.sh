#!/bin/sh
set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
LIST="$(mktemp)"

trap 'rm -f "$LIST"' EXIT HUP INT TERM

cd "$DIR"

INSPECT_OUTPUT_LIST="$LIST" ../inspect.sh --lib "$@"

echo
echo "== arch instructions =="

while IFS= read -r asm; do
    [ -f "$asm" ] || continue

    echo
    echo "---- ${asm##*/} ----"

    grep -n -A25 -E \
        'probe_(nop|relax|nop_loop)' \
        "$asm" || true
done < "$LIST"
