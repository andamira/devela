# Changelog from 2025 on

## [0.23.0-wip] *unreleased*

### Added
- new features: `glsl, _maxest`, `_value_all`, `_value[8|16|32|64|128|256|512|1024]`.
- new traits:
  - data: `DataValue[Copy]`, `DataType[Copy]`, `DataRaw[Copy]`.
  - sys: `DirEnv`, `ExtLog`
  - ui: `MiniquadEventHandlerExt`, `UiService`.
  - work: `ExtProcess`.
- new consts: `FONT_3_3`, `FONT_3_5`, `FONT_5_6`.
- new types:
  - code: `ScopeGuard`.
  - data:
    - xifer: `DataValue*`, `DataType*`, `DataRaw*`, `NoData`, `Base`, `Crockford`, `Rfc4648`, `Rfc4648Hex`.
  - lang: `g_*`.
  - media: `BitmapFont`.
  - sys: `DirApple`, `DirWindows`, `DirUnix`, `DirXdg`, `LogConfig`,
  - ui: `UiCap`, `UiCapImage`, `UiCapInput`, `UiCapSound`, `UiCapSystem`, `UiCapWindow`.
    - `CrosstermService`.
    - `MiniquadPixelBuf`, `MiniquadService`.
  - namespaces: `Fmt`, `Iter`, `Log`.
- new macros: `maybe!`, `xorshift_custom!`.
- new modules:
  - `data::{codec::{self, radix}, list, key, table, uid, xipher}`.
  - `lang::{c, glsl}`.
  - `sys::{log, net}`.
  - `ui::service::{self, crossterm, miniquad}`.
- new macro arms:
  - `str!`: `ip_addr`.
- new methods:
  - `Char::len_utf8`.
  - `Env::*`.
  - `ExtAny`: `type_hash`, `type_hash_with`.
  - `ExtFuture`: `pending`, `poll_fn`, `ready`.
  - prngs: `from_state`, `inner_state`.
- new variants:
  - `IoErrorKind:` `OutOfMemory`, `FilesystemLoop`, `FilesystemQuotaExceeded`, `CrossesDevices`, `InvalidFilename`, `InProgress`.
- new re-exports: `LazyLock`, `SystemAlloc`, `std::{env::*, process::*}`, `::log::*`
- new optional dependencies: `fltk`, `flume`, `fontdue`, `gilrs`, `image`, `itertools`, `orion`, `ring`, `sdl2`, `sdl3`, `simdutf8`, `toml_edit`, `ureq`.
- add musl architectures to `check.rs` script.

### Removed
- remove standalone re-exported fns from `std::{fmt, iter}`.
- remove standalone fns: `future_block`, `future_pending`, `future_ready`.
- remove private variant `IoErrorKind::Uncategorized`.
- remove module `data::collections`.
- disable optional dependencies: `ring`, `rkyv`.
- deprecate `Char::len_to_utf8`.

### Changed
- bump MSRV to 1.84.1.
- rename features:
  - `_docs_max` to `_max`, `_docs_min` to `_docs`.
- rename re-exports: `Layout` to `MemLayout`, `LayoutError` to `MemLayoutError`.
- rename `LoggerConfig` to `LogConfig`.
- rename `TextWrite` trait to `FmtWrite`.
- rename `work::async` to `work::future`.
- rename `work::thread` to `work::process`.
- rename `src/lib.rs` to `src/Lib.rs`.
- rename prngs `next_state` method to `peek_next_state`.
- ungate: `FxHasher`, `Xorshift128p`.
- make customizable: `XorShift[16|32|64]`.
- derive Copy for `Lgc16`.
- update `str!` macro docs and tests.
- make public: `data::error`, `sys::env`, `work::{future, process, sync}`.
- move `data::collections::{array, destaque, list, stack, vec}` inside `data::list`.
- move `data::{bit, hash, serde}` inside `data::codec`.
- move fns: `fmt_write`, `fmt_format` and `format_buf_args` to `Fmt::{write, format, format_buf`, respectively.
- move `bytes_from_bits` fn to `Mem::bytes_from_bits`.
- changed windows `msvc` target for `gnu`.
- improve the documentation of vendored items.

### Fixed
- rename `prim···` flag to `prim··`
- improve build script debug output.
- enable nightly features depending on `alloc` and `std`.
- feature-gate namespaced re-exported unsafe methods with `unsafe··`.
- refactor `rustdoc-header.html` to be modular, more efficient and versatile.
- compile in docs.rs with `cpu-native` flag.

[unreleased]: https://github.com/andamira/devela/compare/v0.23.0-wip...HEAD
