#!/bin/sh
# devela_ffi/examples/run_all.sh

set -e

cd "$(dirname "$0")"

for example in c_load c_link odin_load odin_import; do
    echo
    echo "$example:"
    (cd "$example" && ./run.sh)
done
