#!/bin/sh
set -e

odin run . \
    -out:main \
	-keep-executable \
    -extra-linker-flags:"-ldl -lpthread -lm"
