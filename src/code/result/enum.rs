// devela::code::result::enum
//
//! Defines the [`Enum`] type.
//

use crate::{iif, ConstDefault, ExtAny};

// 12 variants by default
impl_enum!(A:0+1, B:1+2, C:2+3, D:3+4, E:4+5, F:5+6, G:6+7, H:7+8, I:8+9, J:9+10, K:10+11, L:11+12);

/// Defines [`Enum`] and implements all the methods.
//
//
// Supports >= 3 variants.
macro_rules! impl_enum {
    (
    // var_name : var_idx(0-based) + var_nth(1-based)
    $A:ident: $_a:literal + $a:literal,
    $B:ident: $_b:literal + $b:literal,
    $C:ident: $_c:literal + $c:literal,
    $($T:ident : $idx:literal + $nth:literal),*) => {
        impl_enum!(define_enum: $A, $B, $C $(, $T:$nth)*);
        impl_enum!(impl_default: $A, $B, $C $(, $T)*);
        impl_enum!(methods_general: $A:$_a+$a, $B:$_a+$b, $C:$_c+$c $(, $T:$idx+$nth)*);
        impl_enum!(methods_individual: $A, $B, $C $(, $T)*);
    };
    (define_enum: $A:ident, $B:ident, $C:ident, $($rest:ident:$nth:literal),*) => { $crate::paste! {
        /// A generic, parameterized *enum* for expressing structured alternatives.
        ///
        /// Variants are expected to be **contiguous**, meaning `()` (unit types)
        /// should only appear at the **end**.
        ///
        /// The **first variant** (`A`) is considered the default,
        /// implementing [`Default`] when `A: Default`.
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum Enum<const LEN: usize, $A, $B, $C=(), $($rest = ()),*> {
            #[doc = "The 1st variant (default)."] $A($A),
            #[doc = "The 2nd variant."] $B($B),
            #[doc = "The 3rd variant."] $C($C),
            $( #[doc = "The " $nth "th variant."] $rest($rest), )*
        }
    }};
    (
    impl_default: $A:ident $(, $rest:ident)*) => {
        impl<const LEN: usize, $A: Default, $($rest),*> Default for Enum<LEN, $A, $($rest),*> {
            fn default() -> Self { Enum::$A($A::default()) }
        }
        impl<const LEN: usize, $A: ConstDefault, $($rest),*> ConstDefault
            for Enum<LEN, $A, $($rest),*> {
            const DEFAULT: Self = Enum::$A($A::DEFAULT);
        }
    };
    (
    // Implements:
    // - LEN
    // - MAX_ARITY
    // - variant_index
    // - is_variant_index
    // - variant_name
    // - is_variant_name
    //
    // - first_non_unit
    // - validate
    //
    // - into_tuple_options
    // - into_tuple_defaults
    methods_general: $($T:ident : $idx:literal + $nth:literal),+) => {
        /// # Structural methods.
        impl<const LEN:usize,  $($T),+ > Enum<LEN, $($T),+> {
            /// The number of active (non-`()` type) variants.
            pub const LEN: usize = {
                assert![LEN <= Self::MAX_ARITY, "LEN must be <= MAX_ARITY"];
                LEN
            };
            /// The maximum number of generic type parameters in this enum.
            pub const MAX_ARITY: usize = $crate::ident_total!($($T),+);
            // pub const MAX_ARITY: usize = $crate::capture_last![literal $($nth),+]; // BENCH

            /// Returns the current variant index (0-based).
            pub const fn variant_index(&self) -> usize {
                match self { $( Enum::$T(_) => $idx ),+ }
            }
            /// Checks whether the current variant is at `index` (0-based).
            pub const fn is_variant_index(&self, index: usize) -> bool {
                self.variant_index() == index
            }

            /// Returns the current variant name.
            pub const fn variant_name(&self) -> &'static str {
                match self { $( Enum::$T(_) => stringify!($T) ),+ }
            }
            /// Checks whether the current variant has the given `name`.
            pub const fn is_variant_name(&self, name: &str) -> bool {
                $crate::Slice::<&str>::eq(self.variant_name(), name)
            }
        }
        impl<const LEN: usize, $($T: 'static),+ > Enum<LEN, $($T),+> {
            /// Returns the first non-unit variant name, if any.
            // WAIT: [const_type_id](https://github.com/rust-lang/rust/issues/77125)
            pub fn first_non_unit() -> Option<&'static str> {
                $( if <$T>::type_id() != <()>::type_id() { return Some(stringify!($T)); } )+
                None
            }

            /// Validates that inactive `()` variants only appear at the end,
            /// and that `LEN` equals the number of active variants.
            #[allow(unused_assignments, reason = "wont be read in all cases")]
            pub fn validate() -> bool {
                let mut non_unit_count = 0;
                let mut unit_found = false;
                $(
                    if <$T>::type_id() == <()>::type_id() {
                        unit_found = true;
                    } else {
                        if unit_found { return false; }
                        non_unit_count += 1;
                    }
                )+
                LEN == non_unit_count
            }
        }
        /// # Conversion methods.
        impl<const LEN: usize, $($T: Clone),+ > Enum<LEN, $($T),+> {
            /// Returns a tuple with `Some(value)` for the active variant and `None` elsewhere.
            pub fn into_tuple_options(self) -> ($(Option<$T>),+) { $crate::paste! {
                let index = self.variant_index();
                ( $(
                    if $idx == index {
                        self.clone().[<into_ $T:lower>]()
                    } else {
                        None::<$T>
                    }
                ),+ )
            }}

            /// Returns a tuple with the active variant’s inner value in its corresponding position
            /// and `Default::default()` for all others.
            pub fn into_tuple_defaults(self) -> ($($T),+) where $($T: Default),+ { $crate::paste! {
                let index = self.variant_index();
                ( $(
                    if $idx == index {
                        self.clone().[<into_ $T:lower>]().unwrap()
                    } else {
                        Default::default()
                    }
                ),+ )
            }}
        }
        impl<const LEN: usize, $($T),+ > Enum<LEN, $($T),+> {
            /// Returns a tuple with `Some(&value)` for the active variant and `None` elsewhere.
            // WAIT: [const_type_id](https://github.com/rust-lang/rust/issues/77125)
            // FUTURE: make the `()` types not wrapped in option.
            pub fn as_tuple_ref_options(&self) -> ($(Option<&$T>),+) { $crate::paste! {
                ( $(
                    if $idx == self.variant_index() {
                        self.[<as_ref_ $T:lower>]()
                    } else {
                        None::<&$T>
                    }
                ),+ )
            }}
        }
    };
    (
    // Implements methods acting over individual fields.
    methods_individual: $($T:ident),+) => {
        /// # Variant-specific methods.
        impl<const LEN: usize, $($T),+> Enum<LEN, $($T),+> {
            impl_enum!(methods_field_access: $($T),+);
            impl_enum!(methods_map: $($T),+);
        }
    };
    (
    // implements:
    // - is_*
    // - into_*
    // - as_ref_*
    // - as_mut_*
    methods_field_access: $($T:ident),+) => {
        $( impl_enum! { @methods_field_access: $T } )+
    };
    (@methods_field_access: $T:ident) => { $crate::paste! {
        #[doc = "Returns `true` if the value is of type [`" $T "`][Self::" $T "]"]
        pub const fn [<is_ $T:lower>](&self) -> bool { matches!(self, Enum::$T(_)) }

        #[doc = "Returns the inner `" $T "` value, if present."]
        pub fn [<into_ $T:lower>](self) -> Option<$T> {
            iif![let Self::$T([<$T:lower>]) = self; Some([<$T:lower>]); None]
        }
        #[doc = "Returns a reference to the inner `" $T "` value, if present."]
        pub fn [<as_ref_ $T:lower>](&self) -> Option<&$T> {
            iif![let Self::$T([<$T:lower>]) = self; Some([<$T:lower>]); None]
        }
        #[doc = "Returns a reference to the inner `" $T "` value, if present."]
        pub fn [<as_mut_ $T:lower>](&mut self) -> Option<&mut $T> {
            iif![let Self::$T([<$T:lower>]) = self; Some([<$T:lower>]); None]
        }
    }};
    (
    // implements:
    // - map_*
    methods_map: $first:ident $(, $rest:ident)*) => {
        // For the first variant, the `$before` list is empty.
        impl_enum!(@methods_map: $first, (), ($($rest),*));
        // Then, delegate to the helper macro with the first element as the accumulator.
        impl_enum!(@methods_map_helper: ($first), ($($rest),*));

        // NOTE: generates something like the following (e.g. for 6 variants):
        //
        // impl_map_method!(A, (/*$before*/), (B, C, D, E, F));
        // impl_map_method!(B, (A), (C, D, E, F));
        // impl_map_method!(C, (A, B), (D, E, F));
        // impl_map_method!(D, (A, B, C), (E, F));
        // impl_map_method!(E, (A, B, C, D), (F));
        // impl_map_method!(F, (A, B, C, D, E), (/*$after*/));
    };
    (
    @methods_map: $T:ident, ( $($before:ident),* ), ( $($after:ident),* )) => { $crate::paste! {
        #[doc = "Transforms the inner `" $T "` value using `f`, leaving other variants unchanged."]
        pub fn [<map_ $T:lower>]<NEW>(self, f: impl FnOnce($T) -> NEW)
            -> Enum<LEN, $($before,)* NEW, $($after,)* > {
            match self {
                $( Self::$before(val) => Enum::$before(val), )*
                Self::$T(val) => Enum::$T(f(val)),
                $( Self::$after(val) => Enum::$after(val), )*
            }
        }
        // NOTE: Generates methods like the following (e.g. for variant C, of 6):
        //
        // pub fn map_c<NEW>(self, f: impl FnOnce(C) -> NEW) -> Enum<A, B, NEW, D, E, F> {
        //     match self {
        //         Self::A(a) => Enum::A(a),    // $before
        //         Self::B(b) => Enum::B(b),    // …
        //         Self::C(c) => Enum::C(f(c)), // $T
        //         Self::D(d) => Enum::D(d),    // $after
        //         Self::E(e) => Enum::E(e),    // …
        //         Self::F(f) => Enum::F(f),    // …
        //     }
        // }
    }};
    // Stop when there are no types left in the `$after` list.
    (@methods_map_helper: ($($before:ident),*), ()) => {};
    // Recursively take the next type as the current one.
    (@methods_map_helper: ($($before:ident),*), ($first:ident $(, $rest:ident)*)) => {
        impl_enum!(@methods_map: $first, ($($before),*), ($($rest),*));
        // Append the current type to the "before" list and continue.
        impl_enum!(@methods_map_helper: ($($before,)* $first), ($($rest),*));
    };
}
use impl_enum;

#[cfg(test)]
mod tests {
    use super::Enum;

    type Bytes = Enum<2, u8, i8>;
    type Unums = Enum<4, u8, u16, u32, u64>;

    #[test]
    fn validate() {
        assert![Bytes::validate()];
        assert![Unums::validate()];
        assert![Enum::<0, (), (), ()>::validate()];
        assert![Enum::<1, i8, (), ()>::validate()];
        assert![!Enum::<0, i8, (), ()>::validate()];
        assert![!Enum::<2, i8, (), ()>::validate()];
        //
        assert![!Enum::<1, (), i8, ()>::validate()];
        assert![!Enum::<2, i32, (), i8>::validate()];
        assert![!Enum::<1, (), (), i8, ()>::validate()];
    }
    #[test]
    fn map() {
        let a: Enum<2, i32, f64> = Enum::A(10);
        assert_eq![Enum::A(20), a.map_a(|v| v * 2)];
        assert_eq![Enum::A(10), a.map_b(|v| v * 2.0)];
        let b: Enum<2, i32, f64> = Enum::B(3.14);
        assert_eq![Enum::B(3.14), b.map_a(|v| v * 2)];
        assert_eq![Enum::B(6.28), b.map_b(|v| v * 2.0)];
    }
    #[test]
    fn field_access() {
        let mut u = Unums::C(32);
        assert_eq![u.is_c(), true];
        assert_eq![u.into_c(), Some(32)];
        assert_eq![u.as_ref_c(), Some(&32)];
        assert_eq![u.as_mut_c(), Some(&mut 32)];
        //
        assert_eq![u.is_a(), false];
        assert_eq![u.into_a(), None];
        assert_eq![u.as_ref_a(), None];
        assert_eq![u.as_mut_a(), None];
    }
    #[test]
    fn positioning() {
        let u = Unums::C(32);
        assert_eq![u.variant_index(), 2];
        assert_eq![u.is_variant_index(2), true];
        assert_eq![u.is_variant_index(3), false];
        assert_eq![u.variant_name(), "C"];
        assert_eq![u.is_variant_name("C"), true];
        assert_eq![u.is_variant_name("B"), false];

        let u = Unums::A(32);
        assert_eq![u.variant_index(), 0];
        assert_eq![u.is_variant_index(0), true];
        assert_eq![u.is_variant_index(1), false];
        assert_eq![u.variant_name(), "A"];
        assert_eq![u.is_variant_name("A"), true];
        assert_eq![u.is_variant_name("B"), false];
    }
    #[test]
    fn tuple() {
        let u = Unums::C(32);
        assert_eq![
            u.into_tuple_options(),
            (None, None, Some(32), None, None, None, None, None, None, None, None, None)
        ];
        assert_eq![
            u.as_tuple_ref_options(),
            (None, None, Some(&32), None, None, None, None, None, None, None, None, None)
        ];
        assert_eq![
            u.into_tuple_defaults(),
            (0, 0, 32, 0, (), (), (), (), (), (), (), ())
        ];
    }
}
