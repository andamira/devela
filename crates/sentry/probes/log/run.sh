#!/bin/sh
set -eu

case "${1:-host}" in
    host)
        cargo run --bin stderr -F std
        ;;
    android)
        TARGET="${TARGET:-arm64-v8a}"
        API="${API:-21}"

        cargo ndk \
            -t "$TARGET" \
            -P "$API" \
            run --bin android -F android
        ;;
    *)
        echo "usage: $0 [host|android]" >&2
        exit 2
        ;;
esac
