# devela/tools/x-env-native.sh

RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=native"
RUSTDOCFLAGS="${RUSTDOCFLAGS:+$RUSTDOCFLAGS }-C target-cpu=native"

export RUSTFLAGS
export RUSTDOCFLAGS
