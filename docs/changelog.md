# devela changelog

[0.27.0] 2026-04-10
===================

> The creation of a thousand forests is in one acorn.
> — Emerson

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
