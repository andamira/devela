# devela/tools/x-env-nightly.sh
#
# WAIT: https://github.com/rust-lang/cargo/issues/331#issuecomment-1081704033
# "-Zbuild-analysis", # MAYBE

RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-Zunstable-options --cfg nightly_doc"

# WAIT:BUGFIX https://github.com/rust-lang/rust/issues/149089
# RUSTDOCFLAGS="${RUSTDOCFLAGS:+$RUSTDOCFLAGS }-Zunstable-options --cfg nightly_doc --generate-link-to-definition"
RUSTDOCFLAGS="${RUSTDOCFLAGS:+$RUSTDOCFLAGS }-Zunstable-options --cfg nightly_doc"

export RUSTFLAGS
export RUSTDOCFLAGS
