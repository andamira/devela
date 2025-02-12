This is derived work from the
[`icy_sixel`](https://crates.io/crates/icy_sixel/0.1.3) crate,
(a Rust port of [`libsixel`](https://github.com/saitoha/libsixel))
including the following modifications:

- make it work with `no_std`.
- make several functions `const`.
- rename `sixel_dither` to `DitherConf`.
- rename `MethodForRep` to `SixelMean`.
- rename `DiffusionMethod` to `Dither`.
- rename `MethodForLargest` to `SixelSplit`.
- rename `BuiltinDither` to `SixelPalette`, and associate the palettes.
- rename `PaletteType` to `SixelColorModel` and its `ColorBox` variant to `Colors`.
- add new methods to `PixelFormat`: `bits_per_pixel`, `bytes_per_pixel`, `required_bytes`.
- make private: `SixelPalette`, `SixelNode`, `SixelOutput`, `SixelEncodePolicy`, `SixelColorModel`, `FormatType`, many functions.
- new `Sixel` builder struct and make the `sixel_string` function private.
- derive `Default` and `ConstDefault`, when possible.
- rename many items according to rust guidelines.
- simplify lints, comments and attributes.
- improve docs, also for private items.
- improve and simplify error types.
- remove commented-out code.
- big refactor, modularize.
