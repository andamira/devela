// devela::cmp::primitives
//
//! Generate the (const) fns for floating-point primitive comparison.
//

use crate::paste;
use core::cmp::Ordering::{self, *};

macro_rules! primitive_float_const_cmp {
    // multiple impls
    //
    // $b: the bits of the floating-point primitives
    // $sh: the shift amount for the given bits ($b - 1)
    ($($b:literal >> $sh:literal),+ $(,)?) => { paste! {
        $( primitive_float_const_cmp![@$b >> $sh]; )+
    }};

    // single impl
    //
    // $b: the bits of the floating-point primitive
    // $sh: the shift amount for the given bits ($b - 1)
    (@$b:literal >> $sh:literal) => { paste! {
        #[doc = "A port of `f" $b "::`[`total_cmp`][f" $b "#method.total_cmp]."]
        ///
        /// # Features
        /// This function will only be `const` if the `unsafe_cmp_float` is enabled.
        #[inline]
        #[cfg(feature = "unsafe_cmp_float")]
        #[cfg_attr(feature = "nightly",
            doc(cfg(any(feature = "unsafe_cmp_float", feature = ""))))]
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

            if left < right {
                Less
            } else if left > right {
                Greater
            } else {
                Equal
            }
        }
        // safe, non-const version
        #[inline]
        #[cfg(not(feature = "unsafe_cmp_float"))]
        pub fn [<total_cmp_f $b>](a: [<f$b>], b: [<f$b>]) -> Ordering {
            let mut left = a.to_bits() as [<i $b>];
            let mut right = b.to_bits() as [<i $b>];

            left ^= (((left >> $sh) as [<u $b>]) >> 1) as [<i $b>];
            right ^= (((right >> $sh) as [<u $b>]) >> 1) as [<i $b>];

            if left < right {
                Less
            } else if left > right {
                Greater
            } else {
                Equal
            }
        }

        #[doc = "Compares and returns a `f" $b "` clamped between `min` and `max`"]
        #[doc = "using [total ordering][total_cmp_f" $b "]."]
        ///
        /// # Features
        /// This function will only be `const` if the `unsafe_cmp_float` is enabled.
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::cmp::clamp_f" $b ";"]
        ///
        #[doc = "assert_eq![2.0, clamp_f" $b "(5.0, -1.0, 2.0)];"]
        #[doc = "assert_eq![-1.0, clamp_f" $b "(-5.0, -1.0, 2.0)];"]
        /// ```
        #[inline]
        #[cfg(feature = "unsafe_cmp_float")]
        #[cfg_attr(feature = "nightly",
            doc(cfg(any(feature = "unsafe_cmp_float", feature = ""))))]
        pub const fn [<clamp_f $b>](value: [<f $b>], min: [<f $b>], max: [<f $b>]) -> [<f $b>] {
            [<min_f $b>]([<max_f $b>](value, min), max)
        }
        // safe, non-const version
        #[inline]
        #[cfg(not(feature = "unsafe_cmp_float"))]
        pub fn [<clamp_f $b>](value: [<f $b>], min: [<f $b>], max: [<f $b>]) -> [<f $b>] {
            [<min_f $b>]([<max_f $b>](value, min), max)
        }

        #[doc = "Compares and returns the maximum of two `f" $b "` values"]
        #[doc = "using [total ordering][total_cmp_f" $b "]."]
        ///
        /// # Features
        /// This function will only be `const` if the `unsafe_cmp_float` is enabled.
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::cmp::max_f" $b ";"]
        ///
        #[doc = "assert_eq![2.0, max_f" $b "(2.0, -1.0)];"]
        #[doc = "assert_eq![2.0, max_f" $b "(1.0, 2.0)];"]
        #[doc = "assert_eq![0.0, max_f" $b "(-0.0, 0.0)];"]
        #[doc = "assert_eq![f" $b "::INFINITY, max_f" $b
            "(f" $b "::INFINITY, f" $b "::NEG_INFINITY)];"]
        /// ```
        #[inline]
        #[cfg(feature = "unsafe_cmp_float")]
        #[cfg_attr(feature = "nightly",
            doc(cfg(any(feature = "unsafe_cmp_float", feature = ""))))]
        pub const fn [<max_f $b>](a: [<f $b>], b: [<f $b>]) -> [<f $b>] {
            match [<total_cmp_f $b>](a, b) {
                Greater | Equal => a,
                Less => b,
            }
        }
        // safe, non-const version
        #[inline]
        #[cfg(not(feature = "unsafe_cmp_float"))]
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
        /// This function will only be `const` if the `unsafe_cmp_float` is enabled.
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::cmp::min_f" $b ";"]
        ///
        #[doc = "assert_eq![-1.0, min_f" $b "(2.0, -1.0)];"]
        #[doc = "assert_eq![1.0, min_f" $b "(1.0, 2.0)];"]
        #[doc = "assert_eq![-0.0, min_f" $b "(-0.0, 0.0)];"]
        #[doc = "assert_eq![f" $b "::NEG_INFINITY, min_f" $b
            "(f" $b "::INFINITY, f" $b "::NEG_INFINITY)];"]
        /// ```
        #[inline]
        #[cfg(feature = "unsafe_cmp_float")]
        #[cfg_attr(feature = "nightly",
            doc(cfg(any(feature = "unsafe_cmp_float", feature = ""))))]
        pub const fn [<min_f $b>](a: [<f $b>], b: [<f $b>]) -> [<f $b>] {
            match [<total_cmp_f $b>](a, b) {
                Greater | Equal => b,
                Less => a,
            }
        }
        // safe, non-const version
        #[inline]
        #[cfg(not(feature = "unsafe_cmp_float"))]
        pub fn [<min_f $b>](a: [<f $b>], b: [<f $b>]) -> [<f $b>] {
            match a.[<total_cmp>](&b) {
                Greater | Equal => b,
                Less => a,
            }
        }
    }};
}
primitive_float_const_cmp![32 >> 31, 64 >> 63];
