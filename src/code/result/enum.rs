// devela::code::result::enum
//
//! Define the [`Enum`] type.
//

use crate::{iif, ExtAny};

/// A statically sized selection enum allowing up to four possible types.
///
/// Variants are expected to be **contiguous**, meaning `()` (unit types)
/// should only appear at the **end**. Use [`Self::validate`] to check for correctness.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Enum<A, B, C = (), D = (), E = (), F = ()> {
    /// The first selection. (The default)
    A(A),
    /// The second selection.
    B(B),
    /// The third selection.
    C(C),
    /// The fourth selection.
    D(D),
    /// The fifth selection.
    E(E),
    /// The sixth selection.
    F(F),
}
impl<A: Default, B, C, D, E, F> Default for Enum<A, B, C, D, E, F> {
    /// Defaults to [`Self::A`] using `A::default()`.
    fn default() -> Self {
        Enum::A(A::default())
    }
}

/// helper for implementing methods for Enum
///
/// Args:
/// - $T: the uppercase generic identifier
macro_rules! impl_enum {
    (
    // implements:
    // - is_a
    // - as_ref_a
    // - as_mut_a
    methods1: $($T:ident),+) => {
        $( impl_enum! { @methods1: $T } )+
    };
    (@methods1: $T:ident) => { $crate::paste! {
        #[doc = "Returns `true` if the value is of type [`Self::" $T "`]"]
        pub fn [<is_ $T:lower>](&self) -> bool { matches!(self, Enum::$T(_)) }

        #[doc = "Returns the inner `" $T "` value, if present."]
        pub fn [<into_ $T:lower>](self) -> Option<$T> {
            iif![let Self::$T([<$T:lower>]) = self; Some([<$T:lower>]); None]
        }
        #[doc = "Returns a reference to the inner `" $T "` value, if present."]
        pub fn [<as_ref $T:lower>](&self) -> Option<&$T> {
            iif![let Self::$T([<$T:lower>]) = self; Some([<$T:lower>]); None]
        }
        #[doc = "Returns a reference to the inner `" $T "` value, if present."]
        pub fn [<as_mut $T:lower>](&mut self) -> Option<&mut $T> {
            iif![let Self::$T([<$T:lower>]) = self; Some([<$T:lower>]); None]
        }
    }};
    // implements:
    // - validate
    // - first_non_unit
    // - variants_count
    // - eq_variant
    (methods2: $($T:ident),+) => {

        // pub fn variant_name() -> &'static str {
        //     $( if <$T>::type_id() != <()>::type_id() { count += 1} )+
        // }

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
}

impl<A, B, C, D, E, F> Enum<A, B, C, D, E, F> {
    impl_enum!(methods1: A, B, C, D, E, F);
}
impl<A: 'static, B: 'static, C: 'static, D: 'static, E: 'static, F: 'static>
    Enum<A, B, C, D, E, F>
{
    impl_enum!(methods2: A, B, C, D, E, F);
}

// methods difficult to implement automatically
impl<A, B, C, D, E, F> Enum<A, B, C, D, E, F> {
    /// Transforms the inner `A` value using `f`, leaving other variants unchanged.
    pub fn map_a<NEW>(self, f: impl FnOnce(A) -> NEW) -> Enum<NEW, B, C, D, E, F> {
        match self {
            Self::A(a) => Enum::A(f(a)),
            Self::B(b) => Enum::B(b),
            Self::C(c) => Enum::C(c),
            Self::D(d) => Enum::D(d),
            Self::E(e) => Enum::E(e),
            Self::F(f) => Enum::F(f),
        }
    }
    /// Transforms the inner `B` value using `f`, leaving other variants unchanged.
    pub fn map_b<NEW>(self, f: impl FnOnce(B) -> NEW) -> Enum<A, NEW, C, D, E, F> {
        match self {
            Self::A(a) => Enum::A(a),
            Self::B(b) => Enum::B(f(b)),
            Self::C(c) => Enum::C(c),
            Self::D(d) => Enum::D(d),
            Self::E(e) => Enum::E(e),
            Self::F(f) => Enum::F(f),
        }
    }
    /// Transforms the inner `C` value using `f`, leaving other variants unchanged.
    pub fn map_c<NEW>(self, f: impl FnOnce(C) -> NEW) -> Enum<A, B, NEW, D, E, F> {
        match self {
            Self::A(a) => Enum::A(a),
            Self::B(b) => Enum::B(b),
            Self::C(c) => Enum::C(f(c)),
            Self::D(d) => Enum::D(d),
            Self::E(e) => Enum::E(e),
            Self::F(f) => Enum::F(f),
        }
    }
    /// Transforms the inner `D` value using `f`, leaving other variants unchanged.
    pub fn map_d<NEW>(self, f: impl FnOnce(D) -> NEW) -> Enum<A, B, C, NEW, E, F> {
        match self {
            Self::A(a) => Enum::A(a),
            Self::B(b) => Enum::B(b),
            Self::C(c) => Enum::C(c),
            Self::D(d) => Enum::D(f(d)),
            Self::E(e) => Enum::E(e),
            Self::F(f) => Enum::F(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Enum;

    type Bytes = Enum<u8, i8>;
    type Unums = Enum<u8, u16, u32, u64>;

    #[test]
    fn len() {
        assert_eq![Bytes::len(), 2];
        assert_eq![Unums::len(), 4];
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
