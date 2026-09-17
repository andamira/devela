This is derived work from the
[`crunchy`](https://crates.io/crates/crunchy/0.2.2) crate,
including the following modifications:

- use `wrapping_sub` to avoid overflow check.
- integrate with the codegen build system.
- make 64 the default recursion limit.
- update docs and comments.
- misc. refactors.
