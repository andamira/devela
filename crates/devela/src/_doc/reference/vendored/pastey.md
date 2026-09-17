This is derived work from the
[`pastey`](https://crates.io/crates/pastey/0.2.2) crate,
including the following modifications:

- removed hidden `expr` and `item` proc-macros.
- centralized stream expansion and parsing helpers under `Paste`.
- centralized grammar and modifier evaluation under `PasteSegment`.
- preserved upstream-compatible behavior and tests.
- misc. refactors.
