# Devela project launcher integration for sh/bash/zsh
#
# Install:
#   source /path/to/tools/x.sh
#
# Purpose:
#   Makes the `x` command project-aware.
#   If the current cargo workspace contains an `x` script in its root,
#   that script is executed. Otherwise it falls back to `cargo`.
#
# Example usage:
#
#   x build
#   x +nightly build
#
# behave like project commands while remaining compatible with normal cargo.

x() {
    manifest="$(command cargo locate-project --workspace --message-format plain 2>/dev/null)"

    if [ -n "$manifest" ]; then
        root="$(dirname "$manifest")"

        if [ -x "$root/x" ]; then
            "$root/x" "$@"
            return
        fi
    fi

    command cargo "$@"
}
