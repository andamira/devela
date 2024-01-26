This is derived work from the
[`no_std_io`](https://crates.io/crates/no_std_io/0.6.0) crate, a fork of the
[`core2`](https://crates.io/crates/core2/0.4.0) crate,
including the following modifications:

- removed `memchr` dependency.
- replace not very sound implementations using `Initializer` with safer ones.
- add safe and unsafe internal implementations of previously only unsafe ones.
- replaced extensive documentation copied from `std` by links to original docs.
- remove impls and conversions of recreated types for the replaced `std` types.
