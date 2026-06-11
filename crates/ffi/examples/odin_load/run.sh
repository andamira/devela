#!/bin/sh
set -e

odin run . \
    -out:main \
    -keep-executable \
    -extra-linker-flags:"-Wl,-rpath,$PWD/../libs"
