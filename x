#!/usr/bin/env bash
# devela/x
#
# Devela workspace command wrapper
#
# Supports:
#   +toolchain          (e.g. +nightly, +1.94.0)
#   --native            enable CPU optimizations
#   --cfg a,b,c         pass cfg flags
#
set -euo pipefail

#------------------------------------------------------------------------------
# Parse arguments
#------------------------------------------------------------------------------

toolchain=""
native=0
cfg_flags=()

normalize_cfg() {
    local cfg="$1"
    local name value

    # trim surrounding whitespace
    cfg="${cfg#"${cfg%%[![:space:]]*}"}"
    cfg="${cfg%"${cfg##*[![:space:]]}"}"
    [[ -n "$cfg" ]] || return 0

    if [[ "$cfg" == *=* ]]; then
        name="${cfg%%=*}"
        value="${cfg#*=}"

        # Already a Rust string literal: name="value"
        if [[ "${value:0:1}" == '"' && "${value: -1}" == '"' ]]; then
            printf '%s\n' "$cfg"
            return 0
        fi
        # Tolerate literal single quotes if they somehow survive shell parsing.
        if [[ "${value:0:1}" == "'" && "${value: -1}" == "'" ]]; then
            value="${value:1:${#value}-2}"
        fi
        # Escape for Rust string literal.
        value="${value//\\/\\\\}"
        value="${value//\"/\\\"}"

        printf '%s="%s"\n' "$name" "$value"
    else
        printf '%s\n' "$cfg"
    fi
}
push_cfg_list() {
    local raw="$1"
    local list c
    IFS=',' read -ra list <<< "$raw"
    for c in "${list[@]}"; do
        c="$(normalize_cfg "$c")"
        [[ -n "$c" ]] && cfg_flags+=("$c")
    done
}

while [[ $# -gt 0 ]]; do
    case "$1" in

        +*)
            toolchain="$1"
            shift
            ;;

        --native)
            native=1
            shift
            ;;

		--cfg)
            shift
            [[ $# -gt 0 ]] || {
                echo "error: --cfg requires an argument" >&2
                exit 2
            }
            push_cfg_list "$1"
            shift
            ;;

        --cfg=*)
            push_cfg_list "${1#--cfg=}"
            shift
            ;;

        *)
            break
            ;;
    esac
done

#------------------------------------------------------------------------------
# Load environment configuration
#------------------------------------------------------------------------------

x_script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
crate_cargo_toml=$(cargo locate-project --message-format plain 2>/dev/null)
crate_dir=$(dirname "$crate_cargo_toml")
if [[ "${X_DEBUG:-0}" == 1 ]]; then
	echo "workspace dir: $x_script_dir";
	echo "crate dir:     $crate_dir";
	echo "current dir:   $(pwd)";
fi
workspace="$(cd -- "$x_script_dir" && pwd)"

source "$workspace/tools/x-env-common.sh"

if [[ "$toolchain" == "+nightly" ]]; then
    source "$workspace/tools/x-env-nightly.sh"
fi

if [[ "$native" -eq 1 ]]; then
    source "$workspace/tools/x-env-native.sh"
fi

if [[ -f "$workspace/tools/x-env-local.sh" ]]; then
    source "$workspace/tools/x-env-local.sh"
fi

if [[ "$toolchain" == "+nightly" && -f "$workspace/tools/x-env-local-nightly.sh" ]]; then
    source "$workspace/tools/x-env-local-nightly.sh"
fi

#------------------------------------------------------------------------------
# Apply cfg flags
#------------------------------------------------------------------------------

for cfg in "${cfg_flags[@]}"; do
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--cfg $cfg"
    RUSTDOCFLAGS="${RUSTDOCFLAGS:+$RUSTDOCFLAGS }--cfg $cfg"
done

export RUSTFLAGS
export RUSTDOCFLAGS

#------------------------------------------------------------------------------
# Debug
#------------------------------------------------------------------------------

if [[ "${X_DEBUG:-0}" == 1 ]]; then
    echo "toolchain:     ${toolchain:-stable}"
    echo "cargo-native:  $native"
    echo "custom flags:  ${cfg_flags[*]}"
    echo "RUSTFLAGS=\"$RUSTFLAGS\""
    echo "RUSTDOCFLAGS=\"$RUSTDOCFLAGS\""
fi

#------------------------------------------------------------------------------
# Execute cargo
#------------------------------------------------------------------------------

exec cargo ${toolchain:+$toolchain} "$@"
