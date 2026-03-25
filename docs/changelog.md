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
