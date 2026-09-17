#!/bin/sh
set -e

cc -Wall -Wextra -pedantic -std=c11 \
    -I../../pkg/c \
    -o main main.c \
    ../../libs/libdevela_bridge.a \
    -ldl -lpthread -lm

./main
