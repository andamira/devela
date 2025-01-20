# Changelog from 2025 on

## [0.23.0-wip] *unreleased*

### Added
- new features: `_maxest`, `_value_all`, `_value[8|16|32|64|128|256|512|1024]`.
- new traits `DataValue[Copy]`, `DataType[Copy]`, `DataRaw[Copy]`.
- new types: `DataValue*`, `DataType*`, `DataRaw*`, `Iter`,`NoData`.
- new macros: `maybe!`, `xorshift_custom!`.
- new modules: `lang::{c}`.
- new hidden const arrays: `XOROSHIFT_[16|32|64]_TRIPLETS`.
- new method: `Char::len_utf8`.
- new `ExtAny` methods: `type_hash`, `type_hash_with`.
- new re-exports: `SystemAlloc`, `std::env::*`.
- new optional dependencies: `itertools`.
- add more methods to `Env`.

### Removed
- remove standalone re-exported `core::iter` functions.
- deprecate `Char::len_to_utf8`.

### Changed
- rename features:
  - `_docs_max` to `_max`, `_docs_min` to `_docs`.
- rename re-exports: `Layout` to `MemLayout`, `LayoutError` to `MemLayoutError`.
- ungate: `FxHasher`, `Xorshift128p`.
- make customizable: `XorShift[16|32|64]`.

### Fixed
- enable nightly features depending on `alloc` and `std`.
- feature-gate namespaced re-exported unsafe methods with `unsafe··`.

[unreleased]: https://github.com/andamira/devela/compare/v0.23.0-wip...HEAD
