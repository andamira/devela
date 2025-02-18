This is derived work from the
[`encode`](https://crates.io/crates/encode/0.1.2) crate,
including the following modifications:

- define a new `Decodable` trait.
- replace the `Encoder` trait by `IoWrite`.
- use `IoError` as the implicit error type.
- make `Encodable::encode` return the usize len.
- remove the types: `FromError`,`InsufficientSpace`.
- merge the categories of combinators and encoders.
- use the same combinators for encoding and decoding.
- rename combinators:
  - `BE` to `CodecBe`.
  - `LE` to `CodecLe`.
  - `Cond` to `CodecIf`.
  - `Flags` to `CodecFlags`
  - `SizeEncoder` to `CodecLen`.
  - `LengthPrefix` to `CodecLenValue`.
- merge `Iter` and `Separated` into `CodecJoin`, with an optional separator.
- make `EncodableSize::encoded_size` be a provided method.
- make doc tests work with `not(alloc)` as well.
- encode `char` as a 4 byte unicode scalar.
- remove `Encodable` impl for `Result`.
- remove all unnecessary dependencies.
- refactor macros and implementations.
- explain purity as non-mandatory.
- make combinators `must_use`.
- make `FmtAdapter` inline.
- make constructors *const*.
