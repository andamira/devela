Generates a unit-only enum with variants associated to a specified range.

This macro generates an enum with integer variants named `P#` for positive
vales and `N#` for negative values.

It allows to represent integers with valid range of values, and where the
invalid values can be used by the compiler for memory niche optimization.

It only supports 8-bit and 16-bit representations to avoid excessive time
and memory usage during compilation.

# Usage
```
# use devela_macros::enumint;
// [visibility] name, repr, start, end
enumint![pub MyEnum, i8, -10, 10];
```

# Parameters
- `visibility`: Optional visibility indicator. E.g. `pub(crate)`.
- `name`: The name of the enum to be created.
- `repr`: the data representation. E.g `u8`.
- `start`: The starting value for the range of variants (inclusive).
- `end`: The ending value for the range of variants (inclusive).

# Panics
- Panics if any given value is not of the kind expected.
- Panics if `start` or `end` are outside the `repr` representable range.
- Panics if `start` is greater than `end`.

