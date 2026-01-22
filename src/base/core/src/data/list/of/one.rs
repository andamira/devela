// devela_base_core::data::list::one
//
//! Defines the [`Oneof`] type.
//

impl_oneof!();

/// Defines [`Oneof`] and implements all the methods.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[allow(clippy::crate_in_macro_def, reason = "impl_const_init arm")]
macro_rules! impl_oneof {
    /* points of entry, using hardcoded argument list */

    // main point of entry
    // (defines Oneof & implements everything)
    () => { $crate::impl_oneof!(%canonical); };
    //
    // var_name : var_idx(0-based) + var_nth(1-based) : nth_suffix
    ($($T:ident : $idx:literal + $nth:literal : $suf:literal),* $(,)?) => {
        $crate::impl_oneof!(define_enum: $($T:$nth:$suf),+);
        $crate::impl_oneof!(impl_default: $($T),+);
        $crate::impl_oneof!(impl_const_init: $($T),+);
        $crate::impl_oneof!(methods_general: $($T:$idx+$nth:$suf),+);
        $crate::impl_oneof!(methods_individ: $($T:$idx+$nth:$suf),+);
    };

    // point of entry for implementing ConstInitCore
    (impl_const_init) => {
        $crate::impl_oneof!(%canonical %map_ident impl_const_init:);
    };
    // real ConstInitCore implementation
    (impl_const_init: $_0:ident $(, $rest:ident)*) => {
        impl<const LEN: usize, $_0: crate::ConstInitCore, $($rest),*> crate::ConstInitCore
            for Oneof<LEN, $_0, $($rest),*> {
            const INIT: Self = Oneof::$_0($_0::INIT);
        }
    };

    /* helpers */

    // hardcoded canonical list (12 variants)
    (%canonical $($prefix:tt)*) => { $crate::impl_oneof! { $($prefix)*
        _0:0+1:"st", _1:1+2:"nd", _2:2+3:"rd", _3:3+4:"th", _4:4+5:"th", _5:5+6:"th",
        _6:6+7:"th", _7:7+8:"th", _8:8+9:"th", _9:9+10:"th", _10:10+11:"th", _11:11+12:"th",
    }};
    // maps the canonical list to one of idents and forwards them to the prefixed arm.
    (%map_ident $prefix:ident:
     $($T:ident : $idx:literal + $nth:literal : $suf:literal),* $(,)?) => {
        $crate::impl_oneof!($prefix: $($T),*);
    };
    // maps the canonical list to one of idents:nth:suf and forwards them to the prefixed arm.
    (%map_ident_nth_suf $prefix:ident:
     $($T:ident : $idx:literal + $nth:literal : $suf:literal),* $(,)?) => {
        $crate::impl_oneof!($prefix: $($T:$nth:$suf),+);
    };

    /* main arms */

    (define_enum: $($variant:ident : $nth:literal : $suf:literal),+) => { $crate::paste! {
        #[doc = crate::_tags!(code)]
        /// A generic, parameterized *enum* for expressing structured alternatives.
        #[doc = crate::_doc_location!("data/list")]
        ///
        /// Variants are expected to be **contiguous**, meaning `()` (unit types)
        /// should only appear at the **end**.
        ///
        /// The **first variant** (`A`) is considered the default,
        /// implementing [`Default`] when `A: Default`.
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum Oneof<const LEN: usize, $($variant = ()),+> {
            $(
                #[doc = "The " $nth $suf " variant."]
                $variant($variant)
            ),+
        }
    }};
    (impl_default: $_0:ident $(, $rest:ident)*) => {
        impl<const LEN: usize, $_0: Default, $($rest),*> Default for Oneof<LEN, $_0, $($rest),*> {
            fn default() -> Self { Oneof::$_0($_0::default()) }
        }
    };
    (
    // Implements:
    // - LEN
    // - MAX_ARITY
    // - variant_index
    // - is_variant_index
    //
    // - validate
    //
    // - into_tuple_options
    // - into_tuple_defaults
    // - as_tuple_ref_options
    methods_general: $($T:ident : $idx:literal + $nth:literal : $suf:literal),+) => {
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
        }
        impl<const LEN: usize, $($T: 'static),+ > Oneof<LEN, $($T),+> {
            /// Validates that inactive `()` variants only appear at the end,
            /// and that `LEN` equals the number of active variants.
            #[allow(unused_assignments, reason = "wont be read in all cases")]
            // WAIT const PartialEq for TypeId
            pub fn validate() -> bool {
                let mut non_unit_count = 0;
                let mut unit_found = false;
                $(
                    if $crate::TypeId::of::<$T>() == $crate::TypeId::of::<()>() {
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
        // IMPROVE: add a const fn for T: Copy (into_tuple_copy_options / into_tuple_const_init)
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
            // FUTURE: make the `()` types not wrapped in option.
            pub const fn as_tuple_ref_options(&self) -> ($(Option<&$T>),+) { $crate::paste! {
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
    methods_individ: $($T:ident : $idx:literal + $nth:literal : $suf:literal),+) => {
        /// # Variant-specific methods.
        impl<const LEN: usize, $($T),+> Oneof<LEN, $($T),+> {
            // $crate::impl_oneof!(methods_field_access: $($T),+);
            $crate::impl_oneof!(methods_field_access: $($T:$idx+$nth:$suf),+);
            $crate::impl_oneof!(methods_map: $($T),+);
            // $crate::impl_oneof!(methods_map: $($T:$idx+$nth),+);
        }
    };
    (
    // implements:
    // - is_*
    // - into_*
    // - as_ref_*
    // - as_mut_*
    methods_field_access: $($T:ident : $idx:literal + $nth:literal : $suf:literal),+) => {
        $( $crate::impl_oneof! { @methods_field_access: $T : $idx + $nth : $suf} )+
    };
    (@methods_field_access: $T:ident : $idx:literal + $nth:literal : $suf:literal
    ) => { $crate::paste! {
        #[doc = "Returns `true` if there's a value in variant [`"
            $T "`](#variant." $T ") (The " $nth $suf ")."]
        pub const fn [<is $T>](&self) -> bool { matches!(self, Oneof::$T(_)) }

        #[doc = "Returns a shared reference to the inner value in variant `" $T "`, if present."]
        pub const fn [<as_ref $T>](&self) -> Option<&$T> {
            $crate::is![let Self::$T($T) = self; Some($T); None]
        }
        #[doc = "Returns an exclusive reference to the inner value in variant`" $T "`, if present."]
        pub const fn [<as_mut $T>](&mut self) -> Option<&mut $T> {
            $crate::is![let Self::$T($T) = self; Some($T); None]
        }
        #[doc = "Returns a copy of the value in variant `" $T "`, if present."]
        pub const fn [<copy $T >](self) -> Option<$T> where Self: Copy {
            $crate::is![let Self::$T($T) = self; Some($T); None]
        }
        #[doc = "Returns the owned value in variant `" $T "`, if present."]
        #[doc = "<hr/>"] // separator after the last method
        pub fn [<into $T>](self) -> Option<$T> {
            $crate::is![let Self::$T($T) = self; Some($T); None]
        }
    }};
    (
    // implements:
    // - map_*
    methods_map: $first:ident $(, $rest:ident)*) => {
        // For the first variant, the `$before` list is empty.
        $crate::impl_oneof!(@methods_map: $first, (), ($($rest),*));
        // Then, delegate to the helper macro with the first element as the accumulator.
        $crate::impl_oneof!(@methods_map_helper: ($first), ($($rest),*));

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
        #[doc = "Transforms the inner value in variant`" $T
        "` using `f`, leaving other variants unchanged."]
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
        $crate::impl_oneof!(@methods_map: $first, ($($before),*), ($($rest),*));
        // Append the current type to the "before" list and continue.
        $crate::impl_oneof!(@methods_map_helper: ($($before,)* $first), ($($rest),*));
    };
}
#[doc(hidden)]
pub use impl_oneof;

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
        // assert_eq![u.variant_name(), "_2"];
        // assert_eq![u.is_variant_name("_2"), true];
        // assert_eq![u.is_variant_name("_1"), false];

        let u = Unums::_0(32);
        assert_eq![u.variant_index(), 0];
        assert_eq![u.is_variant_index(0), true];
        assert_eq![u.is_variant_index(1), false];
        // assert_eq![u.variant_name(), "_0"];
        // assert_eq![u.is_variant_name("_0"), true];
        // assert_eq![u.is_variant_name("_1"), false];
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
