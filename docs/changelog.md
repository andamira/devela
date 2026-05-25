# devela changelog

[0.28.0-wip] unreleased
=======================

>
>

```
```

# Key changes:

------------------------------------------------------------------------------

# Project

## infra
### cargo
- bump MSRV to 1.95.0.

### tools
- update `check.rs`: downgrade devela version to 0.26.0.
- update `x` workspace command wrapper: add support for flags with values.

## workspace
### examples
- new example scripts: `current_guard`, `otp`, `scope_guard`.

### devela_macros
- enable safety feature guards.
- remove `enumint` example.
- improve integration tests.
- new helper fns: `deny_tokens`, `error_tokens`, `warn_tokens`.

## build
- new native section to detect native library availability.

## dependencies
- remove `pastey`.
- bump `portable-atomic-util` to 0.2.7.
- bump `hashbrown` to 0.17.
- bump `wide` to 1.3.

## docs
- new tags: `_TAG_[CRYPTO|GAME|LANG|PARSER|STRING]`.
- rename tag `_TAG_GEOM_DIR` to `_TAG_DIR`.
- make example-generated `*Example` items visible in the docs.

## features & flags
- new features: `alsa`, `shell`, `web`, `_linux_abi`, `__disable_native_libs`.
- new flags: `devela_macros_warnings`, `devela_macros_errors`, `ffi_xcb_shm··`.
- remove features: `bit`, `mem`, `_value_*`.
- auto-enable feature `unsafe_ffi` via: `web`, `x11`.
- auto-enable feature `unsafe_syscall` via: `linux`.
- remove feature `term` from being enabled by `sys`.
- require the `term` feature for all term-related functionality.
- enable the nightly feature `proc_macro_diagnostic` with the `nightly_stable_later` flag.
- update the intended scope of the `_docs_examples` feature.

---

# Modules

## build
- update `Build`:
  - new methods: `emit_[check_cfg|checked_flag|env|env_marker|flag|flag_if|flag_if_lib]`, `has_lib`.

## code
- new types: `Version`, `VersionFull`.
- impl `ConstInit` for more types.
- fix and update `enumint!`
  - improve error diagnostics.
  - remove `new_unchecked` method.
  - remove automatic documentation.
  - support receiving custom attributes.
  - remove manual `Send` and `Sync` impls.
  - rename `ExampleEnumIntU8` to `EnumintI8Example`.
  - rename constants: `VALID_VALUES` to `VALUES` and `NICHE_VALUES` to `NICHES`.
  - change implementation to be safe by default.

### code::marker
- new trait `IndexRepr`.
- new type `ReprMode`.

### code::util
- new attr-macros: `macro_apply`, `macro_derive`, `macro_derive_with`.
- new decl-macros: `macro_apply_alias!`, `macro_derive_alias!`, `macro_dollar!`, `maybe_slot!`, `paste!`, `read_at!`, `test_size_of!`.
  - vendor `macro_rules_attribute` and `pastey`.
- remake `fn_name!` and `compile_warn!`.
  - rename `compile_warn!` to `const_warn!`.
- update `cdbg!` to add custom prefix syntax.
- update `enumset!`:
  - rename consts and methods by removing prefixes: `ENUM_*`, `enum_*`, `new_*`.
  - accept custom impl blocks for both the enum and the set.
  - remove support for deriving `Default` for the enum.
  - new enum methods: `to_set`, `is_in`.
  - replace `bitfield!` with `set!`.
- update `if!`:
  - support branch-local statement semicolons and empty then/else arms.
- update `impl_trait!` to support custom attributes.
- update `unwrap!`: add missing arms: `ok_or?`, `sok_or?`.
- update `write_at!`:
  - make it return the offset.
  - add `#expr` syntax support for writing unicode scalar values as UTF-8 bytes.
  - add special support for fixed-width unrolled spread sequences.
- reexport `cfg_select!`, `cold_path()`.
- remove `cfg_if!`, replace with `cfg_select!`.

## data
- new macro: `word!`.
- new trait: `Word`.

### data::codec
- new submodules: `bin`, `detect`, `pack`, `symbol`.
- move `hash::check` to `integrity`.
- move `bit` inside `bin`.
- move `schema` inside `data::value`.
- move `deser` inside `data::value::schema`

### data::codec::bin
- new macro: `set!`.
- new types: `BinTag4`.
- recreate `bitfield!` making it much more lean and improved.
- remove the `bitfield` example.

#### data::codec::crypto
- new macro: `digest!`.
- new types: `CryptoError`, `Digest`, `Otp`.
- new example/test `digest!` items: `Md5`, `Sha1`, `Sha256`, `Sha512`.

#### data::codec::pack
- new types `Riff`, `RiffChunk`, `RiffChunkIter`, `RiffError`.

#### data::codec::encode
- remove `encode` vendored items: `CodecBe`, `CodecLe`, `CodecIf`, `CodecFlags`, `CodecJoin`, `CodecLen`, `CodecLenValue`, `Decodable`, `Encodable`, `EncodableLen`.

### data::id
- rename `define_handle!` to `handle!`

### data::layout
- update `buffer_linear!` to add index type guards and optimize zero-index creation.
- rename `DstQueuePopHandle` to `DstQueuePopGuard`.

### data::topol
- rename `ConstListIterator` to `ConstListIter`; add new const-fn `next` method.

#### data storage::key
- rename `define_static_map!` to `map!`

### data::value
- new types: `ValueKind`, `ValueKind4`.
- remove items: `DataValue*`, `DataType*`, `DataRaw*`.

### geom::metric
- new macros: `dis!`, `ext!`, `ori!`, `pos!`, `region!`.
- new methods for `geom::metric` *dim* types: `map`, `map_into`, `try_map`, `try_map_into`.
- new methods for `Region`: `map`, `map_ext`, `map_pos`, `try_map`, `try_map_ext`, `try_map_pos`.

### lang::hum
- new submodule: `code`.

### lang::prog
- new submodules: `calc`, `embed`, `kernel`, `phrase`.
- remove submodule: `dsl`.

##### lang::prog::ffi::js
- rename `JsTextMetrics*` to `JsTextRenderMetrics*`.

#### lang::prog::script
- new submodule: `shell`.
- new types: `ShellLex`, `ShellQuote`, `ShellWordError`.

### media::audio
- new submodules: `acoustic`, `effect`, `format`, `instrument`, `music`, `rhythm`, `synth`.
- new trait: `PcmSampleType`.
- new types: `PcmBuffer`, `PcmPlanar`, `PcmRaw`, `PcmRawBuf`, `PcmRawError`, `PcmSample`, `PcmSpec`, `PcmWav`, `PcmWavBuf`, `PcmWavError`, `PcmWavFmt`.

#### media::visual::color
- new type `ColorDepth`.
- make the `Color` trait not depend on the `color` feature.

#### media::visual::image
- new submodules: `format`, `raster`.
- remove `image` feature-gate from the module.
- update `ImageError`:
  - make `InvalidParsedInteger` variant contain `ParseIntErrorKind`.
  - derive `Hash`.

##### media::visual::image::raster
- new traits: `Raster`, `RasterBuf`,`RasterView`, `RasterBufBytes`, `RasterViewBytes`, `RasterSamplePacked`, `RasterViewPacked`.
- new types: `RasterBytesMut`, `RasterBytesRef`, `RasterFormat`, `RasterLayout`, `RasterMut`, `RasterRef`.
- new private types: `RasterAlpha`, `RasterChannels`, `RasterPackedChannels`, `RasterSampleFormat`, `RasterTransfer`.

#### num::dom::int
- rename `define_divisor!` to `divisor!`.

#### num::dom::real
- make `Float`'s std methods const: `ceil`, `floor`, `mul_add`, `round_ties_away`, `round_ties_even`.

#### num::fin::bit
- update `Bitwise`:
  - rename checked methods to have `_checked` suffix.
  - rename `set_checked_value_checked_range` to `set_value_range_checked_strict` and fix.

### num::grain
- rename `Primitive*` traits to `Prim*`.
- rename `define_lane!` to `lane!`

#### num::grain::niche
- new macro `niche!`.
- remove macros: `nm!`, `nv!`, `nz!`.
- rename `NonExtremeI*` to `NonMin*` and `NonExtremeU*` to `NonMax*`.

#### num::grain::wide
- update `lane!`:
  - document methods in `Lane4_i32Example`.
  - rename `from_bytes` to `from_byte_values`.
  - fix `sub_assign_wide`.

### num::prob
- new module `phys::prob::markov`.

#### num::prob::rand
- rename `define_pcg!` to `rand_pcg!`
- rename `define_xorshift!` to `rand_xorshift!`.

## phys
- new module `phys::subs`.

#### phys::time
- new type: `Timed`.
- new alias: `MaybeTimed`.
- update `NoTime`:
  - implement `TimePoint`, `TimeSource` and `TimeSpan`.
- update `Timecode`:
  - new methods: `write_nanos_u64`, `write_secs_f64`.

#### phys::time::source
- update `TimePoint`:
  - make `Elapsed` require `TimeSpan`.
  - implement for `Duration`.
- new trait `TimeSpan`.

## run
- new types: `RunDriver`, `RunDriverError`, `RunDriverFrameError`.
- update `RunRender` and `RunPresent`:
  - support borrowed render artifacts with GATs.
  - make them use a borrowed `RunFrame`.
  - make `RunRender`'s `S: ?Sized`.

### run::regime
- new type: `RunCapColor`, `RunCapText`.
- new trait `RunServiceProbe`.
- update `RunCap`:
  - new field: `color`.
  - make the system field not depend on `alloc`.
- update `RunCapImage`:
  - rename field `max_bitmap_size` to `max_bitmap_extent` with type `Option<Extent2<NonMaxU32>>`.
  - remove the `rgb` field.
- remake `RunCapSystem` from scratch as a set.
- make `RunCapInput` a set.
- update `RunService`:
  - rename methods with the `run_` prefix.
  - make `version` return `VersionFull`.
  - remove method `version_string`.

### run::time
- update `RunPacer`:
  - change requiring `Duration` for `T: TimeSpan`.
  - make the constructor fallible.
  - add new methods: `interval`, `accum`, `allow`, `allow_checked`, `cycles`, `cycles_checked`.

##### sys::device::audio
- new types `Alsa`, `AlsaError`, `AlsaPcmHandle`, `AudioDevice`, `AudioDeviceCow`, `AudioDeviceDir`, `AudioStreamDir`.
- gate alsa functionality on native `libasound` availability via `ffi_alsa··`.

##### sys::device::display::x11
- new types: `XCpuBuffer`, `XFrontend`, `XImageMode`, `XPresent`, `XRasterRender`, `XShmBuffer`, `XSurfaceFrame`.
- gate SHM functionality on native `xcb-shm` availability via `ffi_xcb_shm··`.
- update `XDisplay`:
  - add fields: `image_format`, `shm_caps`.
  - add methods: `bits_per_pixel`, `scanline_pad_bits`, `bytes_per_line`, `has_shm`.
- fix `XWindow::clear_redraw`.

### sys::env
- update `Env`:
  - make all environment constants available without `std`.
  - add `TARGET`, `FAMILIES`, `ENV`, `ABI`, `ENDIAN`, and `POINTER_WIDTH`.
  - derive `DLL_*` and `EXE_*` constants from target information.
  - align `ARCH`, `OS`, `FAMILY`, `VENDOR`, with Cargo/rustc target cfg values.

### sys::fs
- rename `PathPrefix` to `PathWindowsPrefix`, `PathPrefixComponent` to `PathWindowsPrefixComponent`.

### sys::io
- new traits `TextIn`, `TextOut`.

### sys::log
- new modules `sys::log::{bench, trace}`.
- new types: `DiagLevel`, `DiagOut`.

### sys::mem
- rename `define_arena!` to `arena!`.

### sys::mem::cell
- new types: `MemHedgeCtrl`, `MemHedgeError`, `MemHedgeRead`, `MemHedgeState`.

### sys::mem::view
- update `Slice`: add methods: `get`, `get_mut`:
- new types: `MemReplicaError`, `MemReplicaSlice`.

### sys::mem::size
- refactor the `bit` submodule.

### sys::os
- remove macros: `os_print!`, `os_println!`, `os_eprint!`, `os_eprintln!`.

##### sys::os::browser::web
- new type `WebEventWheel`.
- update `WebEventKind`:
  - add `Wheel` variant, update associated values.
- bring here impls from `ui::event`.
- feature-gate with `event` the event-related impls.
- rename all methods `[from|to]_js*` to `[from|to]_web*`.
- replace `ui::event::KeyState` methods: `[from|to]_js` with `WebEventKind` methods: `[to|from]_key_state`.
- replace `ui::event::EventMouse` methods: `[from|to]_js` with `WebEventMouse` methods: `to_kind_timed`, `from_event_mouse_timed`.
- replace `impl From<WebEventMouse> for EventMouse` with one `for EventKindTimed`.
- replace `impl From<EventMouse> for WebEventMouse` with one `From<Timed<EventMouse, Option<EventTimestamp>>>`.

#### sys::os::linux
- update `LinuxError` conversion to `IoError`.
- feature-gate term-related functionality.

#### sys::os::term
- new types: `AnsiLink`, `AnsiOsc`, `TermCap`, `TermCaps`, `TermInputParser`.
- update `Ansi`:
  - add new constants: `ESC`, `BEL`, `OSC`, `SS3`, `ST`, `HYPERLINK_CLOSE`.
  - add new functions: `title`, `title_icon`, `title_window`, `link`, `link_with_id`, `clipboard_base64`, `clipboard_query`, `clipboard_query_clipboard`.
  - fix `<SET|RESET>_PALETTE`.

## text
- new submodules: `codec`, `draw`, `generate`, `measure`, `unicode`.
- move submodule `char::ascii` to `ascii` and make public.
- move submodule `char::digits` inside `ascii`
- move submodule `char` to `unicode::scalar`
- move submodule `grapheme` inside `unicode`.
- rename `TextLut` to `AsciiLut`, move it to `ascii`.
- move fn `scalar_as_ascii_translit` to `Translit::ascii_scalar`.

### text::ascii
- new type `AsciiSet`.
- update `Digits`:
  - rename methods `*_omit0`  to `*_nonzero`.
  - new method for u16: `digits10_2`.
  - new methods for u16 and smaller impls: `write_digits10_fast`, `write_digits10_fast_nonzero`.
  - new methods for u32 and larger impls: `write_digits10_nonzero`, `write_digits10_fast_nonzero`.
  - make `write_digits10` always an exact-space writer. Update behavior for u8 and u16.
  - make `write_digits_10_fast` methods *const*.
  - remove methods: `digits10_str`, `digits16_str`.

### text::fmt
- new type `DebugWith`.
- update `DebugExt`; add new method `debug_with`.
- make `Fmt::from_fn` const.

### text::parse
- update `TextScanner`
  - new methods:
    - `eat_ascii_set`,  `skip_ascii_set`, `skip_until_ascii_set`, `take_ascii_ident_tail`, `take_ascii_set`, `take_ascii_run`, `take_until_ascii_set`.
    - `peek_char[u]`, `next_char[u]`, `take_char`, `eat_char[u]`, `take_char[u]_if`, `skip_char[u]_while`, `take_char[u]_while`.
    - `next_line`, `next_line_trimmed`, `next_line_trimmed_before`.
    - `skip_byte`, `take_quoted_basic_or_rest`.

- simplify methods impls: `take_ascii_ident`, `take_ascii_ident_tail`.

### text::str
- new types: `StrBuf`, `StringSmallAlloc`.
- impl `AddAssign` for `StringNonul` & `StringU*`.
- make `StringU*` use `NonMax*` for the length.
- remove `StringU32`, `StringUSize`.

#### text::unicode::scalar
- update `Char`:
  - improve documentation.
  - add methods: `as_char`, `ceil_utf8_boundary`, `floor_utf8_boundary`, `is_utf8_continuation`, `write_utf8_to`, `write_utf8_to_unchecked`.
  - add missing method for ref array: `to_char_unchecked`.
- rename `charu` methods:
  - `from_utf8_bytes` to `from_utf8`.
  - `from_utf8_bytes_unchecked`to `from_utf8_unchecked`.
  - `from_utf8_bytes_with_len` to `from_utf8_prefix`.
  - `from_utf8_bytes_with_len_unchecked` to `from_utf8_prefix_unchecked`.
  - `decode_utf8` to `_from_utf8_prefix_trusted`.

### ui::event
- new types: `EventButtons`, `EventTagSet`, `EventWheelUnit`.
- new alias `EventKindTimed`.
- update `Event`:
  - new method: `from_kind_timed_with`.
  - impl `From<EventKindTimed>.
- remove the `timestamp` field from: `EventKey`, `EventKeyFfi`, `EventMouse`, `EventPointer`, `EventWheel`.
- update `EventMouse`:
  - change the `buttons` field to use `EventButtons`.
- update `EventButtonState`: make `Pressed` the default.
- update `Event`:
  - add methods: `has_tag`, `is_in`, `is_wheel`, `some_wheel`.
- update `EventTag`:
  - derive `Copy`.
  - add `Wheel` variant.
  - define with `enumset!`.
  - add methods: `is_wheel`, `some_wheel`.
- update `EventKey`:
  - add methods: `new`, `press`, `modified_press`, `text`, `modified_text`, `with_state`, `with_mods`.
- update `EventKind`:
  - add `Wheel` variant.
  - add methods: `has_tag`, `is_in`, `is_wheel`, `some_wheel`.
- update `EventWheel`:
  - add fields: `unit`, `buttons`.
  - add many convenience methods.

## work
- new `work` submodules: `exec`, `plan`, `task`.
- move `thread` and `process` inside `exec`.
- rename `Task*` items to `Async*`.

#### work::exec::process
- update `cmd!` to parse single string literals as shell words.
- new `CommandFlow` and `ProcessExt` method: `command_shell`.

## yard
- new hidden maintenance aliases for item attributes:
  - `__doc_<show|hide>`, `__cfg_item_[un]safe[_hide]`, `_js_[un]safe_ffi[_doc]`, `_unsafe_syscall_not_miri`.
- update `_use_or_shim!` to add `_doc!` macro support.
- split a new `_doc_vendor!` macro out of `_doc!`
- update syntax of `_devela_policy`.

[0.28.0]: https://github.com/andamira/devela/releases/tag/v0.28.0
