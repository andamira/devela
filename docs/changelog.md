# devela changelog

[0.29.0-wip] unreleased
=======================

> .
> —

```
```

## Key changes:

------------------------------------------------------------------------------

# Project

## worskspace
- add cargo aliases: `c_<x|a|r><32|64>l`.

## features & flags
- new features: `code_all`, `data_all`, `ffi`, `geom_all`, `lang_all`, `media_all`, `num_all`, `org_all`, `phys_all`, `run_all`, `sys_all`, `text_all`, `ui_all`, `vita_all`, `widget`, `work_all`.
- remove features: `layout`, `safe_color`, `safe_draw`, `safe_image`, `_destaque*`, `_stack*`.
- rename features: `lin` to `alg`.
- add more feature gates for: `time`, `ui`.
- remove the `unsafe··` flag requirement for re-exported unsafe std methods.
  - update namespaces: `Alloc`, `Mem`, `Ptr`, `Slice`, `Str`.

## documentation
- make `all` and `all_` root modules public.
- remove `zall` and `zall_` public root modules.
- new tag: `_TAG_<REWORK|STATE|TOPOL>`.
- clarify public module and hidden scope features.
- update katex to v0.18.0.

## examples
- refactor examples directory tree.
- new examples: `alsa`, `ui_term`, `ui_web`, `ui_x11`.
- remove examples: `enumset`, `num/niche.rs`.
- update example: `term_linux`:
  - showcase signal handling.
- update examples: `web_api`, `web_workers`:
  - add optional JS bundling and wasm-opt paths for web builds.
  - move inside examples/sys/os/browser/web.
- update example: `web_api`:
  - leverage `WebEventIngress` and `request_animation_frame` to poll events.
  - showcase web permission querying.
  - decouple from the `time` feature.
  - show keyboard events.

---

# Modules

### code::marker
- fix `IndexRepr` impl for `usize`.

### code::result
- remove `OptResExt::transpose_result` method.
- update `unwrap!`:
  - add new arms: `some_or?`, `=some_or`, `=ok_or`, `err_or?`, `=err_or`, `=sok_or`, `serr_or?`, `=serr_or`, `=some_map`, `=some_map_into`, `=ok_map`, `=ok_map_into`, `=err_map`.
- add match-supporting arms for: `<some|ok|err|sok|err>[_expect|_or|_or?]`.

### code::util
- extend `whilst!` with indexed slice iteration.
- update `enumset!`:
- impl methods for the associated set:
  - `<contains|has|with|without|with_toggled|insert|remove|toggle|>_variant`, `for_each_set[_while]`.
  - unit-only methods: `iter`, `for_each[_while]`.
- impl for the associated enum, unit-only constant: `ALL`.
- impl `From<enum>` for the associated set.

#### data::access::route
- new types: `Route`, `RouteAnchor`, `RouteName`, `RouteSeg`.

### data::codec
- new type `Radix`.
- remove type: `Base`.
- remove type aliases: `Base16`, `Base32`, `Base32Padded`, `Base32Crockford`, `Base32Hex`, `Base64`, `Base64Padded`.

### data::id
- new macros: `handle!`, `handle_gen!`.
- rename old `handle!` to `handle_span!`.
- make `handle` and `uuid` modules public.

#### data::id::uuid
- new types: `Uuid`, `UuidNonNil`, `UuidVariant`, `UuidVersion`.

### data::layout
- remove trait: `DataDesta`.
- remove types: `Destaque`, `Stack`.

#### data::layout::array
- new types: `Array`, `ArrayCoordIter`, `ArrayShape`, `ArrayLayout`.
- remove types: `Array`, `Array2d`, `ArrayUninit`.

#### data::layout::buffer
- new types: `BufferRingU8`, `BufferRingStaticExample`.
- new macro: `buffer_ring!`.
- update `buffer_linear!`:
  - new impl `array` methods: `pop_back`, `pop_back_with`.
  - make impl array methods const: `from_array_clamped[_prim]`.
  - new impl `option` methods: `swap_remove_prim`, `swap_remove_copy_prim`.
  - fix impl `option` methods: `truncate`, `swap_remove`, `from_array_unchecked`.
- rename: `BufferStaticExample` to `BufferLinearStaticExample`, `BufferViewExample` to `BufferLinearViewExample`, `BufferAllocExample` to `BufferLinearAllocExample`.

### data::store
- make `pool` and `arena` modules public.

#### data::store::arena
- rename old `arena!` to `arena_bytes!`.
- new macro: `arena!`.
- new type examples: `ArenaExample`, `ArenaHandleExample`, `ArenaMarkExample`, `ArenaAllocExample`, `ArenaAllocHandleExample`, `ArenaAllocMarkExample`.

#### data::store::pool
- new macro: `pool!`.
- new type: `PoolIter`.

## error
- new root module.
- move `<data|num|text>::error` to `error::<data|num|text>`.
- move `code::error` contents here.

### geom::affine
- new types: `Simplex`, `SimplexFacetView`, `SimplexFacetIter`.
- update `Point`:
  - add robust planar turn and point-segment predicates
  - remove methods for explicit conversion to vectors.
  - implement affine operations with vectors.
  - remove `PartialOrd` and `Ord` impls.
- remove `Points`, `Points2d`, `VecPoints`.
- impl conversions between `Point` and `Position`.

### geom::dir
- fix `Angle` left-hand rule direction.

### geom::space
- make module public.
- new types: `PointSegmentRelation`, `Turn`.

##### lang::prog::ffi::js
- update `Js`:
  - improve safety of string-related methods.
- update `JsInstant:`
  - add const `ZERO`.
  - remove the `time` feature-gate.
  - impl `ConstInit`.

#### lang::prog::script
- new trait: `ScriptHost`.
- new types: `ScriptCall`, `ScriptCallId`, `ScriptError`, `ScriptOp`, `ScriptOutcome`, `ScriptMachine`, `ScriptValue`.

### media::font
- new types: `Bdf`, `Dvbf`, `Fonts`, `FontBitmapView`, `GlyphBitmapView`.
- new font: `termivela`, vendored from Terminus Font.
- rename `FontBitmap` to `FontBitmapWord`.
- move standalone font constants to associated `Fonts` constants without `FONT_` prefix.
- make fonts methods const: `text_<advance|width>`.

#### media::visual::image
- new types: `Coverage8`, `ImageInfo`, `ImageFrameInfo`, `ImageFrameSpan`.
- udpate `ImageError`; add new variant: `InsufficientBuffer`.
- update `Pnm`:
  - implement all classic PNM variants P1..P6.
  - remove the `alloc` feature-gate.
  - make all methods const.

##### media::visual::image::raster
- new types: `RasterElement`, `RasterGrid`, `RasterSlice`, `RasterByteSlice`.
- remove types: `RasterBytesMut`, `RasterBytesRef`, `RasterMut`, `RasterRef`.
- update `RasterViewBytes`: add methods: `raster_bytes_per_pixel_bytes`, `raster_row_start_bytes`.
- update `RasterLayout`:
  - add method `is_valid`.
  - change `bytes_per_line` field to u32.

## num
- rename `num::lin` to `num::alg`.

#### num::alg::vector
- update `Vector`:
  - simplify the fixed-array implementation and module layout.
  - add const primitive operations and checked integer variants.
  - improve floating-point magnitude and normalization.

#### num::alg::matrix
- update `Matrix`:
  - simplify the static owning representation, removing storage-order and algorithm-scratch parameters.
  - add common traits, shape and access utilities, transposition, and const primitive operations.
  - add identity, trace, checked integer variants, and vector and matrix products.

#### num::dom::real
- update `Float`:
  - rename `mul_add_fallback` to `mul_add_unfused`.
  - add `mul_add` no_std fallback version.

#### num::fin::ord
- fix `cmp!` clamp arm.

### num::grain
- new macro `bound_int!`.

### num::quant
- new type: `Scale`.
- add nonzero-denominator primitive aliases: `Ratio<I|U><8|16|32|64|128|size>`.
- overhaul `Ratio` with new functionalty for primitive aliases.

### num::signal
- new module.
- new traits: `SignalAt`, `SignalNext`.
- new types: `CurveRamp`, `Phase[Step|Accum]`, `Signal<Clamp|Const|Fn|Map|Scale|Zip>`.

### phys::time
- remove the `time` feature-gate from `[Maybe]Timed`.

## run
- new types `Permission<Error|Query|State>`.

### sys::io
- re-export missing `IoSeekFrom` from std.

##### sys::device::audio::alsa
- new `Alsa` method: `require_available`.
- add `AlsaError` error variant `Unavailable`.
- keep ALSA handle methods and PCM trait impls visible without `asound`.

##### sys::device::display::x11
- new type `XSurfaceUi`.
- update `XDisplay::wait_event` to skip internal empty events.
- update `XSurfaceFrame`: add `bits_per_pixel` field.
- fix `XPresent`, `XSurfaceFrame` and `XCpuBuffer`'s raster stride and row-orientation handling.

#### sys::mem::alloc
- move `arena` module to `data::store::arena::bytes`.

##### sys::os::browser::web
- new types: `WebCanvasUi`, `WebEventIngress`, `WebEventKey`, `WebPermissionSet`, `WebPermissionSnapshot`.
- remove `WebPermissionState`.
- update `WebEvent*` types:
  - add methods: `<from|to>_event_<key|mouse|pointer|wheel>`, `to_event_kind`.
  - rename the timed variants to `<from|to>_event_kind_timed`.
- update `Web`:
  - new methods `set_fill_rgb[a]`, `set_stroke_rgb[a]`, `set_text_baseline_top`.
  - remove methods `fill_style`, `stroke_style`.
  - take `JsInstant` as an argument for `request_animation_frame`.
  - change `permissions_query` to return `PermissionQuery`.
- move `<from|to>_web_*` conversions from `KeyFfi` to `Key`.
- decouple events from the `time` feature.
- modularize browser JS bindings.

#### sys::os::linux
- new types: `LinuxFd`, `LinuxFileType`, `LinuxOpenOptions`, `LinuxPipe`, `LinuxPipeFlags`, `LinuxSeekFrom`.
- add private types: `LINUX_AT`.
- delete private types: `AT_FDCWD`.
- make crate-private: `LINUX_FILENO`, `LINUX_O_FLAGS`, `LINUX_S_IFMT`, `LINUX_SEEK`.
- update `Linux`:
  - new methods: `open_fd`, `open_fd_at`, `close_fd`, `read_fd`, `write_fd`, `write_fd_all`, `syscall_openat`.
  - re-implement stdio read and write methods.

#### sys::os::term
- new trait: `TermBackend`.
- new types: `TermCellUi`, `TermLinuxRestore`.
- make `ansi` module public.
- update `Termel` to adapt to the new `Textel`.
- reimplement `TermLinuxInputBuf` using `BufferRingU8`.
- update `TermLinux`
  - route polling through `EventQueue`.
  - remove the `event` feature gate when possible.
  - add methods: `listen_signals`, `listen_app_controls`, `listen_resize`.

### text::ascii
- update `Digits`:
  - add methods `write_digits16[_nonzero]` for all bit-sizes.

### text::layout
- new types: `TextBreakKind`, `TextBreakMode`, `TextLine`, `TextLineIter`, `TextElideMode`, `TextSegment`, `TextSegmentKind`, `TextSymbolConfig`, `TextWrapIter`, `TextelWidth`, `TextelWidthMode`.
- update `Textel` with a new `meta` field and methods.

### text::parse
- update `TextScanner`:
  - add methods: `bytes`, `<take|expect>_ascii_u64_radix`, `<take|expect>_ascii_i64[_radix]`, `<take|expect>_ascii_usize`.


### text::str
- update `Str`:
  - add methods: `eq`, `from_utf8_complete_prefix`, `starts_with[_char]`, `ends_with[_char]`, `strip_<prefix|suffix>[_char]`, `strip_circumfix[_chars]`, `translit_ascii_into[_or]`, `graphemes[_charu][_in]`, `grapheme_count`.

### text::unicode
- new type `GraphemeIter`.
- update `CharIter`:
  - add methods: `byte_pos`, `as_bytes`, `as_str`, `remaining_bytes`, `is_empty`, `peek_char`, `peek_charu`, `peek_scalar`.
- fix `next_charu*` methods over byte slice impl.

## ui
- new submodules: `frame`, `route`, `semantic`, `text`, `view`, `widget`.

### ui::event
- reimplement `EventQueue` using `BufferRingU8`.
- remove the `time` feature-gate from `EventKindTimed`.
- remove non_exhaustive from `KeyFfi`.

### ui::frame
- new types: `UiId`, `UiKey`, `UiFrame`, `UiOutput`, `UiOutputView`, `UiPhase`, `UiScope`.

### ui::layout
- new types: `Layout1d`, `LayoutReceipt`, `Lunit`, `UiStack`.
- new aliases: `UiExt`, `UiPos`, `UiRect`, `UiStride`.

### ui::route
- new types: `HitRegion`, `RouteActive`, `RouteCapture`, `RouteFocus`, `RouteHot`.

### ui::semantic
- new types: `UiRole`, `UiAction`, `UiActions`, `UiFlags`, `UiEntry`, `UiText`.

### ui::text
- new types: `TextInput[Action|Config|Outcome|Reject|View]`, `TextInputKeymap[Preset]`.

### ui::view
- new types: `UiCellMetric`, `UiDensity`, `UiDraw`, `UiDrawKind`, `UiDrawList`, `UiDrawListView`, `UiLayer`, `UiRound`, `UiView`, `UiViewFlags`, `UiViewForm`.

### ui::widget
- new types: `UiButton`, `UiResponse`, `UiResponseFlags`.


[0.29.0]: https://github.com/andamira/devela/releases/tag/v0.29.0
