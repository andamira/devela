#!/bin/sh

set -eu

TARGET="${TARGET:-arm64-v8a}"
API="${API:-21}"
ACTION="${1:-run}"

case "$ACTION" in
    check)
        cargo ndk -t "$TARGET" -P "$API" check
        ;;
    build)
        cargo ndk -t "$TARGET" -P "$API" build
        ;;
    run)
        cargo ndk -t "$TARGET" -P "$API" run
        ;;
    *)
        echo "usage: $0 [check|build|run]" >&2
        exit 2
        ;;
esac
