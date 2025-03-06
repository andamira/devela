This is derived work from the
[`jiff`](https://crates.io/crates/jiff/0.2.1) crate,
including the following modifications:

- rename `SignedDuration` to `TimeDelta`.
- new methods:
  - `neg_abs`.
- rename methods:
  - `unsigned_abs` to `abs_duration`.
  - `div_duration_f*` to `div_delta_f*`.
- use eucludean division for conversions.
- remove `jiff`-specific functionality.
- feature-gate float-specific methods.
- update docs, remove examples.
- remove `inline` attributes.
- made more methods *const*.
- add extra conversions.
- misc. refactors.
