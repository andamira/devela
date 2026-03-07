#!/usr/bin/env bash
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
            IFS=',' read -ra list <<< "$1"
            for c in "${list[@]}"; do
                cfg_flags+=("$c")
            done
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

root="$(pwd)"

source "$root/tools/x-env-common.sh"

if [[ "$toolchain" == "+nightly" ]]; then
    source "$root/tools/x-env-nightly.sh"
fi

if [[ "$native" -eq 1 ]]; then
    source "$root/tools/x-env-native.sh"
fi

if [[ -f "$root/tools/x-env-local.sh" ]]; then
    source "$root/tools/x-env-local.sh"
fi

if [[ "$toolchain" == "+nightly" && -f "$root/tools/x-env-local-nightly.sh" ]]; then
    source "$root/tools/x-env-local-nightly.sh"
fi


#------------------------------------------------------------------------------
# Apply cfg flags
#------------------------------------------------------------------------------

for cfg in "${cfg_flags[@]}"; do
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--cfg $cfg"
done

export RUSTFLAGS


#------------------------------------------------------------------------------
# Debug
#------------------------------------------------------------------------------

if [[ "${X_DEBUG:-0}" == 1 ]]; then
    echo "toolchain: ${toolchain:-default}"
    echo "native: $native"
    echo "cfg: ${cfg_flags[*]}"
    echo "RUSTFLAGS=$RUSTFLAGS"
    echo "RUSTDOCFLAGS=$RUSTDOCFLAGS"
fi


#------------------------------------------------------------------------------
# Execute cargo
#------------------------------------------------------------------------------

exec cargo ${toolchain:+$toolchain} "$@"
