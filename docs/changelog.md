# devela changelog

[0.31.0-wip] unreleased
=======================

> …
> —

```
This release…
```

## Highlights

- …

------------------------------------------------------------------------------

# Repository

## workspace
- add crate maturity and coverage ratings, with rationale in `docs/assessment.md`.

## dependencies

## tooling
- add compilation benchmarking and LLVM IR analysis tools under `tools/perf/`,
  with feature-matrix measurements, module-level attribution, and report comparison.
- add nightly rustc self-profiling with phase and query timings, multi-run summaries,
  saved-report comparison, automatic profile-data repair, and optional verbose diagnostics.

---

# devela

## Crate

### features & flags
- refactor module features for selective capabilities and recursive `_all` aggregations.
- add data-related features: `codec_all`, `integrity_all`, `pack_all`, `radix_all`, `symbol_all`,
  `value_all`, `adler`, `base32`, `base64`, `braille`, `byte`, `crc`, `ean`, `riff`, `tile`,
  `uuid`, `value`, `yijing`.
- add num-related features: `bitten`, `bitwise`, `cast`, `niche`, `prob`, `ratio`, `scale`.
- add media-related features: `audio_all`, `font_all`, `image_all`, `bdf`, `bitfont`, `dvbf`,
  `pnm`, `pcm`, `wav`.
- add run-related features: `cap`, `control`.
- add work-related features: `coro`, `future`.
- rename features: `visual` to `visual_all`.
- add `__select_modules` for opt-in root-module selection during development,
  retaining `code` and `yard` as the bootstrap foundation.
- generate `root_*` cfg aliases for selective root-module compilation.
- hide internal root-selection conditions from public rustdoc availability annotations.
- rename feature `rand` to `prng`.

### structure
- introduce a private `boot` module for essential crate-construction macros and doc fragments.
- move `mods_out!` into boot, retaining its public documentation through a rustdoc-only facade.
- move crate-root aggregation documentation out of yard, removing its dependency on `CONST!`.
- untangle foundational `code` from higher-level modules by localizing `ConstInit`
  and `BitSized` implementations and separating buffered size diagnostics.
- move `Version` and `VersionFull` buffered text encoding into `text::fmt`;
  keep the types and their core operations in `code::source`.

## Modules

- move `org` and `vita` modules to the `revela` crate.

#### code::any
- remove `AnyExt::type_hash` while preserving the hasher-parametric `type_hash_with`.

#### code::result
- remove unused `Mismatch<Interval<…>, H>` convenience constructors.

#### data::codec
- update `Radix`: gate Base32 and Base64 codecs.

##### lang::prog::script
- replace the generic `ScriptValue<R>` enum with `Value64` as the canonical scripting word.
- remove `R` from `ScriptOp`, `ScriptOutcome`, `ScriptMachine` and `ScriptHost`.
- add the `script` and `script_all` capability features, gating scripting and its value dependency.
- check arithmetic against the compact signed integer range and support opaque reference-token equality.
- reduce `ScriptMachine<4>` from 80 to 48 bytes on 64-bit targets.

##### media::visual::color
- new type: `Palette`.
- new aliases: `RgbBytes`, `RgbaBytes`.
- add conversion methods to `Rgb8` and `Rgba8`.
- add sRGB hex parsing and decoding,

###### media::visual::image::raster
- new types: `RasterIndexed8`, `RasterIndexed8Error`.

#### num::dom
- remove traits: `Num`, `NumRef`, `NumInt`, and `NumRefInt`.
- replace `dom::real::float` with public `dom::float` and expose `dom::frac`.
- introduce public `dom::fixed` and `dom::complex` modules as placeholders.

##### num::dom::int
- add macro `int!`.
- add example type `IntExample`.
- correct digit counts and digital roots for signed minima; define radices below 2 as zero.
- extract carrier-level digit and digital-root algorithms from `Int` for reuse by generated integer types.

#### num::fin
- add `bitwise!` macro.
- feature gate `Bitwise` and `BitOps` behind `bitwise`.
- migrate `set!`, `bitfield!`, `BitSpan`, and X11 bit-mask checks to `bitwise!`.

#### num::grain
- feature gate `Cast` implementations.
- feature gate `PrimCast`, `PrimJoin` and `PrimSplit` traits.
- add `ReprInt::VALUE_BITS` to expose numeric value-bit capacity across integer representations.
- impl `ReprInt` for `NonValue*` and `NonZero*`.

##### num::grain::niche
- add `BittenU16<B>` for all 17 capacities and `BittenU32<B>` for all 33 capacities, 
  using compact byte-composed representations that preserve niches across byte boundaries.
- expand niche module documentation from the byte-specific type to the supported `BittenU*` family.
- refactor shared `BittenU*` representation selection and integrate `BittenU16` and `BittenU32`
  with `MaybeNiche`, `ReprInt`, `ReprIndex`, and `handle!`.
- make `BittenU*::BITTEN` and `BittenU*::BITS` available generically for every supported capacity.
- gate `NonNiche` and `NonZero` impls from `MaybeNiche`.

#### num::quant
- feature gate `Ratio*` and `Scale` implementations.

### run
- feature-gate the `run` module.

#### run::app
- rename `AppConfig` to `AppInfo` and expose it as application identity metadata.

#### sys::fs
- move application-specific directory resolution from `sys::env` into `sys::fs::app`.
- rename `AppEnv` to `AppPaths` and the platform implementations to `AppPathsXdg`,
  `AppPathsUnix`, `AppPathsApple`, and `AppPathsWindows`.
- fix inverted validation in `AppInfo::new` and add regression tests.

### sys::hw
- correct 16-bit I²C size assertions to account for AVR and MSP430 alignment differences.

### text::fmt
- new macro: `fmt_num!`.
- remove `FmtNum`.
- update `fmtcat!`:
  - Use `Digits` for const decimal formatting of primitive integers.
  - Support explicit custom number formatters via `with Formatter`.

### yard
- reduce compilation overhead by skipping documentation-only
  `_tags!` and `_doc_meta!` expansions outside rustdoc.

---

# devela_bridge

## Crate

## Modules

---

# devela_extend

## Crate

## Modules

---

# devela_micros

## Crate

### examples
- add an Atari 800XL DOS `.xex` example built with Rust-MOS and LLVM-MOS.
- add a shared Atari runner and document rustup-linked MOS and Z80 toolchains.
- add precompiled demos for the ZX Spectrum 48K and the Atari 800 XL,
  with README links to run them online and view their Rust sources.
- move the Spectrum TAP packager to the shared example tools and document its purpose.
- consolidate MSP-EXP430G2 LaunchPad examples into one board project
  with selectable MSP430G2231, G2452 and G2553 memory layouts.
- verify target MCU identity before flashing and isolate build artifacts by chip.
- add MSP430G2553 bare-metal blink example

## Modules

### computer
- add `ComputerAtari800XL`.

### processor
- add `ProcessorMos6502`.

---

# devela_sentry

## Crate


[0.31.0]: https://github.com/andamira/devela/releases/tag/v0.31.0
