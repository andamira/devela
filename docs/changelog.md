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
- bump MSRV to 1.99.0.
- add new member crate `devela_micros`.
- relocate `devela` crate under `crates/devela`.
- nest `devela_macros` under `crates/devela/macros`.
- rename `devela_ffi` to `devela_bridge`.
- rename `devela_sentinel` to `devela_sentry`.
- restore conventional locations for `src/bin` and `tests`.
- split crate-specific documentation from workspace-level documentation.
- establish `progs` and `games` as separate workspace-level incubator trees.
- recognize the experimental `z80` target architecture in cfg checking.
- use ATmega328P as the representative 16-bit-pointer target.
- new cargo alias `c16`.

## build
- extend `__dbg` diagnostics with configured and effective compiler flags.

## dependencies
- remove dependency: `log`.

## tooling
- document Cargo rustflag precedence and configuration guidelines.
- add Android cross-target Cargo aliases for AArch64 and x86-64.

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
- new features: `android`, `hw`, `oneof`, `unsafe_mmio`.
- use nightly feature: `asm_experimental_arch`.
- update nightly feature: `nightly_allocator`.
- use the reflected unsafe cfg to reject `safe` with any `unsafe_*` capability.

### structure
- rename `src/index.rs` to `src/_.rs`.
- remove file paths from all file headers.

### documentation
- add tags: `actuation`, `comm`, `display`, `hw`, `power`, `sensor`, `storage`.
- rename tag: `uid` to `id`.
- rename "EGC" abbreviation to "extended grapheme cluster".

### examples
- add minimal no_std `sys/env` examples.
- add native Android setup documentation and a raw executable example runnable through adb.

## Modules

### code
- add `Build` methods: `emit_link_search`, `rerun_if_changed`.

#### code::util
- update `doclink!` to require a local `__DOCLINK_CUSTOM_DOMAIN!` for custom form.

##### code::util::assert
- update `test_size_of!` with compile-time assertion.
- move `compile_error!` from `error`.

##### code::util::debug
- move `Backtrace` and `BacktraceStatus` from `error`.

##### code::util::synth
- update `use_as!`:
  - replace the source-prefix form `+Prefix` with `Prefix+`.
  - add local-suffix form `+Suffix` and combined form `Prefix+Suffix`.
  - support absolute source paths.
  - prettify the public api.

##### data::codec::hash
- use 32-bit state for default Fx and FNV hashing on 16-bit targets.

#### data::value
- update `OneOf`:
  - feature-gate with `oneof`.
  - replace `()` sentinel values with `Infallible`.
  - update init impls to exclude the empty case.

#### error::kind
- make module public.
- new struct: `AttemptLimitReached`.
- move `Timeout` from `phys::time`.

#### error::text
- new error type: `InteriorNul`.
- update `InvalidText:` add `InteriorNul` variant.

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

###### media::visual::image::raster
- new struct: `BitmapPage8`.

##### num::grain::niche
- new type `BittenU8`.

### phys::time::source
- new `TimeSource` and `TimeSourceCfg` methods: `time_value_<seconds|millis|micros|nanos>`.

### sys
- restrict Linux syscall-backed APIs to compatible Linux/freestanding targets.
- avoid detecting host native libraries as available when cross-compiling.

#### sys::arch
- update `Arch`:
  - add AVR instructions.
  - add portable methods: `nop` and `relax`.
  - support Z80.

###### sys::device::display::x11
- gate `XRasterRenderer` and `XSurfaceFrame::raster_layout` with the `image` feature.

#### sys::hw
- make module public.
- new trait: `CmdDataWrite`.

###### sys::hw::pin
- new traits: `I2cBusRead`, `I2cBusWrite`, `I2cControl`, `SpiBusWrite`, `SpiControl`.
- new types: `I2cAddr7`, `I2cController`, `I2cError`, `I2cTarget`, `SpiController`.

#### sys::log
- remove items: `LogConfig`, `LoggerExt`, `Log`.
- update `DiagLevel`:
  - add `Critical` variant.
  - add `as_str` method.
  - derive `ConstInit`.
- update `DiagOut`: add `critical` method.
- impl `DiagOut` for `AndroidLog`, `Stderr`, `StderrLock`.

#### sys::mem
- make `Ptr` provenance-related methods const:

##### sys::os::android
- new types: `Android`, `AndroidLog`.

##### work::sync::atomic
- gate the core `AtomicBool` re-export on 8-bit atomic support.

## yard
- fix `_doc_location!` and `_doc_test_size_of!` to not hardcode the crate name.
- replace uses of `__crate_name!` macro with `env!("CARGO_PKG_NAME")`.
- gate unchecked unreachable policy on `unsafe_hint` instead of any unsafe capability.
- update `_doc_meta!`:
  - update style, remove horizontal bars, adjust background, margins and padding.
  - add support for vendor information.

---

# devela_micros

## Crate

### features & flags
- add feature groups for microcontrollers, processors, microcomputers, boards, devices, media, and unsafe hardware capabilities.
- expose devela features: `draw`, `font`, `image`, `time`, `unsafe_hint`, `unsafe_mmio`.

### structure
- add root modules: `board`, `computer`, `device`, `mcu`, `processor`.
- add embedded target linker support and a hidden integrated `devela` namespace.

### examples
- add Arduino Due examples: `blink`.
- add Arduino Diecimila examples: `blink`, `usart_chat`.
- add Arduino Mega2560 examples: `blink`, `usart_chat`.
- add Arduino Nano examples: `adc_noise`, `blink`, `timer0_ctc`, `timer0_interrupt`, `timer1_capture`, `timer1_clock`, `timer1_ctc`, `timer2_ctc`, `timer1_pwm`, `tm1638`, `usart_chat`.
- add Solinius Sparrow examples: `blink`.
- add ESP32-C3 SuperMini OLED examples: `blink`, `oled`, `uart_echo`, `usb_serial_chat`.
- add ESP32-C6 Waveshare LED examples: `blink`, `touch_lcd147`.
- add LILYGO T-Deck examples: `blink`.
- add LILYGO T-Display-S3 examples: `blink`.
- add ZX Spectrum 48K examples: `paint`, `screen`.
- add bare-MCU compile/inspection examples for ATtiny4, CH32V003 and MSP430G2231.
- add centralized example tooling under examples/tools/.
- add `examples/tools/build-xtensa-rust.sh` helper for building Espressif Xtensa Rust toolchains.

## Modules

### board
- new AVR boards: `BoardArduinoDiecimila`, `BoardArduinoMega2560`, `BoardArduinoNano`, `BoardSoliniusSparrow`.
- new ESP32 boards: `BoardLilygoTDeckS3`, `BoardLilygoTDisplayS3`, `BoardLilygoTWatchS3`, `BoardSuperMiniOled042`, `BoardWaveshareC6TouchLcd147`.
- new SAM boards: `BoardArduinoDue`.

### computer::zx::spectrum
- new macro: `spectrum_main!`.
- new types: `ComputerSpectrum48`, `SpectrumAttribute`, `SpectrumColor`, `SpectrumKey`, `SpectrumKeys`, `SpectrumUlaOut`.

#### device::display
- new trait: `Tm1638Bus`.
- new types: `Jd9853`, `Tm1638`, `Tm1638Frame`, `Tm1638Brightness`, `Tm1638LedKey8`, `Tm1638AvrBus`, `Ssd13xx`.

#### mcu::avr
- new types: `Atmega328pTimer1Clock`, `Atmega328pTimer1ClockCfg`.
- new types: `AvrAdc`, `AvrAdcInput`, `AvrAdcNoise`, `AvrPin`, `AvrPort`, `AvrReg8`, `AvrUsart`, `AvrUsartTx`.
- new types: `McuAtmega1284`, `McuAtmega168`, `McuAtmega2560`, `McuAtmega328p`.

##### mcu::avr::timer
- new types: `AvrTimer0`, `AvrTimer1`, `AvrTimer2`.

#### mcu::esp32
- new types: `EspI2c`, `EspReg32`, `EspSpi`, `EspUsbSerialJtag`.
- add blocking I²C reads and repeated-start write→read transfers.

##### mcu::esp32::c3
- new macro: `esp32_c3_direct_boot!`.
- new types: `Esp32C3Pin`, `Esp32C3Rng`, `Esp32C3SystemTimer`, `Esp32C3Uart`, `McuEsp32C3`.
- add ESP32-C3 direct-boot startup and linker support, including boot-watchdog handoff.

##### mcu::esp32::c6
- new macro:  `esp32_c6_direct_boot!`.
- new types: `Esp32C6Pin`, `Esp32C6SpiCmdData`, `McuEsp32C6`.
- add ESP32-C6 direct-boot startup and linker support, including boot-watchdog handoff and GPIO pad routing.

##### mcu::esp32::s3
- new macro: `esp32_s3_startup!`.
- new types: `Esp32S3Pin`, `McuEsp32S3`.
- add ESP32-S3 linker support and low-level GPIO, I²C0, and USB Serial/JTAG access.

#### mcu::sam
- new types: `McuSam3x8e`, `SamPin`, `SamPort`, `SamReg32`.
- add SAM3X8E startup and linker support.

### processor::z80
- new type: `ProcessorZ80`.

---

# devela_sentry

## Crate

### structure
- establish `devela_sentry` as an independent workspace for downstream validation.
- add shared cross-target assembly inspection tooling with target aliases and groups.
- add probe packages: `num`, `arch`.

### probes
- add architecture-specific assembly inspection built on the shared tooling.
- add cross-platform logging probe with portable `DiagOut` exercise.
- add numeric and architecture code-generation probes.

### examples
- add a `const_warn!` diagnostic example.

[0.30.0]: https://github.com/andamira/devela/releases/tag/v0.30.0
