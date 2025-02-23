This is derived work from the
[`jiff`](https://crates.io/crates/jiff/0.2.1) crate,
including the following modifications:

- rename `SignedDuration` to `TimeDelta`.
- new methods:
  - `neg_abs`.
- rename methods:
  - `unsigned_abs` to `abs_duration`.
- remove `jiff`-specific functionality.
- feature-gate float-specific methods.
- update docs, remove examples.
- remove `inline` attributes.
- add extra conversions.
- misc. refactors.
