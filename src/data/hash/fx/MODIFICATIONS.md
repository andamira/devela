This is derived work from the
[`fxhash`](https://crates.io/crates/fxhash/0.2.1) crate,
including the following modifications:

- join all structs in a single HasherFx const-generic.
- implement the standalone functions as methods.
- make it `no_std` compatible.
- remove `byteorder` dependency.
- add const methods for hashing byte slices.
- rename the structures.
- misc. refactoring.
