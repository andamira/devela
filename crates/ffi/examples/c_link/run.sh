#!/bin/sh
set -e

cc -Wall -Wextra -pedantic -std=c11 \
    -I../include \
    -o main main.c \
    ../../libs/libdevela_ffi.a \
    -ldl -lpthread -lm

./main
