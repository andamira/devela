// devela::num::ops::fns::counting
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
    code::{cfor, iif, paste},
    data::convert::Casting,
    num::{NumErrors as E, NumResult as Result},
};

// $t:   the input/output type
// $up:  the upcasted type to do the operations on (the ones that can overflow) (not used)
macro_rules! impl_ops {
    (signed $( ($t:ty, $up:ty) ),+) => { $( impl_ops![@signed($t, $up)]; )+ };
    (unsigned $( ($t:ty, $up:ty) ),+) => { $( impl_ops![@unsigned($t, $up)]; )+ };

    // implements signed ops
    (@signed($t:ty, $up:ty) ) => { paste! {
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
        #[doc ="use devela::num::factorial_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(120), factorial_" $t "(5)];"]
        #[doc = "assert_eq![Ok(6), factorial_" $t "(3)];"]
        #[doc = "assert_eq![Ok(1), factorial_" $t "(0)];"]
        #[doc = "assert![factorial_" $t "(-3).is_err()];"]
        #[doc = "assert![devela::num::factorial_i8(6).is_err()];"]
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
        #[doc ="use devela::num::permute_" $t ";\n\n"]
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
        #[doc ="use devela::num::permute_rep_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(27), permute_rep_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(9), permute_rep_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), permute_rep_" $t "(3, 1)];"]
        #[doc = "assert![permute_rep_" $t "(-3, 3).is_err()];"]
        #[doc = "assert![permute_rep_" $t "(3, -2).is_err()];"]
        /// ```
        #[inline]
        pub const fn [<permute_rep_ $t>](n: $t, r: $t) -> Result<$t> {
            iif![n < 0 || r < 0; return Err(E::NonNegativeRequired)];
            let r_u32 = if let Ok(res) = Casting(r).checked_cast_to_u32() {
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
        #[doc ="use devela::num::combine_" $t ";\n\n"]
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
        #[doc ="use devela::num::combine_rep_" $t ";\n\n"]
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
    (@unsigned($t:ty, $up:ty) ) => { paste! {
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
        #[doc ="use devela::num::factorial_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(120), factorial_" $t "(5)];"]
        #[doc = "assert_eq![Ok(6), factorial_" $t "(3)];"]
        #[doc = "assert_eq![Ok(1), factorial_" $t "(0)];"]
        #[doc = "assert![devela::num::factorial_u8(6).is_err()];"]
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
        #[doc ="use devela::num::permute_" $t ";\n\n"]
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
        #[doc ="use devela::num::permute_rep_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(27), permute_rep_" $t "(3, 3)];"]
        #[doc = "assert_eq![Ok(9), permute_rep_" $t "(3, 2)];"]
        #[doc = "assert_eq![Ok(3), permute_rep_" $t "(3, 1)];"]
        /// ```
        #[inline]
        pub const fn [<permute_rep_ $t>](n: $t, r: $t) -> Result<$t> {
            let r_u32 = if let Ok(res) = Casting(r).checked_cast_to_u32() {
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
        #[doc ="use devela::num::combine_" $t ";\n\n"]
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
        #[doc ="use devela::num::combine_rep_" $t ";\n\n"]
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
    signed(i8, i16),
    (i16, i32),
    (i32, i64),
    (i64, i128),
    (i128, i128),
    (isize, isize)
];
impl_ops![
    unsigned(u8, u16),
    (u16, u32),
    (u32, u64),
    (u64, u128),
    (u128, u128),
    (usize, usize)
];
