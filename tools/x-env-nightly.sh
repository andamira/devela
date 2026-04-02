# devela/tools/x-env-nightly.sh
#
# WAIT: https://github.com/rust-lang/cargo/issues/331#issuecomment-1081704033
# "-Zbuild-analysis", # MAYBE

RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-Zunstable-options --cfg nightly_doc"

RUSTDOCFLAGS="${RUSTDOCFLAGS:+$RUSTDOCFLAGS }-Zunstable-options --cfg nightly_doc --generate-link-to-definition"

export RUSTFLAGS
export RUSTDOCFLAGS
