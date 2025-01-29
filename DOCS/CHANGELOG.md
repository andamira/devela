# Changelog from 2025 on

## [0.23.0-wip] *unreleased*

### Added
- new features: `_maxest`, `_value_all`, `_value[8|16|32|64|128|256|512|1024]`.
- new traits `DataValue[Copy]`, `DataType[Copy]`, `DataRaw[Copy]`, `DirEnv`, `ExtLog`, `ExtProcess`.
- new types: `DataValue*`, `DataType*`, `DataRaw*`, `DirApple`, `DirWindows`, `DirUnix`, `DirXdg`, `LogConfig`, `NoData`, .
  - namespaces: `Iter`, `Log`.
- new macros: `maybe!`, `xorshift_custom!`.
- new modules: `data::{codec, list, key, table, uid}`, `lang::{c}`, `sys::log`.
- new hidden const arrays: `XOROSHIFT_[16|32|64]_TRIPLETS`.
- new macro arms:
  - `str!`: `ip_addr`.
- new methods:
  - `Char::len_utf8`.
  - `Env::*`.
  - `ExtAny`: `type_hash`, `type_hash_with`.
  - `ExtFuture`: `pending`, `poll_fn`, `ready`.
  - prngs: `from_state`, `inner_state`.
- new re-exports: `SystemAlloc`, `std::{env::*, process::*}`, `::log::*`
- new optional dependencies: `itertools`.
- add musl architectures to `check.rs` script.

### Removed
- remove standalone re-exported `core::iter` functions.
- remove standalone fns: `future_block`, `future_pending`, `future_ready`.
- remove module `data::collections`.
- deprecate `Char::len_to_utf8`.

### Changed
- rename features:
  - `_docs_max` to `_max`, `_docs_min` to `_docs`.
- rename re-exports: `Layout` to `MemLayout`, `LayoutError` to `MemLayoutError`.
- rename `LoggerConfig` to `LogConfig`.
- rename `work::async` to `work::future`.
- rename `work::thread` to `work::process`.
- rename prngs `next_state` method to `peek_next_state`.
- ungate: `FxHasher`, `Xorshift128p`.
- make customizable: `XorShift[16|32|64]`.
- derive Copy for `Lgc16`.
- update `str!` macro docs and tests.
- make public: `data::error`, `sys::env`, `work::{future, process, sync}`.
- move `data::collections::{array, destaque, list, stack, vec}` inside `data::list`.
- move `data::{bit, hash, serde}` inside `data::codec`.
- changed windows `msvc` target for `gnu`.

### Fixed
- enable nightly features depending on `alloc` and `std`.
- feature-gate namespaced re-exported unsafe methods with `unsafe··`.

[unreleased]: https://github.com/andamira/devela/compare/v0.23.0-wip...HEAD
