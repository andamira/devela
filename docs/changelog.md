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
- add minimal no_std `sys/env` examples.
- add `sys/hw/mcu/board/arduino/nano` examples: `led_on`, `usart_tx`.

---

# Modules

### code::util::assert
- move `compile_error!` from `error`.

### code::util::debug
- move `Backtrace` and `BacktraceStatus` from `error`.

#### data::codec::hash
- use 32-bit state for default Fx and FNV hashing on 16-bit targets.

## error
- make `kind` public.

## sys
- restrict Linux syscall-backed APIs to compatible Linux/freestanding targets.
- avoid detecting host native libraries as available when cross-compiling.

### sys::hw
- make module public.

#### sys::hw::mcu::avr
- new types: `Atmega328p`, `AvrPort`, `AvrReg8`, `AvrUsart`.

#### sys::hw::mcu::board
- new type: `ArduinoNano`.

### sys::mem
- make `Ptr` provenance-related methods const:

[0.30.0]: https://github.com/andamira/devela/releases/tag/v0.30.0
