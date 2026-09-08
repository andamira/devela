This is derived work from the
[`rustc-hash`](https://crates.io/crates/rustc-hash/2.1.3) crate,
including the following modifications:

- generalize `FxHasher` as `HasherFx<T>` over `u32`, `u64`, and `usize` states.
- preserve fixed-width 32-bit and 64-bit variants alongside the native variant.
- add associated methods for hashing values and byte slices.
- rewrite byte loading and compression for const evaluation.
- make byte-slice hashing available in const contexts.
- use `HasherBuildDefault` for the default builder.
- rename items and apply miscellaneous refactors.
