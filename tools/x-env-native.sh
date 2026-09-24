# TODO: inject through composable Cargo target config instead of RUSTFLAGS;
# RUSTFLAGS masks .cargo/config.toml [target.*].rustflags.

RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=native"
RUSTDOCFLAGS="${RUSTDOCFLAGS:+$RUSTDOCFLAGS }-C target-cpu=native"

export RUSTFLAGS
export RUSTDOCFLAGS
