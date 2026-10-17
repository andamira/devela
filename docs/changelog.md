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

## dependencies

## tooling

---

# devela

## Crate

### features & flags
- add features: `bitten`, `bitwise`, `cast`, `niche`, `prob`, `ratio`, `scale`.
- rename `rand` to `prng`.

### structure

## Modules

#### num::fin
- add `bitwise!` macro.
- feature gate `Bitwise` and `BitOps` behind `bitwise`.
- migrate `set!`, `bitfield!`, `BitSpan`, and X11 bit-mask checks to `bitwise!`.


#### num::grain
- feature gate `Cast` implementations.
- feature gate `PrimCast`, `PrimJoin` and `PrimSplit` traits.
- add `BittenU16<B>` for all 17 capacities, using compact byte-composed representations that preserve niches across byte boundaries.
- refactor shared `BittenU*` representation selection and integrate `BittenU16` with `MaybeNiche`, `ReprInt`, `ReprIndex`, and `handle!`.
- gate `NonNiche` and `NonZero` impls from `MaybeNiche`.

#### num::quant
- feature gate `Ratio*` and `Scale` implementations.

### sys::hw
- correct 16-bit I²C size assertions to account for AVR and MSP430 alignment differences.

### yard
- reduce compilation overhead by skipping documentation-only `_tags!` and `_doc_meta!` expansions outside rustdoc.

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
- add precompiled ZX Spectrum 48K TAP demos, with README links to run them online and view their Rust sources.
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
