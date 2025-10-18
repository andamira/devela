This is derived work from the
[`icy_sixel`](https://crates.io/crates/icy_sixel/0.1.3) crate,
(a Rust port of [`libsixel`](https://github.com/saitoha/libsixel))
including the following modifications:

- make it work with `no_std`.
- make it work without `unsafe`.
- make several functions `const`.
- rename `sixel_dither` to `LegcySixelDitherConf`.
- rename `MethodForRep` to `LegacySixelMean`.
- rename `DiffusionMethod` to `LegacySixelDither`.
- rename `MethodForLargest` to `LegacySixelSplit`.
- rename `PixelFormat` to `LegacySixelPixelFormat`.
- rename `BuiltinDither` to `LegacySixelPalette`, and associate the palettes.
- rename `PaletteType` to `LegacySixelColorModel` and its `ColorBox` variant to `Colors`.
- add new methods to `LegacyPixelFormat`: `bits_per_pixel`, `bytes_per_pixel`, `required_bytes`.
- make private: `LegacySixelPalette`, `LegacySixelNode`, `LegacySixelOutput`, `LegacySixelEncodePolicy`, `LegacySixelColorModel`, many functions.
- new `LegacySixel` builder struct and make the `sixel_string` function private.
- derive `Default` and `ConstDefault`, when possible.
- rename many items according to rust guidelines.
- simplify lints, comments and attributes.
- improve docs, also for private items.
- improve and simplify error types.
- remove commented-out code.
- big refactor, modularize.
