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
- add new member crate `devela_micros`.
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

## CI
- add `actionlint.yml` script.

## documentation
- split repository-level and crate-specific READMEs, with stable latest and WIP documentation links.

---

# devela

## Crate

### features & flags
- new features: `hw`, `unsafe_mmio`.
- new nightly feature: `asm_experimental_arch`.
- use the reflected unsafe cfg to reject `safe` with any `unsafe_*` capability.

### structure
- rename `src/index.rs` to `src/_.rs`.
- remove file paths from all file headers.

### documentation
- new tag: `hw`.
- rename tag: `uid` to `id`.
- rename "EGC" abbreviation to "extended grapheme cluster".

### examples
- add minimal no_std `sys/env` examples.

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

#### media::font
- new types: `FontBitmapPixel`, `FontBitmapPixelIter`.
- update `FontBitmapWord`:
  - new method `text_pixels` for bitmap text pixel iteration.
  - new method `draw_canvas` for drawing directly onto a `CanvasRaster`.
  - change `draw_rgba_with` to receive a `FontBitmapPixel` in its color callback.
  - update drawing methods to share the pixel iterator and use signed raster positions.

##### media::visual::draw
- new trait `CanvasRaster`, `CanvasRasterExt`.

##### media::visual::image
- gate with the `image` feature.

##### num::grain::niche
- new type `BittenU8`.

### phys::time::source
- new `TimeSource` and `TimeSourceCfg` methods: `time_value_<seconds|millis|micros|nanos>`.

### sys
- restrict Linux syscall-backed APIs to compatible Linux/freestanding targets.
- avoid detecting host native libraries as available when cross-compiling.

#### sys::arch
- add AVR instructions to `Arch`.

###### sys::device::display::x11
- gate `XRasterRenderer` and `XSurfaceFrame::raster_layout` with the `image` feature.

#### sys::hw
- make module public.

###### sys::hw::pin::i2c
- new traits: `I2cControl`, `I2cWrite`.
- new types: `I2cAddr7`, `I2cController`, `I2cError`.

#### sys::mem
- make `Ptr` provenance-related methods const:

## yard
- fix `_doc_location!` and `_doc_test_size_of!` to not hardcode the crate name.
- replace uses of `__crate_name!` macro with `env!("CARGO_PKG_NAME")`.
- gate unchecked unreachable policy on `unsafe_hint` instead of any unsafe capability.

---

# devela_micros

## Crate

### features & flags
- add feature groups for microcontrollers, boards, devices, media, and unsafe hardware capabilities.
- expose devela features: `draw`, `font`, `image`, `time`, `unsafe_hint`, `unsafe_mmio`.

### structure
- add root modules: `mcu`, `board`, `device`.
- add embedded target linker support and a hidden integrated `devela` namespace.

### examples
- add Arduino Nano examples: `led_on`, `timer0_ctc`, `timer0_interrupt`, `timer1_capture`, `timer1_clock`, `timer1_ctc`, `timer2_ctc`, `timer1_pwm`, `usart_chat`.
- add ESP32-C3 SuperMini OLED examples: `i2c_probe`, `led_on`, `oled`, `uart_echo`, `usb_serial_chat`.
- add minimal ESP32-S3 bring-up example.

## Modules

### board
- new types: `BoardArduinoNano`, `BoardSuperMiniOled042`.

#### device::display
- new types: `BitmapPage8`, `Ssd13xx`, `Ssd13xxI2c`, `Ssd13xxWrite`.

#### mcu::avr
- new types: `Atmega328pTimer1Clock`, `Atmega328pTimer1ClockCfg`, `AvrPin`, `AvrPort`, `AvrReg8`, `AvrUsart`, `McuAtmega328p`.

##### mcu::avr::timer
- new types: `AvrTimer0`, `AvrTimer1`, `AvrTimer2`.

#### mcu::esp32
- new macro: `esp32_c3_direct_boot!`.
- new types: `Esp32C3Pin`, `Esp32C3Uart`, `EspI2c`, `EspReg32`, `EspUsbSerialJtag`, `McuEsp32C3`.
- add ESP32-C3 direct-boot startup and linker support, including boot-watchdog handoff.

[0.30.0]: https://github.com/andamira/devela/releases/tag/v0.30.0
