// devela::math::ops::fns::counting
//
//! counting funcions
//
// TOC
// - factorial

use crate::{
    math::{MathErrors as E, MathResult as Result},
    meta::{cfor, paste},
};

// $t:   the input/output type
// $up:  the upcasted type to do the operations on (the ones that can overflow)
// $ft:  the floating-point type to do some operations on
macro_rules! impl_ops {
    (signed $( ($t:ty, $up:ty, $ft:ty) ),+) => { $( impl_ops![@signed($t, $up, $ft)]; )+ };
    (unsigned $( ($t:ty, $up:ty, $ft:ty) ),+) => { $( impl_ops![@unsigned($t, $up, $ft)]; )+ };

    // implements signed ops
    (@signed($t:ty, $up:ty, $ft:ty) ) => { paste! {
        /// Returns the factorial of `n`.
        ///
        /// Negative values of `n` will be treated as positive.
        ///
        /// These are the maximum numbers whose factorials can fit within
        /// standard signed integer types:
        ///
        /// 5 for `i8`, 7 `i16`, 12 `i32`, 20 `i64` and 33 `i128`.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::factorial_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(1), factorial_" $t "(0)];"]
        #[doc = "assert_eq![Ok(6), factorial_" $t "(3)];"]
        #[doc = "assert_eq![Ok(24), factorial_" $t "(-4)];"]
        #[doc = "assert_eq![Ok(120), factorial_" $t "(5)];"]
        #[doc = "assert![devela::math::factorial_i8(6).is_err()];"]
        /// ```
        #[inline]
        pub const fn [<factorial_ $t>](n: $t) -> Result<$t> {
            let (mut n, mut result): ($t, $t) = (n.abs(), 1);
            while n > 1 {
                result = if let Some(r) = result.checked_mul(n) {
                    r
                } else {
                    return Err(E::Overflow);
                };
                n -= 1;
            }
            Ok(result)
        }
    }};

    // implements unsigned ops
    (@unsigned($t:ty, $up:ty, $ft:ty) ) => { paste! {
        /// Returns the factorial of `n`.
        ///
        /// These are the maximum numbers whose factorials can fit within
        /// standard unsigned integer types:
        ///
        /// 5 for `u8`, 8 `u16`, 12 `u32`, 20 `u64` and 34 `u128`.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::factorial_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(1), factorial_" $t "(0)];"]
        #[doc = "assert_eq![Ok(6), factorial_" $t "(3)];"]
        #[doc = "assert_eq![Ok(120), factorial_" $t "(5)];"]
        #[doc = "assert![devela::math::factorial_u8(6).is_err()];"]
        /// ```
        #[inline]
        pub const fn [<factorial_ $t>](mut n: $t) -> Result<$t> {
            let mut result: $t = 1;
            while n > 1 {
                result = if let Some(r) = result.checked_mul(n) {
                    r
                } else {
                    return Err(E::Overflow);
                };
                n -= 1;
            }
            Ok(result)
        }
    }};
}

impl_ops![
    signed(i8, i16, f32),
    (i16, i32, f32),
    (i32, i64, f32),
    (i64, i128, f64),
    (i128, i128, f64),
    (isize, isize, fsize)
];
impl_ops![
    unsigned(u8, u16, f32),
    (u16, u32, f32),
    (u32, u64, f32),
    (u64, u128, f64),
    (u128, u128, f64),
    (usize, usize, fsize)
];
