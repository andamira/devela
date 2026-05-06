
The generated enum has one variant per integer in the inclusive range.
Positive values are named `P{n}` and negative values are named `N{n}`.

# Safety backends

By default, construction is fully safe and uses generated match arms.
With the `unsafe` feature, construction uses a compact checked conversion
backend to reduce compile time for larger ranges.

# Compile-time cost

This macro generates one enum variant per represented value. Large ranges,
especially broad `u16`/`i16` ranges, can substantially increase compile time
and memory use. For large numeric domains where variant names are not needed,
prefer a checked newtype.

# Usage
```
# use devela_macros::enumint;
// [visibility] name, repr, start, end
enumint![pub MyEnum, i8, -10, 10];
```

# Panics

- Panics if any given value is not of the kind expected.
- Panics if `start` or `end` are outside the `repr` representable range.
- Panics if `start` is greater than `end`.

```compile_fail
# use devela_macros::enumint;
enumint![Bad, u8, 5, 2]; // reversed range
```
```compile_fail
# use devela_macros::enumint;
enumint![Bad, u8, -1, 1]; // negative range
```
```compile_fail
# use devela_macros::enumint;
enumint![Bad, u8, 254, 256]; // unsigned upper overflow
```
