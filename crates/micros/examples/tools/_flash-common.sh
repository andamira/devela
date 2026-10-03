require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}

display_path() {
    case "$1" in
        "$DIR"/*)
            printf '%s\n' "${1#"$DIR"/}"
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

sections() {
    "$SIZE" -A "$ELF" |
        awk 'NR > 1'
}

flash_action() {
    build
    flash
}

inspect() {
    require "$NM"

    cd "$DIR"
    ELF="$(display_path "$ELF")"

    echo
    echo "sections:"
    sections

    echo "largest symbols:"
    "$NM" -S --size-sort "$ELF" | tail -n "${INSPECT_SYMBOLS:-12}"
}

dump_extra_before() {
    :
}

dump_extra_after() {
    :
}

dump_text() {
    require "$SIZE"
    require "$NM"
    require "$OBJDUMP"

    cd "$DIR"
    ELF="$(display_path "$ELF")"

    echo "ELF: $ELF"

    echo
    echo "===== SIZE ====="
    size

    echo
    echo "===== FILE ====="
    "$OBJDUMP" -f "$ELF"

    echo
    echo "===== SECTIONS ====="
    "$OBJDUMP" -h "$ELF"

    echo
    echo "===== SYMBOLS ====="
    "$NM" -n -S "$ELF"

    dump_extra_before

    echo
    echo "===== DISASSEMBLY ====="
    "$OBJDUMP" -d "$ELF"

    dump_extra_after
}

dump() {
    if [ -t 1 ] && [ -n "${EDITOR:-}" ]; then
        DUMP="$TARGET_DIR/$TARGET/release/$NAME.dump.txt"
        dump_text > "$DUMP"

        echo "dump: $DUMP"
        "$EDITOR" "$DUMP"
    else
        dump_text
    fi
}

usage() {
    echo "usage: ${SCRIPT_NAME:-${0##*/}} {build|flash|inspect|dump} [binary]"
}

dispatch() {
    case "$ACTION" in
        build)
            build
            ;;
        flash)
            flash_action
            ;;
        inspect)
            build
            inspect
            ;;
        dump)
            build
            dump
            ;;
        ""|-h|--help|help)
            usage
            ;;
        *)
            echo "error: unknown action: $ACTION" >&2
            usage >&2
            exit 2
            ;;
    esac
}
