This is derived work from the
[`object-id`](https://crates.io/crates/object-id/0.1.4) crate,
including the following modifications:

- make `no_std` compatible.
- rename from `UniqueId` to `IdPinBox`.
- make a non allocating version named `IdPin`.
- modify `Debug` impl to prefix the value with the struct name.
- update documentation.
- refactorize.
