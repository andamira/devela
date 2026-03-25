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

---

# Modules

## code
### code::ops
- re-export `punroll!` from devela.

## data
### data::access
#### data::access::iter
- add `is_empty` methods to `iter_strided!` items.

### data::layout
- update `buffer_linear!`:
  - new methods: `push_slice`, `remaining_capacity`, `remaining_capacity_prim`.
  - new methods for non-alloc backends: `push_slice_copy`, `push_slice_copy_exact`.
  - update `vec` backend methods:
    - update `is_full` semantics.
    - fix `len` implementation.

## lang
### lang::disc
- new submodules: `move`, `narr`.

## media
### media::visual
#### media::visual::draw
- new traits: `Canvas`, `CanvasRead`, `CanvasTextel`.

## num
### num::dom
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

## vita
### vita::play
- new submodule: `game`.

### vita::play::game
- new structs: `GameAction`, `GameCycle`, `GameLegacy`, `GameOutcome`, `GamePhase`, `GameRole`, `GameSession`, `GameTurn`.

[0.26.0]: https://github.com/andamira/devela/releases/tag/v0.26.0
