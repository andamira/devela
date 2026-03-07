# WAIT: https://github.com/rust-lang/cargo/issues/331#issuecomment-1081704033
# "-Zbuild-analysis", # MAYBE

RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-Zunstable-options -Zthreads=6 --cfg nightly_doc"

RUSTDOCFLAGS="${RUSTDOCFLAGS:+$RUSTDOCFLAGS }-Zunstable-options --generate-link-to-definition"

export RUSTFLAGS
export RUSTDOCFLAGS
