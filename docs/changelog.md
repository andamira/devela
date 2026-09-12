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
- new features: `mcu`, `unsafe_mmio`.
- new nightly feature: `asm_experimental_arch`.

## documentation
- new tag: `_TAG_HW`.

## examples
- add minimal no_std `sys/env` examples.
- add `sys/hw/mcu/avr/arduino_nano` examples: `led_on`, `timer0_ctc`, `timer0_interrupt`, `timer1_ctc`, `timer1_capture`, `usart_chat`, `usart_tx`.
- add `sys/hw/mcu/esp32/c3_supermini_oled042` example: `led`.

---

# Modules

### code
- add `Build` methods: `emit_link_search`, `rerun_if_changed`.

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

### sys::Arch
- add AVR instructions.

### sys::hw
- make module public.

#### sys::hw::mcu::avr
- new types: `AvrPin`, `AvrPort`, `AvrReg8`, `AvrUsart`, `McuAtmega328p`.

#### sys::hw::mcu::avr::timer
- new types: `AvrTimer0`, `AvrTimer1`.

#### sys::hw::mcu::board
- new types: `BoardArduinoNano`, `BoardSuperMiniOled042`.

#### sys::hw::mcu::esp32
- new macro: `esp32_c3_direct_boot!`.
- new types: `EspReg32`, `McuEsp32C3`.
- add ESP32-C3 direct-boot startup and linker support.

### sys::mem
- make `Ptr` provenance-related methods const:

[0.30.0]: https://github.com/andamira/devela/releases/tag/v0.30.0
