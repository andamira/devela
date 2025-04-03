// devela::data::list::enum
//
//! Defines the [`Oneof`] type.
//

use crate::{ConstDefault, ExtAny, is};

// 12 variants by default
impl_enum!(
    _0:0+1, _1:1+2, _2:2+3, _3:3+4, _4:4+5, _5:5+6,
    _6:6+7, _7:7+8, _8:8+9, _9:9+10, _10:10+11, _11:11+12
);

/// Defines [`Oneof`] and implements all the methods.
//
// Supports >= 3 variants.
macro_rules! impl_enum {
    (
    // var_name : var_idx(0-based) + var_nth(1-based)
    $_0:ident: $_0a:literal + $_0b:literal,
    $_1:ident: $_1a:literal + $_1b:literal,
    $_2:ident: $_2a:literal + $_2b:literal,
    $($T:ident : $idx:literal + $nth:literal),*) => {
        impl_enum!(define_enum: $_0, $_1, $_2 $(, $T:$nth)*);
        impl_enum!(impl_default: $_0, $_1, $_2 $(, $T)*);
        impl_enum!(methods_general: $_0:$_0a+$_0b, $_1:$_0a+$_1b, $_2:$_2a+$_2b $(, $T:$idx+$nth)*);
        impl_enum!(methods_individual: $_0, $_1, $_2 $(, $T)*);
    };
    (define_enum: $_0:ident, $_1:ident, $_2:ident, $($rest:ident:$nth:literal),*
    ) => { $crate::paste! {
        /// A generic, parameterized *enum* for expressing structured alternatives.
        ///
        /// Variants are expected to be **contiguous**, meaning `()` (unit types)
        /// should only appear at the **end**.
        ///
        /// The **first variant** (`A`) is considered the default,
        /// implementing [`Default`] when `A: Default`.
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum Oneof<const LEN: usize, $_0, $_1, $_2=(), $($rest = ()),*> {
            #[doc = "The 1st variant (default)."] $_0($_0),
            #[doc = "The 2nd variant."] $_1($_1),
            #[doc = "The 3rd variant."] $_2($_2),
            $( #[doc = "The " $nth "th variant."] $rest($rest), )*
        }
    }};
    (
    impl_default: $_0:ident $(, $rest:ident)*) => {
        impl<const LEN: usize, $_0: Default, $($rest),*> Default for Oneof<LEN, $_0, $($rest),*> {
            fn default() -> Self { Oneof::$_0($_0::default()) }
        }
        impl<const LEN: usize, $_0: ConstDefault, $($rest),*> ConstDefault
            for Oneof<LEN, $_0, $($rest),*> {
            const DEFAULT: Self = Oneof::$_0($_0::DEFAULT);
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
        impl<const LEN:usize,  $($T),+ > Oneof<LEN, $($T),+> {
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
                match self { $( Oneof::$T(_) => $idx ),+ }
            }
            /// Checks whether the current variant is at `index` (0-based).
            pub const fn is_variant_index(&self, index: usize) -> bool {
                self.variant_index() == index
            }

            /// Returns the current variant name.
            pub const fn variant_name(&self) -> &'static str {
                match self { $( Oneof::$T(_) => stringify!($T) ),+ }
            }
            /// Checks whether the current variant has the given `name`.
            pub const fn is_variant_name(&self, name: &str) -> bool {
                $crate::Slice::<&str>::eq(self.variant_name(), name)
            }
        }
        impl<const LEN: usize, $($T: 'static),+ > Oneof<LEN, $($T),+> {
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
        impl<const LEN: usize, $($T: Clone),+ > Oneof<LEN, $($T),+> {
            /// Returns a tuple with `Some(value)` for the active variant and `None` elsewhere.
            pub fn into_tuple_options(self) -> ($(Option<$T>),+) { $crate::paste! {
                let index = self.variant_index();
                ( $(
                    if $idx == index {
                        self.clone().[<into $T>]()
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
                        self.clone().[<into $T>]().unwrap()
                    } else {
                        Default::default()
                    }
                ),+ )
            }}
        }
        impl<const LEN: usize, $($T),+ > Oneof<LEN, $($T),+> {
            /// Returns a tuple with `Some(&value)` for the active variant and `None` elsewhere.
            // WAIT: [const_type_id](https://github.com/rust-lang/rust/issues/77125)
            // FUTURE: make the `()` types not wrapped in option.
            pub fn as_tuple_ref_options(&self) -> ($(Option<&$T>),+) { $crate::paste! {
                ( $(
                    if $idx == self.variant_index() {
                        self.[<as_ref $T>]()
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
        impl<const LEN: usize, $($T),+> Oneof<LEN, $($T),+> {
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
        pub const fn [<is $T>](&self) -> bool { matches!(self, Oneof::$T(_)) }

        #[doc = "Returns the inner `" $T "` value, if present."]
        pub fn [<into $T>](self) -> Option<$T> {
            is![let Self::$T($T) = self; Some($T); None]
        }
        #[doc = "Returns a reference to the inner `" $T "` value, if present."]
        pub fn [<as_ref $T>](&self) -> Option<&$T> {
            is![let Self::$T($T) = self; Some($T); None]
        }
        #[doc = "Returns a reference to the inner `" $T "` value, if present."]
        pub fn [<as_mut $T>](&mut self) -> Option<&mut $T> {
            is![let Self::$T($T) = self; Some($T); None]
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
        // impl_map_method!(_0, (/*$before*/), (_1, _2, _3, _4, _5));
        // impl_map_method!(_1, (_0), (_2, _3, _4, _5));
        // impl_map_method!(_2, (_0, _1), (_3, _4, _5));
        // impl_map_method!(_3, (_0, _1, _2), (_4, _5));
        // impl_map_method!(_4, (_0, _1, _2, _3), (_5));
        // impl_map_method!(_5, (_0, _1, _2, _3, _4), (/*$after*/));
    };
    (
    @methods_map: $T:ident, ( $($before:ident),* ), ( $($after:ident),* )) => { $crate::paste! {
        #[doc = "Transforms the inner `" $T "` value using `f`, leaving other variants unchanged."]
        pub fn [<map $T>]<NEW>(self, f: impl FnOnce($T) -> NEW)
            -> Oneof<LEN, $($before,)* NEW, $($after,)* > {
            match self {
                $( Self::$before(val) => Oneof::$before(val), )*
                Self::$T(val) => Oneof::$T(f(val)),
                $( Self::$after(val) => Oneof::$after(val), )*
            }
        }
        // NOTE: Generates methods like the following (e.g. for variant _2, of 6 total):
        //
        // pub fn map_c<NEW>(self, f: impl FnOnce(_2) -> NEW) -> Oneof<_0, _1, NEW, _3, _4, _5> {
        //     match self {
        //         Self::_0(t) => Oneof::_0(t),    // $before
        //         Self::_1(t) => Oneof::_1(t),    // …
        //         Self::_2(t) => Oneof::_2(f(t)), // $T
        //         Self::_3(t) => Oneof::_3(t),    // $after
        //         Self::_4(t) => Oneof::_4(t),    // …
        //         Self::_5(t) => Oneof::_5(t),    // …
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
    use super::Oneof;

    type Bytes = Oneof<2, u8, i8>;
    type Unums = Oneof<4, u8, u16, u32, u64>;

    #[test]
    fn validate() {
        assert![Bytes::validate()];
        assert![Unums::validate()];
        assert![Oneof::<0, (), (), ()>::validate()];
        assert![Oneof::<1, i8, (), ()>::validate()];
        assert![!Oneof::<0, i8, (), ()>::validate()];
        assert![!Oneof::<2, i8, (), ()>::validate()];
        //
        assert![!Oneof::<1, (), i8, ()>::validate()];
        assert![!Oneof::<2, i32, (), i8>::validate()];
        assert![!Oneof::<1, (), (), i8, ()>::validate()];
    }
    #[test]
    fn map() {
        let a: Oneof<2, i32, f64> = Oneof::_0(10);
        assert_eq![Oneof::_0(20), a.map_0(|v| v * 2)];
        assert_eq![Oneof::_0(10), a.map_1(|v| v * 2.0)];
        let b: Oneof<2, i32, f64> = Oneof::_1(3.14);
        assert_eq![Oneof::_1(3.14), b.map_0(|v| v * 2)];
        assert_eq![Oneof::_1(6.28), b.map_1(|v| v * 2.0)];
    }
    #[test]
    fn field_access() {
        let mut u = Unums::_2(32);
        assert_eq![u.is_2(), true];
        assert_eq![u.into_2(), Some(32)];
        assert_eq![u.as_ref_2(), Some(&32)];
        assert_eq![u.as_mut_2(), Some(&mut 32)];
        //
        assert_eq![u.is_0(), false];
        assert_eq![u.into_0(), None];
        assert_eq![u.as_ref_0(), None];
        assert_eq![u.as_mut_0(), None];
    }
    #[test]
    fn positioning() {
        let u = Unums::_2(32);
        assert_eq![u.variant_index(), 2];
        assert_eq![u.is_variant_index(2), true];
        assert_eq![u.is_variant_index(3), false];
        assert_eq![u.variant_name(), "_2"];
        assert_eq![u.is_variant_name("_2"), true];
        assert_eq![u.is_variant_name("_1"), false];

        let u = Unums::_0(32);
        assert_eq![u.variant_index(), 0];
        assert_eq![u.is_variant_index(0), true];
        assert_eq![u.is_variant_index(1), false];
        assert_eq![u.variant_name(), "_0"];
        assert_eq![u.is_variant_name("_0"), true];
        assert_eq![u.is_variant_name("_1"), false];
    }
    #[test]
    fn tuple() {
        let u = Unums::_2(32);
        assert_eq![
            u.into_tuple_options(),
            (None, None, Some(32), None, None, None, None, None, None, None, None, None)
        ];
        assert_eq![
            u.as_tuple_ref_options(),
            (None, None, Some(&32), None, None, None, None, None, None, None, None, None)
        ];
        assert_eq![u.into_tuple_defaults(), (0, 0, 32, 0, (), (), (), (), (), (), (), ())];
    }
}
