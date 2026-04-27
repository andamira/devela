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

## workspace
### examples
- new example scripts: `scope_guard`, `current_guard`.

## dependencies
- bump `portable-atomic-util` to 0.2.7.
- bump `hashbrown` to 0.17.
- bump `wide` to 1.3.

## features & flags
- new feature: `web`.
- auto-enable feature `unsafe_ffi` via: `web`, `x11`.
- update the intended scope of the `_docs_examples` feature.

---

# Modules

## code
- impl `ConstInit` for more types.

### code::util
- new macro `maybe_slot!`.
- update `cdbg!` to add custom prefix syntax.
- update `enumset!`, replace `bitfield!` with `set!`.
- reexport `cfg_select!`, `cold_path()`.
- remove `cfg_if!`, replace with `cfg_select!`.

#### data::codec
- new `set!` macro.
- update `bitfield!`
  - rename method `without_fields` with `new`, and make it always visible.
  - derive `ConstInit`.

### data::id
- rename `define_handle!` to `handle!`

#### data storage::key
- rename `define_static_map!` to `map!`

### geom::metric
- new macros: `dis!`, `ext!`, `ori!`, `pos!`, `region!`.
- new methods for `geom::metric` *dim* types: `map`, `map_into`, `try_map`, `try_map_into`.
- new methods for `Region`: `map`, `map_ext`, `map_pos`, `try_map`, `try_map_ext`, `try_map_pos`.

### lang::prog
- new submodules: `calc`, `embed`, `kernel`, `phrase`.
- remove submodule: `dsl`.

#### media::visual::image
- new traits: `Raster`, `RasterBuf`,`RasterView`, `RasterBufBytes`, `RasterViewBytes`, `RasterSamplePacked`, `RasterViewPacked`.
- remove `image` feature-gate from the module.
- update `ImageError`.
  - make `InvalidParsedInteger` variant contain `ParseIntErrorKind`.
  - derive `Hash`.

#### num::dom::int
- rename `define_divisor!` to `divisor!`.

#### num::dom::real
- make `Float`'s std methods const: `ceil`, `floor`, `mul_add`, `round_ties_away`, `round_ties_even`.

### num::grain
- rename `Primitive*` traits to `Prim*`.
- rename `define_lane!` to `lane!`

### num::prob
- new module `phys::prob::markov`.

#### num::prob::rand
- rename `define_pcg!` to `rand_pcg!`
- rename `define_xorshift!` to `rand_xorshift!`

## phys
- new module `phys::subs`.

#### phys::time
- new type: `Timed`.
- new alias: `MaybeTimed`.
- update `NoTime`:
  - implement `TimePoint`, `TimeSource` and `TimeSpan`.

#### phys::time::source
- update `TimePoint`:
  - make `Elapsed` require `TimeSpan`.
  - implement for `Duration`.
- new trait `TimeSpan`.

## run
- new items: `RunDriver`, `RunDriverError`, `RunDriverFrameError`.
- update `RunRender` and `RunPresent`:
  - support borrowed render artifacts with GATs.
  - make them use a borrowed `RunFrame`.
  - make `RunRender`'s `S: ?Sized`.

## run::time
- update `RunPacer`:
  - change requiring `Duration` for `T: TimeSpan`.
  - make the constructor fallible.
  - add new methods: `interval`, `accum`, `allow`, `allow_checked`, `cycles`, `cycles_checked`.

##### sys::device::display::x11
- new items: `XCpuBuffer`, `XFrontend`, `XImageMode`, `XPresent`, `XRasterRender`, `XShmBuffer`, `XSurfaceFrame`.
- update `XDisplay`:
  - add fields: `image_format`, `shm_caps`.
  - add methods: `bits_per_pixel`, `scanline_pad_bits`, `bytes_per_line`, `has_shm`.
- fix `XWindow::clear_redraw`.

### sys::io
- new traits `TextIn`, `TextOut`.

### sys::log
- new modules `sys::log::{bench, trace}`;
- new types: `DiagLevel`, `DiagOut`.

### sys::mem
- rename `define_arena!` to `arena!`

### sys::mem::cell
- new items: `MemHedgeCtrl`, `MemHedgeError`, `MemHedgeRead`, `MemHedgeState`.

### sys::mem::view
- update `Slice`: add methods: `get`, `get_mut`:
- new items: `MemReplicaError`, `MemReplicaSlice`.

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

### text::fmt
- make `Fmt::from_fn` const.

### text::parse
- update `TextScanner`
  - new methods: `take_ascii_ident_tail`, `next_line`, `next_line_trimmed`, `next_line_trimmed_before`, `take_quoted_basic_or_rest`.

### ui::event
- new types: `EventButtons`, `EventWheelUnit`.
- new alias `EventKindTimed`.
- update `Event`:
  - new method: `from_kind_timed_with`.
  - impl `From<EventKindTimed>.
- remove the `timestamp` field from: `EventKey`, `EventKeyFfi`, `EventMouse`, `EventPointer`, `EventWheel`.
- update `EventMouse`:
  - change the `buttons` field to use `EventButtons`.
- update `EventButtonState`: make `Pressed` the default.
- update `EventKind` and `EventTag`:
  - add `Wheel` variant.
  - add methods: `is_wheel`, `some_wheel`.
- update `EventWheel`:
  - add fields: `unit`, `buttons`.
  - add many convenience methods.

## work
- new `work` submodules: `exec`, `plan`, `task`.
- move `thread` and `process` inside `exec`.
- rename `Task*` items to `Async*`.

## yard
- update `_use_or_shim!` to add `_doc!` macro support.
- split a new `_doc_vendor!` macro out of `_doc!`
- update syntax of `_devela_policy`.rs

[0.28.0]: https://github.com/andamira/devela/releases/tag/v0.28.0
