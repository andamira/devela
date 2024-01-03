// devela::num::int::wrapper::impl_count
//
//! implements counting related functions
//
// TOC
// - sint|uint:
//   - factorial
//   - combine
//   - combine_rep
//   - permute
//   - permute_rep

use super::Int;
use crate::code::{cfor, iif, paste};
use crate::data::Casting;
use crate::num::{NumErrors as E, NumResult as Result};

// $t:   the input/output type
// $dl:  the doclink suffix for the method name
macro_rules! impl_count {
    (signed $( $t:ty : $dl:literal ),+) => { $( impl_count![@signed $t:$dl]; )+ };
    (unsigned $( $t:ty : $dl:literal ),+) => { $( impl_count![@unsigned $t:$dl]; )+ };

    // implements signed ops
    (@signed $t:ty : $dl:literal) => { paste! {
        #[doc = "# Numeric counting related methods for `" $t "`\n\n"]
        #[doc = "- [factorial](#method.factorial" $dl ")"]
        #[doc = "- [combine](#method.combine" $dl ")"]
        #[doc = "- [combine_rep](#method.combine_rep" $dl ")"]
        #[doc = "- [permute](#method.permute" $dl ")"]
        #[doc = "- [permute_rep](#method.permute_rep" $dl ")"]
        impl Int<$t> {
            /// Returns the factorial.
            ///
            /// Permutations of *n* items, ordered, where $n = r$.
            ///
            /// # Formula
            /// $$ n! = 1 \cdot 2 \cdot 3 \cdot \ldots \cdot (n-1) \cdot n $$
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
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(120), Int(5_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(6), Int(3_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(1), Int(0_" $t ").factorial()];"]
            #[doc = "assert![Int(-3_" $t ").factorial().is_err()];"]
            #[doc = "assert![Int(" $t "::MAX).factorial().is_err()];"]
            /// ```
            #[inline]
            pub const fn factorial(self) -> Result<$t> {
                iif![self.0 < 0; return Err(E::NonNegativeRequired)];
                let (mut n, mut result): ($t, $t) = (self.0.abs(), 1);
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
            /// When $n=r$ or $n=r-1$ the result is the same as calculating the factorial $n!$.
            ///
            /// # Formula
            /// $$ \large P(n,r) = \frac{|n|!}{(|n|−|r|)!} $$
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0 \lor r<0$,
            /// [`MismatchedSizes`][E::MismatchedSizes] if $|r| > |n|$, and
            /// [`Overflow`][E::Overflow] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(6), Int(3_" $t ").permute(3)];"]
            #[doc = "assert_eq![Ok(6), Int(3_" $t ").permute(2)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").permute(1)];"]
            #[doc = "assert![Int(-3_" $t ").permute(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").permute(-2).is_err()];"]
            /// ```
            #[inline]
            pub const fn permute(self, r: $t) -> Result<$t> {
                iif![self.0 < 0 || r < 0; return Err(E::NonNegativeRequired)];
                iif![r > self.0; return Err(E::MismatchedSizes)];
                let mut result: $t = 1;
                cfor![i in 0..r => {
                    result = if let Some(res) = result.checked_mul(self.0 - i) {
                        res
                    } else {
                        return Err(E::Overflow)
                    }
                }];
                Ok(result)
            }

            /// Permutations of `n` items taken `r` at a time with repetitions, ordered.
            ///
            /// # Formula
            /// $$ \large P_\text{rep}(n,r) = n_r $$
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0 \lor r<0$,
            /// and [`Overflow`][E::Overflow] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(27), Int(3_" $t ").permute_rep(3)];"]
            #[doc = "assert_eq![Ok(9), Int(3_" $t ").permute_rep(2)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").permute_rep(1)];"]
            #[doc = "assert![Int(-3_" $t ").permute_rep(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").permute_rep(-2).is_err()];"]
            /// ```
            #[inline]
            pub const fn permute_rep(self, r: $t) -> Result<$t> {
                iif![self.0 < 0 || r < 0; return Err(E::NonNegativeRequired)];
                let r_u32 = if let Ok(res) = Casting(r).checked_cast_to_u32() {
                    res
                } else {
                    return Err(E::Overflow);
                };
                if let Some(res) = self.0.checked_pow(r_u32) {
                    Ok(res)
                } else {
                    Err(E::Overflow)
                }
            }

            /// Combinations of `n` items taken `r` at a time, ordered.
            ///
            /// # Formula
            /// $$ \large C(n,r) = {n \choose r} = \frac{n!}{(n−r)!r!} $$
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0 \lor r<0$,
            /// [`MismatchedSizes`][E::MismatchedSizes] if $r > n$, and
            /// [`Overflow`][E::Overflow] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(1), Int(3_" $t ").combine(3)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").combine(2)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").combine(1)];"]
            #[doc = "assert![Int(-3_" $t ").combine(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").combine(-2).is_err()];"]
            /// ```
            #[inline]
            pub const fn combine(self, r: $t) -> Result<$t> {
                iif![self.0 < 0 || r < 0; return Err(E::NonNegativeRequired)];
                iif![r > self.0; return Err(E::MismatchedSizes)];
                let (mut num, mut den): ($t, $t) = (1, 1);
                cfor![i in 0..r => {
                    num = if let Some(res) = num.checked_mul(self.0 - i) {
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
            /// # Formula
            /// $$ \large C(n+r-1,r) = {n+k-1 \choose r} = \frac{(n+r-1)!}{(n−1)!r!} $$
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0 \lor r<0$,
            /// [`Overflow`][E::Overflow] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(10), Int(3_" $t ").combine_rep(3)];"]
            #[doc = "assert_eq![Ok(6), Int(3_" $t ").combine_rep(2)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").combine_rep(1)];"]
            #[doc = "assert![Int(-3_" $t ").combine_rep(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").combine_rep(-2).is_err()];"]
            /// ```
            #[inline]
            pub const fn combine_rep(self, r: $t) -> Result<$t> {
                iif![self.0 < 0 || r < 0; return Err(E::NonNegativeRequired)];
                let (mut num, mut den): ($t, $t) = (1, 1);
                cfor![i in 0..r => {
                    let factor = if let Some(res) = self.0.checked_add(r - 1 - i) {
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
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $dl:literal) => { paste! {
        #[doc = "# Numeric counting related methods for `" $t "`\n\n"]
        #[doc = "- [factorial](#method.factorial" $dl ")"]
        #[doc = "- [combine](#method.combine" $dl ")"]
        #[doc = "- [combine_rep](#method.combine_rep" $dl ")"]
        #[doc = "- [permute](#method.permute" $dl ")"]
        #[doc = "- [permute_rep](#method.permute_rep" $dl ")"]
        impl Int<$t> {
            /// Returns the factorial.
            ///
            /// Permutations of *n* items, ordered, where $n = r$.
            ///
            /// # Formula
            /// $$ n! = 1 \cdot 2 \cdot 3 \cdot \ldots \cdot (n-1) \cdot n $$
            ///
            /// These are the maximum numbers whose factorials can fit within
            /// standard signed integer types:
            ///
            /// 5 for `u8`, 8 for `u16`, 12 for `u32`, 20 for `u64` and 34 for `u128`.
            ///
            /// # Errors
            /// Returns [`Overflow`][E::Overflow] if the result can't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(120), Int(5_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(6), Int(3_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(1), Int(0_" $t ").factorial()];"]
            #[doc = "assert![Int(" $t "::MAX).factorial().is_err()];"]
            /// ```
            #[inline]
            pub const fn factorial(mut self) -> Result<$t> {
                let mut result: $t = 1;
                while self.0 > 1 {
                    result = if let Some(res) = result.checked_mul(self.0) {
                        res
                    } else {
                        return Err(E::Overflow);
                    };
                    self.0 -= 1;
                }
                Ok(result)
            }

            /// Permutations of `n` items taken `r` at a time, ordered.
            ///
            /// When $n=r$ or $n=r-1$ the result is the same as calculating the factorial $n!$.
            ///
            /// # Formula
            /// $$ \large P(n,r) = \frac{n!}{(n−r)!} $$
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`][E::MismatchedSizes] if $r > n$ and
            /// [`Overflow`][E::Overflow] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(6), Int(3_" $t ").permute(3)];"]
            #[doc = "assert_eq![Ok(6), Int(3_" $t ").permute(2)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").permute(1)];"]
            #[doc = "assert![Int(3_" $t ").permute(4_" $t ").is_err()];"]
            #[doc = "assert![Int(" $t "::MAX).permute(" $t "::MAX).is_err()];"]
            /// ```
            #[inline]
            pub const fn permute(self, r: $t) -> Result<$t> {
                iif![r > self.0; return Err(E::MismatchedSizes)];
                let mut result: $t = 1;
                cfor![i in 0..r => {
                    result = if let Some(res) = result.checked_mul(self.0 - i) {
                        res
                    } else {
                        return Err(E::Overflow)
                    }
                }];
                Ok(result)
            }

            /// Permutations of `n` items taken `r` at a time with repetitions, ordered.
            ///
            /// # Formula
            /// $$ \large P_\text{rep}(n,r) = n_r $$
            ///
            /// # Errors
            /// Returns [`Overflow`][E::Overflow] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(27), Int(3_" $t ").permute_rep(3)];"]
            #[doc = "assert_eq![Ok(9), Int(3_" $t ").permute_rep(2)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").permute_rep(1)];"]
            #[doc = "assert![Int(" $t "::MAX).permute_rep(" $t "::MAX).is_err()];"]
            /// ```
            #[inline]
            pub const fn permute_rep(self, r: $t) -> Result<$t> {
                let r_u32 = if let Ok(res) = Casting(r).checked_cast_to_u32() {
                    res
                } else {
                    return Err(E::Overflow);
                };
                if let Some(res) = self.0.checked_pow(r_u32) {
                    Ok(res)
                } else {
                    Err(E::Overflow)
                }
            }

            /// Combinations of `n` items taken `r` at a time, unordered.
            ///
            /// # Formula
            /// $$ \large C(n,r) = {n \choose r} = \frac{n!}{(n−r)!r!} $$
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`][E::MismatchedSizes] if $r > n$ and
            /// [`Overflow`][E::Overflow] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(1), Int(3_" $t ").combine(3)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").combine(2)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").combine(1)];"]
            #[doc = "assert![Int(" $t "::MAX).combine(" $t "::MAX).is_err()];"]
            /// ```
            #[inline]
            pub const fn combine(self, r: $t) -> Result<$t> {
                iif![r > self.0; return Err(E::MismatchedSizes)];
                let (mut num, mut den): ($t, $t) = (1, 1);
                cfor![i in 0..r => {
                    num = if let Some(res) = num.checked_mul(self.0 - i) {
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
            /// # Formula
            /// $$ \large C(n+r-1,r) = {n+k-1 \choose r} = \frac{(n+r-1)!}{(n−1)!r!} $$
            ///
            /// # Errors
            /// Returns [`Overflow`][E::Overflow] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(10), Int(3_" $t ").combine_rep(3)];"]
            #[doc = "assert_eq![Ok(6), Int(3_" $t ").combine_rep(2)];"]
            #[doc = "assert_eq![Ok(3), Int(3_" $t ").combine_rep(1)];"]
            #[doc = "assert![Int(" $t "::MAX).combine_rep(" $t "::MAX).is_err()];"]
            /// ```
            #[inline]
            pub const fn combine_rep(self, r: $t) -> Result<$t> {
                let (mut num, mut den): ($t, $t) = (1, 1);
                cfor![i in 0..r => {
                    let factor = if let Some(res) = self.0.checked_add(r - 1 - i) {
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
        }
    }};
}
impl_count![signed i8:"", i16:"-1", i32:"-2", i64:"-3", i128:"-4", isize:"-5"];
impl_count![unsigned u8:"-6", u16:"-7", u32:"-8", u64:"-9", u128:"-10", usize:"-11"];
