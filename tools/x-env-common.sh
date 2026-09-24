# NOTE: Avoid generic RUSTFLAGS here: they mask [target.*].rustflags.
# See tools/docs/rustflags.md.

RUSTDOCFLAGS=""

DOC_HEADER="$crate_dir/src/_doc/header.html"

# Invoking `x` from the workspace root resolves `crate_dir` to the workspace,
# so fall back to devela's canonical header.
if [[ ! -f "$DOC_HEADER" ]]; then
    DOC_HEADER="$workspace/crates/devela/src/_doc/header.html"
fi

if [[ -f "$DOC_HEADER" ]]; then
    RUSTDOCFLAGS+=" --html-in-header $DOC_HEADER"
fi

export RUSTDOCFLAGS
