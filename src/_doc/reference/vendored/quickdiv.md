This is derived work from the
[`quickdiv`](https://crates.io/crates/quickdiv/0.1.1) crate,
including the following modifications:

- convert into a type defined on-demand via macro, with the option of having
  all implementation over primitives over a single const-generic type.
- constructors return `Option` instead of panicking.
- misc. refactoring and code compression.
