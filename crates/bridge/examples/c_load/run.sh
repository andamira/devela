#!/bin/sh
set -e

# cc -Wall -Wextra -pedantic -std=c11 \
cc -Wall -Wextra -std=c11 \
    -o main main.c \
    -ldl

./main
