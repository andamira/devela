// devela::cmp
//
//! Comparing and ordering, extends [`core::cmp`].
//!
//! This module defines many constant functions for comparing primitives, and the
//! [`pclamp`], [`pmax`] and [`pmin`] functions for comparing partially ordered values.
//

use crate::{ops::iif, paste};

// generate the const fns for primitive comparison
macro_rules! primitive_const_cmp {
    // $p: the types of the primitives
    ($($p:expr),+) => { paste! { $( primitive_const_cmp![@$p]; )+ }};

    // $p: the type of the primitive
    (@$p:expr) => { paste! {
        #[doc = "Compares and returns a `" $p "` clamped between `min` and `max`."]
        #[inline]
        pub const fn [<clamp_$p>](value: $p, min: $p, max: $p) -> $p {
            [<min_$p>]([<max_$p>](value, min), max)
        }

        #[doc = "Compares and returns the maximum of two `" $p "` values."]
        #[inline]
        pub const fn [<max_$p>](a: $p, b: $p) -> $p { if a > b { a } else { b } }

        #[doc = "Compares and returns the minimum of two `" $p "` values."]
        #[inline]
        pub const fn [<min_$p>](a: $p, b: $p) -> $p { if a < b { a } else { b } }
    }};
}
primitive_const_cmp![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

// generate the const fns for floating-point primitive comparison
macro_rules! primitive_float_const_cmp {
    // $b: the bits of the floating-point primitives
    // $sh: the shift amount for the given bits ($b - 1)
    ($($b:literal >> $sh:literal),+) => { paste! { $( primitive_float_const_cmp![@$b >> $sh]; )+ }};

    // $b: the bits of the floating-point primitive
    // $sh: the shift amount for the given bits ($b - 1)
    (@$b:literal >> $sh:literal) => { paste! {
        #[doc = "Constant version of [`total_cmp`][f" $b "#method.total_cmp] for `f" $b "`."]
        // The code is ported from the standard library.
        #[inline]
        #[cfg(feature = "unsafe_const_float")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_const_float")))]
        pub const fn [<total_cmp_f $b>](a: [<f$b>], b: [<f$b>]) -> core::cmp::Ordering {
            // WAIT:const_float_bits_conv https://github.com/rust-lang/rust/issues/72447
            // let mut left = a.to_bits() as [<i $b>];
            // let mut right = b.to_bits() as [<i $b>];

            let mut left = unsafe { core::mem::transmute::<[<f$b>], [<i$b>]>(a) };
            let mut right = unsafe { core::mem::transmute::<[<f$b>], [<i$b>]>(b) };

            left ^= (((left >> $sh) as [<u $b>]) >> 1) as [<i $b>];
            right ^= (((right >> $sh) as [<u $b>]) >> 1) as [<i $b>];

            use core::cmp::Ordering::*;
            iif![left < right; Less; iif![left > right; Greater; Equal]]
        }

        #[doc = "Compares and returns a `f" $b "` clamped between `min` and `max` using total order."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::cmp::clamp_f" $b ";"]
        ///
        #[doc = "assert_eq![2.0, clamp_f" $b "(5.0, -1.0, 2.0)];"]
        #[doc = "assert_eq![-1.0, clamp_f" $b "(-5.0, -1.0, 2.0)];"]
        /// ```
        #[inline]
        #[cfg(feature = "unsafe_const_float")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_const_float")))]
        pub const fn [<clamp_f $b>](value: [<f $b>], min: [<f $b>], max: [<f $b>]) -> [<f $b>] {
            [<min_f $b>]([<max_f $b>](value, min), max)
        }

        #[doc = "Compares and returns the maximum of two `f" $b "` values"]
        #[doc = "using [total ordering][total_cmp_f" $b "]."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::cmp::max_f" $b ";"]
        ///
        #[doc = "assert_eq![2.0, max_f" $b "(2.0, -1.0)];"]
        #[doc = "assert_eq![2.0, max_f" $b "(1.0, 2.0)];"]
        #[doc = "assert_eq![0.0, max_f" $b "(-0.0, 0.0)];"]
        #[doc = "assert_eq![f" $b "::INFINITY, max_f" $b "(f" $b "::INFINITY, f" $b "::NEG_INFINITY)];"]
        /// ```
        #[inline]
        #[cfg(feature = "unsafe_const_float")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_const_float")))]
        pub const fn [<max_f $b>](a: [<f $b>], b: [<f $b>]) -> [<f $b>] {
            use core::cmp::Ordering::*;
            match [<total_cmp_f $b>](a, b) {
                Greater | Equal => a,
                Less => b,
            }
        }

        #[doc = "Compares and returns the minimum of two `f" $b "` values"]
        #[doc = "using [total ordering][total_cmp_f" $b "]."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::cmp::min_f" $b ";"]
        ///
        #[doc = "assert_eq![-1.0, min_f" $b "(2.0, -1.0)];"]
        #[doc = "assert_eq![1.0, min_f" $b "(1.0, 2.0)];"]
        #[doc = "assert_eq![-0.0, min_f" $b "(-0.0, 0.0)];"]
        #[doc = "assert_eq![f" $b "::NEG_INFINITY, min_f" $b "(f" $b "::INFINITY, f" $b "::NEG_INFINITY)];"]
        /// ```
        #[inline]
        #[cfg(feature = "unsafe_const_float")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_const_float")))]
        pub const fn [<min_f $b>](a: [<f $b>], b: [<f $b>]) -> [<f $b>] {
            use core::cmp::Ordering::*;
            match [<total_cmp_f $b>](a, b) {
                Greater | Equal => b,
                Less => a,
            }
        }
    }};
}
primitive_float_const_cmp![32 >> 31, 64 >> 63];

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
#[rustfmt::skip]
pub fn pclamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    pmin(pmax(value, min), max)
}

/// Compares and returns the maximum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`max`][`core::cmp::max] which requires
/// [`Ord`][core::cmp::Ord].
///
/// # Examples
/// ```
/// use devela::cmp::pmax;
///
/// assert_eq![0.4, pmax(0.2, 0.4)];
/// ```
#[inline]
#[rustfmt::skip]
pub fn pmax<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }

/// Compares and returns the minimum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`min`][`core::cmp::min] which requires
/// [`Ord`][core::cmp::Ord].
///
/// # Example
/// ```
/// use devela::cmp::pmin;
///
/// assert_eq![0.2, pmin(0.2, 0.4)];
/// ```
#[inline]
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
