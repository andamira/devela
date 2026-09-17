# devela changelog

[0.30.0] unreleased
===================

> …
> …

```
```

## Highlights

- …

------------------------------------------------------------------------------

# Repository

## workspace
- relocate `devela` crate under `crates/devela`.
- nest `devela_macros` under `crates/devela/macros`.
- rename `devela_ffi` to `devela_bridge`.
- rename `devela_sentinel` to `devela_sentry`.
- restore conventional locations for `src/bin` and `tests`.
- split crate-specific documentation from workspace-level documentation.
- establish `progs` and `games` as separate workspace-level incubator trees.

## tooling
### rustfmt
- make formatting checks reproducible across local and CI environments.
- restrict formatting to tracked Rust files and pin the rustfmt toolchain version.

### CI
- add `actionlint.yml` script.

---

# devela

## Crate

### features & flags
- new features: `mcu`, `unsafe_mmio`.
- new nightly feature: `asm_experimental_arch`.

### structure
- rename `src/index.rs` to `src/_.rs`.
- remove file paths from all file headers.

### documentation
- new tag: `_TAG_HW`.

### examples
- add minimal no_std `sys/env` examples.
- add `sys/hw/mcu/avr/arduino_nano` examples: `led_on`, `timer0_ctc`, `timer0_interrupt`, `timer1_ctc`, `timer1_capture`, `timer2_ctc`, `timer1_pwm`, `usart_chat`.
- add `sys/hw/mcu/esp32/c3_supermini_oled042` examples: `led_on`, `oled`, `uart_echo`, `usb_serial_chat`.
- add `sys/hw/mcu/esp32/s3_bringup` example.

## Modules

### code
- add `Build` methods: `emit_link_search`, `rerun_if_changed`.

##### code::util::assert
- move `compile_error!` from `error`.

##### code::util::debug
- move `Backtrace` and `BacktraceStatus` from `error`.

##### data::codec::hash
- use 32-bit state for default Fx and FNV hashing on 16-bit targets.

### error
- make `kind` public.

##### num::grain::niche
- new type `BittenU8`.

### sys
- restrict Linux syscall-backed APIs to compatible Linux/freestanding targets.
- avoid detecting host native libraries as available when cross-compiling.

#### sys::Arch
- add AVR instructions.

#### sys::hw
- make module public.

###### sys::hw::mcu::avr
- new types: `AvrPin`, `AvrPort`, `AvrReg8`, `AvrUsart`, `McuAtmega328p`.

####### sys::hw::mcu::avr::timer
- new types: `AvrTimer0`, `AvrTimer1`, `AvrTimer2`.

###### sys::hw::mcu::board
- new types: `BoardArduinoNano`, `BoardSuperMiniOled042`.

###### sys::hw::mcu::esp32
- new macro: `esp32_c3_direct_boot!`.
- new types: `Esp32C3Pin`, `Esp32C3Uart`, `EspI2c`, `EspReg32`, `EspUsbSerialJtag`, `McuEsp32C3`.
- add ESP32-C3 direct-boot startup and linker support, including boot-watchdog handoff.

###### sys::hw::pin::i2c
- new types: `I2cAddr7`, `I2cError`.

#### sys::mem
- make `Ptr` provenance-related methods const:

[0.30.0]: https://github.com/andamira/devela/releases/tag/v0.30.0
