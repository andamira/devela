require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}

display_path() {
    case "$1" in
        "$PROBE_DIR"/*)
            printf '%s\n' "${1#"$PROBE_DIR"/}"
            ;;
        "$PROBES_DIR"/*)
            printf '../%s\n' "${1#"$PROBES_DIR"/}"
            ;;
        *)
            printf '%s\n' "$1"
            ;;
    esac
}

size() {
    echo "memory:"
    "$SIZE" "$ELF" |
        awk '{ sub(/[[:space:]]+[^[:space:]]+$/, ""); print }'
}
