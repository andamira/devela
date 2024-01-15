// devela::num::frac::wrapper::impl_frac
//
//! implements non-owning fraction-related methods
//
// TOC
// - prim. array | Int array:
//   - numerator | num
//   - denominator | den
//   - is_valid
//   - is_proper
//   - is_reduced
//   - reduce
//   - gcd
//   - lcm
//   - add

use crate::num::{Frac, Int, NumResult as Result};
#[cfg(doc)]
use {crate::num::NumErrors as E, E::Overflow};

// $i: the integer type.
// $self: the fractional self type.
// $fout: the fractionsl output type.
macro_rules! impl_fract {
    ($( $i:ty ),+ ) => {
        $(
            impl_fract![@array $i, [$i; 2], Frac<[$i; 2]>];
            impl_fract![@int_array $i, [Int<$i>; 2], Frac<[Int<$i>; 2]>];
            // impl_fract![@slice $i, &[$i], [$i; 2]];
        )+
    };

    (@array $i:ty, $self:ty, $fout:ty) => { $crate::code::paste! {
        #[doc = "# Fraction related methods for `[" $i "; 2]`\n\n"]
        /// See the related trait [`NumFrac`][crate::num::NumFrac].
        impl Frac<$self> {
            /// Returns the numerator (the first number of the sequence).
            #[must_use] #[inline]
            pub const fn numerator(self) -> $i { self.0[0] }
            /// Alias of [`numerator`][Self::numerator].
            #[must_use] #[inline] pub const fn num(self) -> $i { self.0[0] }

            /// Returns the denominator (the second number of the sequence).
            #[must_use] #[inline]
            pub const fn denominator(self) -> $i { self.0[1] }
            /// Alias of [`denominator`][Self::denominator].
            #[must_use] #[inline] pub const fn den(self) -> $i { self.0[0] }

            /// Retuns `true` if the fraction is valid `(denominator != 0)`.
            #[must_use] #[inline]
            pub const fn is_valid(self) -> bool { self.0[1] != 0 }

            /// Returns `true` if the fraction is proper
            /// `(numerator.abs() < denominator.abs())`.
            pub const fn is_proper(self) -> bool { Int(self.0[0]).abs() < Int(self.0[1]).abs() }

            /// Retuns `true` if the fraction is in the simplest possible form `(gcd() == 1)`.
            pub const fn is_reduced(self) -> bool { self.gcd() == 1 }

            /// Simplify a fraction.
            #[must_use] #[inline]
            pub const fn reduce(self) -> $fout {
                let g = self.gcd();
                Frac([self.0[0] / g, self.0[1] / g])
            }

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>
            /// between the numerator and the denominator.
            #[must_use] #[inline]
            pub const fn gcd(self) -> $i { Int(self.0[0]).gcd(self.0[1]).0 }

            /// Returns the <abbr title="Least Common Multiple">LCM</abbr>
            /// between the numerator and the denominator.
            /// # Errors
            /// Could [`Overflow`].
            #[inline]
            pub const fn lcm(self) -> Result<$i> {
                match Int(self.numerator()).lcm(self.denominator()) {
                    Ok(res) => Ok(res.0),
                    Err(e) => Err(e)
                }
            }

            /// Adds two fractions.
            #[inline] #[allow(clippy::should_implement_trait)]
            pub fn add(self, other: $self) -> Result<$fout> {
                let [num1, den1, num2, den2] = [self.0[0], self.0[1], other[0], other[1]];
                let lcm_denom = match Int(den1).lcm(den2) {
                    Ok(res) => res.0,
                    Err(e) => { return Err(e); }
                };
                let num = num1 * (lcm_denom / den1) + num2 * (lcm_denom / den2);
                Ok(Frac([num, lcm_denom]).reduce())
            }
        }
    }};

    (@int_array $i:ty, $self:ty, $fout:ty) => { $crate::code::paste! {
        #[doc = "# Fraction related methods for `[Int<" $i ">; 2]`\n\n"]
        /// See the related trait [`NumFrac`][crate::num::NumFrac].
        impl Frac<$self> {
            /// Returns the numerator (the first number of the sequence).
            #[must_use] #[inline]
            pub const fn numerator(self) -> Int<$i> { self.0[0] }

            /// Returns the denominator (the second number of the sequence).
            #[must_use] #[inline]
            pub const fn denominator(self) -> Int<$i> { self.0[1] }

            /// Retuns `true` if the fraction is valid `(denominator != 0)`.
            #[must_use] #[inline]
            pub const fn is_valid(self) -> bool { self.0[1].0 != 0 }

            /// Returns `true` if the fraction is proper
            /// `(numerator.abs() < denominator.abs())`.
            pub const fn is_proper(self) -> bool { self.0[0].abs() < self.0[1].abs() }

            /// Retuns `true` if the fraction is in the simplest possible form `(gcd() == 1)`.
            pub const fn is_reduced(self) -> bool { self.gcd().0 == 1 }

            /// Simplify a fraction.
            #[must_use] #[inline]
            pub const fn reduce(self) -> $fout {
                let g = self.gcd().0;
                Frac([Int(self.0[0].0 / g), Int(self.0[1].0 / g)])
            }

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>
            /// between the numerator and the denominator.
            #[must_use] #[inline]
            pub const fn gcd(self) -> Int<$i> { self.0[0].gcd(self.0[1].0) }

            /// Returns the <abbr title="Least Common Multiple">LCM</abbr>
            /// between the numerator and the denominator.
            /// # Errors
            /// Could [`Overflow`].
            #[inline]
            pub const fn lcm(self) -> Result<Int<$i>> {
                match self.numerator().lcm(self.denominator().0) {
                    Ok(res) => Ok(res),
                    Err(e) => Err(e)
                }
            }

            /// Adds two fractions.
            #[inline] #[allow(clippy::should_implement_trait)]
            pub fn add(self, other: $self) -> Result<$fout> {
                let [num1, den1, num2, den2] = [self.0[0].0, self.0[1].0, other[0].0, other[1].0];
                let lcm_denom = match Int(den1).lcm(den2) {
                    Ok(res) => res.0,
                    Err(e) => { return Err(e); }
                };
                let num = num1 * (lcm_denom / den1) + num2 * (lcm_denom / den2);
                Ok(Frac([Int(num), Int(lcm_denom)]).reduce())
            }
        }
    }};
}
impl_fract![i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
