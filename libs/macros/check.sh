#!/usr/bin/env sh
#
# Verifies multi-feature compilation, test runs, and documentation build.

set -e # stops on error

MSRV="1.60.0" # in sync with ./Cargo.toml & .github/workflows/check.yml
RCMD="rustup -v run $MSRV"

rustup override set $MSRV

# check
cmd="$RCMD cargo c"; echo "check\n$ " $cmd; $cmd

# test
cmd="$RCMD cargo t"; echo "tests\n$" $cmd; $cmd

# docs
cmd="cargo +nightly nd"; echo "docs\n$" $cmd; $cmd
