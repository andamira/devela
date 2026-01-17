# Changelog from 2025

[0.24.0] 2025-08-15
===================

-----------
**Project**

## crate
- bump MSRV to 1.89.0.
- bump dependencies:
  - `allocator-api2` → 0.3.
  - `bumpalo` → 3.18.
  - `bytemuck` → 1.23.
  - `crossterm` → 0.29.
  - `libm` → 2.15.
  - `pyo3` → 0.25.1.
  - `rodio` → 0.21.1.
  - `sdl2` → 0.38.
  - `sysinfo` → 0.36.
  - `toml_edit` → 0.23.
  - `tokio` → 1.47.
- disable dependencies: `sdl3`.
- remove `std` requirement for optional dependencies: `crossterm`, `pyo3`, `regex_lite`, `sysinfo`.

## documentation
- change documentation URL to repository's custom build to avoid docs.rs surprise bugs.
- improve rustdoc header loading, make loading more reliable.
- rename `DOCS/` → `docs/` and make its contents lowercase.
- add the `no-deps` key to docs.rs package metadata.
- fix multiple katex warnings.
- new doc tag: `TAG_NICHE`.

## examples
- new `js_web_worker` example.
- delete the `niche` example.

## flags
- rename flags: `nightly_stable_next*` → `nightly_stable_1_**` to indicate the exact versions.
- add flags:
  - `nightly_become` enabling `explicit_tail_calls` feature (commented out).
  - `nightly_unstable` to group the non-`nightly_stable` features.
- disable `nightly_autodiff` flag.

## libs
- move `devela_macros` code to `devela/libs/macros`.

## meta
- rename `/build/` → `/meta/`.
- rename `/build/generate/` → `/meta/codegen/`.

## tools
- rename `utils` → `tools`.
- change `check.rs`:
  - add new args: `-A` | `--install-arches`, `-N` | `--install-nightly`
  - behavior change: no longer installs components automatically.
  - bump dependencies; fix warnings; refactor.

-----------
**Modules**

## code
- new trait `Introspect`.
- new doc tag: `TAG_ALLOCATOR`.
- rename `set_panic_handler!`'s macro `web_api` arm → `web`.

### result
- add re-exports: `core::option::*`, `core::result::*`.

## data
- add re-exports: `IterFromCoroutine`.
- add modules: `data::list::of`.
- relocate and rename `code::result::Enum` → `data::list::of::Oneof`.
- rename `Oneof` methods → ordinals: `A` → `_0`, `B` → `_1`, ….

## game
- add:
  - module: `game`.
  - features: `game`, `game_safe`.
  - reflection flags: `game··`.

## lang
### ffi
- new types: `Js`, `JsConsole`, `JsValue`, `WebDocument`, `WebWindow`, `WebWindowState`.
- move and rename the `js_str*` fns as public `Js` `read_str*` methods.
- new `Web` methods:
  - `console_count[_reset]`.
- rename types:
  - `Js` → `Web`.
  - `JsEvent*` → `WebEvent*`.
  - `JsPermission*` → `WebPermission*`.
  - `JsWorker*` → `WebWorker*`.

## media
- delete: `[Audio|Color|Draw|Font|Media|Midi][Result|Error]`.

### color
- add types: `Gamma`, `Lum`, `Rgb`, `Rgba`.
  - implement for `u8`, `u16`, `f32`, `f64`.
- add aliases for different `Rgb` color types.
- add `Lum` sub-type aliases: `Lightness`, `LinearLightness`, `Luma`, `Luminance`.
- add module: `media::color::rgb`.
- remove the `Color` namespace.
  - move all its functionality to `Gamma`.
- rename the `ColorBase` trait → `Color`.
- update the `Color` trait:
  - make type `Component` bound on `NumConst`.
  - add constants: `COLOR_[BITS|COUNT|HAS_ALPHA]`, `COLOR_IS_[INT|LINEAR|PREMUL]`.
  - add methods: `color_[bits|has_alpha]`, `color_[red|green|blue|alpha]`, `color_is_[int|linear|premul]`.

## num
- update the `NumConst` trait.
- require the trait bound `PartialEq<Self::Num>`.
- make all its associated constant values be `Option`al.
- add consts: `NUM_[MAX|MIN][_NORM]`, `NUM_IS_[BIG|INT|FLOAT|FIXED|SIGNED|NICHE]`.
- add auto-implemented methods over `&self`, to query the associated constant values.

### float
- add new `Float` methods: `classify`, `next_down`, `next_up`.
- delete the `alg` feature.

### geom
#### metric
- rename `Extent` field `size` → `dim` for consitency.
- add missing attributes `must_use` and `repr(transparent)`.
- remove type aliases: `Extent2d`, `Extent3d`, `Region2d`, `Region3d`.
- remove `metric` feature-gate for `Distance`, `Extent` and `Position`.
- impl `From` arrays and tuples for `Distance`, `Extent`, `Orientation`, `Position` and `Stride`.

#### shape
- update `Point:` make mut accesors *const*.
- add new alias: `Points2d`.
- add new method: `Points::new`.

### quant
- new macro: `interval!`.

### niche
- add new:
  - macros: `ne!`, `nz!`.
  - types: `Non[Extreme|Value][I|U]size`.
  - methods to `NonValue*`: `new_lossy`.
- make `impl_non_value!` private.
- pre-generate all `NonValue*` types.
- improve the efficiency of `NonValue*<MAX>`.

## sys

### arch
- new `Wasm` methods: `heap_base`, `remaining_memory`.
- change `Wasm::memory_grow` to have the same signature as `core::arch::wasm32::memory_grow`.
- add support for new:
  - architectures: `amdgpu`.
  - OS targets: `amdhsa`, `cygwin`, `psx`.
  - vendor targets: `amd`, `mti`, `openwrt`, `vex`.

### mem
- new `Mem` methods: `align_down`, `align_up`, `is_aligned`, `is_aligned_to`.
- new `Ptr` methods `nn_*` to construct `PtrNonNull`.
- remove macros: `addr_of!`, `addr_of_mut!`.

#### alloc
- new types: `BumpAlloc`, `WasmAlloc`.
- vendor `mini-alloc` as `WasmAlloc`.

#### borrow
- new types: `Backing`, `MaybeOwned`.
- new trait: `Ownership`.

### os
#### linux
- add methods:
  - `Linux`: `print[ln]_unchecked[_fast]`, `eprint_bytes`.

## ui
### layout
- delete:
  - types: `LayoutError`, `LayoutResult`.

## work
### process
- new `ExtProcess` method `command`.


[0.23.0] 2025-03-02
===================

### Added
- new features:
  - lang: `glsl`, `js`.
  - num: `linear`, `metric`, `shape`.
  - ui: `desk`, `term`, `web`.
  - capability: `_maxest`, `_value_all`, `_value[8|16|32|64|128|256|512|1024]`.
  - safety: `unsafe_ffi`.
- new flags: `ffi··`, `geom··`.
- new traits:
  - data:
    - codec: `Decodable`, `Encodable`, `EncodabeLen`.
    - table: `DataValue[Copy]`, `DataType[Copy]`, `DataRaw[Copy]`.
  - num: `NumConst`.
  - sys: `AppEnv`, `ExtLog`.
  - ui: `MiniquadEventHandlerExt`, `UiService`.
  - work: `ExtProcess`.
- new consts:
  - media::font: `FONT_3_3`, `FONT_3_5`, `FONT_5_6`.
  - sys::os::linux (namespaced): `LiNUX_EXIT`, `LINUX_O_FLAGS`, `LINUX_S_IFMT`, `LINUX_SEEK`, `LINUX_F_CMD`.
  - `Float`|`FloatConst`: `EXPONENT_BIAS`, `EXPONENT_BITS`, `SIGNIFICANT_BITS`.
  - `AngleDirection::{CounterClockwise, CCW, RightHandRule, RHR, Clockwise, CW, LeftHandRule, LHR}`.
- new types:
  - code: `Enum`, `ScopeGuard`, `TimeError`, `Timeout`.
  - data: `ArrayFrom`, `NoData`.
    - codec: `CodecBe`, `CodecLe`, `CodecIf`, `CodecJoin`, `CodecFlags`, `CodecLen`, `CodecLenValue`.
      - radix `Base`, `Crockford`, `Rfc4648`, `Rfc4648Hex`.
    - key: `StaticMapEntry`.
    - table: `DataValue*`, `DataType*`, `DataRaw*`.
  - lang: `g_*`, `js_*`, `JsEventKind`, `JsEventMouse`, `JsEventPointer`, `JsInstant`, `JsKeyLocation`, `JsPermission`, `JsPermissionState`, `JsTextMetrics`, `JsTextMetricsFull`, `JsTimeout`, `JsWorker`, `JsWorkerError`, `JsWorkerJob`.
  - media: `BitmapFont`, `Sixel`, `Dither`, `PixelFormat`, `SixelError`, `SixelMean`, `SixelQuality`, `SixelSplit`.
  - num:
    - geom:
      - linear: `Matrix`.
      - metric: `Distance`, `Orientation`, `Position`, `Region`, `Stride`, `RegionStrided`.
    - quant: `Cycle`, `CycleCount`, `Ratio`.
  - phys: `TimeDelta`, `TimeGranularity`, `TimeSource`, `TimeSourceFake`.
  - sys:
    - env::app: `AppApple`, `AppConfig`, `AppWindows`, `AppUnix`, `AppXdg`,
    - io: `IoEmpty`, `IoRepeat`.
    - log: `LogConfig`.
    - mem: `Current`, `CurrentGuard`, `SpinLock`, `SpinLockGuard`.
    - os::linux: `LinuxError`, `LinuxResult`, `LinuxSiginfo`, `LinuxStat`.
  - ui: `UiCap`, `UiCapImage`, `UiCapInput`, `UiCapSound`, `UiCapSystem`, `UiCapWindow`.
    - event: `EventButton`, `EventButtonState`, `EventMouse`, `EventPointer`, `EventPointerType`, `EventTimestamp`, `EventKey`,
      - key: `KeyAlpha`, `KeyMedia`, `KeyMod`, `KeyMods`, `KeyPad`, `KeyState`.
    - back: `CrosstermService`, `MiniquadPixels`, `MiniquadService`.
    - front::term: `TermSize`.
  - work: `SleepSpin`.
  - namespaces: `Ansi`, `Fmt`, `Fs`, `FsPath`, `Io`, `Iter`, `Js`, `Linux`, `Log`, `Panic`.
- new macros:
  - `define_static_map!`, `is`, `methods_as_fns!`.
  - `join!`, `js_reexport`, `maybe!`, `miniquad!`, `xorshift_custom!`.
  - `ansi!`, `os_print`, `os_println`, `os_eprint`, `os_eprintln`.
  - `linux_entry`, `set_panic_handler!`.
- new modules:
  - data: `{codec::{self, crypto, radix}, list, key, table, uid}`.
  - lang: `{dsl, ffi::{self, c, glsl}, i18n, ling::{art, nat}}`.
  - media: `{image::sixel, video}`.
  - num: `{geom::metric, ord, quant}`.
  - phys: `{bio, chem, elec, mech, unit}`.
  - sys: `{log, net, fs}`.
  - ui: `{back::{self, crossterm, miniquad}, front}`.
  - work: `sync::mpsc`.
- new macro arms:
  - `format_buf!`: `?`.
  - `str!`: `ip_addr`.
  - `unwrap!`: `ok_err`, `*_guaranteed_or_ub`.
- new methods:
  - `Char`: `len_utf8`, `code_len_utf8[_unchecked]`,`code_to_utf8_bytes[_unchecked]`, `[code_]to_ascii_str[_unchecked]`, `is_valid`, `to_ascii_fold[_unchecked]`, `utf8_bytes_to_code[_unchecked]`, `utf8_len[_checked]`.
  - `Env::*`.
  - `ExtAny`: `type_hash`, `type_hash_with`.
  - `ExtFloat`: `sqrt_nr`.
  - `ExtFuture`: `pending`, `poll_fn`, `ready`.
  - `ExtThread`: `sleep_ms`, `sleep_us`, `sleep_ns`.
  - `Float`: `midpoint`, `recip`, `sqrt_hybrid`, `to_degrees`, `to_radians`.
  - `HasherFx`: `hash_bytes_with_seed`.
  - `IoError`: `other`.
  - `Linux` syscalls: `sys_[open|close|lseek|dup|dup2|fcntl|stat|fstat|getdents|pipe|pipe2]`.
  - `Slice`: `copy_from_slice`, `<u8>::copy_array`, `<u8>::copy_array_at`, `<u8>::copied_array_at`, `<u8>::to_array`.
  - `Str`: `__utf8_bytes_to_str`.
  - prngs: `from_state`, `inner_state`.
- new variants:
  - `IoErrorKind:` `OutOfMemory`, `FilesystemLoop`, `FilesystemQuotaExceeded`, `CrossesDevices`, `InvalidFilename`, `InProgress`.
- new re-exports:
  - core: `any::type_name` as `any_type_name`,
  - lang: `c_void`.
  - std:
    - `alloc::System` as `SystemAlloc`.
    - env: `*`.
    - io: `IoEmpty`, `IoIntoInnerError`, `IoRepeat`, `Std[err|in|out][Lock]`.
    - panic: `PanicHookInfo`.
    - process: `*`.
    - sync: `LazyLock`, `mpsc::*`.
  - log: `*`.
- new optional dependencies: `ffmpeg-the-third`, `fltk`, `flume`, `fontdue`, `gilrs`, `image`, `itertools`, `orion`, `ring`, `sdl2`, `sdl3`, `simdutf8`, `toml_edit`, `ureq`.
- new profile: `wasm`.
- new example: `js_web_api`.
- add musl architectures to `check.rs` script.
- add docs for monitored nightly features and for disabled dependencies.
- add more doc tags: `TAG_[FAKE|FFI|FMT|GEOM|NO|NUM|QUANT|RAND|TEXT|TIME]`.
- add more compile targets for docs.rs.

### Removed
- remove standalone re-exported fns from `std::{fmt, iter}`.
- remove items:
  - sys::os::linux: `LinuxTerminal`, `LinuxTerminalSize`.
- remove standalone fns:
  - io: `io_*`.
  - panic: `panic_*`.
  - sys::os::linux: `linux_*`.
  - text: `crate_root`, `crate_root_string`.
  - work: `future_block`, `future_pending`, `future_ready`.
- remove methods:
  - `Float:` `const_round_ties_odd`.
- remove private variant `IoErrorKind::Uncategorized`.
- remove features: `linux_deps`, `unsafe_async`.
- remove modules:
  - `data::collections`.
  - `num::alg`.
- disable optional dependencies: `fltk`, `js-sys`, `nc`, `ring`, `rustix`, `rkyv`, `tinyaudio`, `tracing`, `wasm-bindgen`, `web-sys`.
- deprecate:
  - `iif!`.
  - `Char::len_to_utf8`.
  - `Float::{const_[clamp|max|min|signum|copysign]}`.

### Changed
- bump MSRV to 1.85.0.
- rename features:
  - `_docs_max` to `_max`, `_docs_min` to `_docs`, `_string_*` to `_str_*`.
- change features into cfg flags:
  - `nightly, [nightly_[allocator|autodiff|bigint|coro|doc|float|simd|stable|stable_[next1|next2|later]`.
- rename flags:
  - `prim···` flag to `prim··`, `_str*` to `_str*`.
- rename/move modules:
  - `code::result::{error, panic}` inside `code`.
  - `data::collections::{array, destaque, list, stack, vec}` inside `data::list`.
  - `data::{bit, hash, serde}` inside `data::codec`.
  - `data::error` inside `code::error`, make private.
  - `num::cmp` to `num::ord`.
  - `num::alg::linear` to `num::geom::linear`.
  - `num::geom::shape::{angle, extent}` inside `num::geom::metric`.
  - `num::wave` to `phys::wave`.
  - `sys::path` inside `sys::fs`, make private.
  - `work::async` to `work::future`.
- rename/move items:
  - `ExtFloatConst` to `FloatConst`.
  - `LoggerConfig` to `LogConfig`.
  - `TextWrite` trait to `FmtWrite`.
  - `Coro` to `CoroWorker`, `CoroYield` to `CoroWork`, `CoroRun` to `CoroManager`.
  - re-exports: `Layout` to `MemLayout`, `LayoutError` to `MemLayoutError`.
  - `impl_error!` to `define_error!`.
- rename examples:
  - `coro_run` to `coro_manager`.
- rename variants:
  - `AngleDirection`: `CounterClockwise` to `Positive`, `Clockwise` to `Negative`.
- rename constants:
  - `LINUX_SIGACTION`: remove `SA_` prefix.
  - `LINUX_SIGNAL`: remove `SIG` prefix.
- rename/move fns/methods:
  - from prngs: `next_state` method to `peek_next_state`.
  - `fmt_write`, `fmt_format` and `format_buf_args` to `Fmt::{write, format, format_buf`, respectively.
  - `bytes_from_bits` fn to `Mem::bytes_from_bits`.
  - `ExtThread::parallelism` to `available_parallelism`.
- deprecate fns/methods:
  - `Char`: `utf8_2bytes_len`, `utf8_3bytes_len`, `utf8_4bytes_len`.
- modify methods:
  - `LinuxSigaction`: `new` method has an additional `restorer` argument.
- remove feature-gates:
  - `hash` for: `FxHasher`.
  - `io` for: `IoError`, `IoErrorKind`, `IoRead`, `IoWrite`, `IoBytes`, `IoChain`, `IoTake`.
  - `rand` for: `Xorshift128p`.
  - `str` for: `Str`.
- auto-enable features:
  - `str`: when enabling `_str_u*`.
- make customizable: `XorShift[16|32|64]`.
- make const methods:
  - `Float`: `clamp`, `copysign`, `div_euclid`, `max`, `min`, `min_total`, `round_ties_odd`, `signum`.
  - `IoBufReader`: `buffer`, `capacity`, `get_ref`, `get_mut`, `new`.
  - `IoChain`: `get_ref`, `get_mut`, `new`.
  - `IoCursor`: `get_ref`, `get_mut`, `new`, `position`, `set_position`.
  - `Mem`: `swap`.
  - `Ptr`: `copy`, `copy_nonoverlapping`, `replace`, `write`, `write_bytes`, `write_unaligned`.
- change attributes:
  - mark `Float` and `Sign` as `must_use`.
- derive Copy for `Lgc16`.
- update `str!` macro docs, tests and syntax: remove `:` suffix.
- make modules public:
  - `data::error`.
  - `num::geom::shape`.
  - `sys::env`.
  - `work::{future, process, sync::atomic}`.
- change `msvc` windows target for `gnu`.
- update scripts:
  - `utils/check.rs`:
    - add target components for the nightly toolchain.
    - add new fn `run_cargo_with_env`, sharing most of the logic with `run_cargo`.
    - enable `__dbg` feature & `-Ctarget-cpu=native`.
  - `build/features.rs`: support cfg flags auto-enabling other flags. Improve docs.
- improve the `.rustfmt.toml` config file.
- improve the docs for vendored items.

### Fixed
- improve build script debug output.
- make `array_init` use absolute paths internally.
- enable nightly features depending on `alloc` and `std`.
- fix and improve `Float` methods: `[cos|sin|tan]_series`.
- feature-gate:
  - `ExtFuture::block`.
  - namespaced re-exported unsafe methods with `unsafe··`.
- refactor `rustdoc-header.html` to be modular, more efficient and versatile.
- compile in docs.rs with `cpu-native` flag.

[0.22.1] 2025-01-13
===================

- fix docs compilation.


[0.22.0] 2025-01-13
===================

### Added

#### New features & flags
- new features for:
  - code: `code`, `error`, `_unroll`, `_unroll_128`, `_unroll_256`, `_unroll_512`, `_unroll_1024`, `_unroll_2048`.
  - data: `hash`.
  - doc: `_docsrs[_stable]_nodep`.
  - num: `alg`, `geom`, `prim`, `cast`, `join`, `split`, `unit`, `wave`, `_cmp_f16`, `_cmp_f128`, `_float_f16`, `_float_f128`.
  - media `media`, `safe_media`, `audio`, `color`, `draw`, `font`, `image`, `midi`.
  - sys: `time`, `linux`, `dep_linux`, `unsafe_syscall`.
  - text: `ascii`, `fmt`, `str`, `_char7`, `_char8`, `_char16`.
  - nightly: `nightly_alloc`, `nightly_autodiff`, `nightly_bigint`, `nightly_float`, `nightly_stable_next1`, `nightly_stable_next2`, `nightly_stable_later`.
  - safety: `safe_audio`, `safe_color`, `safe_draw`, `safe_ffi`, `safe_font`, `safe_image`, `safe_io`, `safe_layout`, `safe_ui`, `safest`, `unsafe_async`.
  - ui: `ui`, `layout`.
  - other: `alloc_deps`, `lang`, `windows`, `__force_miri_dst`.
- new cfg flags: `cargo_primary_package`, `*·`, `_*··`.

#### New items
- new structs:
  - `Interval`, `Pnm`.
  - new namespaces: `Alloc`, `Arch`, `ByteSearch`, `Char`, `Env`, `Mem`, `Ptr`, `Str`.
  - new standalone error types: `FailedErrorConversion`, `DataNotEnough`, `NotImplemented`, `NotSupported`, `ElementNotFound`, `InvalidAxisLength`, `KeyAlreadyExists`, `MismatchedCapacity`, `MismatchedDimensions`, `MismatchedIndices`, `NodeEmpty`, `NodeLinkNotSet`, `NodeLinkNotUnique`, `NotEnoughElements`, `NotEnoughSpace`, `IndexOutOfBounds`, `DataOverflow`, `PartiallyAdded`, `InvalidChar`, `InvalidUtf8`, `SystemTimeError`.
  - new composite error types: `NotAvailable`, `DataNotEnough`, `MismatchedBounds`, `PartialSpace`.
  - `False`, `True`, `UnitBi`, `UnitSi`.
  - `HasherPengy`.
  - `Lgc16`.
  - `TimeSplit`.
  - `TypeResource`.
  - `WaveletHaar`, `WaveletUnitVec`.
- new aliases:
  - `AllocMapFx`, `AllocSetFx`, `FmtResult`, `NoTime`.
  - `MediaResult`, `AudioResult`, `ColorResult`, `DrawResult`,`FontResult`, `ImageResult`.
  - `UiResult`, `LayoutResult`.
  - re-exported primitives: `char`.
  - `TimeSplitYearNano`, `TimeSplitYearDay`, `TimeSplitYearSec`, `TimeSplitHourSec`, `TimeSplitHourNano`, `TimeSplitMilliNano`.
- new enums:
  - `CompressionMode`, `EncodingMode`, `WaveletUnitRole`.
  - `AllError`, `AllErrorKind`, `MediaError`, `ColorError`, `AudioError`, `DrawError`, `FontError`, `ImageError`, `UiError`, `LayoutError`.
- new enum variants:
  - `DataError::ElementNotFound`.
- new traits:
  - `ColorBase`, `ExtCellOption`, `ExtError`, `ExtOptRes`, `ExtPath`, `ExtThread`, `MemPod`, `TypeResourced`, `Unit`.
  - `WaveletCompressionVec`, `WaveletTransformVec`.
- new associated methods and constants for:
  - `Array`: `from_fn`, `contains_[from|to|between]`, `as_bare_mut_slice`, `get[_mut]`.
  - `Array`, when storing `Option<T>`.
  - `Array2d`: `cap_col`, `cap_row`, `cap_major`, `cap_minor`, `num_major`, `num_minor`.
  - `BareBox`.
  - `ExtAny` method `type_id`.
  - `Float` and `ExtFloat`:
    - `FRAC_1_SQRT_2PI`.
    - `eval_poly` to evaluate polynomials.
    - for calculus: `derivative`, `integrate`, `partial_derivative_[x|y]`.
  - `Float` and `ExtFloatConst` consts: `LOW_MARGIN`, `MEDIUM_MARGIN`, `HIGH_MARGIN`.
  - `Graph` methods: `edge_exists_unchecked`, `edge_remove`.
  - `NonValue*`: `is_max`, `is_min`, `[checked|strict|saturating|wrapping]_[add|sub]`.
  - `Slice`: `eq`, `from_mut`, `from_ref`, `from_raw_parts`, `from_raw_parts_mut`, `range[_mut][_checked]`, `range_from[_mut][_checked]`, `range_to[_mut][_checked]`, `take_first[_mut][_checked]`, `take_last[_mut][_checked]`, `take_omit_last[_mut][_checked]`.
  - `UnicodeScalar` & `char*` types: `MIN`.
- new macros:
  - `capture_first!`, `capture_last!`, `capture_tail_tuple!`, `const_bool!`, `field_of!`, `id_seq!`, `impl_non_value!`, `impl_trait!`, `include_from!`, `mod_from!`, `str!`, `type_marker!`, `type_resource!`, `unroll`.
  - private: `doc_availability!`, `impl_error`, `EMOJI_*`, `TAG_*`.
- new vendored items
  - structs: `CacheAlign`, `ConstList`, `FatPtr`, `IdPinBox`, `IdPin`, `VecChunk`.
  - macros: `cfg_if!`, `const_assert!`.
  - traits: `ConstBool`.
- new optional dependencies:
  - `allocator-api2`, `bumpalo`, `crossterm`, `jiff`, `js-sys`, `kira`, `midir`, `nc`, `pyo3`, `rayon`, `regex-lite`, `rkyv`, `rodio`, `rustix`, `safe_arch`, `serde`, `stringzilla`, `symphonia`, `sysinfo`, `tinyaudio`, `tokio`, `tracing`, `wasm-bindgen`, `web-sys`, `winnow`.
- new re-exported:
  - items from: `alloc::alloc`,`core::{cell, num, ops, result}`, `std::{backtrace, fmt, path}`.
  - fns: `array_from_fn`, `array_from_mut`, `array_from_ref`.
  - macros:
    - `assert_unchecked!`, `autodiff`, `compile_error!`, `concat`, `enumint!`, `format!`, `format_args!`, `option_env!`, `stringify`, `thread_local!`, `write!`, `writeln!`.
    - wrapped: `env!`, as `env_!`, `vec!` as `vec_!`.
  - structs:
    - `NonZero`, `Saturating`, `Wrapping`, `OsStr`, `OsString`.
    - `HashMapEntry` and `BTreeMapEntry`.
    - `HashMap` and `BTreeMap` from `std` if `hashbrown` is disabled.
    - `FromStr`, `IterChars`.
  - crate items from multiple related modules, like errors and strings.
- new modules: `num::alg`, `sys::sound`, `media::{audio, color, draw, font, image, layout}`, `phys`, `ui`.
- new `sys::os::linux` module and example `linux`.
- new `NonValue*` constants `MAX`, `MIN`.
- new lints.

#### Examples, utilities, manifest, files
- new examples:
  - `id_pin`.
  - `id_seq` and type `ExampleIdSeqUsize`.
  - `enumint` and type `ExampleEnumIntU8`.
- manifest:
  - add `patches` section.
  - add table of contents.
  - add profiles: `dev-lto`, `release-lto`.
- new `.clippy.toml` configuration file.
- new github workflows: `get_errno.yml`, `get_syscall.yml`.
- new scripts in `utils/`: `cargo-native`, `manifest.sh`, `release_dates.rs`, `get_errno.sh`, `get_syscall.sh`, `docs_coverage.sh`, `docs_items.rs`.
- new convenience fn: `manifest_dir` in `build::utils`.
- hide `no_inline` items re-exports.
- new file `DOCS/VENDORED_rustdoc.md`
- show docs for the build scripts (private).
- add system of internal structural modules.
- rustdoc html header:
  - load katexextensions `mchem` and `copy-tex`.
  - trust `\href` commands.
  - move to `config/`.
- new directory `config/`.
  - put here a copy of `Cargo.toml::dep_all`.

### Removed
- remove custom no_std `Error` definition.
- remove items: `NeverOk`, `NeverErr`, `HasherFx32`, `HasherFx64`.
- remove types: `char24`, `char32`, `InRange*`, `NonRange*`, `HourMilliSplit`, `SecNanoSplit`, `YearSecSplit`.
- remove aliases of text-related types.
- remove features: `_default`, `_max`, `_non_value_*`, `_in_range`, `num_geom`, `unsafe_const`.
- remove standalone `char_*` fns (namespaced in `Char`).
- remove standalone fn `hash_pengy` (made part of `HasherPengy`).
- remove most re-exported fns from `std::mem` (namespaced in `Mem`).
- remove re-exported fns from `std::ptr` (namespaced in `Ptr`).
- remove convenience fn: `out_dir` from the build script.
- disable `Graph*`, `Node*`, and `NodeIndex*` types.
- remove `Float::const_abs`.

### Changed

#### Misc.
- bump rust version to 1.83.0.
- start using `core::error::Error`.

#### Features & flags
- rename features:
  - `unsafe_dyn` to `unsafe_layout`.
  - `nightly_stabilized` to `nightly_stable`.
  - `_[max|min]_docs` to `_docs_[max|min]`, `_docsrs_max` to `_docsrs`.
  - `dep_linux` to `linux_deps`, `dep_text` to `text_deps`, `dep_work` to `work_deps`.
- rename compilation flags:
  - `_some_*` to `_*··`.
  - `_int_i_·` to `_int_i··`, `_int_u_·` to `_int_u··`, `_string_u_·` to `_string_u··`.
- modify how features `_non_value_u8`, `_non_value_u16` are enabled for `Char*` types.
- feature gate methods returning `DataResult` in: `Array`, `Array2d`, `ArrayUninit`, `Bitwise`, `BitOps`.
- show build *env* variables if `__dbg` feature is enabled.

#### Items
- structs:
  - make `data::dst` types use `MemPod` instead of `bytemuck::Pod`.
  - rename:
    - `Also` to `Tap`, `Apply` to `Chain`.
    - `COLOR` to `Color`.
    - `Color` to `ColorBase`.
    - `GcdExt` to `GcdResult`.
    - `AllocMap` to `HashMap`, `AllocSet` to `HashSet`.
    - `AllocOrdMap` to `BTreeMap`, `AllocOrdSet` to `BTreeSet`.
    - `AllocPrioQueue` to `BinaryHeap`.
    - `AllocLinkedList` to `LinkedList`.
    - `Dst*` types const-generic `N` to `CAP`.
    - `sys::io` items by prefixing them with `Io`.
    - `Egc` to `Grapheme`, `EgcString` to `GraphemeString`.
    - `EgcNonul` to `GraphemeNonul`, `EgcU8` to `GraphemeU8`.
    - `UninitArray` to `ArrayUninit`.
- `Compare`
  - make `Compare<usize>` always compiled.
  - implement for `f16` and `128`.
- functions and constant:
  - `Graph::edge_exists` no loger panics.
  - rename:
    - `Array::len` to `capacity`.
    - `mem_*` prefixed fns as `Mem` methods.
    - `ptr_*` prefixed fns as `Ptr` methods.
  - make *const* most of the `Angle` methods.
  - make *const* all the `Float` methods that were previously feature-gated, and add:
    `eval_poly`, `factorial`, `mul_add_fallback`, `scale`, `lerp`, `ln*_series`, `log[10|2]_series`.
  - make *const* versions of the following `Float` methods:
    `clamp_nan`, `fisr`, `hypot_fisr`, `max_nan`, `min_nan`, `cbrt_nr`, `sqrt_nr`, `hypot_nr`, `rem_euclid`, `*_series`, `*_series_terms*`.
  - add additional *const* methods: `const_floor`, `const_ceil`, `const_round`, `const_round_ties_away`, `const_round_ties_even`, `const_round_ties_odd`, `const_trunc`, `const_fract`, `const_split`, `const_signum`, `const_copysign`, `const_clamp`, `const_min`, `const_max`, `const_powi`.
  - improve precision of `ExtFloatConst` constants from 35 to 80 decimals.
  - remove all `inline` attributes for most functions
- macros:
  - update `cdbg!` to support a single `@`.
  - update `reexport!` to support receiving an optional tag argument.
  - rename `mem_size_of_expr!` to `size_of_expr!`.
  - rename re-wrapped macros to avoid prelude collision when glob importing:
    - `env`→`env_`, `panic`→`panic_`, `vec`→`vec_`.
- modules:
  - make modules public: `data::serde`, `sys::arch`, `text::fmt`.
  - rename:
    - `num::geom::algebra` to `num::algebra::linear`.
    - `exec` to `work`, and related features.
    - `mem` to `sys::mem`.
    - `sys::ffi` to `ffi.
    - `time` to `phys::time`.
    - `_alloc` & `_std` inside `_dep`.
    - `_dep::_core` to `_core`.
    - `_deps` to `_dep`.
    - `_lib*` by removing the `lib` prefix.
- traits:
  - impl `Num` for niche types.
- dependencies:
  - make `bytemuck` an optional dependency.

#### Examples, utilities, manifest, files
- rename `DOCS/DERIVED.md` to `VENDORED.md`.
- rename the `tools` directory to `utils`.
- simplify aliases for `cargo-asm`.
- refactor the build script.

### Fixed
- reduce noise from required features on methods from `Divisor`, `Int`, `Float`, `Frac`.
- make `utils/check.rs` not compile with `all_dep` when cross-compiling certain arches.
- hide public macros from the crate root when `cfg(cargo_primary_package)`.
- fix build script utility call paths, add missing `_tuple*` features.
- fix `bitfield` and `enumset` being able to be called from the root.
- several fixes for linux syscalls on multiple architectures.
- simplify system of documentable & testable examples.
- fix reexported fns: `fmt_format`, `fmt_write`.
- fix `f64::NR_TOLERANCE` from 1e-14 to 1e-12.
- fix a few tests and examples.
- fix `RcWeak` re-export.
- fix `HashSetFx` alias.


[macros/0.12.1] - 2025-01-07
============================

- new macro `field_of`.
- make `std` a default feature.
- remove `hashbrown` from default features.
- remove inline attributes.

[0.24.0]: https://github.com/andamira/devela/releases/tag/v0.24.0
[0.23.0]: https://github.com/andamira/devela/releases/tag/v0.23.0
[0.22.1]: https://github.com/andamira/devela/releases/tag/v0.22.1
[0.22.0]: https://github.com/andamira/devela/releases/tag/v0.22.0

[macros/0.12.1]: https://github.com/andamira/devela/releases/tag/macros/v0.12.1
