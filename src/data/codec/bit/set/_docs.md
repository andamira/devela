Each named constant represents one or more bits.
Constants may be defined from single bit indices,
inclusive bit ranges, or comma-separated mixtures of both.

The generated struct derives
`Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq` and `Hash`.

One or more custom `impl { ... }` blocks may be provided after the declarations.
They are emitted before all generated associated constants and methods,
and may still refer to them through `Self`.

# Generated methods

For a generated set type `Set`, the macro defines the following methods.

## Core construction and access
- `new()`
- `from_bits(bits)`
- `bits()`
- `all()`

## Core predicates
- `is_empty()`
- `is_full()`
- `contains(other)`
- `has(other)`
- `intersects(other)`
- `is_subset(other)`
- `is_superset(other)`

`contains` and `has` return whether all bits in `other` are present.
`has` is a shorter alias intended for flag-like use.

## By-value modification
- `with(other)`
- `without(other)`
- `toggled(other)`
- `with_if(condition, other)`

## Set algebra
- `union(other)`
- `intersection(other)`
- `difference(other)`
- `symmetric_difference(other)`

## In-place modification
- `clear()`
- `insert(other)`
- `remove(other)`
- `toggle(other)`

## Per-constant methods

For each named constant `NAME`, the macro generates:
- `contains_name()`
- `intersects_name()`
- `with_name()`
- `with_name_if(condition)`
- `without_name()`
- `set_name()`
- `set_name_if(condition)`
- `unset_name()`

For constants declared as a single bit, it also generates:
- `has_name()`

For grouped constants, use `contains_name()` for “all bits are present”
and `intersects_name()` for “at least one bit is present”.

# Example
```
# use devela::set;
set! {
    /// A small set example.
    pub struct SmallSet(u16) {
        /// A single bit.
        A = 0;

        /// Another single bit.
        B = 1;

        /// A grouped constant from explicit bits.
        AB = 0, 1;

        /// A grouped constant from an inclusive range.
        BC = 1..=2;

        /// A grouped constant from mixed bits and ranges.
        MIXED = 0, 3..=5, 7;
    }
    /// Custom semantic helpers emitted before generated methods.
    impl {
        /// A custom named combination.
        pub const ABC: Self = Self::AB.with_bc();

        pub const fn contains_abc(self) -> bool { self.contains(Self::ABC) }
    }
}

let mut s = SmallSet::new()
    .with_a()
    .with_mixed_if(true);

assert!(s.has_a());
assert!(s.contains_mixed());
assert!(s.intersects_ab());

s.unset_a();
assert!(!s.has_a());
```

- Some examples of structs defined with the `set!` macro are:
  [`AsciiSet`] and [`EventButtons`].
- Another macro that leverages `set!` is:
  [`enumset!`][crate::enumset].

[`AsciiSet`]: crate::AsciiSet
[`EventButtons`]: crate::EventButtons
[`enumset!`]: crate::enumset
