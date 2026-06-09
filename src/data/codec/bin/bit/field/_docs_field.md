Each named field represents one bit or an inclusive range of bits.
Field values are read and written as the same unsigned integer type
used by the backing storage.

Unchecked setters mask values to the field width.
Checked setters return an error when the value does not fit.

One or more custom `impl { ... }` blocks may be provided after the declarations.
They are emitted before all generated associated constants and methods,
and may still refer to them through `Self`.

# Generated methods

For a generated set type `Bitfield`, the macro defines the following methods.

## Core construction and access
- `new()`
- `from_bits(bits)`
- `bits()`
- `mask()`
- `all()`

## Core predicates
- `is_empty()`
- `is_full()`
- `fields_are_zero()`
- `fields_are_full()`
- `contains_mask()`
- `has_extra_bits()`

## Core modifiers
- `clear()`
- `clear_fields()`
- `without_fields()`
- `with_mask()`
- `with_mask_if()`
- `without_mask()`
- `without_mask_if()`

## Per-field methods

For each named field `NAME`, the macro generates:
- `get_name()`
- `with_name()`
- `try_with_name()`
- `set_name()`
- `try_set_name()`
- `clear_name()`
- `is_name_zero()`
- `is_name_max()`

- `name_start()`
- `name_end()`
- `name_width()`
- `name_max()`

## Reserved field names

Some field names are reserved because their generated methods
would collide with common methods. Avoid:
- `FIELDS`
- `MASK`
- `MASK_IF`

The names `EMPTY` and `FULL` are discouraged for fields
because `bitfield!` generates the methods `is_empty` and `is_full`.

# Examples
```
# use devela::bitfield;
bitfield! {
    struct Header(u16) {
        KIND = 0..=3;
        LEN  = 4..=11;
        FLAG = 12;
    }
    /// Custom semantic helpers emitted before generated methods.
    impl {
        /// Returns whether this header has a payload.
        pub const fn has_payload(self) -> bool { self.get_len() != 0 }
    }
}
let mut h = Header::new().with_kind(3).with_len(120).with_flag(1);

assert_eq!(h.get_kind(), 3);
assert_eq!(h.get_len(), 120);
assert_eq!(h.get_flag(), 1);
assert!(h.has_payload());

assert!(h.try_set_kind(15).is_ok());
assert!(h.try_set_kind(16).is_err()); // 16 does not fit in 4 bits.
```

- An example struct defined with the `bitfield!` macro is [`TermCaps`].
- See also the [`BitfieldExample`] and the [`set!`][crate::set] macro.

[`TermCaps`]: crate::TermCaps
[`BitfieldExample`]: crate::BitfieldExample
