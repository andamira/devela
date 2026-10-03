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

dump_disassembly() {
    "$OBJDUMP" -d "$ELF"
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
    dump_disassembly

    dump_extra_after
}

dump() {
    if [ -t 1 ] && [ -n "${EDITOR:-}" ]; then
        DUMP="$TARGET_DIR/$TARGET/release/$NAME.dump.txt"
        dump_text > "$DUMP"

        echo "dump: $(display_path "$DUMP")"

        "$EDITOR" "$DUMP"
    else
        dump_text
    fi
}

usage() {
    echo "usage: ${SCRIPT_NAME:-${0##*/}} {$ACTIONS} [binary]"
}

unsupported_action() {
    echo "error: unknown action: $ACTION" >&2
    usage >&2
    exit 2
}

dispatch() {
    case "$ACTION" in
        build)
            build
            ;;
        inspect)
            build
            inspect
            ;;
        dump)
            build
            dump
            ;;
        flash)
            build
            flash_action
            ;;
        run)
            build
            run_action
            ;;
        ""|-h|--help|help)
            usage
            ;;
        *)
            unsupported_action
            ;;
    esac
}
