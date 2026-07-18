# Changelogs from 2026

[0.28.0] 2026-06-12
===================

> One must still have chaos in oneself to give birth to a dancing star.
> — Friedrich Nietzsche

```
This release consolidates a wide platform-facing cycle of devela, expanding
terminal, web, X11, audio, raster, event, runtime, and system foundations,
while pruning older surfaces and tightening the crate toward a clearer 0.29.
```

## Key changes:

- msrv bump: minimum supported rust version increased to 1.96.0.
- platform foundations: expand Linux, terminal, X11, web, and FFI support.
- media groundwork: expand audio, image, raster, font, and color abstractions.
- data/text cleanup: add cursors, packing, crypto, scanners, and string utilities.
- event unification: normalize keyboard, mouse, pointer, window, and timing events.
- terminal runtime: add sessions, input parsing, capabilities, grids, and rendering.
- macro refinement: improve generator macros, set utilities, diagnostics, and helpers.
- feature/build cleanup: clarify gates, target reflection, native detection, and tooling.

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
- bump `hashbrown` to 0.17.
- bump `wide` to 1.5.

## docs
- new tags: `_TAG_<ABI|CRYPTO|GAME|LANG|MEMBER|ORD|PARSER|PROTOCOL|SET|SIGNAL|STRING>`.
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
  - new methods: `emit_<check_cfg|checked_flag|env|env_marker|flag|flag_if|flag_if_lib>`, `has_lib`.

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

### code::result
- stop re-exporting: `<Option|Result><IntoIter|Iter|IterMut>`.

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
- new type: `ByteCursor`.

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
- new type: `UnexpectedEof`.

### data::id
- rename `define_handle!` to `handle!`

### data::layout
- update `buffer_linear!`:
  - add index type guards and optimize zero-index creation.
  - new methods for option and uninit impls: `clear_copy`, `truncate_copy`, `truncate_prim_copy`.
  - new method for option impl: `truncate_prim`.
  - made const fn all static `as_slice_mut` functions.
- rename `DstQueuePopHandle` to `DstQueuePopGuard`.
- stop re-exporting: `<Option|Result><IntoIter|Iter|IterMut>`.

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
- new types: `RandFake`, `RandQualities`, `SplitMix64`, `StdRand`.
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
- stop re-exporting: `Iter<Args|ArgsOs|Vars|VarsOs>`.

#### sys::fs::path
- make module public.
- rename `PathPrefix` to `PathWindowsPrefix`, `PathPrefixComponent` to `PathWindowsPrefixComponent`.
- stop re-exporting: `Iter<SplitPaths|PathAncestors|PathComponents|Path>, PathDisplay`.

### sys::io
- new traits `TextIn`, `TextOut`.
- stop re-exporting: `Std<in|err|out>Lock`.

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

#### sys::mem::cell
- new types: `MemHedgeCtrl`, `MemHedgeError`, `MemHedgeRead`, `MemHedgeState`.

#### sys::mem::view
- update `Slice`: add methods: `get`, `get_mut`:
- new types: `MemReplicaError`, `MemReplicaSlice`.

#### sys::mem::size
- refactor the `bit` submodule.

### sys::net
- stop re-exporting: `TcpIncoming`.

#### sys::net::http
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
- rename all methods `<from|to>_js*` to `<from|to>_web*`.
- replace `ui::event::KeyState` methods: `<from|to>_js` with `WebEventKind` methods: `<to|from>_key_state`.
- replace `ui::event::EventMouse` methods: `<from|to>_js` with `WebEventMouse` methods: `to_kind_timed`, `from_event_mouse_timed`.
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

[0.27.0] 2026-04-10
===================

> The creation of a thousand forests is in one acorn.
> — Ralph Waldo Emerson

```
This release reunifies devela's split base layers back into the main crate,
simplifying the workspace while preserving what was learned from the split.
```

# Key changes:
- workspace reunification: merge the split base library crates back into devela.
- structural simplification: remove duplicated scaffolding, and workspace indirection.
- maintenance reduction: simplify build, feature, and module organization across the project.

------------------------------------------------------------------------------

# Project

## workspace
- reduce the workspace to `devela` and `devela_macros`.
- reunify the split `devela_base_*` crates back into `devela`.

## build & docs
- simplify internal build and documentation plumbing after the workspace reunification.

## features & flags
- simplify feature wiring previously spread across the split base crates.

## metrics
- reduce packaged file count from 1385 to 1027 files (-25.84%).
- reduce workspace source totals from 158039 to 150513 lines (-4.76%).
- reduce workspace code totals from 101195 to 96506 lines (-4.63%).
- compile times: local cold-build measurements showed a modest regression in regular builds,
  roughly from +4% to +8%, while documentation builds improved by roughly 8% to 12%.

---

# Modules

## code
- merge `ConstInitCore` and `ConstInit` into `ConstInit`.

### data::layout
- remove `SortAlloc`; merge its methods in `Sort`.

#### media::visual::color
- merge `GammaConst` and `Gamma` into `Gamma`.

#### num::grain::niche
- impl `ConstInit` for `NonValue*` using `0` when allowed.

### num::int
- remove `IntAlloc`; merge its methods into `Int`.

#### num::real::float
- remove `FloatStd`; merge its methods into `Float`.

### sys::mem
- fix standalone `guard.rs` example.

## yard
- add the new workspace-internal `_use_or_shim!` macro.

[0.27.0]: https://github.com/andamira/devela/releases/tag/v0.27.0

[0.26.0-wip] 2026-04-06
=======================

> Everything must ripen before it is ready.
> — Rainer Maria Rilke

```
This release expands devela's foundational surface with new runtime, time,
parsing, storage, and drawing abstractions, while refining geometry, text,
and system interfaces toward clearer typed contracts and deeper coherence.
```

## Key changes:
- runtime foundations: new runtime, cycle, pacing, and presentation abstractions.
- typed time overhaul: reworked time sources around clearer point and elapsed models.
- geometry refinement: clearer metric organization and a more expressive region model.
- text parsing groundwork: new scanner, range, and reusable parsing error infrastructure.
- storage and drawing expansion: new storage roots and canvas-oriented drawing traits.
- system cleanup: improved X11 handling, refined FFI gating, and Linux clock fixes.
- data layout improvements: expanded buffer utilities for alloc and non-alloc use.
- msrv bump: minimum supported rust version increased to 1.94.1.

------------------------------------------------------------------------------

# Project

## infra
### cargo
- bump MSRV to 1.94.1.

### CI
- bump `actions/checkout` to v6.
- bump `upload-artifact` to v7.
- bump `docker/setup-qemu-action` to v4.

## workspace
### examples
- fix examples: `web_api`, `web_worker`.

### tools
- update `tools/check.rs`:
  - bump `devela` to 0.25.0.
- update `x` command wrapper to apply cfg flags in rustdoc.

## dependencies
- bump dependencies:
  - `portable-atomic-util` to 0.2.6.
  - `wide` to 1.2.

## features & flags
- make `x11` feature auto-enable: `alloc` and `event`.

---

# Modules

### code::ops
- re-export `punroll!` from devela.

#### data::access::iter
- add `is_empty` methods to `iter_strided!` items.

### data::layout
- update `buffer_linear!`:
  - derive `Default` for impls: `option`, `uninit`.
  - new methods: `push_slice`, `remaining_capacity`, `remaining_capacity_prim`.
  - new methods for non-alloc backends: `push_slice_copy`, `push_slice_copy_exact`.
  - update `vec` backend methods:
    - update `is_full` semantics.
    - fix `len` implementation.

### data::store
- new module.
- move `data::id::key` to `data::store::key`.

## geom
- remove all `*_ref` and `*_mut` accessor methods for `dir` and `metric` types.

### geom::dir
- new aliases: `Orientation<1|2|3>`.

### geom::metric
- make the module public.
- fix re-export of `RegionStrided`.
- new aliases: `Distance<1|2|3>`, `Extent<1|2|3>`, `Position<1|2|3>`, `Region[S]<1|2|3>`.
- update `Extent`:
  - move the accesor methods from being macro implemented to generic for `T: Copy`.
  - add the doc-alias `Size`.
- update `Region`:
  - change the single `T` generic into two `P` and `E` generics.
  - remove methods: `extent`, `position`.
  - rename `size` field to `ext`.
  - add tuple constructors.

### lang::disc
- new submodules: `move`, `narr`.

#### media::visual::draw
- new traits: `Canvas`, `CanvasRead`, `CanvasTextel`.

#### num::dom::int
- update `Int`:
  - use rust's implementations in `midpoint` methods.
  - rename old signed `midpoint` method to `midpoint_floor`.

#### num::dom::real::float
- fix `Float::ln_series` algorithm.

#### phys::time::source
- new trait: `TimePoint`.
- update TimeSurce and TimeSourceCfg:
  - make them generic over `TimePoint`.
  - replace required `time_now_millis*` trait surface with: `time_now`, `time_point_value`, `time_elapsed_value`.
  - add shared default helpers for point/elapsed conversion and elapsed-since queries.
- rework source impls to typed point forms:
  - `SystemTime`: `SystemTime`, `u64`, `u32`.
  - `SystemInstant`: `SystemInstant`, `u64`, `u32`.
  - `JsInstant`: `u64`, `u32`.
  - `LinuxInstant`: `u64`, `u32`.
  - `LinuxTime`: `TimeSourceCfg<u64>`.
  - `TimeFakeRef`: `TimeSourceCfg<u64>`.

## run
- new traits: `RunApp`, `RunBackend`, `RunRender`, `RunPresent`.

### run::cycle
- new items: `RunControl`, `RunCycle`, `RunPhase`.

### run::time
- move `RuntimeTick` to [base].
- new items: `RunPacer`, `RunStep`, `Runtime`.

##### sys::device::display::x11
- change `unsafe_syscall` feature-gate to `unsafe_ffi`.
- update `XWindow`:
  - new methods: `destroy`, `extent`, `position`, `clear_redraw`, `needs_redraw`.
  - remove method `size`.
  - method `new` now requires an exclusive reference to `XDisplay`.
  - move the inner state to a new `XDisplay`'s window registry.
- update `XDisplay`:
  - only emit window move and resize events when they're actually different.
  - emit events with the right window target.
  - add a managed window registry.
- new raw fns: `xcb_free_gc`, `xcb_destroy_window`.

#### sys::os::c
- change `unsafe_syscall` feature-gate to `unsafe_ffi`.

#### sys::os::linux
- update `LinuxClock::is_monotonic` to include `ProcessCpuTimeId` and `ThreadCpuTimeId`.

## text
- new type `TextRange`.
- new private module `text::unit`.
- move `TextCursor`, `TextIndex` and `TextUnit` out of `text::layout`.

### text::layout
- rename `TextSpan` to `TextLayoutSpan`.
  - refactor to include `TextRange`.
  - new methods: `from_prim`, `with_range`, `start`, `end`.

### text::parse
- new items: `TextParseError`, `TextParseErrorKind`, `TextScanner`.

### ui::event
- update `EventWindow` variants:
  - `Resized` now contains `Extent`.
  - `Moved` now contains `Position`.
- update `Event`:
  - new methods: `from_window`, `from_device`.
- update `EventTarget`:
  - new methods: `some_window`, `some_device`.
- impl `From<u32>` for `WindowId` and `DeviceId`.

### vita::play::game
- new submodule.
- new structs: `GameAction`, `GameCycle`, `GameLegacy`, `GameOutcome`, `GamePhase`, `GameRole`, `GameSession`, `GameTurn`.

[0.26.0]: https://github.com/andamira/devela/releases/tag/v0.26.0

[0.25.0] 2026-03-11
===================

> A place for everything, and everything in its place.

```
This release changes the library's structure from a single crate to multiple crates,
in order to try to improve compile times while maintaining most of the cohesiveness.
Many feature gates have been removed in order to make the features always available.
```

## Key changes:
- core extraction: separate foundational types, traits, and macros into base crates.
- feature consolidation: removed feature gates, make functionality always available.
- text system overhaul: major improvements to scalar, string, and grapheme handling.
- const evolution: many methods made const across numeric, text, and system modules.
- build system enhancements: improved build configuration and post-build processing
- memory & system improvements: enhanced slice operations and system arches support.
- error system refinement: updated error macros and type organization.
- msrv bump: minimum supported rust version increased to 1.93.0.

------------------------------------------------------------------------------

# Project

## infra
### build
- define `CRATE_NAME` constant.
- move `/meta/build` to `/build/main`.
- move build fn utils as `Build` methods.
- new `Build` namespace in `devela_base_std`.
- make `devela_base_std` optional for builds.
- add rerun instructions for changed env vars.
- new internal env vars `__DEVELA_MEMBER`, `__DEVELA_MEMBER_NAME`.
- make sure `CARGO_TARGET_DIR` and `CARGO_WORKSPACE_DIR` are always defined.
- add new `devela_postbuild` crate to `build/post`.
- add build config flag aliases: `any_target_arch_linux`, `any_target_arch_riscv`.
- add [base] symlinks to `devela/main/<alias|environment|features>.rs`.
- move `/config/dep_all.rs` to `/build/main/dep_all`.

### cargo
- bump MSRV to 1.93.0.
- add new cargo env var `CARGO_WORKSPACE_DIR`.
- fix updated syntax for unstable cargo-include in `.cargo/config.toml`.

#### config
- remove `./cargo/nightly.toml`.
- add cargo aliases: `L0r`, `w*` (workspace).

#### manifest
- add workspace hierarchy diagram.
- add *binaries* and *metrics* sections.
- add lints: `missing_debug_implementations`, `unused_features`.
- make keys parts of the workspace: edition, version, authors, license, documentation.
- simplify debugging info in `dev` profile.
- add `debugging` profile.

### tools
- new `x` workspace command wrapper.
- new files in `tools`: `x.fish`, `x.sh`, `x-env-common.sh`, `x-env-native.sh`, `x-env-nightly.sh`.
- remove `tools/cargo-native`.
- update `tools/check.rs`:
  - bump `devela` to 0.24.0.
  - test all workspace crates.
  - start testing without dependencies.
  - switch rust-script for cargo-script.
  - simplify and homogenize toolchain selection syntax.
  - configure the exact nightly version to install and use.

### CI
- bump `actions/checkout` to v5.
- add more `no_std` targets, retry downloads and disable fail-fast.

## workspace
- new `/crates/` directory.
- add `devela_sentinel` crate.
- remove the `game` root module.
- declare the `std` external crate.
- add `_reexports` structural modules.
- remove `_always` structural modules.
- add new root modules: `org`, `vita`.
- refactor all structural access modules.
- enable `_docsrs` for workspace dependencies.
- support having external optional dependencies.
- new workspace library crates: `devela_base_alloc`, `devela_base_core`, `devela_base_macros`, `devela_base_std`.
- prepare future workspace library crates related to root modules.
- use a single version, changelog and readme for all workspace libs.
  - move `devela_macros` changelog into `devela` archived changelog history.
  - replace `paste` dependency with `pastey` and move to [base].
- add flat re-exports of root modules to `zall_` & re-export hidden as `all_`.
- rename `all` to `zall` & re-export hidden as `all`.
- rename all `lib.rs` to `index.rs`.
- rename `_info` to `_doc`.
  - move `config/rustdoc-header.html` to `src/_doc/header.html`.
  - update `src/_doc/header.html` to support multiple crates with custom whitelists.

### [base]
- add `_workspace_internal` structural module (replacing `_internal`).

#### `devela_base_macros`
- move devela_macros macros: `devela_macros`: `cif!`, `compile!`, `compile_attr!`, `ident_total!`, `ident_total_unique!`, `ident_unique!`, `coalesce!`, `field_of!`.
- new macro: `repeat!`.
- new compiler predicates: `env`, `env_eq`, `env_ne`, `env_empty`, `env_nonempty`, `nota`.

### `devela_macros`
- use workspace's crate version.
- make it an optional dependency.
- add `devela_base_core` as a dependency.
- enable `doc_cfg` via `nightly_doc` flag.
- remove dependency `hashbrown`.
- remove features: `alloc`, `std`, `nightly`, `nightly_doc`.

## dependencies
- make all optional external optional dependencies part of the workspace.
- move `core`, `alloc` & `std` re-exports to `src/yard/_dep`.
- remove `_core` and `_dep` re-exports from public docs.
- move `_dep` to `yard/_dep` & re-export from the root.
- re-export hidden workspace dependencies from `_dep`.
- re-export `alloc` from devela and [base_alloc].
- bump dependencies:
  - `hashbrown` to 0.16.
  - `memchr` to 2.8.
  - `rand_core` to 0.10.
  - [macros]:
    - `proc-macro2` to 1.0.101.
    - `quote` to 1.0.45.
- remove dependencies:
  - `const-str`, and related `str!` macro.
  - `crossterm` and `miniquad`; move to revela.
  - `libm` and related `Float` and `ExtFloat` functionality.
  - itertools and related re-exports.
  - remove: `allocator-api2`, `bumpalo`, `fontdue`, `ffmpeg-the-third`, `flume`, `fontdue`,  `gilrs`, `image`, `kira`, `midir`, `rayon`, `regex-lite`, `rodio`, `sdl2`, `sdl3`, `stringzilla`, `symphonia`, `sysinfo`, `toml_edit`, `tokio`, `unicode-segmentation`, `unicode-width`, `ureq`, `winnow`.
- add optional dependencies to [base]: `memchr`, `rand_core`, `simdutf8`.

## features & flags
- new features: `__publish`, `__std`, `__docs_internal`, `_docs_examples`, `_docs_max`, `event`, `grapheme`, `int`, `org`, `safe_build`, `safe_org`, `safe_vita`, `translit`, `vita`, `x11`.
- remove features: `_bit*`, `_char*`, `_cmp*`, `_float_*`, `_int_*`, `_num?_all`, `_sort*`, `_str_*`, `_str_nonul`, `_str_u*`, `_text_all`, `ascii`, `cast`, `desk`, `error`, `fmt`, `join`, `metric`, `nightly_bigint`, `prim`, `safe_layout`, `split`, `str`.
- remove flags: `bit··`, `char··`, `cmp··`, `_float··`, `_int*··`, `_nums··`, `prim··`, `sort··`, `str··`, `str_u··`.
- add an additional `nightly_stable_1_??` flag for the 3rd next version.
- rename:
  - `_docs` to _`docs_min`.
  - `_docsrs` to `_docs`.
  - `_docsrs_nodep` to `_docs_nodep`.
  - `__no_test` to `__exclude_test`.
  - `linear` to `lin` and move from `geom` to `num`.
- add default feature `alloc` to [base_alloc].
- add default feature `std` to [base_std].

## metrics
- rename directory `/benches` to `/metrics`.

---

# Modules

## code
- rename `ExtAny` to `AnyExt`.
- new trait: `ConstInitCore`.
- new types: `CodeLocation`, `CodeSpan`.
  - implement for `NonZero*` and many other types.
- move to [base]:
  - all `ConstInit*` impls.
  - `impl_cdef!` workspace-internal macro.
    - modify it to receive the trait as an argument.
- move `ScopeGuard` to [base_core].

### code::error
- update `define_error!` macro.
  - move to `code::error`.
  - update docs, add example.
  - allow accepting multiple tags.
  - do not automatically derive `Default`.
  - make conversion method optional const.
- remove items: `AllError`, `AllResult`, `DataError`, `DataResult`, `ExtError`.
- update `ArrayFmt` to support the rest of the core formatting traits.

### code::marker
- make `TypeResource` and `type_marker!` constructors *const*.
- new traits: `Prim`, `PrimFitPtr`, `PrimIndex`.

### code::ops
- new macro: `punroll!`.
- new types: `CallSemantics`, `CallBindTime`, `CallContext`, `CallDispatch`, `CallOpenness`, `CallStorage`.

### code::panic
- move to [base]: `Panic`.

### code::result
- move to [base]:
  - functions: `serr`, `sok`.
  - macros: `unwrap!`.
  - traits: `Morph`, `Hook`, `OptionExt`, `OptResExt`, `ResultExt`.
  - types: `Mismatch`, `OptRes`, `OptionFmt`, `OptionFmtOr`, `OptionFmtOrElse`, `Own`.
- new macros: `hook!`, `morph!`.
- rename:
  - `Chain` to `Morph`.
  - `ExtOption` to `OptionExt`.
  - `ExtOptRes` to `OptResExt`.
  - `ExtResult` to `ResultExt`.
- update `unwrap!`:
  - add arms: `err?`, `err_map*`, `ok_err_map`, `ok_if*`, `ok_map*`, `ok_some_map`, `some_if*`, `some_map*`, `some_ok_map*`.
  - receive expect `$msg` args as `expr` instead of `literal`, to be compatible with `concat!`.
  - make *const* the arms that call `unreachable`.
  - rename arms:
    - `ok_map_err`? to `ok_err_map?`.
    - `ok_if_map_err`? to `ok_if_err_map?`.

### code::utils
- new macros: `compile_warn!`, `doc_location!`, `doclink!`, `fn_name!`, `lets!`, `mod_path!`, `repeat!`, `type_count!`, `whilst!`, `write_at!`.
- move to [base]:
  - public macros: `CONST!`, `assert_eq_all!`, `assert_approx_eq_all!`, `capture_first!`, `capture_last!`, `capture_tail_tuple!`, `cfg_if!`, `const_assert!`, `define_error!`, `deprecate!`, `enumset!`, `ident_const_index!`, `impl_trait!`, `include_from!`, `is!`, `items!`, `maybe!`, `methods_as_fns!`, `mod_from!`, `sf!`, , `structural_mods!`, `type_marker!`.
  - internal macros: `__crate_name!`, `__dbg!`, `__std!`, `_EMOJI_*`, `_TAG_*`, `_doc!`, `_doc_availability!`, `_doc_miri_warn!`,  `_reexport!`, `_tags!`, `_use!`.
- add tags: `_DOC_*`, `_TAG_<ALG|APPLE|ASSERT|AUDIO|BIT|CODE|CODEC|CODEGEN_BUILD|CONSTRUCTION|COLOR|CONCURRENCY|DATA|DEBUG|EVENT|EXAMPLE|EXEC|FS|GEOM_DIR|GUARD|HASH|ID|IMAGE|INIT|INTERACTION|INTERNAL|INTROSPECT|IO|LAYOUT|LIFETIME|LIN|LINUX|LIST|LOGIC|MAYBE|MEM|PLATFORM|PROC_MACRO|RUNTIME|SYMB|TERM|UNIX|VALUE|WAVE|WINDOWS|WIP>`.
- change the emoji for `_TAG_DATA_STRUCTURE`.
- new re-exports: `select_unpredictable`.
- new functions: `cold_path`, `likely`, `unlikely`.
- rename `reexport!` internal macro to `_reexport!`.
  - mark as `doc(no_inline)`.
  - allow accepting multiple tags.
  - fix rendering of std path links.
- prefix internal constants `TAG_*` & `EMOJI_*` with `_`.
- define `_std_core` separately and privately per crate.
- update `CONST!` macro with new arms: `hidden macro_export`, `inline macro_export`.
- update `impl_traits!` macro:
  - add new arms for: `Display+Error`, `FromStr`, `PartialEq`.
  - change syntax from a single <gen> group to a double [decl][args] group, to support const generics.
- update `const_assert!` macro:
  - add new arms: `ne_buf`, `ne_str`.
  - add support for comparing slices of primitives and slices of slices of primitives.
- update `is!` macro.
  - change expr separators from `;` to `,`, to reduce potential ambiguity.
  - remove temporary value binding functionality unnecessary after rust v1.89.
- remove vendored macro `cfor!`, replace with `whilst!`.
- remove deprecated `iif!` macro.

## data
- move to [base]:
  - macros: `bitfield!`, `init_array!`.
  - traits: `BitOps`.
  - types: `ArrayFrom`, `Bitwise`, `Oneof`, `Sort`.
- remove module `data::list`.
- new macro: `define_handle!`.
- new type: `HandleExample`.
- new `SortAlloc` wrapper for `Sort`.
- make `Sort` methods take `&mut self` instead of `self`.
- make `Sort` public `quick_*` methods take `&mut self` as well.

### data::access
- new module `data::access`.
- move here `address` & `iter`.

#### data::access::iter
- new traits: `IteratorLending`, `IteratorLendingDoubleEnded`, `IteratorLendingExactSize`, `IteratorLendingPeek`, `IteratorLendingPeekDoubleEnded`.
- new macro: `iter_strided!`.
- new types: `StridedIter`, `StridedIterMut`.

### data::codec
- move here: `bit/`.
- rename `serde/` to `deser/`.

#### data::codec::bit
- move `BitOps` & `Bitwise` to `num::bit`.
- make all `bitfield!` methods consts.
- make the module private.

#### data::codec::hash
  - move to [base]: `HasherFx`, `HasherBuildFx`.
  - new module `data::codec::hash::check`.
  - new type `Adler32`.

#### data::codec::radix
- move `Base` to [base].
- remove methods that allocate.

### data::error
- recreate the type `MismatchedCapacity`.
- remove type: `DataOverflow`.
- update `MismatchedBounds`.

### data::id
- new module `data::id`.
- move here `key/` and `uid/`.

#### data::id::key
- update `define_static_map!`:
  - support custom attributes and visibility.
  - add example items: `StaticMapConstU8Example`, `StaticMapTypeIdExample`, `StaticMapU16Example`.
  - improve docs.

#### data::id::uid
- move `IdPin` to [base].
- new type `IdRegistry`.

### data::layout
- new module `data::layout`.
- move here: `DataCollection`, `dst/`, `pool/`, `sort/`, `table/`.
- move here: `list/` submodules: `array/`, `buf/`→`buffer/`, `queue/`, `stack/`.

#### data::layout::array
- move to [base]:
  - traits: `ArrayExt`.
  - types: `ArrayFmt`, `ConstList`.
- rename:
  - `ExtArray` to `ArrayExt`.
  - `ExtVec` to `VecExt`.
- update `init_array!`:
  - rename `array_init!` to `init_array!`.
  - require `ConstInit` and `Vec` in scope if needed.
  - rename `const_init` arm related to traits to `INIT in`.
  - rename `const_init*` arms unrelated to traits to `const_fn*`.

#### data::layout::buffer
- new macros: `buffer_linear!`.
- new example types: `BufferExample`, `BuffeAllocExample`, `BufferViewExample`.

### data::topol
- new module `data::topol`.
- move here: `listlink/`→`linked/`.

### data::value
- move here: `NoData`, `of/`, `tuple/`.
- update `Oneof`:
  - new methods: `copy_*`.
  - remove methods: `variant_name`, `is_variant_name`, `first_non_unit`.
  - make methods const: `as_mut_*`, `as_ref_*`, `as_tuple_ref_options`.

## geom
- make `num::geom` a new `geom` root module.
- new modules: `affine/`, `rel/`, `space/`.
- rename `shape` to `fig`, as well as the related feature.

### geom::dir
- new module.
- move here `Orientation`, `Angle`, `AngleDirection` & `AngleKind`.
- new types: `Boundary1d`, `Boundary2d`, `Boundary3d`.

### geom::metric
- move to [base]:
  - internal macro `_impl_metric!`.
  - types: `Distance`, `Extent`, `Orientation`, `Position`, `Region`, `RegionStrided`, `Stride`.
- remove `metric` feature gate for `Orientation`, `Region` & `Stride`.
- update `_impl_metric!` helper macro.
  - implement 2d|3d common methods.
  - implement `ConstInit*`.
- update `Extent`:
  - add methods for 2|3d: `length[_ref|_mut]`, `width[_ref|_mut]`, `height[_ref|_mut]`, `depth[_ref|_mut]`, `breadth[_ref|_mut]`.
- remove `c_` prefix from int methods.

## lang
- rename `lang::ling` to `lang::hum`.
- rename `lang::ling::grammar` to `lang::hum::gram`.
- move `lang::i18n` to `lang::hum::i18n`.
- new modules: `disc`, `gram`, `prog`, `repr`, `sem`.
- move `ffi` to `prog::ffi`.

### lang::prog
- new module: `dsl`.

#### lang::prog::ffi
##### lang::prg::ffi::c
- new type aliases: `c_mode_t`, `c_off_t`.

##### lang::prg::ffi::js
- update `JsInstant`:
  - make method const: `delta_since`.
  - remove methods: `const_delta_since`.

## media
- new modules: `compo`, `doc`, `visual`.

### media::audio
- new types: `AudioChannel`, `AudioChannels`.

### media::font
- new type: `FontArt`.
- new const: `FONT_ART_3_4`.
- rename:
  - `BitmapFont` to `FontBitmap`.
  - `FONT_3_3` to `FONT_BIT_3_3`.
  - `FONT_3_5` to `FONT_BIT_3_5`.
  - `FONT_5_6` to `FONT_BIT_5_6`.

### media::visual
#### media::visual::color
- move to [base]:
  - types: `Gamma`, `Lum`, `Rgb`, `Rgba`.
  - aliases: `Lightness`, `LinearLightness`, `Luma`, `Luminance`, `Rgb8`, `Rgba8`, `RgbaPre8`, `Rgb16`, `Rgba16`, `RgbaPre16`, `RgbF32`, `RgbaF32`, `RgbaPreF32`, `RgbF64`, `RgbaF64`, `RgbaPreF64`, `RgbLinF32`, `RgbaLinF32`, `RgbaLinPreF32`, `RgbLinF64`, `RgbaLinF64`, `RgbaLinPreF64`.
- new type `GammaConst`.

#### media::visual::image
- add a new sixel implementation:
  - new types: `SixelChar`, `SixelColor`, `SixelEncoder`, `SixelPalette`, `SixelPaletteIter`.
- remove legacy vendored sixel implementation.

## num
- move to [base]:
  - all data, numeric, text & time error types.
  - types: `Cast`, `Int`, `True`.
  - traits: `NumConst`.
- make `Num*` traits depend on the `num` feature.
- move `num::unit` to `phys::unit`.
- make `num::error` public.

### num::dom
- new module `num::dom`.
- move here `num::float`, `num::frac`, `num::int`, `num::traits`.

#### num::dom::real
##### num::dom::real::float
- rename: `ExtFloat` to `FloatExt`.
- new types: `f32bits`, `f32bits_niche`, `f64bits`, `f64bits_niche`.
- update `Float`:
  - new methods: `poly_eval_const`, `sin_minimax`, `cos_minimax`, `sin_cos_minimax`.
  - remove deprecated methods: `const_signum`, `const_copysign`, `const_clamp`, `const_max`, `const_min`.
  - make std methods *const*: `fract`, `normalize`, `set_normalized`, `split`, `trunc`.
  - split out std-enabled implementation as internal `FloatStd`.
- Change `ExtFloat` to use `*_minimax` methods by default.
- move to [base]:
  - aliases: `fsize`.
  - traits: `FloatConst`.
  - types: `Float`.
  - float shared docs prefixed with `_FLOAT_`.

#### num::dom::int
- move to [base]: `Int`, `<i|u>size_*`.
- prefix int shared docs with `_INT_`.
- new macros: `define_divisor!`.
- new types: `DivisorExample`, `IntAlloc`, `IntError`, `IntResult`.
- remove type: `Divisor`.
- make all `Int` methods *const*.

### num::fin
- new module `num::fin`.
- move here `num::bit`, `num::logic` and `num::ord`.

#### num::fin::bit
- new module `num::bit`.
- update `BitOps` & `Bitwise`.
  - rearrange methods in thematic impl blocks.
  - new methods: `[is_][un]set[_checked][_range]`, `[un]set_all`, `flip[_checked]`, `flip[_checked]_range_if`, `[is_][un]set_mask`.
- separate documentations for `BitOps` and `Bitwise` as individual constants.

#### num::fin::logic
- move to [base]: `ConstBool`, `False`, `True`, `const_bool!`, `<i|u>size_*`.

#### num::fin::ord
- rename `Compare` to `Cmp`.
- new macro `cmp!`.
- update `Cmp`:
  - move to [base].
  - new impl for `Ordering`.
  - new methods: `minmax`, `pminmax`, `total_cmp`.
  - un-gate impls and many dependent const methods.

### num::grain
- new module `num::grain`.
- new macro `cast!`.
- new traits `PrimScalar`, `PrimInt`, `PrimSint`, `PrimUint`, `PrimFloat`.
- move inside: `num::cast`, `num::niche`, `num::wide`.
- fix `Cast` wrapping methods performance, and correctness for negative integers.

#### num::grain::niche
- new macros: `niche_prim!`, `nv!`.
- new types: `MaybeNiche`, `NonNiche`, `NicheValueError`.
- move to [base]: `NonExtreme*`, `NonValue*`, `ne!`, `nz!`.
- update macros: `ne`, `nv`, `nz`, adding lossy constructors.

#### num::grain::wide
- new module `num::grain::wide`.
- new macro: `define_lane!`.
- new internal helper macros.
- new example type `Lane4_i32Example`.
- support `nightly_simd` & `dep_wide` in [base_core].
- re-export some of `core::simd` types and traits.

### num::lin
- new module: `num::lin`.
- move here `geom::linear::{matrix, vector}`.

### num::prob
- new module `num::prob`.
- move here `num::rand`.

#### num::prob::rand
- move `num::prob::rand` to [base].
- rename `Lgc16` to `Lcg16`.
- rename `xorshift_custom!` to `define_xorshift!`.
- new macro: `define_pcg!`.
- new traits: `Rand`, `RandAlloc`, `RandStd`.
- new type: `Pcg32`.

### num::quant
- move to [base]: `Cycle`, `CycleCount`, `Interval`,  `Sign`.
- update `Interval`:
  - use individual `IncompatibleBounds` error.
  - add methods: `bound`, `bound_mut`, `bound_as_ref`.
- update `Sign`:
  - make part of `quant`.
  - rename variant `None` to `Zero`.
  - add methods: `eq`, `is_negative`, `is_positive`, `is_zero`, `is_nonzero`, `invert`, `same_direction`, `combine`, `pow`, `abs`, `neg_abs`, `fold`, `fold_slice`.
- move `ValueQuant` from `code::result` to `num::quant`.

### num::symb
- new module `num::symb`.

## org
- new `org` module.

## phys
- new modules: `phys::astro`

### phys::time
- new public module: `phys::time::source`.
- new types: `TimeFake`, `TimeFakeRef`.
- remove `TimeError` alias.
- remove `time` feature gate for `NoTime`, `Timecode`, `TimeDelta`, `TimeSplit`.
- rename `TimeGranularity` to `TimeScale`.
  - add `Ratio` variant.
  - add new variant aliases: `Mins`, `Secs`.
  - add new methods: `convert[_simulated]`, `is_fixed`, `new_ratio`, `some_ratio`, `to_ratio[_simulated]`.
- update `TimeSource`:
  - rename `granularity` method to `time_source` and return `TimeSource`.
  - change `MONOTONIC` from a const generic to the `time_is_monotonic` method.
  - new method: `time_is_absolute`.
  - rename methods: `now_*` to `time_now*`.
  - remove methods: `epoch_*`.
  - fix impl for `SystemInstant`.
- update `TimeDelta`:
  - make method const: `from_js`, `<div|mul>_f<32|64>`, `[try_]from_<millis|secs>_f<32|64>`.
  - remove methods: `const_from_js`, const_try_from_millis_f64.
- update `TimeSplitMilliNanoNorm`:
  - add method `from_duration`.
  - rename `from_duration` method to `from_duration_subsec`.
- update `WeekDay`: make all methods const.
- rename `UnixTimeI64` to `TimeUnixI64`, `UnixTimeU32` to `TimeUnixU32`.
  - make their `new` method const.

## run
- new `run` root module.

### run::regime
- renme `UiService` to `RunService` and move here.
- move capabilities from `ui::back::cap` to `run::setup::cap`.
  - rename `UiCap*` to `RunCap*`.

### run::time
- new items: `RunTick`.

## sys
### sys::arch
- new `Arch` methods: `cntvct`, `cycles`, `rdcycle`, `rdtsc`, `rdtscp`.
- new internal macro `ARCH!`.

### sys::device
- new module: `sys::device`.
- move `media::midi` to `sys::device::midi`.
- move `sys::sound` to `sys::device::audio`.

#### sys::device::display
- new module: `sys::device::display::x11`.
- new types: `XDisplay`, `XError`, `XEvent`, `XWindow`.

### sys::env
- vendor `argv` as `IterArgSOsRef` struct and `Env` method `args_os_ref`.

### sys::fs
- rename `ExtPath` to `PathExt`.

### sys::hw
- new module `sys::hw`.

### sys::io
- new `IoDuplex` trait.
- refactor the `sys::io` module.
- update `Io`: add `pipe` method.
- re-export `IoPipeReader`, `IoPipeWriter`.

### sys::log
- new type `LoggerStatic`.
- new macro `slog!`.
- rename `ExtLogger` to `LoggerExt`.

### sys::mem
- new alias: `MaybeByte`.
- new types: `ArenaExample`, `ArenaHandleExample`, `ArenaMarkExample`.
- new macros: `define_arena`.
- new submodules: `alloc`, `bound`, `layout`, `view`.
- move previous submodules:
  - `align`, `pin`, `ptr` inside `bound`.
  - `alloc`, `arena`, `storage` inside `alloc`.
  - `borrow`, `slice` inside `view`.
- move to [base]:
  - macros: `cswap!`.
  - traits: `MemAligned`.
  - types: `CacheAlign`, `Mem`, `Ptr`.
- rename `ExtMem` to `MemExt`.
- update `Mem`:
  - rename method `bytes_from_bits` to `bytes_from_bits_saturating`.
  - new method: `bytes_from_bits` with a faster impl.

#### sys::mem::cell
- rename `ExtCellOption` to `CellOptionExt`.

#### sys::mem::size
- move `ByteSized` and `size_of_expr!` to [base].

#### sys::mem::slice
- new macro `slice!`.
- new types: `SliceIter`, `SliceIterMut`.
- move to [base]:
  - macros: `const_join!`.
  - types: `Slice`.
- rename:
  - `join!` to `const_join!`.
  - `ExtSlice` to `SliceExt`.
- update `Slice`:
  - rename methods:
    - `copy_from_slice` to `copy`.
    - `trim_leading_bytes` to `trim_leading`.
    - `replace_leading_bytes` to `replace_leading`.
  - add new methods:
    - `range_to_inclusive*`.
    - `chunk[_mut]`, `chunks_exact[_mut]`.
    - `clone`, `copy_into`, `copy_str_into`, `copy_utf8_into`.
    - `trim_leading_keep`, `trim_leading_min_len`, `trim_trailing`, `trim_trailing_keep`, `trim_trailing_min_len`, `trim_edges_keep`, `trim_edges_min_len_left`, `trim_edges_min_len_right`.
  - add new `eq` methods for slices of slices of primitives and string slices.

### sys::os
- repurpose module to include operating supervisors.
- new `Libc` namespace.
- new modules: `sys::os::browser`, `sys::os::fd`.

#### sys::os::browser
- move `lang::ffi::js::web` to `sys::os::browser::web`.
- move `examples::lang::js_web*` to `examples::sys::web*`.

#### sys::os::linux
- new types `LinuxClock`, `LinuxTime`, `LinuxInstant`.
- new `Linux` methods: `clock_getres`, `clock_gettime`, `exit`.
- new `Linux` syscalls: `sys_clock_getres`, `sys_clock_gettime`.
- fix `Linux`-related warnings & avoid use of `transmute`.
- rename syscalls doc constants, prefix with `_DOC_`.
- update `LinuxError`:
  - new `Other` variant.
  - impl `Error`.
- improve `LinuxTimespec`.
  - impl `Display` and `ConstInit`.
  - rename method `with` to `try_with_duration` and make fallible.
  - add corresponding method `try_to_duration`.
  - add saturating methods to convert from/to `Duration`.
  - add corresponding conversions and methods from/to `TimeDelta`.

#### sys::os::term
- new module `sys::os::term`.
- feature-gate with `term`.
- rename `AnsiColor3b` to `AnsiColor3` and `AnsiColor8b` `AnsiColor8`.
- move to [base]:
  - types: `Ansi`, `AnsiColor3`, `AnsiColor8`, `TermSize`.
- change `Ansi::print*` methods to `ansi_print*` functions.
- new type: `AnsiColor`.
- update `Ansi`:
  - reverse the order of arguments in `CURSOR_MOVE*` to be columns first.
  - add methods: `COLOR_<BG|FG>_BRIGHT`, `CURSOR_MOVE`, `DEFAULT_<BG|FG>`, `MOUSE_X10_<ENABLE|DISABLE>`, `MOUSE_<NORMAL|TRACKING|UTF8>`, `MOUSE_SGR<_PIXELS>`, `strip_codes`.
  - rename current associated const items with a `_B` suffix.
  - add duplicated items with the old name returning a string slice or a `StringNonul`.
  - update digits formatting methods to use `Digits::write_digits10`.
  - modify `CURSOR_*` methods taking `u32` to take `u16`.
  - make all escape-sequence methods *const*.
  - fix codes related to alternate screen.
- update `ansi!`:
  - add new arms `p!` and `@p!` that auto-unwrap.
  - fix macro visibility.

## text
- new struct: `TextLut`.
- move to [base]:
  - traits: `NumToStr`.
  - types: `ByteSearch`, `Digits`, `GraphemeNonul`, `GraphemeU*`, `Str`, `StringNonul`, `StringU*`, `char7`, `char8`, `char16`.

### text::char
- new macro: `ch!`.
- new fn: `scalar_as_ascii_translit`.
- new types: `CharIter`, `charu`, `charu_niche`.
- new `char7` methods: `to_byte`, `to_str`, `try_from_u8_array`, `from_u8_array_unchecked`, `read_u8_slice`, `read_u8_slice_trunc`, `write_u8_slice`, `write_str`, `to_ascii_lowercase_array`, `eq_ignore_ascii_case`.
- new `char<7|8|16>` methods: `to_charu`, `try_from_charu`.
- new `TextLut` consts: `ASCII_BASE36_OFFSET`, `DIGITS_BASE36`, `DECIMAL_PAIRS`, `POWERS10`.
- mark `char<7|8|16>` as must_use.
- impl `ConstInit` for `char*`.
- rename `AsciiChar` to `CharAscii`.
- rename `char*` methods:
  - `to_u32` to `to_scalar`.
  - `*ascii_char*` to `*char_ascii*`.
- make `text::char` module public.
- move `text::ascii` to `text::char::ascii`.
- remove previously re-exported `IterChars`.
- update `Char`:
  - change `to_ascii_fold` to convert `Æ|Œ` to `E` & `æ|œ` to `e`.
  - remove deprecated methods: `len_to_utf8`, `utf8_?bytes_len`.
  - make it a tuple struct with a single a generic parameter.
  - add methods: `decode_surrogate_pair`, `has_overlong_encoding`, `has_valid_continuation`, `is_combining`, `is_combining_common`, `is_control`, `is_control_common`, `is_fullwidth`, `is_fullwidth_common`, `is_surrogate*`, `is_utf8_boundary`, `is_valid_code`, `to_char`, `to_char_lenient`, `to_char_unchecked`, `len_utf8_match`, `len_utf8_match_naive`.
  - rename methods:
    - `is_7bit` to `is_ascii`.
    - `is_valid` to `is_valid_scalar`.
    - `to_ascii_str` to `as_ascii`.
    - `to_ascii_str_unchecked` to `as_ascii_unchecked`.
    - `to_code*` to `to_scalar*`.
    - `utf8_len` to `len_utf8_unchecked`.
    - `utf8_len_checked` to `len_utf8`.
  - remove `utf8_bytes_` prefix from `Char<&[u8]>` methods.
  - add private consts: `CONT_MASK` `UTF8_CHAR_LEN`.
  - remove `code_` prefix from `Char<u32>` methods.
  - rename method `byte_len` to `len_bytes`.
  - modify all methods to take `self`.
  - return lengths as usize.
- update `UnicodeScalar`:
  - new methods: `as_ascii_translit`, `is_combining`, `is_combining_common`, `is_control`, `is_control_common`, `is_fullwidth`, `is_fullwidth_common`, `to_char`, `to_scalar`.
  - rename method `byte_len` to `len_bytes`.
  - reorder methods.

#### text::char::ascii
- rename `ASCII_TABLE` to `LUT_ASCII_CHARS` and make it a public *const*.
- rename `Ascii` to `Digits`.
- update `Digits`:
  - new const: `MAX_DIGITS_16`.
  - new methods: `count_digits16`, `digit_at_index10[_checked]`, `digit_at_index16[_checked]`, `digit_value_at_index10[_checked]`, `digit_value_at_index16[_checked]`, `digits16`, `write_digits10`, `write_digits10_fast`, `write_digits10_omit0`, `write_digits16`, `write_digits16_omit0`.
  - new private method: `digit_at_power16`.
  - rename const: `MAX_DIGITS` to `MAX_DIGITS_10` and make them of type `u8`.
  - rename methods:
    - `calc_digit` to `digit_at_power10` and make private.
    - `count_digits` to `count_digits10`.
    - `digits_*` to `digits10_*`.
    - `digits` to `digits10`.

### text::error
- re-export std's `FromUtf8Error`.

### text::fmt
- new macro: `fmtcat`.
- new trait: `DebugExt`.
- new types: `FmtNum`, `FmtNumConf`, `FmtNumGroup`, `FmtNumShape`, `FmtNumSign`, `FmtWriter`.
- re-export `FromFn` as `FmtFromFn`.
- move to [base]:
  - macros: `format_buf!`.
- remove vendored `numtoa` crate, `NumToStr` trait replaced with `Digits` struct.
- add method `Fmt::from_fn`.

### text::grapheme
- new types: `GraphemeKind`, `GraphemeScanner`.
- feature-bound all grapheme-related items.
- vendor `grapheme_machine` as items: `GraphemeBoundary`, `GraphemeMachine`, `GraphemePropCb`, `GraphemePropInCb`, `GraphemeProps`.
- impl `Grapheme` for scalar types.
- update the `Grapheme` trait:
  - add new methods: `grapheme_chars`, `grapheme_is_kind`, `grapheme_kind`, `grapheme_len_bytes`, `grapheme_len_chars`, `grapheme_len_utf8`.
- update `Grapheme<Nonul|U*>`:
  - remove methods: `to_cstring`.
  - make `new` method panic.
  - add new methods: `eq`, `<as|into>_string_<nonul|u8>`, `from_charu[_unchecked]`, `from_str`, `new_checked`.
  - make methods unsafe: `as_bytes_mut`, `as_str_mut`.
  - implement `PartialEq` and `Hash` manually.
  - implement `PartialEq` between string and grapheme types.
  - make all methods *const*.

### text::layout
- new module `text::layout`.
- new types: `TextCohesion`, `TextCursor`, `TextFit`,  `TextIndex`, `TextLayout`, `TextLayoutStep`, `TextSpan`, `TextSymbol`, `TextUnit`.

### text::str
- remove methods: `to_cstring`, from `String*`.
- make `chars` methods *const* when possible.
- add more impls of `PartialEq` against string slices.
- update `Str`:
  - add methods for returning substrings in compile time: `range*` `take*`, `*split*`.
  - remove method `from_boxed_utf8_unchecked`.
- common updates for `StringNonul` and `StringU`:
  - new methods: `eq`, `from_charu`, `from_charu_unchecked`, `from_str`, `from_str_truncate`, `from_str_unchecked`, `new_checked`.
  - impl `AsRef<&str>`, `Deref`, `Extend`, `FmtWrite`, `FromIterator`.
  - make methods unsafe: `as_bytes_mut`, `as_str_mut`.
  - make **all** methods *const*.
  - make `new` method panic.
  - rename methods `from_bytes*` to `from_array*`.
- update `StringNonul`:
  - new methods: `chars`, `char_count`.
  - rename methods `from_byte_array*` to `from_array*`.
  - impl `PartialEq`.
- update `StringU*`:
  - new methods: `pop_unchecked`, `push_charu`, `sanitize`.
  - modify methods:
  - `as_mut_str`: make safe.
  - `push_str`, make const, improve efficiency, update docs & examples.
  - `try_push_str*`, make const, return `Result<usize, usize>`, update docs & examples.
  - remove `AsMut` & `DerefMut` impls.
  - fix `TryFrom<&str>` impl.
  - impl `PartialEq`.
  - improve docs.
- add new method `repeat_into_slice` to `Str` & `StrExt`.
- improve method `repeat_into` for `Str` & `StrExt`.
- improve method `new_counter` for `Str`, `StrExt` & `StringExt`.

## ui
- remove modules: `ui::back`, `ui::front`.
- new modules: `ui::intent`, `ui::view`.

### ui::event
- new types: `DeviceId`, `Event`, `EventKind`, `EventQueue`, `EventTag`, `EventTarget`, `EventTimestampMode`, `EventWindow`, `KeyDead`, `WindowId`.
- change `EventPointer.pressure` field to be `f32bits_niche`.
- rename `time_stamp` fields to `timestamp`.
- derive `Eq` & `Hash` for all event types.
- implement `ConstInit` for all types.
- update Event:
  - new fields `count`, `processed`, `target`.
  - new methods: `mark_processed`, `mark_count`, `clear_count`, `finalize`, `count`, `tag`.
- update `Key` & `KeyFfi`:
  - rename `Period` variant to `Dot`.
  - add new `Dead` variant.
- update `KeyState`: add variant `Repeat`.
- update `EventTimestamp`:
  - manually impl `Debug` and `DebugExt`.
  - remove all inner unsafe.
  - make it support timestamps of 0 ms.
  - change inner representation to `f32bits_niche`.
  - add methods: `as_millis_f32_to_u32`, `as_millis_u32`, `from_millis_u32_as_f32`.
  - remove methods: `try_from_js`, `try_from_millis_f32`,  `try_from_millis_u32`, `try_from_secs_f32`.
- change `EventKeyFfi.timestamp` field to be `f32bits`.
- update `EventWindow`:
  - fix `Moved` variant to use `i32`.
  - new methods `is_<geometry|resize|move|focus|close|redraw|visibility|pointer_crossing|text_input|stream_signal>`, `<resize|move>_coords`.
  - new variants: `Entered`, `Left`, `Minimized`, `Maximized`, `Restored`, `FullscreenEntered`, `FullscreenExited`.

#### ui::event::key
- update `KeyPad`, add variant `Comma`.
- update `KeyMod`:
  - rename variant `IsoLevel3Shift` to `AltGr`.
  - remove variants: `LeftMeta`, `RightMeta`, `LeftHyper`, `RightHyper`.
- update `KeyMods`:
  - rename `ctrl` in methods to  `control`.
  - rename method `has_meta` to `has_super`.
  - add bits and methods for `IsoLevel5Shift`.
  - add getter and setter methods: `set_*`, `unset_*`.
  - remove old obsolete variants.
- remove `KeyAlpha`.
- update `Key` & `KeyFfi`:
  - rename `F(u8)` variant to `Fn(u8)`.
  - integrate `KeyAlpha`'s variants.
  - add new punctuation variants.
  - add new `Scancode(u16)` variant.

## vita
- new `vita` module.

## work
### work::future
- rename:
  - `ExtFuture` to `FutureExt`.
  - `ExtProcess` to `ProcessExt`.
    - `ProcessExt::id` to `self_pid`.

### work::process
- new macro: `cmd!`.
- new trait: `OutputExt`.
- new types: `CommandFlow`, `ExitStatusError`.
- rename re-exported types back by removing the `Process` prefix, except for `Child*`→`Process*` renames and `ProcessTermination`.

### work::sync
- move `portable-atomic-util` dependent re-exports to [base_alloc]: `Arc`, `ArkWeak`.

## yard
- new internal root module `yard`.
- new workspace-internal macro `_devela_policy!`.
- move here most private macros from `code::util`.

[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0

