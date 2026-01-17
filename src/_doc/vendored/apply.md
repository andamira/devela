This is derived work from the
[`apply`](https://crates.io/crates/apply/0.3.0) crate,
including the following modifications:

- rename `Also` to `Hook`.
  - rename method `also` to `hook`, add a non-mutating `tap` method.
- rename `Apply` to `Morph`.
  - rename methods `apply*` to `morph*`.
- add corresponding macros `hook!` and `morph!`.
- improve docs and examples.
- misc. refactors.
