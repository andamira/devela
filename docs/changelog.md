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

## lang
### lang::disc
- new submodules: `move`, `narr`.

## vita
### vita::play
- new submodule: `game`.

### vita::play::game
- new structs: `GameAction`, `GameCycle`, `GameLegacy`, `GameOutcome`, `GamePhase`, `GameRole`, `GameSession`, `GameTurn`.

[0.26.0]: https://github.com/andamira/devela/releases/tag/v0.26.0
