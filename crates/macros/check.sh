#!/usr/bin/env sh
#
# Verifies multi-feature compilation, test runs, and documentation build.

set -e # stops on error

MSRV="1.89.0" # in sync with .github/workflows/check.yml, Cargo.toml, README.md
RCMD="rustup -v run $MSRV"

rustup override set $MSRV

# check
cmd="$RCMD cargo c"; echo "check\n$ " $cmd; $cmd

# test
cmd="$RCMD cargo t"; echo "tests\n$" $cmd; $cmd

# docs
cmd="cargo +nightly d"; echo "docs\n$" $cmd; $cmd
