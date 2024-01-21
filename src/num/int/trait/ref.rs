// devela::num::int::trait::ref
//
//!
//
// TOC
// - trait NumIntRef
//   - base
//   - core
//   - combinatorics
//   - division
//   - factors
//   - prime
//   - sqrt

use crate::num::{Num, NumInt, NumRef, NumResult as Result};
#[cfg(feature = "alloc")]
use ::_alloc::vec::Vec;
use core::ops::Deref;

/// Common trait for referenced integer types.
///
/// # Notes
/// - This is automatically implemented for references of types implementing [`NumInt`].
/// - Mutable operations are only available for exclusive (`&mut`) references.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[rustfmt::skip] #[allow(unused_variables)]
pub trait NumRefInt<'a>: NumRef<'a>
where
    Self: Deref<Target = <Self as NumRef<'a>>::Own>,
    <Self as NumRef<'a>>::Own: NumInt
{
    /* base */

    /// *Calls `NumInt::`[`int_ref_digital_root`][NumInt::int_ref_digital_root].
    fn int_ref_digital_root(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digital_root() }
    /// *Calls `NumInt::`[`int_ref_digital_root_base`][NumInt::int_ref_digital_root_base]*.
    fn int_ref_digital_root_base(&self, base: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digital_root_base(base) }

    /// *Calls `NumInt::`[`int_ref_digits`][NumInt::int_ref_digits]*.
    fn int_ref_digits(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits() }
    /// *Calls `NumInt::`[`int_ref_digits_sign`][NumInt::int_ref_digits_sign]*.
    fn int_ref_digits_sign(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits_sign() }
    /// *Calls `NumInt::`[`int_ref_digits_base`][NumInt::int_ref_digits_base]*.
    fn int_ref_digits_base(&self, base: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits_base(base) }
    /// *Calls `NumInt::`[`int_ref_digits_base_sign`][NumInt::int_ref_digits_base_sign]*.
    fn int_ref_digits_base_sign(&self, base: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits_base_sign(base) }

    /* core */

    /// *Calls `NumInt::`[`int_ref_abs`][NumInt::int_ref_abs]*.
    fn int_ref_abs(&self) -> Result<<Self::Own as Num>::Out> { self.deref().int_ref_abs() }

    /// *Calls `NumInt::`[`int_ref_is_even`][NumInt::int_ref_is_even]*.
    fn int_ref_is_even(&self) -> Result<bool> { self.deref().int_ref_is_even() }
    /// *Calls `NumInt::`[`int_ref_is_odd`][NumInt::int_ref_is_odd]*.
    fn int_ref_is_odd(&self) -> Result<bool> { self.deref().int_ref_is_odd() }

    /// *Calls `NumInt::`[`int_ref_gcd`][NumInt::int_ref_gcd]*.
    fn int_ref_gcd(&self, other: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_gcd(other) }
    /// *Calls `NumInt::`[`int_ref_gcd_ext`][NumInt::int_ref_gcd_ext]*.
    fn int_ref_gcd_ext(&self, other: &<Self::Own as Num>::Rhs)
        -> Result<[<Self::Own as Num>::Out; 3]> {
            self.deref().int_ref_gcd_ext(other) }
    /// *Calls `NumInt::`[`int_ref_lcm`][NumInt::int_ref_lcm]*.
    fn int_ref_lcm(&self, other: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_lcm(other) }
    /// *Calls `NumInt::`[`int_ref_scale`][NumInt::int_ref_scale]*.
    fn int_ref_scale(&self,
        min: &<Self::Own as Num>::Rhs, max: &<Self::Own as Num>::Rhs,
        a: &<Self::Own as Num>::Rhs, b: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().int_ref_scale(min, max, a, b) }
    /// *Calls `NumInt::`[`int_ref_scale_wrap`][NumInt::int_ref_scale_wrap]*.
    fn int_ref_scale_wrap(&self,
        min: &<Self::Own as Num>::Rhs, max: &<Self::Own as Num>::Rhs,
        a: &<Self::Own as Num>::Rhs, b: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().int_ref_scale_wrap(min, max, a, b) }

    /* combinatorics */

    /// *Calls `NumInt::`[`int_ref_factorial`][NumInt::int_ref_factorial]*.
    fn int_ref_factorial(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_factorial() }
    /// *Calls `NumInt::`[`int_ref_subfactorial`][NumInt::int_ref_subfactorial]*.
    fn int_ref_subfactorial(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_subfactorial() }
    /// *Calls `NumInt::`[`int_ref_permute`][NumInt::int_ref_permute]*.
    fn int_ref_permute(&self, r: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_permute(r) }
    /// *Calls `NumInt::`[`int_ref_permute_rep`][NumInt::int_ref_permute_rep]*.
    fn int_ref_permute_rep(&self, r: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_permute_rep(r) }
    /// *Calls `NumInt::`[`int_ref_combine`][NumInt::int_ref_combine]*.
    fn int_ref_combine(&self, r: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_combine(r) }
    /// *Calls `NumInt::`[`int_ref_combine_rep`][NumInt::int_ref_combine_rep]*.
    fn int_ref_combine_rep(&self, r: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_combine_rep(r) }

    /* division */

    /// *Calls `NumInt::`[`int_ref_div_rem`][NumInt::int_ref_div_rem]*.
    fn int_ref_div_rem(&self, b: &<Self::Own as Num>::Rhs) -> Result<[<Self::Own as Num>::Out; 2]> {
            self.deref().int_ref_div_rem(b) }
    /// *Calls `NumInt::`[`int_ref_div_ceil`][NumInt::int_ref_div_ceil]*.
    fn int_ref_div_ceil(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ceil(b) }
    /// *Calls `NumInt::`[`int_ref_div_floor`][NumInt::int_ref_div_floor]*.
    fn int_ref_div_floor(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_floor(b) }
    /// *Calls `NumInt::`[`int_ref_div_ties_away`][NumInt::int_ref_div_ties_away]*.
    fn int_ref_div_ties_away(&self, b: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ties_away(b) }
    /// *Calls `NumInt::`[`int_ref_div_ties_towards`][NumInt::int_ref_div_ties_towards]*.
    fn int_ref_div_ties_towards(&self, b: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ties_towards(b) }
    /// *Calls `NumInt::`[`int_ref_div_ties_even`][NumInt::int_ref_div_ties_even]*.
    fn int_ref_div_ties_even(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ties_even(b) }
    /// *Calls `NumInt::`[`int_ref_div_ties_odd`][NumInt::int_ref_div_ties_odd]*.
    fn int_ref_div_ties_odd(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ties_odd(b) }

    /* factors (allocating) */

    /// *Calls `NumInt::`[`int_ref_factors`][NumInt::int_ref_factors]*.
    #[cfg(feature = "alloc")] #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    fn int_ref_factors(&self) -> Result<Vec<<Self::Own as Num>::Out>> { 
            self.deref().int_ref_factors() }
    /// *Calls `NumInt::`[`int_ref_factors_proper`][NumInt::int_ref_factors_proper]*.
    #[cfg(feature = "alloc")] #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    fn int_ref_factors_proper(&self) -> Result<Vec<<Self::Own as Num>::Out>> { 
            self.deref().int_ref_factors_proper() }
    /// *Calls `NumInt::`[`int_ref_factors_prime`][NumInt::int_ref_factors_prime]*.
    #[cfg(feature = "alloc")] #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    fn int_ref_factors_prime(&self) -> Result<Vec<<Self::Own as Num>::Out>> { 
            self.deref().int_ref_factors_prime() }
    /// *Calls `NumInt::`[`int_ref_factors_prime_unique`][NumInt::int_ref_factors_prime_unique]*.
    #[cfg(feature = "alloc")] #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    fn int_ref_factors_prime_unique(&self) -> Result<Vec<<Self::Own as Num>::Out>> { 
            self.deref().int_ref_factors_prime_unique() }

    /* factors (non-allocating) */

    /// *Calls `NumInt::`[`int_ref_factors_buf`][NumInt::int_ref_factors_buf]*.
    fn int_ref_factors_buf(&self,
        fbuf: &mut [<Self::Own as Num>::Out], upfbuf: &mut [<Self::Own as Num>::Out])
        -> Result<(usize, usize)> { 
            self.deref().int_ref_factors_buf(fbuf, upfbuf) }
    /// *Calls `NumInt::`[`int_ref_factors_proper_buf`][NumInt::int_ref_factors_proper_buf]*.
    fn int_ref_factors_proper_buf(&self,
        fbuf: &mut [<Self::Own as Num>::Out], upfbuf: &mut [<Self::Own as Num>::Out])
        -> Result<(usize, usize)> { 
            self.deref().int_ref_factors_proper_buf(fbuf, upfbuf) }
    /// *Calls `NumInt::`[`int_ref_factors_prime_buf`][NumInt::int_ref_factors_prime_buf]*.
    fn int_ref_factors_prime_buf(&self, buffer: &mut [<Self::Own as Num>::Out]) -> Result<usize> { 
            self.deref().int_ref_factors_prime_buf(buffer) }
    /// *Calls
    /// `NumInt::`[`int_ref_factors_prime_unique_buf`][NumInt::int_ref_factors_prime_unique_buf]*.
    fn int_ref_factors_prime_unique_buf(&self, buffer: &mut [<Self::Own as Num>::Out])
        -> Result<usize> { 
            self.deref().int_ref_factors_prime_unique_buf(buffer) }

    /* primes */

    /// *Calls `NumInt::`[`int_ref_is_prime`][NumInt::int_ref_is_prime]*.
    fn int_ref_is_prime(&self) -> Result<bool> {
            self.deref().int_ref_is_prime() }
    /// *Calls `NumInt::`[`int_ref_prime_nth`][NumInt::int_ref_prime_nth]*.
    fn int_ref_prime_nth(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_prime_nth() }
    /// *Calls `NumInt::`[`int_ref_prime_pi`][NumInt::int_ref_prime_pi]*.
    fn int_ref_prime_pi(&self) -> Result<usize> {
            self.deref().int_ref_prime_pi() }
    /// *Calls `NumInt::`[`int_ref_totient`][NumInt::int_ref_totient]*.
    fn int_ref_totient(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_totient() }

    /* square root */

    /// *Calls `NumInt::`[`int_ref_is_square`][NumInt::int_ref_is_square]*.
    fn int_ref_is_square(&self) -> Result<bool> {
            self.deref().int_ref_is_square() }
    /// *Calls `NumInt::`[`int_ref_sqrt_ceil`][NumInt::int_ref_sqrt_ceil]*.
    fn int_ref_sqrt_ceil(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_sqrt_ceil() }
    /// *Calls `NumInt::`[`int_ref_sqrt_floor`][NumInt::int_ref_sqrt_floor]*.
    fn int_ref_sqrt_floor(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_sqrt_floor() }
    /// *Calls `NumInt::`[`int_ref_sqrt_round`][NumInt::int_ref_sqrt_round]*.
    fn int_ref_sqrt_round(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_sqrt_round() }
}
