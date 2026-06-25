It uses the [`set!`][crate::set] macro to create the associated set.

You have to give unique names both to the enum and to the associated set.

One or more custom `impl enum { ... }` and `impl set { ... }` blocks
may be provided after the variant block. They are emitted
before all generated associated constants and methods,
and may still refer to them through `Self`.

The enum may contain unit, tuple, and struct variants.
Every variant gets a corresponding associated set constant,
and enum values can be converted to their singleton set with `to_set`.

When all declared variants are unit variants, the enum also gets `ALL`,
and the associated set gets enum-value iteration methods:
`iter`, `for_each`, and `for_each_while`.

For enums with tuple or struct variants, enum-value iteration is not
generated because the set stores only variant presence, not payload data.

# Limitations
- Deriving [`Default`] on the generated enum is not currently supported,
  and has to be manually implemented instead.
- Outer attributes for custom impl blocks are placed
  after the `impl enum` or `impl set` marker.

## Reserved variant names

Some set names are reserved because their generated methods
would collide with common methods. Avoid:
- `IF`, `VARIANT`.

# Examples
```
# use devela::enumset;
enumset! {
    #[derive(Debug)]
    pub enum MyEnum(pub MyEnumSet: u8) {
        Variant1,
        Variant2(bool),
        Variant3{a: u8, b: u16},
    }
   impl enum
   /// Extra methods for [`MyEnum`].
   {
       /// Returns whether this variant belongs to [`MyEnumSet::DATA`].
       pub const fn is_data(&self) -> bool {
           self.is_in(MyEnumSet::DATA)
       }
   }
   impl set
   /// Extra constants for [`MyEnumSet`].
   {
       /// Variants carrying payload data.
       pub const DATA: Self = Self::Variant2.with(Self::Variant3);
   }
}
assert_eq![3, MyEnum::VARIANTS];
let mut es = MyEnumSet::default();
assert![es.is_empty()];
es.insert(MyEnumSet::Variant1);
assert![es.contains(MyEnumSet::Variant1)];
```

Unit-only enums also support enum-value iteration:
```
# use devela::enumset;
enumset! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Mode(pub ModeSet: u8) {
        Read,
        Write,
        Exec,
    }
}
let set = ModeSet::Read.with(ModeSet::Exec);
let mut count = 0;
set.for_each(|mode| {
    assert!(mode.is_in(set));
    count += 1;
});
assert_eq!(count, 2);
assert_eq!(Mode::ALL, [Mode::Read, Mode::Write, Mode::Exec]);
```

See also [`EnumExample`][crate::EnumExample] and [`EnumSetExample`][crate::EnumSetExample].

