This is derived work from the
[`numtoa`](https://crates.io/crates/numtoa/0.2.4) crate,
including the following modifications:

- renamed `NumToA` trait to `NumToStr`.
- renamed methods: `numtoa` to `to_bytes_base` and `numtoa_str` to `to_str_base`.
- make `unsafe` use optional.
- refactorized.
