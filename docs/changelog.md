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

[0.30.0]: https://github.com/andamira/devela/releases/tag/v0.30.0
