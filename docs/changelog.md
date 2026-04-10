# devela changelog

[0.27.0-wip] unreleased
=======================

> The creation of a thousand forests is in one acorn.
> — Emerson

```
This release reunifies devela's split base layers back into the main crate,
simplifying the workspace while preserving what was learned from the split.
```

# Key changes:
- workspace reunification: merge the split base library crates back into devela.
- structural simplification: remove duplicated scaffolding, and workspace indirection.
- orphan-rule relief: recover simpler implementation paths across the library surface.
- maintenance reduction: simplify build, feature, and module organization across the project.

------------------------------------------------------------------------------

# Project

## workspace
- reduce the workspace to `devela` and `devela_macros`.
- reunify the split `devela_base_*` crates back into `devela`.

## features & flags
- simplify feature wiring previously spread across the split base crates.

---

# Modules

## code
- merge `ConstInitCore` and `ConstInit` into `ConstInit`.

### data::layout
- remove `SortAlloc`; merge its methods in `Sort`.

#### media::visual::color
- merge `GammaConst` and `Gamma` into `Gamma`.

#### num::int
- remove `IntAlloc`; merge its methods in `Int`.

#### num::real::float
- remove `FloatStd`; merge its methods in `Float`.

### sys::mem
- fix standalone `guard.rs` example.

## yard
- new workspace-internal macro: `_use_or_shim!`.

[0.27.0]: https://github.com/andamira/devela/releases/tag/v0.27.0
