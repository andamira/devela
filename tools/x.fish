# Devela project launcher integration for fish
#
# Install:
#   cp tools/x.fish ~/.config/fish/functions/x.fish
#
# Purpose:
#   Makes the `x` command project-aware.
#   If the current cargo workspace contains an `x` script in its root,
#   that script is executed. Otherwise it falls back to `cargo`.
#
# Result:
#   x build
#   x nightly build
#
# behave like project commands while remaining compatible with normal cargo.

function x
    set manifest (command cargo locate-project --workspace --message-format plain 2>/dev/null)

    if test -n "$manifest"
        set root (dirname $manifest)

        if test -x "$root/x"
            "$root/x" $argv
            return
        end
    end

    command cargo $argv
end
