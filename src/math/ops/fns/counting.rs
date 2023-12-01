// devela::math::ops::fns::counting
//
//! counting funcions
//
// TOC
// - sint|uint:
//   - factorial
//   - combine
//   - combine_rep
//   - permute
//   - permute_rep

use crate::{
    data::convert::*,
    math::{MathErrors as E, MathResult as Result},
    meta::{cfor, iif, paste},
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
        /// $$ n! $$.
        ///
        /// These are the maximum numbers whose factorials can fit within
        /// standard signed integer types:
        ///
        /// 5 for `i8`, 7 for `i16`, 12 for `i32`, 20 for `i64` and 33 for `i128`.
        ///
        /// # Errors
        /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0$,
        /// and [`Overflow`][E::Overflow] if the result can't fit the type.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::factorial_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(120), factorial_" $t "(5)];"]
        #[doc = "assert_eq![Ok(6), factorial_" $t "(3)];"]
        #[doc = "assert_eq![Ok(1), factorial_" $t "(0)];"]
        #[doc = "assert![factorial_" $t "(-3).is_err()];"]
        #[doc = "assert![devela::math::factorial_i8(6).is_err()];"]
        /// ```
        #[inline]
        pub const fn [<factorial_ $t>](n: $t) -> Result<$t> {
            iif![n < 0; return Err(E::NonNegativeRequired)];
            let (mut n, mut result): ($t, $t) = (n.abs(), 1);
            while n > 1 {
                result = if let Some(res) = result.checked_mul(n) {
                    res
                } else {
                    return Err(E::Overflow);
                };
                n -= 1;
            }
            Ok(result)
        }

        /// Permutations of `n` items taken `r` at a time, ordered.
        ///
        /// When $n = r$ or $n = r - 1$ the result is the same as calculating the factorial $n!$.
        ///
        /// $$ \large P(n,r) = \frac{|n|!}{(|n|−|r|)!} $$
        ///
        /// # Errors
        /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0 \lor r<0$,
        /// [`MismatchedSizes`][E::MismatchedSizes] if $|r| > |n|$, and
        /// [`Overflow`][E::Overflow] if the result cant't fit the type.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::permute_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(6), permute_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(6), permute_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), permute_" $t "(3, 1)];"]
        #[doc = "assert![permute_" $t "(-3, 3).is_err()];"]
        #[doc = "assert![permute_" $t "(3, -2).is_err()];"]
        /// ```
        #[inline]
        pub const fn [<permute_ $t>](n: $t, r: $t) -> Result<$t> {
            iif![n < 0 || r < 0; return Err(E::NonNegativeRequired)];
            iif![r > n; return Err(E::MismatchedSizes)];
            let mut result: $t = 1;
            cfor![i in 0..r => {
                result = if let Some(res) = result.checked_mul(n - i) {
                    res
                } else {
                    return Err(E::Overflow)
                }
            }];
            Ok(result)
        }

        /// Permutations of `n` items taken `r` at a time with repetitions, ordered.
        ///
        /// $$ \large P_\text{rep}(n,r) = n_r $$
        ///
        /// # Errors
        /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0 \lor r<0$,
        /// and [`Overflow`][E::Overflow] if the result cant't fit the type.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::permute_rep_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(27), permute_rep_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(9), permute_rep_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), permute_rep_" $t "(3, 1)];"]
        #[doc = "assert![permute_rep_" $t "(-3, 3).is_err()];"]
        #[doc = "assert![permute_rep_" $t "(3, -2).is_err()];"]
        /// ```
        #[inline]
        pub const fn [<permute_rep_ $t>](n: $t, r: $t) -> Result<$t> {
            iif![n < 0 || r < 0; return Err(E::NonNegativeRequired)];
            let r_u32 = if let Ok(res) = [<checked_cast_ $t _to_u32>](r) {
                res
            } else {
                return Err(E::Overflow);
            };
            if let Some(res) = n.checked_pow(r_u32) {
                Ok(res)
            } else {
                Err(E::Overflow)
            }
        }

        /// Combinations of `n` items taken `r` at a time, ordered.
        ///
        /// $$ \large C(n,r) = {n \choose r} = \frac{n!}{(n−r)!r!} $$
        ///
        /// # Errors
        /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0 \lor r<0$,
        /// [`MismatchedSizes`][E::MismatchedSizes] if $r > n$, and
        /// [`Overflow`][E::Overflow] if the result cant't fit the type.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::combine_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(1), combine_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(3), combine_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), combine_" $t "(3, 1)];"]
        #[doc = "assert![combine_" $t "(-3, 3).is_err()];"]
        #[doc = "assert![combine_" $t "(3, -2).is_err()];"]
        /// ```
        #[inline]
        pub const fn [<combine_ $t>](n: $t, r: $t) -> Result<$t> {
            iif![n < 0 || r < 0; return Err(E::NonNegativeRequired)];
            iif![r > n; return Err(E::MismatchedSizes)];
            let (mut num, mut den): ($t, $t) = (1, 1);
            cfor![i in 0..r => {
                num = if let Some(res) = num.checked_mul(n - i) {
                    res
                } else {
                    return Err(E::Overflow)
                };
                den = if let Some(res) = den.checked_mul(i + 1) {
                    res
                } else {
                    return Err(E::Overflow)
                };
            }];
            Ok(num / den)
        }

        /// Combinations of `n` items taken `r` at a time with repetitions, unordered.
        ///
        /// Also known as *multichoose*.
        ///
        /// $$ \large C(n+r-1,r) = ${n+k-1 \choose r} = \frac{(n+r-1)!}{(n−1)!r!} $$
        ///
        /// # Errors
        /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0 \lor r<0$,
        /// [`Overflow`][E::Overflow] if the result cant't fit the type.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::combine_rep_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(10), combine_rep_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(6), combine_rep_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), combine_rep_" $t "(3, 1)];"]
        #[doc = "assert![combine_rep_" $t "(-3, 3).is_err()];"]
        #[doc = "assert![combine_rep_" $t "(3, -2).is_err()];"]
        /// ```
        #[inline]
        pub const fn [<combine_rep_ $t>](n: $t, r: $t) -> Result<$t> {
            iif![n < 0 || r < 0; return Err(E::NonNegativeRequired)];
            let (mut num, mut den): ($t, $t) = (1, 1);
            cfor![i in 0..r => {
                let factor = if let Some(res) = n.checked_add(r - 1 - i) {
                    res
                } else {
                    return Err(E::Overflow)
                };
                num = if let Some(res) = num.checked_mul(factor) {
                    res
                } else {
                    return Err(E::Overflow)
                };
                den = if let Some(res) = den.checked_mul(i + 1) {
                    res
                } else {
                    return Err(E::Overflow)
                };
            }];
            Ok(num / den)
        }
    }};

    // implements unsigned ops
    (@unsigned($t:ty, $up:ty, $ft:ty) ) => { paste! {
        /// Returns the factorial of `n`.
        ///
        /// $$ n! $$
        ///
        /// Permutations of *n* items, ordered, where $n = r$.
        ///
        /// These are the maximum numbers whose factorials can fit within
        /// standard unsigned integer types:
        ///
        /// 5 for `u8`, 8 `u16`, 12 `u32`, 20 `u64` and 34 `u128`.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::factorial_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(120), factorial_" $t "(5)];"]
        #[doc = "assert_eq![Ok(6), factorial_" $t "(3)];"]
        #[doc = "assert_eq![Ok(1), factorial_" $t "(0)];"]
        #[doc = "assert![devela::math::factorial_u8(6).is_err()];"]
        /// ```
        #[inline]
        pub const fn [<factorial_ $t>](mut n: $t) -> Result<$t> {
            let mut result: $t = 1;
            while n > 1 {
                result = if let Some(res) = result.checked_mul(n) {
                    res
                } else {
                    return Err(E::Overflow);
                };
                n -= 1;
            }
            Ok(result)
        }

        /// Permutations of `n` items taken `r` at a time, ordered.
        ///
        /// When $n = r$ or $n = r - 1$ the result is the same as calculating the factorial $n!$.
        ///
        /// $$ \large P(n,r) = \frac{n!}{(n−r)!} $$
        ///
        /// # Errors
        /// Returns [`MismatchedSizes`][E::MismatchedSizes] if $r > n$ and
        /// [`Overflow`][E::Overflow] if the result cant't fit the type.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::permute_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(6), permute_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(6), permute_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), permute_" $t "(3, 1)];"]
        /// ```
        #[inline]
        pub const fn [<permute_ $t>](n: $t, r: $t) -> Result<$t> {
            iif![r > n; return Err(E::MismatchedSizes)];
            let mut result: $t = 1;
            cfor![i in 0..r => {
                result = if let Some(res) = result.checked_mul(n - i) {
                    res
                } else {
                    return Err(E::Overflow)
                }
            }];
            Ok(result)
        }

        /// Permutations of `n` items taken `r` at a time with repetitions, ordered.
        ///
        /// $$ \large P_\text{rep}(n,r) = n_r $$
        ///
        /// # Errors
        /// Returns [`Overflow`][E::Overflow] if the result cant't fit the type.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::permute_rep_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(27), permute_rep_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(9), permute_rep_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), permute_rep_" $t "(3, 1)];"]
        /// ```
        #[inline]
        pub const fn [<permute_rep_ $t>](n: $t, r: $t) -> Result<$t> {
            let r_u32 = if let Ok(res) = [<checked_cast_ $t _to_u32>](r) {
                res
            } else {
                return Err(E::Overflow);
            };
            if let Some(res) = n.checked_pow(r_u32) {
                Ok(res)
            } else {
                Err(E::Overflow)
            }
        }

        /// Combinations of `n` items taken `r` at a time, unordered.
        ///
        /// $$ \large C(n,r) = {n \choose r} = \frac{n!}{(n−r)!r!} $$
        ///
        /// # Errors
        /// Returns [`MismatchedSizes`][E::MismatchedSizes] if $r > n$ and
        /// [`Overflow`][E::Overflow] if the result cant't fit the type.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::combine_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(1), combine_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(3), combine_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), combine_" $t "(3, 1)];"]
        /// ```
        #[inline]
        pub const fn [<combine_ $t>](n: $t, r: $t) -> Result<$t> {
            iif![r > n; return Err(E::MismatchedSizes)];
            let (mut num, mut den): ($t, $t) = (1, 1);
            cfor![i in 0..r => {
                num = if let Some(res) = num.checked_mul(n - i) {
                    res
                } else {
                    return Err(E::Overflow)
                };
                den = if let Some(res) = den.checked_mul(i + 1) {
                    res
                } else {
                    return Err(E::Overflow)
                };
            }];
            Ok(num / den)
        }

        /// Combinations of `n` items taken `r` at a time with repetitions, unordered.
        ///
        /// Also known as *multichoose*.
        ///
        /// $$ \large C(n+r-1,r) =  {n+k-1 \choose r} = \frac{(n+r-1)!}{(n−1)!r!} $$
        ///
        /// # Errors
        /// Returns [`Overflow`][E::Overflow] if the result cant't fit the type.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::combine_rep_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(10), combine_rep_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(6), combine_rep_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), combine_rep_" $t "(3, 1)];"]
        /// ```
        #[inline]
        pub const fn [<combine_rep_ $t>](n: $t, r: $t) -> Result<$t> {
            let (mut num, mut den): ($t, $t) = (1, 1);
            cfor![i in 0..r => {
                let factor = if let Some(res) = n.checked_add(r - 1 - i) {
                    res
                } else {
                    return Err(E::Overflow)
                };
                num = if let Some(res) = num.checked_mul(factor) {
                    res
                } else {
                    return Err(E::Overflow)
                };
                den = if let Some(res) = den.checked_mul(i + 1) {
                    res
                } else {
                    return Err(E::Overflow)
                };
            }];
            Ok(num / den)
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
