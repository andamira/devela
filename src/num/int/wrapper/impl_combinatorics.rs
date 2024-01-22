// devela::num::int::wrapper::impl_combinatorics
//
//! implements combinatorics related functions
//
// TOC
// - signed|unsigned:
//   - factorial
//   - combine
//   - combine_rep
//   - permute
//   - permute_rep

use crate::{
    code::{cfor, iif, paste},
    num::{Int, NumErrors, NumResult as Result, Primiting},
};
use NumErrors::{MismatchedSizes, NonNegativeRequired, Overflow};
#[cfg(feature = "num_int_niche")]
use {
    crate::num::{impl_niche, niche::*},
    NumErrors::Invalid,
};

// $t:   the input/output type
// $d:  the doclink suffix for the method name
macro_rules! impl_combinatorics {
    (signed $( $t:ty : $d:literal ),+) => { $( impl_combinatorics![@signed $t:$d]; )+ };
    (unsigned $( $t:ty : $d:literal ),+) => { $( impl_combinatorics![@unsigned $t:$d]; )+ };

    // implements signed ops
    (@signed $t:ty : $d:literal) => { paste! {
        #[doc = "# Integer combinatorics related methods for `" $t "`\n\n"]
        #[doc = "- [factorial](#method.factorial" $d ")"]
        #[doc = "- [subfactorial](#method.subfactorial" $d ")"]
        #[doc = "- [permute](#method.permute" $d ")"]
        #[doc = "- [permute_rep](#method.permute_rep" $d ")"]
        #[doc = "- [combine](#method.combine" $d ")"]
        #[doc = "- [combine_rep](#method.combine_rep" $d ")"]
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
            /// Returns [`NonNegativeRequired`] if $n<0$,
            /// and [`Overflow`] if the result can't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(120)), Int(5_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(Int(1)), Int(0_" $t ").factorial()];"]
            #[doc = "assert![Int(-3_" $t ").factorial().is_err()];"]
            #[doc = "assert![Int(" $t "::MAX).factorial().is_err()];"]
            /// ```
            #[inline]
            pub const fn factorial(self) -> Result<Int<$t>> {
                let n = self.0;
                iif![n < 0; return Err(NonNegativeRequired)];
                let (mut n, mut result): ($t, $t) = (n.abs(), 1);
                while n > 1 {
                    result = if let Some(res) = result.checked_mul(n) {
                        res
                    } else {
                        return Err(Overflow(None));
                    };
                    n -= 1;
                }
                Ok(Int(result))
            }

            /// Returns the subfactorial, or the number of derangements.
            ///
            /// Permutations of *n* items which no element appears in its original position.
            ///
            /// # Algorithm
            /// The current implementation uses following recursive algorithm:
            /// $$ !n = (n - 1) * (!(n - 1) + !(n - 2)) $$
            //
            // Other possible formulas are:
            // $$
            // \begin{alignat}{1}
            // !n & = n! \times \sum_{k=0}^{n} \frac{(-1)^k}{k!} \newline
            // !n & = \left\lfloor \frac{n!}{e} + \frac{1}{2} \right\rfloor =
            //     \left\lfloor n! \times \left(\frac{1}{e}\right) + \frac{1}{2} \right\rfloor
            // \end{alignat}
            // $$
            //
            /// These are the maximum numbers whose subfactorials can fit within
            /// standard signed integer types:
            ///
            /// 5 for `i8`, 8 for `i16`, 12 for `i32`, 20 for `i64` and 35 for `i128`.
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if $n<0$,
            /// and [`Overflow`] if the result can't fit the type.
            ///
            /// # Panics
            /// For very large values (e.g. `i32:MAX`) the stack can overflow.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(44)), Int(5_" $t ").subfactorial()];"]
            #[doc = "assert_eq![Ok(Int(9)), Int(4_" $t ").subfactorial()];"]
            #[doc = "assert_eq![Ok(Int(1)), Int(0_" $t ").subfactorial()];"]
            #[doc = "assert![Int(-3_" $t ").subfactorial().is_err()];"]
            #[doc = "assert![Int(127_" $t ").subfactorial().is_err()];"]
            /// ```
            /// # Links
            /// - The list of subfactorials is available in <https://oeis.org/A000166>.
            #[inline]
            pub const fn subfactorial(self) -> Result<Int<$t>> {
                let n = self.0;
                iif![n < 0; return Err(NonNegativeRequired)];
                match n {
                    0 => Ok(Int(1)),
                    1 => Ok(Int(0)),
                    _ => {
                        let a = match Self::subfactorial(Int(n - 1)) {
                            Ok(a) => a.0, Err(e) => return Err(e),
                        };
                        let b = match Self::subfactorial(Int(n - 2)) {
                            Ok(b) => b.0, Err(e) => return Err(e),
                        };
                        let sum = a + b; // can't overflow
                        if let Some(res) = (n - 1).checked_mul(sum) {
                            Ok(Int(res))
                        } else {
                            return Err(Overflow(None));
                        }
                    }
                }
            }

            /// Combinations of `n` items taken `r` at a time, ordered.
            ///
            /// # Formula
            /// $$ \large C(n,r) = {n \choose r} = \frac{n!}{(n−r)!r!} $$
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if $n<0 \lor r<0$,
            /// [`MismatchedSizes`] if $r > n$, and
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(1)), Int(3_" $t ").combine(3)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine(1)];"]
            #[doc = "assert![Int(-3_" $t ").combine(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").combine(-2).is_err()];"]
            /// ```
            #[inline]
            pub const fn combine(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                iif![n < 0 || r < 0; return Err(NonNegativeRequired)];
                iif![r > n; return Err(MismatchedSizes)];
                let (mut num, mut den): ($t, $t) = (1, 1);
                cfor![i in 0..r => {
                    num = if let Some(res) = num.checked_mul(n - i) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                    den = if let Some(res) = den.checked_mul(i + 1) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                }];
                Ok(Int(num / den))
            }

            /// Combinations of `n` items taken `r` at a time with repetitions, unordered.
            ///
            /// Also known as *multichoose*.
            ///
            /// # Formula
            /// $$ \large C(n+r-1,r) = {n+k-1 \choose r} = \frac{(n+r-1)!}{(n−1)!r!} $$
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if $n<0 \lor r<0$,
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(10)), Int(3_" $t ").combine_rep(3)];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").combine_rep(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine_rep(1)];"]
            #[doc = "assert![Int(-3_" $t ").combine_rep(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").combine_rep(-2).is_err()];"]
            /// ```
            #[inline]
            pub const fn combine_rep(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                iif![n < 0 || r < 0; return Err(NonNegativeRequired)];
                let (mut num, mut den): ($t, $t) = (1, 1);
                cfor![i in 0..r => {
                    let factor = if let Some(res) = n.checked_add(r - 1 - i) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                    num = if let Some(res) = num.checked_mul(factor) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                    den = if let Some(res) = den.checked_mul(i + 1) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                }];
                Ok(Int(num / den))
            }

            /// Permutations of `n` items taken `r` at a time, ordered.
            ///
            /// When $n=r$ or $n=r-1$ the result is the same as calculating the factorial $n!$.
            ///
            /// # Formula
            /// $$ \large P(n,r) = \frac{|n|!}{(|n|−|r|)!} $$
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if $n<0 \lor r<0$,
            /// [`MismatchedSizes`] if $|r| > |n|$, and
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").permute(3)];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").permute(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").permute(1)];"]
            #[doc = "assert![Int(-3_" $t ").permute(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").permute(-2).is_err()];"]
            /// ```
            #[inline]
            pub const fn permute(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                iif![n < 0 || r < 0; return Err(NonNegativeRequired)];
                iif![r > n; return Err(MismatchedSizes)];
                let mut result: $t = 1;
                cfor![i in 0..r => {
                    result = if let Some(res) = result.checked_mul(n - i) {
                        res
                    } else {
                        return Err(Overflow(None))
                    }
                }];
                Ok(Int(result))
            }

            /// Permutations of `n` items taken `r` at a time with repetitions, ordered.
            ///
            /// # Formula
            /// $$ \large P_\text{rep}(n,r) = n_r $$
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if $n<0 \lor r<0$,
            /// and [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, Num};
            #[doc = "assert_eq![Ok(Int(27)), Int(3_" $t ").permute_rep(3)];"]
            #[doc = "assert_eq![Ok(Int(9)), Int(3_" $t ").permute_rep(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").permute_rep(1)];"]
            #[doc = "assert![Int(-3_" $t ").permute_rep(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").permute_rep(-2).is_err()];"]
            /// ```
            #[inline]
            pub const fn permute_rep(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                iif![n < 0 || r < 0; return Err(NonNegativeRequired)];
                let r_u32 = if let Ok(res) = Primiting(r).checked_cast_to_u32() {
                    res
                } else {
                    return Err(Overflow(None));
                };
                if let Some(res) = n.checked_pow(r_u32) {
                    Ok(Int(res))
                } else {
                    Err(Overflow(None))
                }
            }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $d:literal) => { paste! {
        #[doc = "# Integer combinatorics related methods for `" $t "`\n\n"]
        #[doc = "- [factorial](#method.factorial" $d ")"]
        #[doc = "- [subfactorial](#method.subfactorial" $d ")"]
        #[doc = "- [combine](#method.combine" $d ")"]
        #[doc = "- [combine_rep](#method.combine_rep" $d ")"]
        #[doc = "- [permute](#method.permute" $d ")"]
        #[doc = "- [permute_rep](#method.permute_rep" $d ")"]
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
            /// Returns [`Overflow`] if the result can't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(120)), Int(5_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(Int(1)), Int(0_" $t ").factorial()];"]
            #[doc = "assert![Int(" $t "::MAX).factorial().is_err()];"]
            /// ```
            #[inline]
            pub const fn factorial(self) -> Result<Int<$t>> {
                let [mut n, mut result] = [self.0, 1];
                while n > 1 {
                    result = if let Some(res) = result.checked_mul(n) {
                        res
                    } else {
                        return Err(Overflow(None));
                    };
                    n -= 1;
                }
                Ok(Int(result))
            }

            /// Returns the subfactorial, or the number of derangements.
            ///
            /// Permutations of *n* items which no element appears in its original position.
            ///
            /// # Algorithm
            /// The current implementation uses following recursive algorithm:
            /// $$ !n = (n - 1) * (!(n - 1) + !(n - 2)) $$
            //
            // Other possible formulas are:
            // $$
            // \begin{alignat}{1}
            // !n & = n! \times \sum_{k=0}^{n} \frac{(-1)^k}{k!} \newline
            // !n & = \left\lfloor \frac{n!}{e} + \frac{1}{2} \right\rfloor =
            //     \left\lfloor n! \times \left(\frac{1}{e}\right) + \frac{1}{2} \right\rfloor
            // \end{alignat}
            // $$
            //
            /// These are the maximum numbers whose subfactorials can fit within
            /// standard signed integer types:
            ///
            /// 5 for `u8`, 8 for `u16`, 13 for `u32`, 20 for `u64` and 35 for `u128`.
            ///
            /// # Errors
            /// Returns [`Overflow`] if the result can't fit the type.
            ///
            /// # Panics
            /// For very large values (e.g. `u32:MAX`) the stack can overflow.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(44)), Int(5_" $t ").subfactorial()];"]
            #[doc = "assert_eq![Ok(Int(9)), Int(4_" $t ").subfactorial()];"]
            #[doc = "assert_eq![Ok(Int(1)), Int(0_" $t ").subfactorial()];"]
            #[doc = "assert![Int(255_" $t ").subfactorial().is_err()];"]
            /// ```
            /// # Links
            /// - The list of subfactorials is available in <https://oeis.org/A000166>.
            #[inline]
            pub const fn subfactorial(self) -> Result<Int<$t>> {
                let n = self.0;
                match n {
                    0 => Ok(Int(1)),
                    1 => Ok(Int(0)),
                    _ => {
                        let a = match Self::subfactorial(Int(n - 1)) {
                            Ok(a) => a.0, Err(e) => return Err(e),
                        };
                        let b = match Self::subfactorial(Int(n - 2)) {
                            Ok(b) => b.0, Err(e) => return Err(e),
                        };
                        let sum = a + b; // can't overflow
                        if let Some(res) = (n - 1).checked_mul(sum) {
                            Ok(Int(res))
                        } else {
                            return Err(Overflow(None));
                        }
                    }
                }
            }

            /// Combinations of `n` items taken `r` at a time, unordered.
            ///
            /// # Formula
            /// $$ \large C(n,r) = {n \choose r} = \frac{n!}{(n−r)!r!} $$
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if $r > n$ and
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(1)), Int(3_" $t ").combine(3)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine(1)];"]
            #[doc = "assert![Int(" $t "::MAX).combine(" $t "::MAX).is_err()];"]
            /// ```
            #[inline]
            pub const fn combine(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                iif![r > n; return Err(MismatchedSizes)];
                let (mut num, mut den): ($t, $t) = (1, 1);
                cfor![i in 0..r => {
                    num = if let Some(res) = num.checked_mul(n - i) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                    den = if let Some(res) = den.checked_mul(i + 1) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                }];
                Ok(Int(num / den))
            }

            /// Combinations of `n` items taken `r` at a time with repetitions, unordered.
            ///
            /// Also known as *multichoose*.
            ///
            /// # Formula
            /// $$ \large C(n+r-1,r) = {n+k-1 \choose r} = \frac{(n+r-1)!}{(n−1)!r!} $$
            ///
            /// # Errors
            /// Returns [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(10)), Int(3_" $t ").combine_rep(3)];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").combine_rep(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine_rep(1)];"]
            #[doc = "assert![Int(" $t "::MAX).combine_rep(" $t "::MAX).is_err()];"]
            /// ```
            #[inline]
            pub const fn combine_rep(self, r: $t) -> Result<Int<$t>> {
                let [n, mut num, mut den] = [self.0, 1, 1];
                cfor![i in 0..r => {
                    let factor = if let Some(res) = n.checked_add(r - 1 - i) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                    num = if let Some(res) = num.checked_mul(factor) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                    den = if let Some(res) = den.checked_mul(i + 1) {
                        res
                    } else {
                        return Err(Overflow(None))
                    };
                }];
                Ok(Int(num / den))
            }

            /// Permutations of `n` items taken `r` at a time, ordered.
            ///
            /// When $n=r$ or $n=r-1$ the result is the same as calculating the factorial $n!$.
            ///
            /// # Formula
            /// $$ \large P(n,r) = \frac{n!}{(n−r)!} $$
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if $r > n$ and
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").permute(3)];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").permute(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").permute(1)];"]
            #[doc = "assert![Int(3_" $t ").permute(4_" $t ").is_err()];"]
            #[doc = "assert![Int(" $t "::MAX).permute(" $t "::MAX).is_err()];"]
            /// ```
            #[inline]
            pub const fn permute(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                iif![r > n; return Err(MismatchedSizes)];
                let mut result: $t = 1;
                cfor![i in 0..r => {
                    result = if let Some(res) = result.checked_mul(n - i) {
                        res
                    } else {
                        return Err(Overflow(None))
                    }
                }];
                Ok(Int(result))
            }

            /// Permutations of `n` items taken `r` at a time with repetitions, ordered.
            ///
            /// # Formula
            /// $$ \large P_\text{rep}(n,r) = n_r $$
            ///
            /// # Errors
            /// Returns [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(27)), Int(3_" $t ").permute_rep(3)];"]
            #[doc = "assert_eq![Ok(Int(9)), Int(3_" $t ").permute_rep(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").permute_rep(1)];"]
            #[doc = "assert![Int(" $t "::MAX).permute_rep(" $t "::MAX).is_err()];"]
            /// ```
            #[inline]
            pub const fn permute_rep(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                let r_u32 = if let Ok(res) = Primiting(r).checked_cast_to_u32() {
                    res
                } else {
                    return Err(Overflow(None));
                };
                if let Some(res) = n.checked_pow(r_u32) {
                    Ok(Int(res))
                } else {
                    Err(Overflow(None))
                }
            }
        }
    }};

    // $n:  the niche type name prefix (e.g. NonRange)
    // $t:  the niche inner type (the associated primitive integer) (e.g. u8)
    // $($g)*: an optional list of const generics (e.g. RMIN, RMAX)
    // $d:  the doclink suffix for the method name
    // $dt: the doclink suffix for the associated method name implemented for the inner primitive
    (niche $( $n:ident : $t:ident <$($g:ident),*> : $d:literal : $dt: literal),+ $(,)? ) => {
        $( impl_combinatorics![@niche $n:$t <$($g),*> : $d:$dt ]; )+
    };
    (@niche $n:ident : $t:ident <$($g:ident),*> : $d:literal : $dt: literal) => { paste! {
        #[doc = "# Integer combinatorics related methods for `" $t "`\n\n"]
        #[doc = "- [factorial](#method.factorial" $d ")"]
        #[doc = "- [subfactorial](#method.subfactorial" $d ")"]
        #[doc = "- [combine](#method.combine" $d ")"]
        #[doc = "- [combine_rep](#method.combine_rep" $d ")"]
        #[doc = "- [permute](#method.permute" $d ")"]
        #[doc = "- [permute_rep](#method.permute_rep" $d ")"]
        ///
        /// Each method calls its specific inner primitive implementation.
        /// # Errors
        /// Every method can return [`Invalid`] if the result is invalid for the niche type.
        impl<$(const $g:$t,)*> Int<[<$n$t:camel>]<$($g,)*>> {
            impl_niche![Int=>res $n:$t:$dt<$($g),*>, +const factorial, self];
            impl_niche![Int=>res $n:$t:$dt<$($g),*>, +const subfactorial, self];
            impl_niche![Int=>res $n:$t:$dt<$($g),*>, +const combine, self, r: $t];
            impl_niche![Int=>res $n:$t:$dt<$($g),*>, +const combine_rep, self, r: $t];
            impl_niche![Int=>res $n:$t:$dt<$($g),*>, +const permute, self, r: $t];
            impl_niche![Int=>res $n:$t:$dt<$($g),*>, +const permute_rep, self, r: $t];
        }
    }};
}
impl_combinatorics![signed i8:"", i16:"-1", i32:"-2", i64:"-3", i128:"-4", isize:"-5"];
impl_combinatorics![unsigned u8:"-6", u16:"-7", u32:"-8", u64:"-9", u128:"-10", usize:"-11"];
#[cfg(feature = "num_int_niche")]
impl_combinatorics![niche
    NonZero:i8<>:"-18":"", NonZero:i16<>:"-19":"-1",
    NonZero:i32<>:"-20":"-2", NonZero:i64<>:"-21":"-3",
    NonZero:i128<>:"-22":"-4", NonZero:isize<>:"-23":"-5",
    NonZero:u8<>:"-12":"-6", NonZero:u16<>:"-13":"-7",
    NonZero:u32<>:"-14":"-8", NonZero:u64<>:"-15":"-9",
    NonZero:u128<>:"-16":"-10", NonZero:usize<>:"-17":"-11",
    //
    NonSpecific:i8<V>:"-30":"", NonSpecific:i16<V>:"-31":"-1",
    NonSpecific:i32<V>:"-32":"-2", NonSpecific:i64<V>:"-33":"-3",
    NonSpecific:i128<V>:"-34":"-4", NonSpecific:isize<V>:"-35":"-5",
    NonSpecific:u8<V>:"-24":"-6", NonSpecific:u16<V>:"-25":"-7",
    NonSpecific:u32<V>:"-26":"-8", NonSpecific:u64<V>:"-27":"-9",
    NonSpecific:u128<V>:"-28":"-10", NonSpecific:usize<V>:"-29":"-11",
    //
    NonRange:i8<RMIN,RMAX>:"-42":"", NonRange:i16<RMIN,RMAX>:"-43":"-1",
    NonRange:i32<RMIN,RMAX>:"-44":"-2", NonRange:i64<RMIN,RMAX>:"-45":"-3",
    NonRange:i128<RMIN,RMAX>:"-46":"-4", NonRange:isize<RMIN,RMAX>:"-47":"-5",
    NonRange:u8<RMIN,RMAX>:"-36":"-6", NonRange:u16<RMIN,RMAX>:"-37":"-7",
    NonRange:u32<RMIN,RMAX>:"-38":"-8", NonRange:u64<RMIN,RMAX>:"-39":"-9",
    NonRange:u128<RMIN,RMAX>:"-40":"-10", NonRange:usize<RMIN,RMAX>:"-41":"11",
    //
    Range:i8<RMIN,RMAX>:"-54":"", Range:i16<RMIN,RMAX>:"-55":"-1",
    Range:i32<RMIN,RMAX>:"-56":"-2", Range:i64<RMIN,RMAX>:"-57":"-3",
    Range:i128<RMIN,RMAX>:"-58":"-4", Range:isize<RMIN,RMAX>:"-59":"-5",
    Range:u8<RMIN,RMAX>:"-48":"-6", Range:u16<RMIN,RMAX>:"-49":"-7",
    Range:u32<RMIN,RMAX>:"-50":"-8", Range:u64<RMIN,RMAX>:"-51":"-9",
    Range:u128<RMIN,RMAX>:"-52":"-10", Range:usize<RMIN,RMAX>:"-53":"-11",
];