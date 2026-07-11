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
- remove features: `layout`.
- add more feature gates for: `time`, `ui`.

## documentation
- make `all` and `all_` root modules public.
- remove `zall` and `zall_` public root modules.
- clarify public module and hidden scope features.

## examples
- refactor examples directory tree.
- new examples: `alsa`.
- remove examples: `enumset`, `num/niche.rs`.
- update example: `term_linux`:
  - showcase signal handling.
- update examples: `web_api`, `web_workers`:
  - add optional JS bundling and wasm-opt paths for web builds.
- update example: `web_api`:
  - leverage `WebEventIngress` and `request_animation_frame` to poll events.
  - decouple from the `time` feature.
  - show keyboard events.

---

# Modules

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

#### data::layout::buffer
- new types: `BufferRingU8`, `BufferRingStaticExample`.
- new macro: `buffer_ring!`.
- update `buffer_linear!`:
  - new impl `option` methods: `swap_remove_prim`, `swap_remove_copy_prim`.
  - fix impl `option` methods: `truncate`, `swap_remove`, `from_array_unchecked`.
- rename: `BufferStaticExample` to `BufferLinearStaticExample`, `BufferViewExample` to `BufferLinearViewExample`, `BufferAllocExample` to `BufferLinearAllocExample`.

## error
- new root module.
- move `<data|num|text>::error` to `error::<data|num|text>`.
- move `code::error` contents here.

##### lang::prog::ffi::js
- update `JsInstant:`
  - add const `ZERO`.
  - remove the `time` feature-gate.
  - impl `ConstInit`.

#### media::visual::image
- new types: `ImageInfo`, `ImageFrameInfo`, `ImageFrameSpan`.
- udpate `ImageError`; add new variant: `InsufficientBuffer`.
- update `Pnm`:
  - implement all classic PNM variants P1..P6.
  - remove the `alloc` feature-gate.
  - make all methods const.

#### num::fin::ord
- fix `cmp!` clamp arm.

### num::grain
- new macro `bound_int!`.

### num::quant
- add nonzero-denominator primitive aliases: `Ratio<I|U><8|16|32|64|128|size>`.
- overhaul `Ratio` with new functionalty for primitive aliases.

### num::signal
- new module.
- new traits: `SignalAt`, `SignalNext`.
- new types: `CurveRamp`, `Phase[Step|Accum]`, `Signal<Clamp|Const|Fn|Map|Scale|Zip>`.

### phys::time
- remove the `time` feature-gate from `[Maybe]Timed`.

### sys::io
- re-export missing `IoSeekFrom` from std.

##### sys::device::audio::alsa
- new `Alsa` method: `require_available`.
- add `AlsaError` error variant `Unavailable`.
- keep ALSA handle methods and PCM trait impls visible without `asound`.

##### sys::os::browser::web
- new types `WebEventIngress`, `WebEventKey`.
- update `WebEvent*` types:
  - add methods: `<from|to>_event_<key|mouse|pointer|wheel>`, `to_event_kind`.
  - rename the timed variants to `<from|to>_event_kind_timed`.
- update `Web`:
  - take `JsInstant` as an argument for `request_animation_frame`.
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
- new types: `TermLinuxRestore`.
- make `ansi` module public.
- update `Termel` to adapt to the new `Textel`.
- reimplement `TermLinuxInputBuf` using `BufferRingU8`.
- update `TermLinux`
  - route polling through `EventQueue`.
  - remove the `event` feature gate when possible.
  - add methods: `listen_signals`, `listen_app_controls`, `listen_resize`.

### text::layout
- new types: `TextBreakKind`, `TextBreakMode`, `TextLine`, `TextLineIter`, `TextElideMode`, `TextSegment`, `TextSegmentKind`, `TextSymbolConfig`, `TextWrapIter`, `TextelWidth`, `TextelWidthMode`.
- update `Textel` with a new `meta` field and methods.

### text::str
- update `Str`:
  - add methods: `eq`, `starts_with[_char]`, `ends_with[_char]`, `strip_<prefix|suffix>[_char]`, `strip_circumfix[_chars]`, `translit_ascii_into[_or]`, `graphemes[_charu][_in]`, `grapheme_count`.

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
- new types: `UiId`, `UiKey`, `UiScope`, `UiFrame`, `UiPhase`.

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
- new types: `UiDensity`, `UiDraw`, `UiDrawKind`, `UiDrawList`, `UiDrawListView`, `UiLayer`, `UiRound`, `UiView`, `UiViewFlags`, `UiViewForm`.

### ui::widget
- new types: `UiButton`, `UiResponse`, `UiResponseFlags`.


[0.29.0]: https://github.com/andamira/devela/releases/tag/v0.29.0
