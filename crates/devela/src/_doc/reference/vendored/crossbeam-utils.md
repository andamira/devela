This is derived work from the `CachePadded` struct in the
[crossbeam-utils](https://crates.io/crates/crossbeam-utils/0.8.21) crate,
including the following modifications:

- rename to `CacheAlign`.
- add missing attributes.
- add fn `into_inner_copy`.
- add associated const `ALIGN`.
- feature-bound `unsafe` uses.
- misc. refactors.
