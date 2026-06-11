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
- bump MSRV to 1.96.0.

### tools
- update `check.rs`: downgrade devela version to 0.26.0.
- update `x` workspace command wrapper: add support for flags with values.

## workspace
### examples
- new example scripts: `current_guard`, `otp`, `scope_guard`.
- update the examples: `term_linux`, `web_api`, `web_workers`.

### devela_ffi
- new experimental crate for ffi interop.

### devela_macros
- enable safety feature guards.
- remove `enumint` example.
- improve integration tests.
- new helper fns: `deny_tokens`, `error_tokens`, `warn_tokens`.
- update `compile_attr` to avoid reparsing input items.

## build
- new native section to detect native library availability.

## dependencies
- remove dependencies: `bytemuck`, `pastey`, `serde`.
- bump `portable-atomic-util` to 0.2.7.
- bump `hashbrown` to 0.17.
- bump `wide` to 1.3.

## docs
- new tags: `_TAG_[ABI|CRYPTO|GAME|LANG|MEMBER|ORD|PARSER|PROTOCOL|SET|SIGNAL|STRING]`.
- rename tag `_TAG_GEOM_DIR` to `_TAG_DIR`.
- make example-generated `*Example` items visible in the docs.
- enable `doc_cfg` for all doctest crates under `nightly_doc`.

## features & flags
- new features: `alsa`, `http`, `net`, `shell`, `web`, `_linux_abi`, `__disable_native_libs`.
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
- new types: `InfallibleResult`, `Version`, `VersionFull`.
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

### code::ops
- re-export the new range types.
- remove re-exports for legacy range types.

### code::util
- new attr-macros: `macro_apply`, `macro_derive`, `macro_derive_with`.
- new decl-macros: `macro_apply_alias!`, `macro_derive_alias!`, `macro_dollar!`, `maybe_slot!`, `paste!`, `read_at!`, `test_size_of!`, `use_as!`.
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
- update `is!`:
  - support branch-local statement semicolons and empty then/else arms.
- update `impl_trait!` to support custom attributes.
- update `unwrap!`: add missing arms: `ok_or?`, `sok_or?`.
- update `write_at!`:
  - accept the buffer as an expresion.
  - make it return the offset.
  - add `#expr` syntax support for writing unicode scalar values as UTF-8 bytes.
  - add special support for fixed-width unrolled spread sequences.
- reexport `cfg_select!`, `cold_path()`, `[debug_]assert_matches!`.
- remove `cfg_if!`, replace with `cfg_select!`.

## data
- new macro: `word!`.
- new trait: `Word`.

### data::access
- new type: [`ByteCursor`].

#### data::access::iter
- new `Iter` method: `from_coroutine`.
- stop re-exporting: `Iter<Chain|Cloned|Copied|Cycle|Empty|Enumerate|Filter|FilterMap|FlatMap|Flatten|FromFn|Fuse|Inspect|Map|MapWhile|Once|OnceWith|Peekable|Repeat|RepeatN|RepeatWith|Rev|Scan|Skip|SkipWhile|StepBy|Successors|Take|TakeWhile|Zip>`, `iter_from_coroutine`.

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

### data::error
- new type: [`UnexpectedEof`].

### data::id
- rename `define_handle!` to `handle!`

### data::layout
- update `buffer_linear!`:
  - add index type guards and optimize zero-index creation.
  - new methods for option and uninit impls: `clear_copy`, `truncate_copy`, `truncate_prim_copy`.
  - new method for option impl: `truncate_prim`.
  - made const fn all static `as_slice_mut` functions.
- rename `DstQueuePopHandle` to `DstQueuePopGuard`.

### data::topol
- rename `ConstListIterator` to `ConstListIter`; add new const-fn `next` method.

#### data storage::key
new types `SparseSet[Array|Error]`, `LinuxSparseSet`.
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

##### lang::prog::ffi::c
- new type aliases: `c_bool`, `c_size_t`, `c_ssize_t`.

##### lang::prog::ffi::js
- rename `JsTextMetrics*` to `JsTextRenderMetrics*`.

#### lang::prog::script
- new submodule: `shell`.
- new types: `ShellLex`, `ShellQuote`, `ShellWordError`.

### media::audio
- new submodules: `acoustic`, `effect`, `format`, `instrument`, `music`, `rhythm`, `synth`.
- new `AudioChannels::MAX_COUNT` constant.

#### media::audio::format
- new types: `PcmRaw`, `PcmRawBuf`, `PcmRawError`, `PcmWav`, `PcmWavBuf`, `PcmWavError`, `PcmWavFmt`.

#### media::audio::pcm
- new traits: `PcmDrain`, `PcmSampleType`, `PcmSink`, `PcmSinkPlanar`, `PcmSource`, `PcmSourcePlanar`, `PcmStream`.
- new types: `PcmBuf`, `PcmLayout`, `PcmSample`, `PcmSampleType`, `PcmSpec`.

### media::font
- overhaul `FontBitmap`.

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
- new type `BitSpan`.
- update `Bitwise`:
  - rename checked methods to have `_checked` suffix.
  - rename `set_checked_value_checked_range` to `set_value_range_checked_strict` and fix.

#### num::fin::bit
- update `Order` with row-col methods.

### num::grain
- rename `Primitive*` traits to `Prim*`.
- rename `define_lane!` to `lane!`

#### num::grain::niche
- new macro `niche!`.
- remove macros: `nm!`, `nv!`, `nz!`.
- rename `NonExtremeI*` to `NonMin*` and `NonExtremeU*` to `NonMax*`.
- add more comparsion methods to `MaybeNiche`: `ne`, `gt`, `ge`, `lt`, `le`.

#### num::grain::wide
- update `lane!`:
  - document methods in `Lane4_i32Example`.
  - rename `from_bytes` to `from_byte_values`.
  - fix `sub_assign_wide`.

### num::prob
- new module `phys::prob::markov`.

#### num::prob::rand
- new traits: `FromRand`, `FromRandTry`, `RandSeedable`, `RandTry`.
- new types `RandQualities`, `SplitMix64`, `StdRand`.
- change the `Rand` trait to depend on `RandTry`.
- remove the traits: `RandAlloc`, `RandStd`.
- rename `define_pcg!` to `rand_pcg!`
- rename `define_xorshift!` to `rand_xorshift!`.
- feature-gate `XorShift128p`.

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
- new submodule `run::app`.
- update `RunRender` and `RunPresent`:
  - support borrowed render artifacts with GATs.
  - make them use a borrowed `RunFrame`.
  - make `RunRender`'s `S: ?Sized`.

### run::app
- new type: `AppControl`.

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
  - correctly parse KeyMods for mouse and wheel events.
- fix `XkbState::key_mods` conversion.
- fix `XWindow::clear_redraw`.

### sys::env
- update `Env`:
  - make all environment constants available without `std`.
  - add `TARGET`, `FAMILIES`, `ENV`, `ABI`, `ENDIAN`, and `POINTER_WIDTH`.
  - derive `DLL_*` and `EXE_*` constants from target information.
  - align `ARCH`, `OS`, `FAMILY`, `VENDOR`, with Cargo/rustc target cfg values.

### sys::fs
- rename `PathPrefix` to `PathWindowsPrefix`, `PathPrefixComponent` to `PathWindowsPrefixComponent`.
- make `path` submodule public.

### sys::io
- new traits `TextIn`, `TextOut`.

### sys::log
- new modules `sys::log::{bench, trace}`.
- new types: `DiagLevel`, `DiagOut`.

### sys::mem
- rename `define_arena!` to `arena!`.

#### sys::mem::alloc
- new type `LinuxMmapAlloc`.
- update `Alloc`:
  - add methods: `random_weak_u64`, `random_weak_bytes`.
  - impl `RandTry`.

### sys::mem::cell
- new types: `MemHedgeCtrl`, `MemHedgeError`, `MemHedgeRead`, `MemHedgeState`.

### sys::mem::view
- update `Slice`: add methods: `get`, `get_mut`:
- new types: `MemReplicaError`, `MemReplicaSlice`.

### sys::mem::size
- refactor the `bit` submodule.

### sys::net::http
- new module.
- new types: `Http`, `HttpError`, `HttpMethod`, `HttpRequestLine`, `HttpResponseHead`, `HttpStatus`, `HttpStatusClass`, `HttpVersion`.

### sys::os
- remove macros: `os_print!`, `os_println!`, `os_eprint!`, `os_eprintln!`.

##### sys::os::browser::web
- new type `WebEventWheel`.
- update `WebEventKind`:
  - add `Wheel` variant, update associated values.
- add new `mods` field to: `WebEventMouse`, `WebEventWheel` and `WebEventPointer`.
- add new `kind`, `button` and `buttons` fields to: `WebEventPointer`.
- bring here impls from `ui::event`.
- feature-gate with `event` the event-related impls.
- rename all methods `[from|to]_js*` to `[from|to]_web*`.
- replace `ui::event::KeyState` methods: `[from|to]_js` with `WebEventKind` methods: `[to|from]_key_state`.
- replace `ui::event::EventMouse` methods: `[from|to]_js` with `WebEventMouse` methods: `to_kind_timed`, `from_event_mouse_timed`.
- replace `impl From<WebEventMouse> for EventMouse` with one `for EventKindTimed`.
- replace `impl From<EventMouse> for WebEventMouse` with one `From<Timed<EventMouse, Option<EventTimestamp>>>`.
- impl `WebEventPointer` methods: from_event_pointer_timed and to_kind_timed.
- allow receiving duplicated mouse and pointer web events.

#### sys::os::linux
- new type `LinuxRandomMode`.
- update `LinuxError` conversion to `IoError`.
- feature-gate term-related functionality.
- refactor Linux signal handlers around RT sigaction.
- update `Linux`:
  - new methods: `random_*_with`, `scoped_event_mode`, `scoped_termios_update`.
  - rename method: `disable_raw_mode` to `reset_cooked_mode`.
  - make `scoped_raw_mode` method return crate-private `LinuxTermModeGuard`.
  - make the default random mode cryptographically secure.

#### sys::os::linux::io
- new types: `LinuxTermios<Input|Output|Control|Local>Flags`, `LinuxTermiosCC`, `LinuxTermiosCharSize`.
- update `Linux`:
  - change the signatures of the methods: `sig_handler`, `sig_handler_info` to improve ergonomics.
- update `LinuxTermios`:
  - new methods: `update_state`, `enable_event_mode`, `make_event`, `make_raw`, `make_cooked_reset`, `set_echo`, `set_canonical`, `set_signals`, `set_extensions`, `set_break_interrupt`, `set_input_cr_to_lf`, `set_software_flow`, `set_output_processing`, `set_read_min_timeout`.
  - rename methods: `get_state` to `read_state`, `save_state` to `write_state`, `get_winsize` to `read_window_size`, `disable_raw_mode` to `reset_cooked_mode`
- make `LINUX_TERMIOS_*` type namespaces private.

#### sys::os::linux::process
- new types: `LinuxSigactionFlags`, `LinuxSignal`, `LinuxSignalSet`.
- make `LINUX_SIG*` type namespaces private.
- fix signal restorer & `sys_getpid` in x86.

#### sys::os::term
- new types: `AnsiLink`, `AnsiOsc`, `TermCap`, `TermCaps`, `TermInputParser`, `TermLineMode`, `TermLinux`, `TermMode`, `TermPen`, `TermPollPolicy`, `TermRenderer`, `TermSession`.
- update `Ansi`:
  - add new functions: `title`, `title_icon`, `title_window`, `link`, `link_with_id`, `clipboard_base64`, `clipboard_query`, `clipboard_query_clipboard`, `QUERY_DEC_PRIVATE_MODE_N`, `DEFAULT_UNDERLINE_COLOR`, `UNDERLINE_<COLOR8|RGB>`, `default_<fg|bg>_<query|reset>`, `cursor_color_<query|reset>`.
  - add new constants: `ESC`, `BEL`, `OSC`, `SS3`, `ST`, `HYPERLINK_CLOSE`, `DISABLE_MOUSE[_<MOTION|SGR|SGR_PIXELS>]`, `<EN|DIS>ABLE_<ALTERNATE_SCREEN_ONLY|BRACKETED_PASTE|FOCUS_EVENTS|SYNC_UPDATE`, `QUERY_*`.
- new font-related constants: `FONT`, `HIDDEN[_OFF]`, `BLINK_FAST`, `FRAKTUR`, `FRAME`, `ENCIRCLE`, `FRAME_ENCIRCLE_OFF`, `OVERLINE[_OFF]`, `<SUPER|SUB>SCRIPT`, `BASELINE`, `UNDERLINE_<DOUBLE|CURLY|DOTTED|DASHED|DOUBLE_ECMA>`,.
  - remove `MOUSE_UTF8`.
  - rename `MOUSE_X10_<EN|DIS>ABLE` to `<EN|DIS>ABLE_MOUSE_X10`.
  - rename `MOUSE_SGR_PIXELS` to `ENABLE_MOUSE_SGR_PIXELS`.
  - rename `MOUSE_TRACKING` to `ENABLE_MOUSE_MOTION`.
  - rename `MOUSE_SGR` to `ENABLE_MOUSE_SGR`.
  - rename `MOUSE_NORMAL` to `ENABLE_MOUSE`.
  - fix `<SET|RESET>_PALETTE`.
- update `ansi!`:
  - add support to write to a renderer.
  - type-check the fallback backend print arms.

#### sys::os::term::grid
- new items: `TermColor`, `TermColors`, `TermColorKind`, `TermColorMode`, `TermGrid`, `TermGridError`, `TermOccupancy`, `TermStyle`, `TermStyleExt`, `Termel`, TermelMeta,

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
new type: `Textel`.

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
  - add methods: `as_char`, `ceil_utf8_boundary`, `floor_utf8_boundary`, `is_utf8_continuation`, `random_from_seed`, `random_next`, `scalar_from_rank`, `scalar_rank`, `write_utf8_to`, `write_utf8_to_unchecked`.
  - add missing method for ref array: `to_char_unchecked`.
- rename `charu` methods:
  - `from_utf8_bytes` to `from_utf8`.
  - `from_utf8_bytes_unchecked`to `from_utf8_unchecked`.
  - `from_utf8_bytes_with_len` to `from_utf8_prefix`.
  - `from_utf8_bytes_with_len_unchecked` to `from_utf8_prefix_unchecked`.
  - `decode_utf8` to `_from_utf8_prefix_trusted`.

## ui
### ui::event
- new types: `EventButtons`, `EventTagSet`, `EventWheelUnit`.
- new alias `EventKindTimed`.
- update `Event`:
  - new method: `from_kind_timed_with`.
  - impl `From<EventKindTimed>.
- remove the `timestamp` field from: `EventKey`, `EventKeyFfi`, `EventMouse`, `EventPointer`, `EventWheel`.
- update `EventMouse`:
  - change the `buttons` field to use `EventButtons`.
  - add a `mods` field.
- update `EventButtonState`: make `Pressed` the default.
- update `EventButton`:
  - change `Other` repr to u8.
  - add variants: `X1`, `X2`, `X3`, `X4`, `X5`.
  - rename method `primary_from_mask` to `from_one_bit_mask`.
  - new method `to_mask`.
  - fix method `to_web`.
- update `Event`:
  - add methods: `has_tag`, `is_in`, `is_wheel`, `some_wheel`.
- update `EventTag`:
  - add variants: Control, `Wheel`.
  - add methods: `is_wheel`, `some_wheel`.
  - define with `enumset!`.
  - derive `Copy`.
- update `EventKind`:
  - add variants: `Control`, `Wheel`.
  - add methods: `has_tag`, `is_in`, `is_wheel`, `some_wheel`.
- update `EventKey`:
  - add methods: `new`, `press`, `modified_press`, `text`, `modified_text`, `with_state`, `with_mods`.
- update `KeyMods`:
  - add methods: `from_web`, `to_web`.
  - change implementation to use `set!`.
- update `EventPointer`:
  - add fields: `buttons`, `mods`.
- update `EventWheel`:
  - add fields: `unit`, `buttons`, `mods`.
  - add many convenience methods.
- update `EventWindow`: remove variants: `Continue`, `EndOfInput`.
- rename `EventPointerType` to `EventPointerKind`.

## work
- new `work` submodules: `exec`, `plan`, `task`.
- move `thread` and `process` inside `exec`.
- rename `Task*` items to `Async*`.

#### work::exec::process
- update `cmd!`:
  - support parsing single string literals as shell words.
  - require the @ prefix to split a single string literal.
- new `CommandFlow` and `ProcessExt` method: `command_shell`.

#### work::exec::thread
- update `sleep4!`:
  - support no_std sleeping in linux.
  - support receiving seconds as floating-point.

## yard
- new macros: `_doc_meta!`, `_doc_test_size_of!`.
- new hidden maintenance aliases for item attributes.
- update `_use_or_shim!` to add `_doc!` macro support.
- split a new `_doc_vendor!` macro out of `_doc!`
- rename `_doc_miri_warn!` to `_doc_warn_miri!`.
- update syntax of `_devela_policy`.

[0.28.0]: https://github.com/andamira/devela/releases/tag/v0.28.0
