// devela_base_num::num::int::wrapper::impl_combinatorics
//
//! Implements combinatorics-related methods for [`Int`].
//
// TOC
// - signed|unsigned:
//   - factorial
//   - combine
//   - combine_rep
//   - permute
//   - permute_rep

use super::super::_docs::*;
use crate::{
    Cast, Int,
    IntError::{MismatchedSizes, NonNegativeRequired, Overflow},
    IntResult as Result, is, paste, whilst,
};

/// Implements combinatorics-related methods for [`Int`].
///
/// # Args
/// $t:   the input/output type
///
/// $d:   the doclink suffix for the method name
macro_rules! impl_combinatorics {
    () => {
        impl_combinatorics![signed
            i8     |"",
            i16    |"-1",
            i32    |"-2",
            i64    |"-3",
            i128   |"-4",
            isize  |"-5"
        ];
        impl_combinatorics![unsigned
            u8     |"-6",
            u16    |"-7",
            u32    |"-8",
            u64    |"-9",
            u128   |"-10",
            usize  |"-11"
        ];
    };
    (signed $( $t:ty | $d:literal ),+) => {
        $( impl_combinatorics![@signed $t| $d]; )+
    };
    (unsigned $( $t:ty | $d:literal ),+) => {
        $( impl_combinatorics![@unsigned $t| $d]; )+
    };
    (
    // implements signed ops
    @signed $t:ty | $d:literal) => { paste! {
        ///
        #[doc = "# Integer combinatorics related methods for `" $t "`\n\n"]
        #[doc = "- [factorial](#method.factorial" $d ")"]
        #[doc = "- [subfactorial](#method.subfactorial" $d ")"]
        #[doc = "- [permute](#method.permute" $d ")"]
        #[doc = "- [permute_rep](#method.permute_rep" $d ")"]
        #[doc = "- [combine](#method.combine" $d ")"]
        #[doc = "- [combine_rep](#method.combine_rep" $d ")"]
        ///
        impl Int<$t> {
            /// Returns the factorial.
            ///
            /// Permutations of *n* items, ordered, where $n = r$.
            ///
            /// # Formula
            #[doc = _INT_FORMULA_FACTORIAL!()]
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
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(120)), Int(5_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(Int(1)), Int(0_" $t ").factorial()];"]
            #[doc = "assert![Int(-3_" $t ").factorial().is_err()];"]
            #[doc = "assert![Int(" $t "::MAX).factorial().is_err()];"]
            /// ```
            pub const fn factorial(self) -> Result<Int<$t>> {
                let n = self.0;
                is![n < 0; return Err(NonNegativeRequired)];
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
            /// # Formulation
            /// ## Algorithm
            /// The current implementation uses the recursive definition:
            #[doc = _INT_ALGORITHM_SUBFACTORIAL!()]
            ///
            /// ## Closed-Form Formulas
            /// Other equivalent formulas for \( !n \) include:
            ///
            /// 1. **Summation Formula**:
            #[doc = _INT_FORMULA_SUBFACTORIAL_SUMMATION!()]
            /// 2. **Approximation Formula**:
            #[doc = _INT_FORMULA_SUBFACTORIAL_APPROXIMATION!()]
            ///
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
            /// # use devela_base_num::Int;
            /// # #[cfg(not(miri))] { // too slow for miri
            #[doc = "assert_eq![Ok(Int(44)), Int(5_" $t ").subfactorial()];"]
            #[doc = "assert_eq![Ok(Int(9)), Int(4_" $t ").subfactorial()];"]
            #[doc = "assert_eq![Ok(Int(1)), Int(0_" $t ").subfactorial()];"]
            #[doc = "assert![Int(-3_" $t ").subfactorial().is_err()];"]
            #[doc = "assert![Int(127_" $t ").subfactorial().is_err()];"]
            /// # }
            /// ```
            /// # Links
            /// - The list of subfactorials is available in <https://oeis.org/A000166>.
            pub const fn subfactorial(self) -> Result<Int<$t>> {
                let n = self.0;
                is![n < 0; return Err(NonNegativeRequired)];
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
            #[doc = _INT_FORMULA_COMBINE!()]
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if $n<0 \lor r<0$,
            /// [`MismatchedSizes`] if $r > n$, and
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(1)), Int(3_" $t ").combine(3)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine(1)];"]
            #[doc = "assert![Int(-3_" $t ").combine(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").combine(-2).is_err()];"]
            /// ```
            pub const fn combine(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                is![n < 0 || r < 0; return Err(NonNegativeRequired)];
                is![r > n; return Err(MismatchedSizes)];
                let (mut num, mut den): ($t, $t) = (1, 1);
                whilst![i in 0..r; {
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
            #[doc = _INT_FORMULA_COMBINE_REP!()]
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if $n<0 \lor r<0$,
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(10)), Int(3_" $t ").combine_rep(3)];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").combine_rep(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine_rep(1)];"]
            #[doc = "assert![Int(-3_" $t ").combine_rep(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").combine_rep(-2).is_err()];"]
            /// ```
            pub const fn combine_rep(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                is![n < 0 || r < 0; return Err(NonNegativeRequired)];
                let (mut num, mut den): ($t, $t) = (1, 1);
                whilst![i in 0..r; {
                    let Some(factor) = n.checked_add(r - 1 - i) else {
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
            #[doc = _INT_FORMULA_PERMUTE!()]
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if $n<0 \lor r<0$,
            /// [`MismatchedSizes`] if $|r| > |n|$, and
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").permute(3)];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").permute(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").permute(1)];"]
            #[doc = "assert![Int(-3_" $t ").permute(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").permute(-2).is_err()];"]
            /// ```
            pub const fn permute(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                is![n < 0 || r < 0; return Err(NonNegativeRequired)];
                is![r > n; return Err(MismatchedSizes)];
                let mut result: $t = 1;
                whilst![i in 0..r; {
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
            #[doc = _INT_FORMULA_PERMUTE_REP!()]
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if $n<0 \lor r<0$,
            /// and [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(27)), Int(3_" $t ").permute_rep(3)];"]
            #[doc = "assert_eq![Ok(Int(9)), Int(3_" $t ").permute_rep(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").permute_rep(1)];"]
            #[doc = "assert![Int(-3_" $t ").permute_rep(3).is_err()];"]
            #[doc = "assert![Int(3_" $t ").permute_rep(-2).is_err()];"]
            /// ```
            pub const fn permute_rep(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                is![n < 0 || r < 0; return Err(NonNegativeRequired)];
                let Ok(r_u32) = Cast(r).checked_cast_to_u32() else {
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
    (
    // implements unsigned ops
    @unsigned $t:ty | $d:literal) => { paste! {
        ///
        #[doc = "# Integer combinatorics related methods for `" $t "`\n\n"]
        #[doc = "- [factorial](#method.factorial" $d ")"]
        #[doc = "- [subfactorial](#method.subfactorial" $d ")"]
        #[doc = "- [combine](#method.combine" $d ")"]
        #[doc = "- [combine_rep](#method.combine_rep" $d ")"]
        #[doc = "- [permute](#method.permute" $d ")"]
        #[doc = "- [permute_rep](#method.permute_rep" $d ")"]
        ///
        impl Int<$t> {
            /// Returns the factorial.
            ///
            /// Permutations of *n* items, ordered, where $n = r$.
            ///
            /// # Formula
            #[doc = _INT_FORMULA_FACTORIAL!()]
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
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(120)), Int(5_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").factorial()];"]
            #[doc = "assert_eq![Ok(Int(1)), Int(0_" $t ").factorial()];"]
            #[doc = "assert![Int(" $t "::MAX).factorial().is_err()];"]
            /// ```
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
            /// # Formulation
            /// ## Algorithm
            /// The current implementation uses the recursive definition:
            #[doc = _INT_ALGORITHM_SUBFACTORIAL!()]
            ///
            /// ## Closed-Form Formulas
            /// Other equivalent formulas for \( !n \) include:
            ///
            /// 1. **Summation Formula**:
            #[doc = _INT_FORMULA_SUBFACTORIAL_SUMMATION!()]
            /// 2. **Approximation Formula**:
            #[doc = _INT_FORMULA_SUBFACTORIAL_APPROXIMATION!()]
            ///
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
            /// # use devela_base_num::Int;
            /// # #[cfg(not(miri))] { // too slow for miri
            #[doc = "assert_eq![Ok(Int(44)), Int(5_" $t ").subfactorial()];"]
            #[doc = "assert_eq![Ok(Int(9)), Int(4_" $t ").subfactorial()];"]
            #[doc = "assert_eq![Ok(Int(1)), Int(0_" $t ").subfactorial()];"]
            #[doc = "assert![Int(255_" $t ").subfactorial().is_err()];"]
            /// # }
            /// ```
            /// # Links
            /// - The list of subfactorials is available in <https://oeis.org/A000166>.
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
            #[doc = _INT_FORMULA_COMBINE!()]
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if $r > n$ and
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(1)), Int(3_" $t ").combine(3)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine(1)];"]
            #[doc = "assert![Int(" $t "::MAX).combine(" $t "::MAX).is_err()];"]
            /// ```
            pub const fn combine(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                is![r > n; return Err(MismatchedSizes)];
                let (mut num, mut den): ($t, $t) = (1, 1);
                whilst![i in 0..r; {
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
            #[doc = _INT_FORMULA_COMBINE_REP!()]
            ///
            /// # Errors
            /// Returns [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(10)), Int(3_" $t ").combine_rep(3)];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").combine_rep(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").combine_rep(1)];"]
            #[doc = "assert![Int(" $t "::MAX).combine_rep(" $t "::MAX).is_err()];"]
            /// ```
            pub const fn combine_rep(self, r: $t) -> Result<Int<$t>> {
                let [n, mut num, mut den] = [self.0, 1, 1];
                whilst![i in 0..r; {
                    let Some(factor) = n.checked_add(r - 1 - i) else {
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
            #[doc = _INT_FORMULA_PERMUTE!()]
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if $r > n$ and
            /// [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").permute(3)];"]
            #[doc = "assert_eq![Ok(Int(6)), Int(3_" $t ").permute(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").permute(1)];"]
            #[doc = "assert![Int(3_" $t ").permute(4_" $t ").is_err()];"]
            #[doc = "assert![Int(" $t "::MAX).permute(" $t "::MAX).is_err()];"]
            /// ```
            pub const fn permute(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                is![r > n; return Err(MismatchedSizes)];
                let mut result: $t = 1;
                whilst![i in 0..r; {
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
            #[doc = _INT_FORMULA_PERMUTE_REP!()]
            ///
            /// # Errors
            /// Returns [`Overflow`] if the result cant't fit the type.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_num::Int;
            #[doc = "assert_eq![Ok(Int(27)), Int(3_" $t ").permute_rep(3)];"]
            #[doc = "assert_eq![Ok(Int(9)), Int(3_" $t ").permute_rep(2)];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(3_" $t ").permute_rep(1)];"]
            #[doc = "assert![Int(" $t "::MAX).permute_rep(" $t "::MAX).is_err()];"]
            /// ```
            pub const fn permute_rep(self, r: $t) -> Result<Int<$t>> {
                let n = self.0;
                let Ok(r_u32) = Cast(r).checked_cast_to_u32() else {
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
}
impl_combinatorics!();
