# devela changelog

[0.26.0-wip] unreleased
=======================

> The creation of a thousand forests is in one acorn.
> — Emerson

```
```

 ------------------------------------------------------------------------------

# Project

## infra
### cargo
- bump MSRV to 1.94.0.

### CI
- bump `actions/checkout` to v6.
- bump `upload-artifact` to v7.
- bump `docker/setup-qemu-action` to v4.

### tools
- update `tools/check.rs`:
  - bump `devela` to 0.25.0.
- update `x` command wrapper to apply cfg flags to rustdoc.

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
  - new methods: `push_slice`, `remaining_capacity`, `remaining_capacity_prim`.
  - new methods for non-alloc backends: `push_slice_copy`, `push_slice_copy_exact`.
  - update `vec` backend methods:
    - update `is_full` semantics.
    - fix `len` implementation.

## geom
- remove all `*_ref` and `*_mut` accessor methods for `dir` and `metric` types.

### geom::dir
- new aliases: `Orientation[1|2|3]`.

### geom::metric
- make the module public.
- fix re-export of `RegionStrided`.
- new aliases: `Distance[1|2|3]`, `Extent[1|2|3]`, `Position[1|2|3]`, `Region[S][1|2|3]`.
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

## run
- new traits: `RunApp`, `RunRender`, `RunPresent`.

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
