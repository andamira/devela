// devela::num::int::num_trait
//
//!
//
// TOC
// - trait NumInt
//   - base
//   - core
//   - combinatorics
//   - division
//   - factors
//   - primes
//   - roots

use super::shared_docs::*;
#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{GcdReturn, Num, NumError as E, NumResult as Result, ValueQuant};
#[cfg(doc)]
use E::{
    MismatchedSizes, NonNegativeRequired, NonZeroRequired, NotImplemented, NotSupported, Overflow,
};

#[cfg(_int_·)]
mod impls;
mod r#ref;
pub use r#ref::NumRefInt;

mod auto_impls {
    use super::{NumInt, NumRefInt};

    #[rustfmt::skip]
    impl<T: NumInt> NumRefInt<'_> for &T {}
    #[rustfmt::skip]
    impl<T: NumInt> NumRefInt<'_> for &mut T {}
}

/// Common auto-trait for integer types.
///
/// # Notes
/// - We use `n` to refer to the `self` argument in all the method descriptions and formulas.
///
/// - Every method in this trait returns [`NotImplemented`] by default.
/// - Any concrete implementation must implement the operations it wants to support.
/// - Any operations specifically not supported should ideally return [`NotSupported`].
/// - This trait tries to offer the same methods everywhere, giving a result when possible.
/// - Operations are generally supported if they can be valid for some values.
/// - Most methods come in two variants, starting with different prefixes:
///   - `int_*` methods take the arguments by value.
///   - `int_ref_*` methods take the arguments by reference.
///
/// # Methods
/// - base:
///     [`digital_root`][Self::int_digital_root],
///     [`digital_root_base`][Self::int_digital_root_base],
///     [`digits`][Self::int_digits],
///     [`digits_sign`][Self::int_digits_sign],
///     [`digits_base`][Self::int_digits_base],
///     [`digits_base_sign`][Self::int_digits_base_sign].
/// - core:
///     [`abs`][Self::int_abs],
///     [`is_even`][Self::int_is_even],
///     [`is_odd`][Self::int_is_odd],
///     [`gcd`][Self::int_gcd],
///     [`gcd_ext`][Self::int_gcd_ext],
///     [`lcm`][Self::int_lcm],
///     [`scale`][Self::int_scale].
///     [`midpoint`][Self::int_midpoint].
/// - combinatorics:
///     [`factorial`][Self::int_factorial],
///     [`subfactorial`][Self::int_subfactorial],
///     [`permute`][Self::int_permute],
///     [`permute_rep`][Self::int_permute_rep],
///     [`combine`][Self::int_combine],
///     [`combine_rep`][Self::int_combine_rep].
/// - division:
///     [`div_rem`][Self::int_div_rem],
///     [`div_ceil`][Self::int_div_ceil],
///     [`div_floor`][Self::int_div_floor],
///     [`div_ties_away`][Self::int_div_ties_away],
///     [`div_ties_towards`][Self::int_div_ties_towards]
///     [`div_ties_even`][Self::int_div_ties_even],
///     [`div_ties_odd`][Self::int_div_ties_odd].
/// - factors:
///     [`factors`][Self::int_factors],
///     [`factors_proper`][Self::int_factors_proper],
///     [`factors_prime`][Self::int_factors_prime],
///     [`factors_prime_unique`][Self::int_factors_prime_unique],
///     [`factors_buf`][Self::int_factors_buf`],
///     [`factors_proper_buf`][Self::int_factors_proper_buf`],
///     [`factors_prime_buf`][Self::int_factors_prime_buf`],
///     [`factors_prime_unique_buf`][Self::int_factors_prime_unique_buf`].
/// - modulo:
///     [`modulo`][Self::int_modulo],
///     [`modulo_cycles`][Self::int_modulo_cycles],
///     [`modulo_add`][Self::int_modulo_add],
///     [`modulo_add_cycles`][Self::int_modulo_add_cycles],
///     [`modulo_add_inv`][Self::int_modulo_add_inv],
///     [`modulo_sub`][Self::int_modulo_sub],
///     [`modulo_sub_cycles`][Self::int_modulo_sub_cycles],
///     [`modulo_mul`][Self::int_modulo_mul],
///     [`modulo_mul_cycles`][Self::int_modulo_mul_cycles],
///     [`modulo_mul_inv`][Self::int_modulo_mul_inv],
///     [`modulo_div`][Self::int_modulo_div].
/// - primes:
///     [`is_prime`][Self::int_is_prime],
///     [`prime_nth`][Self::int_prime_nth],
///     [`prime_pi`][Self::int_prime_pi],
///     [`totient`][Self::int_totient].
/// - roots:
///     [`is_square`][Self::int_is_square],
//     [`is_power`][Self::int_is_power], TODO
///     [`sqrt_ceil`][Self::int_sqrt_ceil],
///     [`sqrt_floor`][Self::int_sqrt_floor],
///     [`sqrt_round`][Self::int_sqrt_round],
///     [`root_ceil`][Self::int_root_ceil],
///     [`root_floor`][Self::int_root_floor].
//     [`root_round`][Self::int_root_round], TODO
///
/// See also [`NumRefInt`] which is automatically implemented for `NumInt` references.
//
// In sync with src/num/int/num_trait/ref.rs (int_ref_* methods)
#[cfg_attr(feature = "nightly_doc", doc(notable_trait))]
#[expect(unused_variables, reason = "pretty signatures")]
#[rustfmt::skip]
pub trait NumInt: Num {
    /// Specifically signed output type for some operations (▶ `int_gcd_ext`).
    type OutI;

    /* base */

    /// Returns the digital root in base 10.
    fn int_digital_root(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_digital_root"]]
    fn int_ref_digital_root(&self) -> Result<Self::Out> { E::ni() }

    /// Returns the digital root in the given `base`.
    fn int_digital_root_base(self, base: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_digital_root_base"]]
    fn int_ref_digital_root_base(&self, base: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the number of digits in base 10.
    fn int_digits(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_digits"]]
    fn int_ref_digits(&self) -> Result<Self::Out> { E::ni() }

    /// Returns the number of digits in base 10 including the negative sign.
    fn int_digits_sign(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_digits_sign"]]
    fn int_ref_digits_sign(&self) -> Result<Self::Out> { E::ni() }

    /// Returns the number of digits in the given `base`.
    fn int_digits_base(self, base: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_digits_base"]]
    fn int_ref_digits_base(&self, base: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the number of digits in the given `base`.
    fn int_digits_base_sign(self, base: Self::Rhs)
        -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_digits_base_sign"]]
    fn int_ref_digits_base_sign(&self, base: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /* core */

    /// Returns the absolute value.
    fn int_abs(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_abs"]]
    fn int_ref_abs(&self) -> Result<Self::Out> { E::ni() }

    /// Returns `true` if `self` is even.
    fn int_is_even(self) -> Result<bool> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_is_even"]]
    fn int_ref_is_even(&self) -> Result<bool> { E::ni() }

    /// Returns `true` if `self` is odd.
    fn int_is_odd(self) -> Result<bool> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_is_odd"]]
    fn int_ref_is_odd(&self) -> Result<bool> { E::ni() }

    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
    fn int_gcd(self, other: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_gcd"]]
    fn int_ref_gcd(&self, other: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr> and the Bézout coeficients.
    fn int_gcd_ext(self, other: Self::Rhs)
        -> Result<GcdReturn<Self::Out, Self::OutI>> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_gcd_ext"]]
    fn int_ref_gcd_ext(&self, other: &Self::Rhs)
        -> Result<GcdReturn<Self::Out, Self::OutI>> { E::ni() }

    /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
    fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_lcm"]]
    fn int_ref_lcm(&self, other: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns a scaled value in `[min..=max]` to a new range `[a..=b]`.
    /// If the value lies outside of `[min..=max]` it will result in extrapolation.
    ///
    /// # Errors
    /// Can [`Overflow`] for extrapolated values that can't fit the type,
    /// and for overflowing arithmetic operations in the following formula:
    ///
    /// # Formula
    #[doc = FORMULA_SCALE!()]
    fn int_scale(self, min: Self::Rhs, max: Self::Rhs, a: Self::Rhs, b: Self::Rhs)
        -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_scale"]]
    fn int_ref_scale(&self, min: &Self::Rhs, max: &Self::Rhs, a: &Self::Rhs, b: &Self::Rhs)
        -> Result<Self::Out> { E::ni() }

    /// Returns a scaled value between `[min..=max]` to a new range `[a..=b]`.
    ///
    /// If the value lies outside of `[min..=max]` it will result in extrapolation, and
    /// if the value doesn't fit in the type it will wrap around its boundaries.
    ///
    /// # Panics
    /// Could panic for very large values on some implementations.
    ///
    /// # Formula
    #[doc = FORMULA_SCALE!()] // (same as scale)
    fn int_scale_wrap(self, min: Self::Rhs, max: Self::Rhs, a: Self::Rhs, b: Self::Rhs)
        -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_scale_wrap"]]
    fn int_ref_scale_wrap(&self, min: &Self::Rhs, max: &Self::Rhs, a: &Self::Rhs, b: &Self::Rhs)
        -> Result<Self::Out> { E::ni() }

    /// Returns the midpoint of `self` and `other`.
    fn int_midpoint(self, other: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_midpoint"]]
    fn int_ref_midpoint(&self, other: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /* combinatorics */

    /// Returns the factorial.
    ///
    /// Permutations of *n* items, ordered, where $n = r$.
    ///
    /// # Formula
    #[doc = FORMULA_FACTORIAL!()]
    ///
    /// These are the maximum numbers whose factorials can fit within
    /// standard signed integer types:
    /// - 5 for `i8`, 7 for `i16`, 12 for `i32`, 20 for `i64` and 33 for `i128`.
    /// - 5 for `u8`, 8 for `u16`, 12 for `u32`, 20 for `u64` and 34 for `u128`.
    /// # Errors
    /// Returns [`NonNegativeRequired`] if $n<0$ and [`Overflow`] if the result can't fit the type.
    fn int_factorial(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_factorial`][Self::int_factorial] but takes the arguments by reference.*
    #[doc = ref_fn!["int_factorial"]]
    fn int_ref_factorial(&self) -> Result<Self::Out> { E::ni() }

    /// Returns the subfactorial, or the number of derangements.
    ///
    /// Permutations of *n* items which no element appears in its original position.
    ///
    /// # Algorithm
    /// The current implementation uses following recursive algorithm:
    #[doc = ALGORITHM_SUBFACTORIAL!()]
    ///
    /// Other possible formulas are:
    #[doc = FORMULA_SUBFACTORIAL_OTHER!()]
    ///
    /// These are the maximum numbers whose subfactorials can fit within
    /// standard signed integer types:
    /// - 5 for `i8`, 8 for `i16`, 12 for `i32`, 20 for `i64` and 35 for `i128`.
    /// - 5 for `u8`, 8 for `u16`, 13 for `u32`, 20 for `u64` and 35 for `u128`.
    ///
    /// # Errors
    /// Returns [`NonNegativeRequired`] if $n<0$,
    /// and [`Overflow`] if the result can't fit the type.
    /// # Links
    /// - The list of subfactorials is available in <https://oeis.org/A000166>.
    fn int_subfactorial(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_subfactorial`][Self::int_subfactorial] but takes the arguments by reference.*
    #[doc = ref_fn!["int_subfactorial"]]
    fn int_ref_subfactorial(&self) -> Result<Self::Out> { E::ni() }

    /// Returns the number of combinations of `n` items taken `r` at a time, unordered.
    ///
    /// # Formula
    #[doc = FORMULA_COMBINE_REP!()]
    ///
    /// # Errors
    /// Returns [`MismatchedSizes`] if $r > n$ and [`Overflow`] if the result cant't fit the type.
    fn int_combine(self, r: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_combine`][Self::int_combine] but takes the arguments by reference.*
    #[doc = ref_fn!["int_combine"]]
    fn int_ref_combine(&self, r: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the number of permutations of `n` items taken `r` at a time with repetitions,
    /// unordered.
    ///
    /// Also known as *multichoose*.
    ///
    /// # Formula
    #[doc = FORMULA_COMBINE_REP!()]
    ///
    /// # Errors
    /// Returns [`Overflow`] if the result cant't fit the type.
    fn int_combine_rep(self, r: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_combine_rep`][Self::int_combine_rep] but takes the arguments by reference.*
    #[doc = ref_fn!["int_combine_rep"]]
    fn int_ref_combine_rep(&self, r: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the number of permutations of `n` items taken `r` at a time, ordered.
    ///
    /// When $n=r$ or $n=r-1$ the result is the same as calculating the factorial $n!$.
    ///
    /// # Formula
    #[doc = FORMULA_PERMUTE!()]
    ///
    /// # Errors
    /// Returns [`MismatchedSizes`] if $r > n$ and [`Overflow`] if the result cant't fit the type.
    fn int_permute(self, r: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_permute`][Self::int_permute] but takes the arguments by reference.*
    #[doc = ref_fn!["int_permute"]]
    fn int_ref_permute(&self, r: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the number of permutations of n` items taken `r` at a time with repetitions,
    /// ordered.
    ///
    /// # Formula
    #[doc = FORMULA_PERMUTE_REP!()]
    ///
    /// # Errors
    /// Returns [`Overflow`] if the result cant't fit the type.
    fn int_permute_rep(self, r: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_permute_rep`][Self::int_permute_rep] but takes the arguments by reference.*
    #[doc = ref_fn!["int_permute_rep"]]
    fn int_ref_permute_rep(&self, r: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /* division */

    /// Returns the truncated quotient and the remainder.
    fn int_div_rem(self, b: Self::Rhs) -> Result<[Self::Out; 2]> where Self: Sized { E::ni() }
    /// *Like [`int_div_rem`][Self::int_div_rem] but takes the arguments by reference.*
    #[doc = ref_fn!["int_div_rem"]]
    fn int_ref_div_rem(&self, b: &Self::Rhs) -> Result<[Self::Out; 2]> { E::ni() }

    /// Returns the quotient, rounding the result towards positive infinity.
    ///
    /// # Notation
    #[doc = NOTATION_DIV_CEIL!()]
    fn int_div_ceil(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_div_ceil`][Self::int_div_ceil] but takes the arguments by reference.*
    #[doc = ref_fn!["int_div_ceil"]]
    fn int_ref_div_ceil(&self, b: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the quotient, rounding the result towards negative infinity.
    ///
    /// # Notation
    #[doc = NOTATION_DIV_FLOOR!()]
    fn int_div_floor(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_div_floor`][Self::int_div_floor] but takes the arguments by reference.*
    #[doc = ref_fn!["int_div_floor"]]
    fn int_ref_div_floor(&self, b: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the quotient, rounding ties away from zero.
    fn int_div_ties_away(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_div_ties_away`][Self::int_div_ties_away] but takes the arguments by reference.*
    #[doc = ref_fn!["int_div_ties_away"]]
    fn int_ref_div_ties_away(&self, b: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the quotient, rounding ties towards from zero.
    fn int_div_ties_towards(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_div_ties_towards`][Self::int_div_ties_towards]
    /// but takes the arguments by reference.*
    #[doc = ref_fn!["int_div_ties_towards"]]
    fn int_ref_div_ties_towards(&self, b: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the quotient, rounding ties to the nearest even number.
    fn int_div_ties_even(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_div_ties_even`][Self::int_div_ties_even] but takes the arguments by reference.*
    #[doc = ref_fn!["int_div_ties_even"]]
    fn int_ref_div_ties_even(&self, b: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Returns the quotient, rounding ties to the nearest odd number.
    fn int_div_ties_odd(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`int_div_ties_odd`][Self::int_div_ties_odd] but takes the arguments by reference.*
    #[doc = ref_fn!["int_div_ties_odd"]]
    fn int_ref_div_ties_odd(&self, b: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /* factors (allocating) */

    /// Returns the factors (including 1 and self).
    ///
    /// This is the allocating version of [`int_factors_buf`][Self::int_factors_buf].
    ///
    /// # Examples
    /// ```
    /// # use devela::NumInt;
    /// assert_eq![24_i32.int_factors(), Ok(vec![1, 2, 3, 4, 6, 8, 12, 24])];
    /// assert_eq![(-24_i32).int_factors(), Ok(vec![1, 2, 3, 4, 6, 8, 12, 24])];
    /// assert_eq![0_i32.int_factors(), Ok(vec![])];
    /// assert_eq![1_i32.int_factors(), Ok(vec![1])];
    /// assert_eq![7_i32.int_factors(), Ok(vec![1, 7])];
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    fn int_factors(self) -> Result<Vec<Self::Out>> where Self: Sized { E::ni() }
    /// *Like [`int_factors`][Self::int_factors]
    /// but takes the arguments by reference.*
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    #[doc = ref_fn!["int_factors"]]
    fn int_ref_factors(&self) -> Result<Vec<Self::Out>> { E::ni() }

    /// Returns the proper factors.
    ///
    /// This is the allocating version of [`int_factors_proper_buf`][Self::int_factors_proper_buf].
    ///
    /// # Examples
    /// ```
    /// # use devela::NumInt;
    /// assert_eq![24_i32.int_factors_proper(), Ok(vec![2, 3, 4, 6, 8, 12])];
    /// assert_eq![(-24_i32).int_factors_proper(), Ok(vec![2, 3, 4, 6, 8, 12])];
    /// assert_eq![0_i32.int_factors_proper(), Ok(vec![])];
    /// assert_eq![1_i32.int_factors_proper(), Ok(vec![])];
    /// assert_eq![7_i32.int_factors_proper(), Ok(vec![])];
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    fn int_factors_proper(self) -> Result<Vec<Self::Out>> where Self: Sized { E::ni() }
    /// *Like [`int_factors_proper`][Self::int_factors_proper]
    /// but takes the arguments by reference.*
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    #[doc = ref_fn!["int_factors_proper"]]
    fn int_ref_factors_proper(&self) -> Result<Vec<Self::Out>> { E::ni() }

    /// Returns the prime factors.
    ///
    /// This is the allocating version of [`int_factors_prime_buf`][Self::int_factors_prime_buf].
    ///
    /// # Examples
    /// ```
    /// # use devela::NumInt;
    /// assert_eq![24_i32.int_factors_prime(), Ok(vec![2, 2, 2, 3])];
    /// assert_eq![(-24_i32).int_factors_prime(), Ok(vec![2, 2, 2, 3])];
    /// assert_eq![0_i32.int_factors_prime(), Ok(vec![])];
    /// assert_eq![1_i32.int_factors_prime(), Ok(vec![])];
    /// assert_eq![7_i32.int_factors_prime(), Ok(vec![7])];
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    fn int_factors_prime(self) -> Result<Vec<Self::Out>> where Self: Sized { E::ni() }
    /// *Like [`int_factors_prime`][Self::int_factors_prime]
    /// but takes the arguments by reference.*
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    #[doc = ref_fn!["int_factors_prime"]]
    fn int_ref_factors_prime(&self) -> Result<Vec<Self::Out>> { E::ni() }

    /// Returns the unique prime factors.
    ///
    /// This is the allocating version of
    /// [`int_factors_prime_unique_buf`][Self::int_factors_prime_unique_buf].
    ///
    /// # Examples
    /// ```
    /// # use devela::NumInt;
    /// assert_eq![24_i32.int_factors_prime_unique(), Ok(vec![2, 3])];
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    fn int_factors_prime_unique(self) -> Result<Vec<Self::Out>> where Self: Sized { E::ni() }
    /// *Like [`int_factors_prime_unique`][Self::int_factors_prime_unique]
    /// but takes the arguments by reference.*
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    #[doc = ref_fn!["int_factors_prime_unique"]]
    fn int_ref_factors_prime_unique(&self) -> Result<Vec<Self::Out>> { E::ni() }

    /* factors (non-allocating) */

    /// Writes the factors in `fbuf`, and the unique prime factors in `upfbuf`.
    ///
    /// This is the non-allocating version of [`int_factors`][Self::int_factors].
    ///
    /// Returns a tuple with the number of factors and unique prime factors found.
    ///
    /// # Errors
    /// Returns [`MismatchedSizes`] if the total number of factors is greater
    /// than the length of any buffer.
    ///
    /// # Examples
    /// ```
    /// # use devela::NumInt;
    /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
    /// assert_eq![24_i32.int_factors_buf(&mut fbuf, &mut upbuf), Ok((8, 2))];
    ///
    /// assert_eq![fbuf[..8], [1, 2, 3, 4, 6, 8, 12, 24]];
    /// assert_eq![upbuf[..2], [2, 3]];
    /// ```
    fn int_factors_buf(self, fbuf: &mut [Self::Out], upfbuf: &mut [Self::Out])
        -> Result<(usize, usize)> where Self: Sized { E::ni() }
    /// *Like [`int_factors_buf`][Self::int_factors_buf] but takes the arguments by reference.*
    #[doc = ref_fn!["int_factors_buf"]]
    fn int_ref_factors_buf(&self, fbuf: &mut [Self::Out], upfbuf: &mut [Self::Out])
        -> Result<(usize, usize)> { E::ni() }

    /// Writes the proper factors in `fbuf`, and the unique prime factors in `upfbuf`.
    ///
    /// This is the non-allocating version of [`int_factors_proper`][Self::int_factors_proper].
    ///
    /// Returns a tuple with the number of factors and unique prime factors found.
    ///
    /// # Errors
    /// Returns [`MismatchedSizes`] if the total number of factors is greater
    /// than the length of any buffer.
    ///
    /// # Examples
    /// ```
    /// # use devela::NumInt;
    /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
    /// assert_eq![24_i32.int_factors_proper_buf(&mut fbuf, &mut upbuf), Ok((6, 2))];
    ///
    /// assert_eq![fbuf[..6], [2, 3, 4, 6, 8, 12,]];
    /// assert_eq![upbuf[..2], [2, 3]];
    /// ```
    fn int_factors_proper_buf(self, fbuf: &mut [Self], upfbuf: &mut [Self])
        -> Result<(usize, usize)> where Self: Sized { E::ni() }
    /// *Like [`int_factors_proper_buf`][Self::int_factors_proper_buf]
    /// but takes the arguments by reference.*
    #[doc = ref_fn!["int_factors_proper_buf"]]
    fn int_ref_factors_proper_buf(&self, fbuf: &mut [Self::Out], upfbuf: &mut [Self::Out])
        -> Result<(usize, usize)> { E::ni() }

    /// Writes the prime factors in the given `buffer`.
    ///
    /// This is the non-allocating version of [`int_factors_prime`][Self::int_factors_prime].
    ///
    /// Returns the number of factors found.
    ///
    /// # Errors
    /// Returns [`MismatchedSizes`] if the total number of factors is greater
    /// than the length of the `buffer`.
    ///
    /// # Examples
    /// ```
    /// # use devela::NumInt;
    /// let mut buf = [0; 5];
    /// assert_eq![24_i32.int_factors_prime_buf(&mut buf), Ok(4)];
    ///
    /// assert_eq![buf[..4], [2, 2, 2, 3]];
    /// assert![(24_i32 * 4).int_factors_prime_buf(&mut buf).is_err()];
    /// assert_eq![buf, [2, 2, 2, 2, 2]]; // the 3 didn't fit
    ///
    /// assert_eq![0_i32.int_factors_prime_buf(&mut buf), Ok(0)];
    /// assert_eq![1_i32.int_factors_prime_buf(&mut buf), Ok(0)];
    /// assert_eq![7_i32.int_factors_prime_buf(&mut buf), Ok(1)];
    /// assert_eq![buf[..1], [7]];
    /// ```
    fn int_factors_prime_buf(self, buffer: &mut [Self])
        -> Result<usize> where Self: Sized { E::ni() }
    /// *Like [`int_factors_prime_buf`][Self::int_factors_prime_buf]
    /// but takes the arguments by reference.*
    #[doc = ref_fn!["int_factors_prime_buf"]]
    fn int_ref_factors_prime_buf(&self, buffer: &mut [Self::Out]) -> Result<usize> { E::ni() }

    /// Writes the prime factors in the given `buffer`.
    ///
    /// This is the non-allocating version of
    /// [`int_factors_prime_unique`][Self::int_factors_prime_unique].
    ///
    /// The buffer must be large enough to hold all the non-unique factors of `n`.
    /// In that case the function will return the number of unique factors found.
    ///
    /// # Errors
    /// Returns [`MismatchedSizes`] if the unique number of factors is greater than the
    /// length of the `buffer`. In that case the buffer will only contain the non-unique
    /// factors that can fit, like [`int_factors_prime_buf`][Self::int_factors_prime_buf].
    ///
    /// # Examples
    /// ```
    /// # use devela::NumInt;
    /// let mut uniq = [0; 5];
    /// assert_eq![24_i32.int_factors_prime_unique_buf(&mut uniq), Ok(2)];
    /// assert_eq![uniq, [2, 3, 2, 3, 0]];
    /// ```
    fn int_factors_prime_unique_buf(self, buffer: &mut [Self])
        -> Result<usize> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_factors_prime_unique_buf"]]
    fn int_ref_factors_prime_unique_buf(&self, buffer: &mut [Self::Out])
        -> Result<usize> { E::ni() }

    /* primes */

    /// Returns `true` if `n` is prime.
    fn int_is_prime(self) -> Result<bool> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_is_prime"]]
    fn int_ref_is_prime(&self) -> Result<bool> { E::ni() }

    /// Finds the 0-indexed `nth` prime number.
    ///
    /// Note: If `nth` is negative, this function should treat it as its absolute value.
    ///
    /// # Errors
    /// Returns [`Overflow`] if the result can't fit the type.
    fn int_prime_nth(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_prime_nth"]]
    fn int_ref_prime_nth(&self) -> Result<Self::Out> { E::ni() }

    /// Counts the number of primes upto and including `n`.
    fn int_prime_pi(self) -> Result<usize> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_prime_pi"]]
    fn int_ref_prime_pi(&self) -> Result<usize> { E::ni() }

    /// Counts the number of integers $<|n|$ that are relatively prime to `n`.
    ///
    /// Note: If `n` is negative, this function should treat it as its absolute value.
    fn int_totient(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_totient"]]
    fn int_ref_totient(&self) -> Result<Self::Out> { E::ni() }

    /* roots (square) */

    /// Returns `true` if it's a perfect square.
    ///
    /// Returns `false` otherwise, which includes all possible negative values.
    ///
    /// # Errors
    /// Returns [`NonNegativeRequired`] if $n<0$ and [`Overflow`] if the result can't fit the type.
    ///
    /// # Algorithm
    #[doc = ALGORITHM_IS_SQUARE!()]
    fn int_is_square(self) -> Result<bool> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_is_square"]]
    fn int_ref_is_square(&self) -> Result<bool> { E::ni() }

    /// Returns the ceiled integer square root.
    ///
    /// # Errors
    /// Returns [`NonNegativeRequired`] if `self` is negative.
    ///
    /// # Algorithm
    #[doc = ALGORITHM_SQRT_CEIL!()]
    fn int_sqrt_ceil(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_sqrt_ceil"]]
    fn int_ref_sqrt_ceil(&self) -> Result<Self::Out> { E::ni() }

    /// Returns the floored integer square root.
    ///
    /// # Errors
    /// Returns [`NonNegativeRequired`] if `self` is negative.
    ///
    /// # Algorithm
    #[doc = ALGORITHM_SQRT_FLOOR!()]
    fn int_sqrt_floor(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_sqrt_floor"]]
    fn int_ref_sqrt_floor(&self) -> Result<Self::Out> { E::ni() }

    /// Returns the rounded integer square root.
    ///
    /// # Algorithm
    #[doc = ALGORITHM_SQRT_ROUND!()]
    fn int_sqrt_round(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_sqrt_round"]]
    fn int_ref_sqrt_round(&self) -> Result<Self::Out> { E::ni() }

    /* roots */

    /// Returns the ceiled integer `nth` root.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `nth` is 0, or
    /// [`NonNegativeRequired`] if `self` is negative and `nth` is even.
    /// # Algorithm
    #[doc = ALGORITHM_ROOT_CEIL_SIGNED!()]
    fn int_root_ceil(self, nth: u32) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_root_ceil"]]
    fn int_ref_root_ceil(&self, nth: u32) -> Result<Self::Out> { E::ni() }

    /// Returns the floored integer `nth` root.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `nth` is 0, or
    /// [`NonNegativeRequired`] if `self` is negative and `nth` is even.
    /// # Algorithm
    #[doc = ALGORITHM_ROOT_FLOOR_SIGNED!()]
    fn int_root_floor(self, nth: u32) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_root_floor"]]
    fn int_ref_root_floor(&self, nth: u32) -> Result<Self::Out> { E::ni() }

    /* modulo */

    /// Computes the non-negative modulo of `self` over |`modulus`|.
    ///
    /// The result is non-negative and less than the absolute value of `modulus`,
    /// i.e., in the range $ [0, |\text{modulus}|) $.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// and it could also return [`Overflow`].
    fn int_modulo(self, modulus: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo"]]
    fn int_ref_modulo(&self, modulus: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Computes the non-negative modulo of `self` over |`modulus`|,
    /// and the number of cycles it is reduced.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// and it could also return [`Overflow`].
    fn int_modulo_cycles(self, modulus: Self::Rhs)
        -> Result<ValueQuant<Self::Out, Self::Out>> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_cycles"]]
    fn int_ref_modulo_cycles(&self, modulus: &Self::Rhs)
        -> Result<ValueQuant<Self::Out, Self::Out>> where Self: Sized { E::ni() }

    /// Computes the modulo of `self + other` over |`modulus`|.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// and it could also return [`Overflow`].
    fn int_modulo_add(self, other: Self::Rhs, modulus: Self::Rhs)
        -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_add"]]
    fn int_ref_modulo_add(&self, other: &Self::Rhs, modulus: &Self::Rhs)
        -> Result<Self::Out> { E::ni() }

    /// Computes the modulo of `self + other` over |`modulus`|,
    /// and the number of cycles the result is reduced.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// and it could also return [`Overflow`].
    fn int_modulo_add_cycles(self, other: Self::Rhs, modulus: Self::Rhs)
        -> Result<ValueQuant<Self::Out, Self::Out>> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_add_cycles"]]
    fn int_ref_modulo_add_cycles(&self, other: &Self::Rhs, modulus: &Self::Rhs)
        -> Result<ValueQuant<Self::Out, Self::Out>> where Self: Sized { E::ni() }

    /// Calculates the modular additive inverse.
    ///
    /// The modular additive inverse of *self* modulo *modulus*
    /// is an integer *b* such that $ a+b \equiv 0 (\mod m) $.
    ///
    /// The modular multiplicative inverse always exists and is simply
    /// `modulus - self` if `self != 0`, or 0 otherwise.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`.
    fn int_modulo_add_inv(self, modulus: Self::Rhs)
        -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_add_inv"]]
    fn int_ref_modulo_add_inv(&self, modulus: &Self::Rhs)
        -> Result<Self::Out> { E::ni() }

    /// Computes the modulo of `self - other` over |`modulus`|.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// and [`Overflow`] if the result would be a negative value.
    fn int_modulo_sub(self, other: Self::Rhs, modulus: Self::Rhs)
        -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_sub"]]
    fn int_ref_modulo_sub(&self, other: &Self::Rhs, modulus: &Self::Rhs)
        -> Result<Self::Out> { E::ni() }

    /// Computes the modulo of `self - other` over |`modulus`|,
    /// and the number of cycles the result is reduced.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// and [`Overflow`] if the result would be a negative value.
    fn int_modulo_sub_cycles(self, other: Self::Rhs, modulus: Self::Rhs)
        -> Result<ValueQuant<Self::Out, Self::Out>> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_sub_cycles"]]
    fn int_ref_modulo_sub_cycles(&self, other: &Self::Rhs, modulus: &Self::Rhs)
        -> Result<ValueQuant<Self::Out, Self::Out>> where Self: Sized { E::ni() }

    /// Computes the modulo of `self + other` over |`modulus`|.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// and it could also return [`Overflow`].
    fn int_modulo_mul(self, other: Self::Rhs, modulus: Self::Rhs)
        -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_mul"]]
    fn int_ref_modulo_mul(&self, other: &Self::Rhs, modulus: &Self::Rhs)
        -> Result<Self::Out> { E::ni() }

    /// Computes the modulo of `self + other` over |`modulus`|,
    /// and the number of cycles the result is reduced.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// and it could also return [`Overflow`].
    fn int_modulo_mul_cycles(self, other: Self::Rhs, modulus: Self::Rhs)
        -> Result<ValueQuant<Self::Out, Self::Out>> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_mul_cycles"]]
    fn int_ref_modulo_mul_cycles(&self, other: &Self::Rhs, modulus: &Self::Rhs)
        -> Result<ValueQuant<Self::Out, Self::Out>> where Self: Sized { E::ni() }

    /// Calculates the modular multiplicative inverse.
    ///
    /// The modular multiplicative inverse of *a* modulo *m*
    /// is an integer *b* such that $ ab \equiv 1 (\mod m) $.
    ///
    /// The modular multiplicative inverse exists only if `self` and
    /// `modulus` are coprime, meaning their greatest common divisor is 1.
    ///
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// [`NoInverse`] if there's no inverse,
    /// and it could also return [`Overflow`].
    fn int_modulo_mul_inv(self, modulus: Self::Rhs)
        -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_mul_inv"]]
    fn int_ref_modulo_mul_inv(&self, modulus: &Self::Rhs)
        -> Result<Self::Out> { E::ni() }

    /// Computes `self / other` over |`modulus`|.
    ///
    /// $a / b \mod m$ is equivalent to $a * b^{-1} \mod m$,
    /// where $b^{-1}$ is the modular multiplicative inverse
    /// of $b$ modulo $m$.
    /// # Errors
    /// Returns [`NonZeroRequired`] if `modulus == 0`,
    /// [`NoInverse`] if there's no multiplicative inverse of `other`,
    /// and it could also return [`Overflow`].
    fn int_modulo_div(self, other: Self::Rhs, modulus: Self::Rhs)
        -> Result<Self::Out> where Self: Sized { E::ni() }
    #[doc = ref_fn!["int_modulo_div"]]
    fn int_ref_modulo_div(&self, other: &Self::Rhs, modulus: &Self::Rhs)
        -> Result<Self::Out> { E::ni() }
}

macro_rules! ref_fn {
    () => {};
    ($fn:literal) => { concat! {
        "*Like [`", $fn, "`][Self::", $fn, "] but takes the arguments by reference.*"
    }};
}
use ref_fn;
