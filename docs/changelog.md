# devela changelog

[0.30.0] unreleased
===================

> …
> …

```
```

## Key changes:

- …

------------------------------------------------------------------------------

# Project

## workspace
- rename `src/index.rs` to `src/_.rs`.

## features & flags
- new feature: `unsafe_mmio`.

## documentation
- new tag: `_TAG_HW`.

## examples
- add minimal no_std environment examples.

---

# Modules

### code::util::assert
- move `compile_error!` from `error`.

### code::util::debug
- move `Backtrace` and `BacktraceStatus` from `error`.

## error
- make `kind` public.

## sys
- restrict Linux syscall-backed APIs to compatible Linux/freestanding targets, fixing cross-target builds.
- avoid detecting host native libraries as available when cross-compiling.

### sys::hw
- make module public.

#### sys::hw::mcu::avr
- new types: `AvrPort`, `AvrReg8`.

### sys::mem
- make `Ptr` provenance-related methods const:

[0.30.0]: https://github.com/andamira/devela/releases/tag/v0.30.0
