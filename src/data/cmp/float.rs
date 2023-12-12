// devela::data::cmp::float
//
//! Generate the (const) fns for floating-point primitive comparison.
//

#![cfg_attr(not(feature = "data"), allow(unused))]

#[cfg(any(doc, target_pointer_width = "32", target_pointer_width = "64"))]
use crate::math::all::fsize;
use crate::meta::{iif, paste};
use core::cmp::Ordering::{self, *};

primitive_float_const_cmp![32 >> 31, 64 >> 63];
macro_rules! primitive_float_const_cmp {
    // multiple impls
    //
    // $b:  the bits of the floating-point primitives
    // $sh: the shift amount for the given bits ($b - 1)
    ($($b:literal >> $sh:literal),+ $(,)?) => { paste! {
        $( primitive_float_const_cmp![@$b >> $sh]; )+
    }};

    // single impl
    //
    // $b:  the bits of the floating-point primitive
    // $sh: the shift amount for the given bits ($b - 1)
    (@$b:literal >> $sh:literal) => { paste! {
        #[doc = "A (`const`) port of `f" $b "::`[`total_cmp`][f" $b "#method.total_cmp]."]
        ///
        /// # Features
        /// This function will only be `const` if the `unsafe_data` feature is enabled.
        #[inline]
        #[must_use]
        #[cfg(feature = "unsafe_data")]
        pub const fn [<total_cmp_f $b>](a: [<f$b>], b: [<f$b>]) -> Ordering {
            // WAIT:const_float_bits_conv https://github.com/rust-lang/rust/issues/72447
            // let mut left = a.to_bits() as [<i $b>];
            // let mut right = b.to_bits() as [<i $b>];
            //
            // SAFETY: transmuting from f32 to i32 or f64 to i64 is safe.
            let mut left = unsafe { core::mem::transmute::<[<f$b>], [<i$b>]>(a) };
            let mut right = unsafe { core::mem::transmute::<[<f$b>], [<i$b>]>(b) };

            left ^= (((left >> $sh) as [<u $b>]) >> 1) as [<i $b>];
            right ^= (((right >> $sh) as [<u $b>]) >> 1) as [<i $b>];

            iif![left < right; Less; iif![left > right; Greater; Equal]]
        }
        // safe, non-const version (undocumented)
        #[inline]
        #[must_use]
        #[allow(missing_docs)]
        #[cfg(not(feature = "unsafe_data"))]
        pub fn [<total_cmp_f $b>](a: [<f$b>], b: [<f$b>]) -> Ordering {
            let mut left = a.to_bits() as [<i $b>];
            let mut right = b.to_bits() as [<i $b>];

            left ^= (((left >> $sh) as [<u $b>]) >> 1) as [<i $b>];
            right ^= (((right >> $sh) as [<u $b>]) >> 1) as [<i $b>];

            iif![left < right; Less; iif![left > right; Greater; Equal]]
        }

        #[doc = "Compares and returns a clamped [total ordered] `f" $b "` between `min` and `max`."]
        ///
        /// # Features
        /// This function will only be `const` if the `unsafe_data` feature is enabled.
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::data::cmp::clamp_f" $b ";"]
        ///
        #[doc = "assert_eq![2.0, clamp_f" $b "(5.0, -1.0, 2.0)];"]
        #[doc = "assert_eq![-1.0, clamp_f" $b "(-5.0, -1.0, 2.0)];"]
        /// ```
        ///
        #[doc = "[total ordered]: total_cmp_f" $b]
        #[inline]
        #[must_use]
        #[cfg(feature = "unsafe_data")]
        pub const fn [<clamp_f $b>](value: [<f $b>], min: [<f $b>], max: [<f $b>]) -> [<f $b>] {
            [<min_f $b>]([<max_f $b>](value, min), max)
        }
        // safe, non-const version (undocumented)
        #[inline]
        #[must_use]
        #[allow(missing_docs)]
        #[cfg(not(feature = "unsafe_data"))]
        pub fn [<clamp_f $b>](value: [<f $b>], min: [<f $b>], max: [<f $b>]) -> [<f $b>] {
            [<min_f $b>]([<max_f $b>](value, min), max)
        }

        #[doc = "Compares and returns the maximum of two `f" $b "` values"]
        #[doc = "using [total ordering][total_cmp_f" $b "]."]
        ///
        /// # Features
        /// This function will only be `const` if the `unsafe_data` feature is enabled.
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::data::cmp::max_f" $b ";"]
        ///
        #[doc = "assert_eq![2.0, max_f" $b "(2.0, -1.0)];"]
        #[doc = "assert_eq![2.0, max_f" $b "(1.0, 2.0)];"]
        #[doc = "assert_eq![0.0, max_f" $b "(-0.0, 0.0)];"]
        #[doc = "assert_eq![f" $b "::INFINITY, max_f" $b
            "(f" $b "::INFINITY, f" $b "::NEG_INFINITY)];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "unsafe_data")]
        pub const fn [<max_f $b>](a: [<f $b>], b: [<f $b>]) -> [<f $b>] {
            match [<total_cmp_f $b>](a, b) {
                Greater | Equal => a,
                Less => b,
            }
        }
        // safe, non-const version (undocumented)
        #[inline]
        #[must_use]
        #[allow(missing_docs)]
        #[cfg(not(feature = "unsafe_data"))]
        pub fn [<max_f $b>](a: [<f $b>], b: [<f $b>]) -> [<f $b>] {
            match a.[<total_cmp>](&b) {
                Greater | Equal => a,
                Less => b,
            }
        }

        #[doc = "Compares and returns the minimum of two `f" $b "` values"]
        #[doc = "using [total ordering][total_cmp_f" $b "]."]
        ///
        /// # Features
        /// This function will only be `const` if the `unsafe_data` feature is enabled.
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::data::cmp::min_f" $b ";"]
        ///
        #[doc = "assert_eq![-1.0, min_f" $b "(2.0, -1.0)];"]
        #[doc = "assert_eq![1.0, min_f" $b "(1.0, 2.0)];"]
        #[doc = "assert_eq![-0.0, min_f" $b "(-0.0, 0.0)];"]
        #[doc = "assert_eq![f" $b "::NEG_INFINITY, min_f" $b
            "(f" $b "::INFINITY, f" $b "::NEG_INFINITY)];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "unsafe_data")]
        pub const fn [<min_f $b>](a: [<f $b>], b: [<f $b>]) -> [<f $b>] {
            match [<total_cmp_f $b>](a, b) {
                Greater | Equal => b,
                Less => a,
            }
        }
        // safe, non-const version (undocumented)
        #[inline]
        #[must_use]
        #[allow(missing_docs)]
        #[cfg(not(feature = "unsafe_data"))]
        pub fn [<min_f $b>](a: [<f $b>], b: [<f $b>]) -> [<f $b>] {
            match a.[<total_cmp>](&b) {
                Greater | Equal => b,
                Less => a,
            }
        }
    }};
}
use primitive_float_const_cmp;

/* ptr-size aliases */

/// A pointer-sized redirect to `total_cmp_f[32|64]`.
///
/// # Features
/// This function will only be `const` if the `unsafe_data` feature is enabled.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(target_pointer_width = "32", target_pointer_width = "64")))
)]
#[cfg(all(
    feature = "unsafe_data",
    any(target_pointer_width = "32", target_pointer_width = "64")
))]
#[inline]
#[must_use]
pub const fn total_cmp_fsize(a: fsize, b: fsize) -> Ordering {
    #[cfg(target_pointer_width = "32")]
    return total_cmp_f32(a, b);
    #[cfg(target_pointer_width = "64")]
    return total_cmp_f64(a, b);
}
#[cfg(all(
    not(feature = "unsafe_data"),
    any(target_pointer_width = "32", target_pointer_width = "64")
))]
#[inline]
#[must_use]
#[allow(missing_docs)]
pub fn total_cmp_fsize(a: fsize, b: fsize) -> Ordering {
    #[cfg(target_pointer_width = "32")]
    return total_cmp_f32(a, b);
    #[cfg(target_pointer_width = "64")]
    return total_cmp_f64(a, b);
}

/// Compares and returns a clamped total ordered [`fsize`] between `min` and `max`.
///
/// # Features
/// This function will only be `const` if the `unsafe_data` feature is enabled.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(target_pointer_width = "32", target_pointer_width = "64")))
)]
#[cfg(all(
    feature = "unsafe_data",
    any(target_pointer_width = "32", target_pointer_width = "64")
))]
#[inline]
#[must_use]
pub const fn clamp_fsize(value: fsize, min: fsize, max: fsize) -> fsize {
    #[cfg(target_pointer_width = "32")]
    return clamp_f32(value, min, max);
    #[cfg(target_pointer_width = "64")]
    return clamp_f64(value, min, max);
}
#[cfg(all(
    not(feature = "unsafe_data"),
    any(target_pointer_width = "32", target_pointer_width = "64")
))]
#[inline]
#[must_use]
#[allow(missing_docs)]
pub fn clamp_fsize(value: fsize, min: fsize, max: fsize) -> fsize {
    #[cfg(target_pointer_width = "32")]
    return clamp_f32(value, min, max);
    #[cfg(target_pointer_width = "64")]
    return clamp_f64(value, min, max);
}

/// Compares and returns the maximum of two [`fsize`] values using total ordering.
///
/// # Features
/// This function will only be `const` if the `unsafe_data` feature is enabled.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(any(target_pointer_width = "32", target_pointer_width = "64"))))
)]
#[cfg(all(
    feature = "unsafe_data",
    any(target_pointer_width = "32", target_pointer_width = "64")
))]
#[inline]
#[must_use]
pub const fn max_fsize(a: fsize, b: fsize) -> fsize {
    #[cfg(target_pointer_width = "32")]
    return max_f32(a, b);
    #[cfg(target_pointer_width = "64")]
    return max_f64(a, b);
}
#[cfg(all(
    not(feature = "unsafe_data"),
    any(target_pointer_width = "32", target_pointer_width = "64")
))]
#[inline]
#[must_use]
#[allow(missing_docs)]
pub fn max_fsize(a: fsize, b: fsize) -> fsize {
    #[cfg(target_pointer_width = "32")]
    return max_f32(a, b);
    #[cfg(target_pointer_width = "64")]
    return max_f64(a, b);
}

/// Compares and returns the minimum of two [`fsize`] values using total ordering.
///
/// # Features
/// This function will only be `const` if the `unsafe_data` feature is enabled.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(target_pointer_width = "32", target_pointer_width = "64")))
)]
#[cfg(all(
    feature = "unsafe_data",
    any(target_pointer_width = "32", target_pointer_width = "64")
))]
#[inline]
#[must_use]
pub const fn min_fsize(a: fsize, b: fsize) -> fsize {
    #[cfg(target_pointer_width = "32")]
    return min_f32(a, b);
    #[cfg(target_pointer_width = "64")]
    return min_f64(a, b);
}
#[cfg(all(
    not(feature = "unsafe_data"),
    any(target_pointer_width = "32", target_pointer_width = "64")
))]
#[inline]
#[must_use]
#[allow(missing_docs)]
pub fn min_fsize(a: fsize, b: fsize) -> fsize {
    #[cfg(target_pointer_width = "32")]
    return min_f32(a, b);
    #[cfg(target_pointer_width = "64")]
    return min_f64(a, b);
}
