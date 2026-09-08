This is derived work from the
[`bytehound-preload`](https://github.com/koute/bytehound/blob/77ea03c7ed90ad4f176c316cd837a77bc09aa6f3/preload/src/spin_lock.rs) project,
including the following modifications:

- add const generics: `SPIN`, `YIELD`, `SLEEP` for adaptive backoff strategy.
- remove methods: - `SpinLock`: `unsafe_as_ref`, `SpinLockguard`: `as_ref`, `unwrap`.
- rename `force_unlock` to `debug_force_unlock` and make it for *debug* only.
- add `SpinLock` methods: `is_locked`, `into_inner`, `try_into_inner`.
- custom implement `Debug` for both items.
- derive `Default` for `SpinLock`.
- make `SpinLock` fields private.
- make `no_std` compatible.
- improve documentation.
- add doc examples.
- misc. refactors.
