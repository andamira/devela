// devela::num::int::trait
//
//!
//

use crate::num::{Num, NumErrors as E, NumRef, NumResult as Result};
use core::ops::Deref;
#[cfg(doc)]
use E::{MismatchedSizes, NonNegativeRequired, NotImplemented, NotSupported, Overflow};

mod impls;

mod auto_impls {
    use super::{NumInt, NumRefInt};

    #[rustfmt::skip]
    impl<'a, T: NumInt> NumRefInt<'a> for &T {}
    #[rustfmt::skip]
    impl<'a, T: NumInt> NumRefInt<'a> for &mut T {}
}

/// Common trait for integer types.
///
/// # Notes
/// - We use `n` to refer to the `self` argument in all the method descriptions and formulas.
///
/// - Every method in this trait returns [`NotImplemented`] by default.
/// - Any concrete implementation must implement the operations it wants to support.
/// - Any operations specifically not supported should ideally return [`NotSupported`].
///
/// - Most methods come in two variants, starting with different prefixes:
///   - `int_*` methods take the arguments by value.
///   - `int_ref_*` methods take the arguments by reference.
///
/// - This trait tries to offer the same methods everywhere, giving a result when possible.
/// - Operations are generally supported if they can be valid for some values,
///
/// See also [`NumRefInt`] which is automatically implemented for `NumInt` references.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[rustfmt::skip] #[allow(unused_variables)]
pub trait NumInt: Num {
    /* division */

    /// Returns the truncated quotient and the remainder.
    fn int_div_rem(self, b: Self::Rhs) -> Result<[Self::Out; 2]> where Self: Sized{ E::ni() }
    /// *Like [`int_div_rem`][Self::int_div_rem] but takes the arguments by reference.*
    fn int_ref_div_rem(&self, b: &Self::Rhs) -> Result<[Self::Out; 2]> where Self: Sized{ E::ni() }

    /// Returns the quotient, rounding the result towards positive infinity.
    /// # Notation
    /// $$ \large \left\lceil \frac{x}{y} \right\rceil $$
    fn int_div_ceil(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_div_ceil`][Self::int_div_ceil] but takes the arguments by reference.*
    fn int_ref_div_ceil(&self, b: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the quotient, rounding the result towards negative infinity.
    /// # Notation
    /// $$ \large \left\lfloor \frac{x}{y} \right\rfloor $$
    fn int_div_floor(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_div_floor`][Self::int_div_floor] but takes the arguments by reference.*
    fn int_ref_div_floor(&self, b: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the quotient, rounding ties away from zero.
    fn int_div_ties_away(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_div_ties_away`][Self::int_div_ties_away] but takes the arguments by reference.*
    fn int_ref_div_ties_away(&self, b: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the quotient, rounding ties towards from zero.
    fn int_div_ties_towards(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_div_ties_towards`][Self::int_div_ties_towards] but takes the arguments by reference.*
    fn int_ref_div_ties_towards(&self, b: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the quotient, rounding ties to the nearest even number.
    fn int_div_ties_even(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_div_ties_even`][Self::int_div_ties_even] but takes the arguments by reference.*
    fn int_ref_div_ties_even(&self, b: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the quotient, rounding ties to the nearest odd number.
    fn int_div_ties_odd(self, b: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_div_ties_odd`][Self::int_div_ties_odd] but takes the arguments by reference.*
    fn int_ref_div_ties_odd(&self, b: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /* combinatorics */

    /// Returns the factorial.
    ///
    /// Permutations of *n* items, ordered, where $n = r$.
    ///
    /// # Formula
    /// $$ n! = 1 \cdot 2 \cdot 3 \cdot \ldots \cdot (n-1) \cdot n $$
    ///
    /// These are the maximum numbers whose factorials can fit within
    /// standard signed integer types:
    /// - 5 for `i8`, 7 for `i16`, 12 for `i32`, 20 for `i64` and 33 for `i128`.
    /// - 5 for `u8`, 8 for `u16`, 12 for `u32`, 20 for `u64` and 34 for `u128`.
    /// # Errors
    /// Returns [`NonNegativeRequired`] if $n<0$ and [`Overflow`] if the result can't fit the type.
    fn int_factorial(self) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_factorial`][Self::int_factorial] but takes the arguments by reference.*
    fn int_ref_factorial(&self) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the subfactorial, or the number of derangements.
    ///
    /// Permutations of *n* items which no element appears in its original position.
    ///
    /// # Algorithm
    /// The current implementation uses following recursive algorithm:
    /// $$ !n = (n - 1) * (!(n - 1) + !(n - 2)) $$
    ///
    /// Other possible formulas are:
    /// $$
    /// \begin{alignat}{1}
    /// !n & = n! \times \sum_{k=0}^{n} \frac{(-1)^k}{k!} \newline
    /// !n & = \left\lfloor \frac{n!}{e} + \frac{1}{2} \right\rfloor =
    ///     \left\lfloor n! \times \left(\frac{1}{e}\right) + \frac{1}{2} \right\rfloor
    /// \end{alignat}
    /// $$
    ///
    /// These are the maximum numbers whose subfactorials can fit within
    /// standard signed integer types:
    /// - 5 for `i8`, 8 for `i16`, 12 for `i32`, 20 for `i64` and 35 for `i128`.
    /// - 5 for `u8`, 8 for `u16`, 13 for `u32`, 20 for `u64` and 35 for `u128`.
    ///
    /// # Errors
    /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0$,
    /// and [`Overflow`][E::Overflow] if the result can't fit the type.
    /// # Links
    /// - The list of subfactorials is available in <https://oeis.org/A000166>.
    fn int_subfactorial(self) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_subfactorial`][Self::int_subfactorial] but takes the arguments by reference.*
    fn int_ref_subfactorial(&self) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the number of permutations of `n` items taken `r` at a time, ordered.
    ///
    /// When $n=r$ or $n=r-1$ the result is the same as calculating the factorial $n!$.
    ///
    /// # Formula
    /// $$ \large P(n,r) = \frac{n!}{(n−r)!} $$
    /// # Errors
    /// Returns [`MismatchedSizes`] if $r > n$ and [`Overflow`] if the result cant't fit the type.
    fn int_permute(self, r: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_permute`][Self::int_permute] but takes the arguments by reference.*
    fn int_ref_permute(&self, r: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the number of permutations of n` items taken `r` at a time with repetitions,
    /// ordered.
    ///
    /// # Formula
    /// $$ \large P_\text{rep}(n,r) = n_r $$
    /// # Errors
    /// Returns [`Overflow`] if the result cant't fit the type.
    fn int_permute_rep(self, r: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_permute_rep`][Self::int_permute_rep] but takes the arguments by reference.*
    fn int_ref_permute_rep(&self, r: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the number of combinations of `n` items taken `r` at a time, unordered.
    ///
    /// # Formula
    /// $$ \large C(n,r) = {n \choose r} = \frac{n!}{(n−r)!r!} $$
    /// # Errors
    /// Returns [`MismatchedSizes`] if $r > n$ and [`Overflow`] if the result cant't fit the type.
    fn int_combine(self, r: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_combine`][Self::int_combine] but takes the arguments by reference.*
    fn int_ref_combine(&self, r: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the number of permutations of `n` items taken `r` at a time with repetitions,
    /// unordered.
    ///
    /// Also known as *multichoose*.
    ///
    /// # Formula
    /// $$ \large C(n+r-1,r) = {n+k-1 \choose r} = \frac{(n+r-1)!}{(n−1)!r!} $$
    /// # Errors
    /// Returns [`Overflow`][E::Overflow] if the result cant't fit the type.
    fn int_combine_rep(self, r: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_combine_rep`][Self::int_combine_rep] but takes the arguments by reference.*
    fn int_ref_combine_rep(&self, r: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /* digital root */

    /// Returns the digital root in base 10.
    fn int_digital_root(self) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_digital_root`][Self::int_digital_root] but takes the arguments by reference.*
    fn int_ref_digital_root(&self) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the digital root in the given `base`.
    fn int_digital_root_base(self, base: Self::Rhs) -> Result<Self::Out>
        where Self: Sized{ E::ni() }
    /// *Like [`int_digital_root_base`][Self::int_digital_root_base] but takes the arguments by reference.*
    fn int_ref_digital_root_base(&self, base: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized{ E::ni() }

    /* digits */

    /// Returns the number of digits in base 10.
    fn int_digits(self) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_digits`][Self::int_digits] but takes the arguments by reference.*
    fn int_ref_digits(&self) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the number of digits in base 10 including the negative sign.
    fn int_digits_sign(self) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_digits_sign`][Self::int_digits_sign] but takes the arguments by reference.*
    fn int_ref_digits_sign(&self) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the number of digits in the given `base`.
    fn int_digits_base(self, base: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_digits_base`][Self::int_digits_base] but takes the arguments by reference.*
    fn int_ref_digits_base(&self, base: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized{ E::ni() }

    /// Returns the number of digits in the given `base`.
    fn int_digits_base_sign(self, base: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_digits_base_sign`][Self::int_digits_base_sign] but takes the arguments by reference.*
    fn int_ref_digits_base_sign(&self, base: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized{ E::ni() }

    /* gcd & lcm */

    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
    fn int_gcd(self, other: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_gcd`][Self::int_gcd] but takes the arguments by reference.*
    fn int_ref_gcd(&self, other: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr> and the Bézout coeficients.
    fn int_gcd_ext(self, other: Self::Rhs) -> Result<[Self::Out; 3]> where Self: Sized{ E::ni() }
    /// *Like [`int_gcd_ext`][Self::int_gcd_ext] but takes the arguments by reference.*
    fn int_ref_gcd_ext(&self, other: &Self::Rhs) -> Result<[Self::Out; 3]>
        where Self: Sized{ E::ni() }

    /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
    fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// *Like [`int_lcm`][Self::int_lcm] but takes the arguments by reference.*
    fn int_ref_lcm(&self, other: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
}

/// Common trait for referenced integer types.
///
/// # Notes
/// - It is automatically implemented for references of types implementing [`NumInt`].
/// - Mutable operations are only available for exclusive (`&mut`) references.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[rustfmt::skip] #[allow(unused_variables)]
pub trait NumRefInt<'a>: NumRef<'a>
where
    Self: Deref<Target = <Self as NumRef<'a>>::Own>,
    <Self as NumRef<'a>>::Own: NumInt
{
    /* division */

    /// *Calls `NumInt::`[`int_ref_div_rem`][NumInt::int_ref_div_rem]*.
    fn int_ref_div_rem(&self, b: &<Self::Own as Num>::Rhs) -> Result<[<Self::Own as Num>::Out; 2]> {
            self.deref().int_ref_div_rem(b)
    }
    /// *Calls `NumInt::`[`int_ref_div_ceil`][NumInt::int_ref_div_ceil]*.
    fn int_ref_div_ceil(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ceil(b)
    }
    /// *Calls `NumInt::`[`int_ref_div_floor`][NumInt::int_ref_div_floor]*.
    fn int_ref_div_floor(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_floor(b)
    }
    /// *Calls `NumInt::`[`int_ref_div_ties_away`][NumInt::int_ref_div_ties_away]*.
    fn int_ref_div_ties_away(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ties_away(b)
    }
    /// *Calls `NumInt::`[`int_ref_div_ties_towards`][NumInt::int_ref_div_ties_towards]*.
    fn int_ref_div_ties_towards(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ties_towards(b)
    }
    /// *Calls `NumInt::`[`int_ref_div_ties_even`][NumInt::int_ref_div_ties_even]*.
    fn int_ref_div_ties_even(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ties_even(b)
    }
    /// *Calls `NumInt::`[`int_ref_div_ties_odd`][NumInt::int_ref_div_ties_odd]*.
    fn int_ref_div_ties_odd(&self, b: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_div_ties_odd(b)
    }

    /* combinatorics */

    /// *Calls `NumInt::`[`int_ref_factorial`][NumInt::int_ref_factorial]*.
    fn int_ref_factorial(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_factorial()
    }
    /// *Calls `NumInt::`[`int_ref_subfactorial`][NumInt::int_ref_subfactorial]*.
    fn int_ref_subfactorial(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_subfactorial()
    }
    /// *Calls `NumInt::`[`int_ref_permute`][NumInt::int_ref_permute]*.
    fn int_ref_permute(&self, r: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_permute(r)
    }
    /// *Calls `NumInt::`[`int_ref_permute_rep`][NumInt::int_ref_permute_rep]*.
    fn int_ref_permute_rep(&self, r: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_permute_rep(r)
    }
    /// *Calls `NumInt::`[`int_ref_combine`][NumInt::int_ref_combine]*.
    fn int_ref_combine(&self, r: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_combine(r)
    }
    /// *Calls `NumInt::`[`int_ref_combine_rep`][NumInt::int_ref_combine_rep]*.
    fn int_ref_combine_rep(&self, r: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_combine_rep(r)
    }

    /* digital root */

    /// *Calls `NumInt::`[`int_ref_digital_root`][NumInt::int_ref_digital_root].
    fn int_ref_digital_root(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digital_root()
    }
    /// *Calls `NumInt::`[`int_ref_digital_root_base`][NumInt::int_ref_digital_root_base]*.
    fn int_ref_digital_root_base(&self, base: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digital_root_base(base)
    }

    /* digits */

    /// *Calls `NumInt::`[`int_ref_digits`][NumInt::int_ref_digits]*.
    fn int_ref_digits(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits()
    }
    /// *Calls `NumInt::`[`int_ref_digits_sign`][NumInt::int_ref_digits_sign]*.
    fn int_ref_digits_sign(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits_sign()
    }
    /// *Calls `NumInt::`[`int_ref_digits_base`][NumInt::int_ref_digits_base]*.
    fn int_ref_digits_base(&self, base: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits_base(base)
    }
    /// *Calls `NumInt::`[`int_ref_digits_base_sign`][NumInt::int_ref_digits_base_sign]*.
    fn int_ref_digits_base_sign(&self, base: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits_base_sign(base)
    }

    /* gcd & lcm */

    /// *Calls `NumInt::`[`int_ref_gcd`][NumInt::int_ref_gcd]*.
    fn int_ref_gcd(&self, other: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_gcd(other)
    }
    /// *Calls `NumInt::`[`int_ref_gcd_ext`][NumInt::int_ref_gcd_ext]*.
    fn int_ref_gcd_ext(&self, other: &<Self::Own as Num>::Rhs)
        -> Result<[<Self::Own as Num>::Out; 3]> {
            self.deref().int_ref_gcd_ext(other)
    }
    /// *Calls `NumInt::`[`int_ref_lcm`][NumInt::int_ref_lcm]*.
    fn int_ref_lcm(&self, other: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_lcm(other)
    }
}
