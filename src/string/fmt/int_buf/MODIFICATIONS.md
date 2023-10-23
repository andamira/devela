This is derived work from the
[`itoa`](https://crates.io/crates/itoa/1.0.9) crate,
including the following modifications:

- renamed `Integer` to [`IntBufAble`].
  - implemented also for `NonZero*` and `NonSpecific*` types.
- renamed `Buffer` to [`IntBuf`].
	- divided fn `write` into `write_bytes` and `write_str`.
	- renamed fn `format` to `to_str`.
	- created fn `to_bytes`.
- updated docs and examples.
- removed `no_panic`.
- refactorized.
