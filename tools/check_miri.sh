#!/usr/bin/env sh
#
# Verifies multi-feature compilation, test runs, and documentation build.

set -e # stops on error

MSRV="1.72.0" # sync with readme, Cargo.toml & .github/workflows/check.yml
RCMD="rustup -v run $MSRV"
MIRIFLAGS="-Zmiri-disable-isolation" # enables OS facilities

rustup override set $MSRV

# install additional targets
#
# (1) x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu
#
# (2) x86_64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
#
# (3) x86_64-apple-darwin
rustup target add x86_64-apple-darwin
#
# (5) Bare x86_64, softfloat, 64-bit, no_std
# https://doc.rust-lang.org/nightly/rustc/platform-support/x86_64-unknown-none.html
rustup target add x86_64-unknown-none # tier 2 (_64)
#
# (6) Linux i686, 32-bit, std, little-endian, (kernel 3.2+, glibc 2.17+)
# may need to install libc6-dev-amd64-i386-cross for testing
rustup target add i686-unknown-linux-gnu # tier 1 (_686)
#
# (7) Bare ARM64, hardfloat, 64-bit, no_std, little-endian, A64 set, (M1, M2 processors)
rustup target add aarch64-unknown-none # tier 2 (_aarch)
#
# (8) Bare ARMv7-M, 32-bit, no_std, little-endian, Thumb set, (Cortex-M processors)
rustup target add thumbv7m-none-eabi # tier 2 (_thumb)


# miri tests
(
	# std

	export MIRIFLAGS # enable OS facilities

	# cmd="cargo +nightly m"; echo "miri, std\n$" $cmd; $cmd;
	# cmd="cargo +nightly mu"; echo "miri, std, unsafe\n$" $cmd; $cmd

	# check additional targets
	cmd="cargo +nightly muT1"; echo "miri, std, unsafe\n$" $cmd; $cmd
	cmd="cargo +nightly muT2"; echo "miri, std, unsafe\n$" $cmd; $cmd
	cmd="cargo +nightly muT3"; echo "miri, std, unsafe\n$" $cmd; $cmd
	cmd="cargo +nightly muT6"; echo "miri, std, unsafe\n$" $cmd; $cmd

	# no_std

	unset MIRIFLAGS # disable OS facilities

	cmd="cargo +nightly mnu"; echo "miri, no_std, unsafe\n$" $cmd; $cmd
	cmd="cargo +nightly mnuT1"; echo "miri, std, unsafe\n$" $cmd; $cmd
	cmd="cargo +nightly mnuT2"; echo "miri, std, unsafe\n$" $cmd; $cmd
	cmd="cargo +nightly mnuT3"; echo "miri, std, unsafe\n$" $cmd; $cmd
	cmd="cargo +nightly mnuT6"; echo "miri, std, unsafe\n$" $cmd; $cmd

	# cmd="cargo +nightly mnuT5"; echo "miri no_std, no-alloc, unsafe\n$" $cmd; $cmd
	# cmd="cargo +nightly mnuT7"; echo "miri no_std, no-alloc, unsafe\n$" $cmd; $cmd
	# cmd="cargo +nightly mnuT8"; echo "miri no_std, no-alloc, unsafe\n$" $cmd; $cmd
	# cmd="cargo +nightly mnuT5"; echo "miri no_std, alloc, unsafe\n$" $cmd; $cmd
	# cmd="cargo +nightly mnuT7"; echo "miri no_std, alloc, unsafe\n$" $cmd; $cmd
	# cmd="cargo +nightly mnuT8"; echo "miri no_std, alloc, unsafe\n$" $cmd; $cmd
)
