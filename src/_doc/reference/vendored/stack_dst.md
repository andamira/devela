This is derived work from the
[`stack_dst`](https://crates.io/crates/stack_dst/0.8.1) crate,
including the following modifications:

- removed `array_buf` macro and multiple aliases.
- removed the dependency on `generic-array` and by extension on `typenum`.
- removed the features: `unsize`, `const_generics`, `full_const_generics`.
- renamed `Value*` to `DstValue*`, `Fifo*` to `DstQueue*`, `Stack*` to `DstStack*`.
- renamed `ConstArrayBuf` to `DstArray` and reimplemented as an `Array` wrapper.
- replaced custom `Pod` trait for crate's `MemPod` trait.
- changed using a raw pointer to `usize` for alignment.
- refactored and updated the code to 2021 edition.
