This is derived work from the `MiniAlloc` struct in the
[mini-alloc](https://crates.io/crates/mini-alloc/0.9.0) crate,
including the following modifications:

- rename `MiniAlloc` to `WasmAlloc`.
- move `make_aligned` fn to `Mem::align_down`.
- move `__heap_base` ops to `Wasm::heap_base`.
- leverage `Mem` and `Wasm` namespaces.
- update documentation.
- misc. refactors.
