// devela::code::result::enum
//
//! Defines the [`Enum`] type.
//
// TOC
// - enum Enum
// - impl methods
// - macro impl_enum! helper
// - tests

use crate::{iif, ExtAny};

/// A generic, parameterized *enum* for expressing structured alternatives.
///
/// Variants are expected to be **contiguous**, meaning `()` (unit types)
/// should only appear at the **end**.
///
/// The **first variant** (`A`) is considered the default,
/// implementing [`Default`] when `A: Default`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Enum<A, B, C = (), D = (), E = (), F = ()> {
    /// The first variant (default).
    A(A),
    /// The second variant.
    B(B),
    /// The third variant.
    C(C),
    /// The fourth variant.
    D(D),
    /// The fifth variant.
    E(E),
    /// The sixth variant.
    F(F),
}
impl<A: Default, B, C, D, E, F> Default for Enum<A, B, C, D, E, F> {
    /// Defaults to [`Self::A`] using `A::default()`.
    fn default() -> Self {
        Enum::A(A::default())
    }
}

/// General methods.
impl<A: 'static, B: 'static, C: 'static, D: 'static, E: 'static, F: 'static>
    Enum<A, B, C, D, E, F>
{
    impl_enum!(methods_general: A, B, C, D, E, F);
}

/// # Field-specific methods.
impl<A, B, C, D, E, F> Enum<A, B, C, D, E, F> {
    impl_enum!(methods_field_access: A, B, C, D, E, F);
    impl_enum!(methods_map: A, B, C, D, E, F);
}

/// helper for implementing methods for Enum
///
/// Args:
/// - $T: the uppercase generic identifier
macro_rules! impl_enum {
    (
    // implements:
    // - len
    // - is_empty
    // - validate
    // - first_non_unit
    // - variants_count
    // - eq_variant
    methods_general: $($T:ident),+) => {
        /// Returns the number of active (non-`()` type) variants.
        // WAIT: [const_type_id](https://github.com/rust-lang/rust/issues/77125)
        // WAIT: [variant_count](https://github.com/rust-lang/rust/issues/73662)
        pub fn len() -> usize {
            let mut count = 0;
            $( if <$T>::type_id() != <()>::type_id() { count += 1} )+
            count
        }
        /// Returns `true` if all the variants are of the unit type.
        pub fn is_empty() -> bool { Self::len() == 0 }

        /// Returns the first non-unit variant name, if any.
        // WAIT: [const_type_id](https://github.com/rust-lang/rust/issues/77125)
        pub fn first_non_unit() -> Option<&'static str> {
            $( if <$T>::type_id() != <()>::type_id() { return Some(stringify!($T)); } )+
            None
        }

        /// Validates that `()` variants only appear at the end.
        #[allow(unused_assignments, reason = "wont be read in all cases")]
        pub fn validate() -> bool {
            let mut u = false; // unit variant found
            $( if <$T>::type_id() == <()>::type_id() { u = true; } else if u { return false; } )+
            true
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
        pub fn [<is_ $T:lower>](&self) -> bool { matches!(self, Enum::$T(_)) }

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
            -> Enum< $($before,)* NEW, $($after,)* > {
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

    type Bytes = Enum<u8, i8>;
    type Unums = Enum<u8, u16, u32, u64>;

    #[test]
    fn len() {
        assert_eq![Bytes::len(), 2];
        assert_eq![Unums::len(), 4];
        assert![!Bytes::is_empty()];
        assert![Enum::<(), ()>::is_empty()];
    }
    #[test]
    fn validate() {
        assert![Bytes::validate()];
        assert![Unums::validate()];
        assert![Enum::<i8, ()>::validate()];
        assert![!Enum::<(), i8>::validate()];
        assert![!Enum::<i32, (), i8>::validate()];
        assert![!Enum::<(), (), i8, ()>::validate()];
    }
    #[test]
    fn map() {
        let a: Enum<i32, f64> = Enum::A(10);
        assert_eq![Enum::A(20), a.map_a(|v| v * 2)];
        assert_eq![Enum::A(10), a.map_b(|v| v * 2.0)];
        let b: Enum<i32, f64> = Enum::B(3.14);
        assert_eq![Enum::B(3.14), b.map_a(|v| v * 2)];
        assert_eq![Enum::B(6.28), b.map_b(|v| v * 2.0)];
    }
}
