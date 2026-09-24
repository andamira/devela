#
# WAIT: https://github.com/rust-lang/cargo/issues/331#issuecomment-1081704033
# "-Zbuild-analysis", # MAYBE

RUSTDOCFLAGS="${RUSTDOCFLAGS:+$RUSTDOCFLAGS }-Zunstable-options"
RUSTDOCFLAGS+=" --cfg nightly_doc --generate-link-to-definition"

export RUSTDOCFLAGS
