This is derived work from the
[`encode`](https://crates.io/crates/encode/0.1.2) crate,
including the following modifications:

- replace the `Encoder` trait by `IoWrite`.
- use `IoError` as the implicit error type.
- make `Encodable::encode` return the usize len.
- remove the types: `FromError`,`InsufficientSpace`.
- merge the categories of combinators and encoders.
- rename items:
  - `BE` to `EncodeBe`.
  - `LE` to `EncodeLe`.
  - `Cond` to `EncodeIf`.
  - `Flags` to `EncodeFlags`
  - `SizeEncoder` to `EncodeLen`.
  - `LengthPrefix` to `EncodeLenValue`.
- merge `Iter` and `Separated` into `EncodeJoin`, with an optional separator.
- make `EncodableSize::encoded_size` be a provided method.
- make doc tests work with `not(alloc)` as well.
- remove all unnecessary dependencies.
- refactor macros and implementations.
- explain purity as non-mandatory.
- make combinators `must_use`.
- make constructors *const*.
