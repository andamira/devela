// devela/src/code/util/enumset.rs
//
//! An enum with an associated bit set.
//

#[doc = crate::_tags!(code construction member set bit)]
/// Defines an enum and an associated type set of its variants.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// It uses the [`set!`][crate::set] macro to create the associated set.
///
/// You have to give unique names both to the enum and to the associated set.
///
/// One or more custom `impl enum { ... }` and `impl set { ... }` blocks
/// may be provided after the variant block. They are emitted
/// before all generated associated constants and methods,
/// and may still refer to them through `Self`.
///
/// The enum may contain unit, tuple, and struct variants.
/// Every variant gets a corresponding associated set constant,
/// and enum values can be converted to their singleton set with `to_set`.
///
/// When all declared variants are unit variants, the enum also gets `ALL`,
/// and the associated set gets enum-value iteration methods:
/// `iter`, `for_each`, and `for_each_while`.
///
/// For enums with tuple or struct variants, enum-value iteration is not
/// generated because the set stores only variant presence, not payload data.
///
/// # Limitations
/// - Deriving [`Default`] on the generated enum is not currently supported,
///   and has to be manually implemented instead.
/// - Outer attributes for custom impl blocks are placed
///   after the `impl enum` or `impl set` marker.
///
/// ## Reserved variant names
///
/// Some set names are reserved because their generated methods
/// would collide with common methods. Avoid:
/// - `IF`, `VARIANT`.
///
/// # Examples
/// ```
/// # use devela::enumset;
/// enumset! {
///     #[derive(Debug)]
///     pub enum MyEnum(pub MyEnumSet: u8) {
///         Variant1,
///         Variant2(bool),
///         Variant3{a: u8, b: u16},
///     }
///    impl enum
///    /// Extra methods for [`MyEnum`].
///    {
///        /// Returns whether this variant belongs to [`MyEnumSet::DATA`].
///        pub const fn is_data(&self) -> bool {
///            self.is_in(MyEnumSet::DATA)
///        }
///    }
///    impl set
///    /// Extra constants for [`MyEnumSet`].
///    {
///        /// Variants carrying payload data.
///        pub const DATA: Self = Self::Variant2.with(Self::Variant3);
///    }
/// }
/// assert_eq![3, MyEnum::VARIANTS];
/// let mut es = MyEnumSet::default();
/// assert![es.is_empty()];
/// es.insert(MyEnumSet::Variant1);
/// assert![es.contains(MyEnumSet::Variant1)];
/// ```
///
/// Unit-only enums also support enum-value iteration:
/// ```
/// # use devela::enumset;
/// enumset! {
///     #[derive(Clone, Copy, Debug, PartialEq, Eq)]
///     pub enum Mode(pub ModeSet: u8) {
///         Read,
///         Write,
///         Exec,
///     }
/// }
/// let set = ModeSet::Read.with(ModeSet::Exec);
/// let mut count = 0;
/// set.for_each(|mode| {
///     assert!(mode.is_in(set));
///     count += 1;
/// });
/// assert_eq!(count, 2);
/// assert_eq!(Mode::ALL, [Mode::Read, Mode::Write, Mode::Exec]);
/// ```
///
/// See also [`EnumExample`][crate::EnumExample] and [`EnumSetExample`][crate::EnumSetExample],
/// generated in the [enumset][crate::_doc::examples::enumset] example.
///
#[doc(hidden)]
#[macro_export]
macro_rules! enumset· {
    (
        // $enum_attr: the attributes of the enum.
        // $enum_vis:  the visibility of the enum.
        // $enum_name: the name of the new enum.
        // $set_attr:  the attributes of the set.
        // $set_vis:   the visibility of the set.
        // $set_name:  the name of the associated set.
        // $set_ty:    the inner unsigned integer primitive for the bit set (u8, u16, u32…).
        $( #[$enum_attr:meta] )*
        $enum_vis:vis enum $enum_name:ident
            $( <$($gen:tt),* $(,)?> )? // optional generics and lifetimes
            // attributes, visibility, name and inner type of the set, between ():
            ( $( #[$set_attr:meta] )* $set_vis:vis $set_name:ident: $set_ty:ty )
            $([where $($where:tt)+ $(,)? ] $(,)? )? // optional where clauses, between []
        {
            $(
                $( #[$variant_attr:meta] )*
                $variant_name:ident
                $(( $($tuple_type:ty),* $(,)? ))?
                $({ $( $( #[$field_attr:meta] )* $field_name:ident : $field_type:ty),* $(,)? })?
                $(= $discriminant:expr)?
                $(,)?
            )*
        }
        $( // optional impl blocks for the enum
            impl enum $(#[$impl_enum_attrs:meta])* { $($impl_enum:item)* }
        )*
        $( // optional impl blocks for the set
            impl set $(#[$impl_set_attrs:meta])* { $($impl_set:item)* }
        )*
    ) => {
        /* define enum */

        $( #[$enum_attr] )*
        // #[doc = "\n\nSee also the associated type set of variants [`" $set_name "`]."]
        #[allow(missing_debug_implementations, reason = "custom impl")]
        $enum_vis enum $enum_name $( <$($gen),*> )? $(where $($where)+)? {
            $(
                $( #[$variant_attr] )*
                $variant_name
                $(( $($tuple_type),* ))?
                $({ $( $( #[$field_attr] )* $field_name : $field_type),* })?
                $(= $discriminant)?
            ),*
        }
        /* impl enum methods */
        $crate::__enumset_impl_enum_blocks! {
            [$( <$($gen),*> )?] [$enum_name] [$( <$($gen),*> )?] [$( where $($where)+ )?]
            $(
                [$( #[$impl_enum_attrs] )*]
                { $($impl_enum)* }
            )*
        }

        /// # Variant-set methods
        #[allow(clippy::double_must_use, dead_code)]
        impl $( <$($gen),*> )? $enum_name $( <$($gen),*> )? $( where $($where)* )? {
            /// The total number of *declared* variants.
            $enum_vis const VARIANTS: usize = $crate::paste! { [<_$enum_name _private>]::VARIANTS };

            /// Returns the total number of *declared* variants.
            #[must_use]
            $enum_vis const fn variants(&self) -> usize { Self::VARIANTS }
            /// Returns the associated empty set.
            #[must_use]
            $enum_vis const fn empty_set() -> $set_name { $set_name::new() }
            /// Returns the associated full set.
            #[must_use]
            $enum_vis const fn full_set() -> $set_name { $set_name::all() }
            /// Returns the singleton set for this variant.
            #[must_use]
            $enum_vis const fn to_set(&self) -> $set_name {
                #[allow(unused_doc_comments, reason = "attributes could be comments")]
                match self {
                    $(
                        $( #[$variant_attr] )*
                        $crate::__enumset_to_set_pattern!(
                            $enum_name, $variant_name
                            $(( $($tuple_type),* ))?
                            $({ $( $(#[$field_attr])* $field_name : $field_type),* })?
                        ) => $set_name::$variant_name,
                    )*
                }
            }
            /// Returns whether this variant belongs to `set`.
            #[must_use]
            $enum_vis const fn is_in(&self, set: $set_name) -> bool {
                set.contains(self.to_set())
            }
        }

        /* define the associated bit set */
        $crate::paste! {
            #[allow(non_snake_case)]
            mod [<_$enum_name _private>] {
                pub(super) const VARIANTS: usize = $crate::ident_total!($($variant_name),*);
                $crate::ident_const_index!(pub(super), VARIANTS; $($variant_name),*);
            }
            $crate::data::set! {
                $( #[$set_attr] )*
                $set_vis struct $set_name($set_ty) {
                    $(
                        #[doc = "The bit index that corresponds to `" $enum_name "::`[`"
                            $variant_name "`][" $enum_name "::" $variant_name "]."]
                        #[allow(non_upper_case_globals)]
                        $variant_name = ([<_$enum_name _private>]::$variant_name);
                    )*
                }
                $( // optional set impl blocks
                    $( #[$impl_set_attrs] )*
                    impl { $($impl_set)* }
                )*
            }
        }

        /// # Associated set methods
        impl $set_name {
            /* queries accepting enum variants */

            /// Returns `true` if the set contains the variant of `value`.
            #[must_use]
            $set_vis const fn contains_variant$(<$($gen),*>)?(self, value: &$enum_name$(<$($gen),*>)?)
            -> bool $( where $($where)+ )? {
                self.contains(value.to_set())
            }
            /// An alias of [`contains_variant`](Self::contains_variant).
            #[must_use]
            $set_vis const fn has_variant$(<$($gen),*>)?(self, value: &$enum_name$(< $($gen),*>)?)
                -> bool $( where $($where)+ )? {
                self.contains_variant(value)
            }

            /* pure set transforms accepting enum variants */

            /// Returns `self` with the variant of `value` inserted.
            #[must_use]
            $set_vis const fn with_variant$(<$($gen),*>)?(self, value: &$enum_name$(< $($gen),*>)?)
                -> Self $( where $($where)+ )? {
                self.with(value.to_set())
            }
            /// Returns `self` with the variant of `value` removed.
            #[must_use]
            $set_vis const fn without_variant$(<$($gen),*>)?(self, value: &$enum_name$(<$($gen),*>)?)
                -> Self $( where $($where)+ )? {
                self.without(value.to_set())
            }
            /// Returns `self` with the variant of `value` toggled.
            #[must_use]
            $set_vis const fn toggled_variant$(<$($gen),*>)?(self, value: &$enum_name$(<$($gen),*>)?)
                -> Self $( where $($where)+ )? {
                self.toggled(value.to_set())
            }

            /* in-place mutation accepting enum variants */

            /// Inserts the variant of `value`.
            $set_vis const fn insert_variant$(<$($gen),*>)?(&mut self, value: &$enum_name$(<$($gen),*>)?)
                $( where $($where)+ )? {
                self.insert(value.to_set());
            }
            /// Removes the variant of `value`.
            $set_vis const fn remove_variant$(<$($gen),*>)?(&mut self, value: &$enum_name$(<$($gen),*>)?)
                $( where $($where)+ )? {
                self.remove(value.to_set());
            }
            /// Toggles the variant of `value`.
            $set_vis const fn toggle_variant$(<$($gen),*>)?(&mut self, value:&$enum_name$(<$($gen),*>)?)
                $( where $($where)+ )? {
                self.toggle(value.to_set());
            }

            /* set-constant iteration, always valid */

            /// Calls `f` for each associated set constant present in this set.
            $set_vis fn for_each_set(self, mut f: impl FnMut(Self)) {
                $(
                    if self.contains(Self::$variant_name) { f(Self::$variant_name); }
                )*
            }
            /// Calls `f` for each associated set constant present in this set while it returns `true`.
            ///
            /// Returns `true` if all set constants were visited,
            /// or `false` if iteration stopped early.
            $set_vis fn for_each_set_while(self, mut f: impl FnMut(Self) -> bool) -> bool {
                $(
                    if self.contains(Self::$variant_name) && !f(Self::$variant_name) {
                        return false;
                    }
                )*
                true
            }
        }

        // enum-value iteration, only if unit-only
        $crate::__enumset_impl_unit_iter! {
            [$enum_vis] [$enum_name] [$set_name]
            [$( < $($gen),* > )?]
            [$( < $($gen),* > )?]
            [$( where $($where)+ )?]
            {
                $(
                    $( #[$variant_attr] )*
                    $variant_name
                    $(( $($tuple_type),* ))?
                    $({ $( $(#[$field_attr])* $field_name : $field_type),* })?
                    $(= $discriminant)?
                ),*
            }
        }

        impl $(<$($gen),*>)? From<$enum_name $(<$($gen),*>)?> for $set_name $(where $($where)+)? {
            /// Converts an enum value into its singleton variant set.
            fn from(value: $enum_name $(<$($gen),*>)?) -> Self { value.to_set() }
        }
        impl $(<$($gen),*>)? From<&$enum_name $(<$($gen),*>)?> for $set_name $(where $($where)+)? {
            /// Converts an enum value into its singleton variant set.
            fn from(value: &$enum_name $(<$($gen),*>)?) -> Self { value.to_set() }
        }
    };
}
#[doc(inline)]
pub use enumset· as enumset;

#[doc(hidden)]
#[macro_export]
macro_rules! __enumset_to_set_pattern {
    ($enum_name:ident, $variant_name:ident) => {
        $enum_name::$variant_name // Unit variant
    };
    ($enum_name:ident, $variant_name:ident ( $($field:tt)*) ) => {
        $enum_name::$variant_name(..) // Tuple variant
    };
    ($enum_name:ident, $variant_name:ident { $($field:tt)*} ) => {
        $enum_name::$variant_name { .. } // Struct variant
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __enumset_impl_enum_blocks {
    ( // End
        [$($impl_gen:tt)*][$enum_name:ident][$($ty_gen:tt)*][$($where:tt)*] ) => {};
    ( // Emit one impl block, then recurse
        [$($impl_gen:tt)*][$enum_name:ident][$($ty_gen:tt)*][$($where:tt)*]
        [$($impl_attrs:tt)*] { $($impl_item:item)* } $($rest:tt)*
    ) => {
        $($impl_attrs)*
        impl $($impl_gen)* $enum_name $($ty_gen)* $($where)* { $($impl_item)* }
        $crate::__enumset_impl_enum_blocks! {
            [$($impl_gen)*] [$enum_name] [$($ty_gen)*] [$($where)*] $($rest)*
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __enumset_impl_unit_iter {
    (
        [$enum_vis:vis] [$enum_name:ident] [$set_name:ident]
        [$($impl_gen:tt)*] [$($ty_gen:tt)*] [$($where:tt)*]
        {
            $(
                $( #[$variant_attr:meta] )*
                $variant_name:ident
                $(= $discriminant:expr)?
            ),* $(,)?
        }
    ) => {
        /// # Unit-only variant iteration
        ///
        /// Generated only when all declared variants are unit variants.
        impl $($impl_gen)* $enum_name $($ty_gen)* $($where)* {
            /// All declared variants, in declaration order.
            $enum_vis const ALL: [Self; Self::VARIANTS] = [$( Self::$variant_name ),*];
        }
        /// # Unit-only variant iteration
        ///
        /// Generated only when all declared variants are unit variants.
        impl $set_name {
            /// Iterates over the enum variants present in this set.
            pub fn iter(self) -> impl Iterator<Item = $enum_name $($ty_gen)*> {
                $enum_name::ALL.into_iter().filter(move |variant| variant.is_in(self))
            }
            /// Calls `f` for each variant present in this set.
            pub fn for_each(self, mut f: impl FnMut($enum_name $($ty_gen)*)) {
                for variant in $enum_name::ALL {
                    if variant.is_in(self) { f(variant); }
                }
            }
            /// Calls `f` for each enum variant present in this set while it returns `true`.
            ///
            /// Returns `true` if all variants were visited, or `false` if iteration stopped early.
            pub fn for_each_while(self, mut f: impl FnMut($enum_name $($ty_gen)*) -> bool) -> bool {
                for variant in $enum_name::ALL {
                    if variant.is_in(self) && !f(variant) { return false; }
                }
                true
            }
        }
    };
    // Non-unit enum: do not generate enum-value iteration.
    ($($tt:tt)*) => {};
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use crate::enumset;

    enumset! {
        enum TestEnum(TestEnumSet: u8) {
            A,
            B(bool),
            C { x: u8 },
        }
    }
    #[test]
    fn uses_set_macro() {
        assert_eq!(TestEnum::VARIANTS, 3);
        assert_eq!(TestEnumSet::A.bits(), 0b001);
        assert_eq!(TestEnumSet::B.bits(), 0b010);
        assert_eq!(TestEnumSet::C.bits(), 0b100);
        assert_eq!(TestEnum::empty_set().bits(), 0);
        assert_eq!(TestEnum::full_set().bits(), 0b111);
        let ab = TestEnumSet::A.union(TestEnumSet::B);
        assert!(ab.contains(TestEnumSet::A));
        assert!(ab.contains(TestEnumSet::B));
        assert!(!ab.contains(TestEnumSet::C));
        let mut s = TestEnumSet::A;
        s.insert(TestEnumSet::B);
        assert_eq!(s.bits(), 0b011);
        s.remove(TestEnumSet::A);
        assert_eq!(s.bits(), 0b010);
        s.toggle(TestEnumSet::C);
        assert_eq!(s.bits(), 0b110);
        s.clear();
        assert_eq!(s.bits(), 0);
        assert!(s.is_empty());
        assert_eq!((TestEnumSet::A | TestEnumSet::B).bits(), 0b011);
        assert_eq!((TestEnumSet::A | TestEnumSet::B) & TestEnumSet::B, TestEnumSet::B);
        assert_eq!((TestEnumSet::A | TestEnumSet::B) - TestEnumSet::A, TestEnumSet::B);
        assert_eq!((TestEnumSet::A | TestEnumSet::B) ^ TestEnumSet::B, TestEnumSet::A);
        assert_eq!((!TestEnumSet::A).bits(), 0b110);
    }
    #[test]
    fn is_in_set() {
        let a = TestEnum::A;
        let b = TestEnum::B(true);
        let c = TestEnum::C { x: 7 };
        assert_eq!(a.to_set(), TestEnumSet::A);
        assert_eq!(b.to_set(), TestEnumSet::B);
        assert_eq!(c.to_set(), TestEnumSet::C);
        assert!(a.is_in(TestEnumSet::A));
        assert!(b.is_in(TestEnumSet::A | TestEnumSet::B));
        assert!(c.is_in(TestEnumSet::C));
        assert!(!c.is_in(TestEnumSet::A | TestEnumSet::B));
    }
    #[test]
    fn generics() {
        enumset! {
            enum GenEnum<'a, T>(pub GenEnumSet: u8) {
                A(T),
                B(&'a str),
            }
            impl enum {
                pub fn is_a(&self) -> bool {
                    self.is_in(GenEnumSet::A)
                }
            }
        }
        let a = GenEnum::A(());
        assert!(a.is_a());
    }
    #[test]
    fn generic_payload_variant_methods() {
        enumset! {
            #[derive(Clone, Copy, Debug)]
            enum E<'a, T>(pub ES: u8) [where T: Copy] {
                A(&'a T),
                B { value: T },
            }
        }
        let x = 7u8;
        let a = E::A(&x);
        let b = E::B { value: x };
        let mut set = ES::default();
        assert!(!set.contains_variant(&a));
        set.insert_variant(&a);
        assert!(set.contains_variant(&a));
        assert!(set.has_variant(&a));
        set = set.with_variant(&b);
        assert!(set.contains_variant(&b));
        set.remove_variant(&a);
        assert!(!set.contains_variant(&a));
        set.toggle_variant(&b);
        assert!(!set.contains_variant(&b));
        assert!(set.with_variant(&a).without_variant(&a).is_empty());
        assert!(set.toggled_variant(&a).contains_variant(&a));
    }
    #[test]
    fn generic_payload_from_enum() {
        enumset! {
            #[derive(Clone, Copy, Debug)]
            enum E<'a, T>(pub ES: u8) [where T: Copy] {
                A(&'a T),
                B(T),
            }
        }
        let x = 3u8;
        let a = E::A(&x);
        let b = E::B(x);
        let sa: ES = a.into();
        let sb: ES = (&b).into();
        assert!(sa.contains(ES::A));
        assert!(sb.contains(ES::B));
    }
    #[test]
    fn unit_only_iteration() {
        enumset! {
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            enum Mode(pub ModeSet: u8) {
                Read,
                Write,
                Exec,
            }
        }
        assert_eq!(Mode::ALL, [Mode::Read, Mode::Write, Mode::Exec]);
        let set = ModeSet::Read.with(ModeSet::Exec);
        let mut seen = [false; 3];
        set.for_each(|m| match m {
            Mode::Read => seen[0] = true,
            Mode::Write => seen[1] = true,
            Mode::Exec => seen[2] = true,
        });
        assert_eq!(seen, [true, false, true]);
        let mut count = 0;
        let completed = set.for_each_while(|_| {
            count += 1;
            false
        });
        assert!(!completed);
        assert_eq!(count, 1);
    }

    /**
    ```compile_fail
    # use devela::enumset;
    enumset! { enum E(pub ES: u8) { A(bool) } }
    let _ = E::ALL;
    ```
    **/
    #[allow(dead_code)]
    fn unit_only_constant() {}
}
