require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}

flash_action() {
    build
    flash
}

inspect() {
    require "$SIZE"
    require "$NM"

    echo
    echo "elf: $ELF"

    echo
    echo "sections:"
    "$SIZE" -A "$ELF"

    echo
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

    echo "ELF: $ELF"

    echo
    echo "===== SIZE ====="
    "$SIZE" -A "$ELF"

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
        *)
            echo "usage: $0 [build|flash|inspect|dump] [binary]" >&2
            exit 2
            ;;
    esac
}
