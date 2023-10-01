// devela::cmp
//
//! Comparing and ordering, extends [`core::cmp`].
//!
//! This module defines many constant functions for comparing primitives, and the
//! [`pclamp`], [`pmax`] and [`pmin`] functions for comparing partially ordered values.
//

use crate::codegen::paste;

mod float;
pub use float::*;

// generate the const fns for primitive comparison
macro_rules! primitive_const_cmp {
    // $p: the types of the primitives
    ($($p:expr),+ $(,)?) => { paste! { $( primitive_const_cmp![@$p]; )+ }};

    // $p: the type of the primitive
    (@$p:expr) => { paste! {
        #[doc = "Compares and returns a clamped `" $p "` between `min` and `max`."]
        #[inline]
        #[must_use]
        pub const fn [<clamp_$p>](value: $p, min: $p, max: $p) -> $p {
            [<min_$p>]([<max_$p>](value, min), max)
        }

        #[doc = "Compares and returns the maximum of two `" $p "` values."]
        #[inline]
        #[must_use]
        pub const fn [<max_$p>](a: $p, b: $p) -> $p { if a > b { a } else { b } }

        #[doc = "Compares and returns the minimum of two `" $p "` values."]
        #[inline]
        #[must_use]
        pub const fn [<min_$p>](a: $p, b: $p) -> $p { if a < b { a } else { b } }
    }};
}
primitive_const_cmp![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

/// Compares and returns a [`PartialOrd`]ered `value` clamped between `min` and `max`.
///
/// # Examples
/// ```
/// use devela::cmp::pclamp;
///
/// assert_eq![0.4, pclamp(1.0, 0.2, 0.4)];
/// assert_eq![0.2, pclamp(0.0, 0.2, 0.4)];
/// ```
#[inline]
#[must_use]
#[rustfmt::skip]
pub fn pclamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    pmin(pmax(value, min), max)
}

/// Compares and returns the maximum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`max`][`core::cmp::max] which requires
/// [`Ord`]
///
/// # Examples
/// ```
/// use devela::cmp::pmax;
///
/// assert_eq![0.4, pmax(0.2, 0.4)];
/// ```
#[inline]
#[must_use]
#[rustfmt::skip]
pub fn pmax<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }

/// Compares and returns the minimum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`min`][`core::cmp::min] which requires
/// [`Ord`]
///
/// # Example
/// ```
/// use devela::cmp::pmin;
///
/// assert_eq![0.2, pmin(0.2, 0.4)];
/// ```
#[inline]
#[must_use]
#[rustfmt::skip]
pub fn pmin<T: PartialOrd>(a: T, b: T) -> T { if a < b { a } else { b } }

#[cfg(test)]
mod test_min_max_clamp {
    use super::{pclamp, pmax, pmin};

    #[test]
    fn min_max_clamp() {
        assert_eq![2, pmin(2, 5)];
        assert_eq![2, pmin(5, 2)];
        assert_eq![2., pmin(2., 5.)];

        assert_eq![5, pmax(2, 5)];
        assert_eq![5, pmax(5, 2)];
        assert_eq![5., pmax(2., 5.)];

        assert_eq![3, pclamp(3, 2, 5)];
        assert_eq![3., pclamp(3., 2., 5.)];
        assert_eq![2, pclamp(1, 2, 5)];
        assert_eq![5, pclamp(7, 2, 5)];
    }
}
