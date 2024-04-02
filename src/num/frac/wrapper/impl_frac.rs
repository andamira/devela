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
use {crate::num::NumError as E, E::Overflow};

// $i:    the integer type.
// $self: the fractional self type.
// $fout: the fractionsl output type.
// $cap:  the capability feature that enables the given implementation. E.g "_i8".
macro_rules! impl_frac {
    [] => {
        impl_frac![
            i8:"_i8", i16:"_i16", i32:"_i32", i64:"_i64", i128:"_i128", isize:"_isize",
            u8:"_u8", u16:"_u16", u32:"_u32", u64:"_u64", u128:"_u128", usize:"_usize"
        ];
    };

    ($( $i:ty : $cap:literal ),+ ) => {
        $(
            impl_frac![@array $i:$cap, [$i; 2], Frac<[$i; 2]>];
            impl_frac![@int_array $i:$cap, [Int<$i>; 2], Frac<[Int<$i>; 2]>];
            // impl_frac![@slice $i:$cap, &[$i], [$i; 2]]; // MAYBE
        )+
    };

    // both for signed and unsigned
    (@array $i:ty : $cap:literal, $self:ty, $fout:ty) => { $crate::code::paste! {
        #[doc = "# Fraction related methods for `[" $i "; 2]`\n\n"]
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
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
            /// # Examples
            /// ```
            /// # use devela::num::Frac;
            #[doc = "assert![Frac([2_" $i ", 1]).is_valid()];"]
            #[doc = "assert![!Frac([2_" $i ", 0]).is_valid()];"]
            /// ```
            #[must_use] #[inline]
            pub const fn is_valid(self) -> bool { self.0[1] != 0 }

            /// Returns `true` if the fraction is proper
            /// `(numerator.abs() < denominator.abs())`.
            /// # Examples
            /// ```
            /// # use devela::num::Frac;
            #[doc = "assert![Frac([2_" $i ", 3]).is_proper()];"]
            #[doc = "assert![!Frac([3_" $i ", 3]).is_proper()];"]
            #[doc = "assert![!Frac([4_" $i ", 3]).is_proper()];"]
            /// ```
            pub const fn is_proper(self) -> bool { Int(self.0[0]).abs().0 < Int(self.0[1]).abs().0 }

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

    (@int_array $i:ty : $cap:literal, $self:ty, $fout:ty) => { $crate::code::paste! {
        #[doc = "# Fraction related methods for `[Int<" $i ">; 2]`\n\n"]
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Frac<$self> {
            /// Returns the numerator (the first number of the sequence).
            #[must_use] #[inline]
            pub const fn numerator(self) -> Int<$i> { self.0[0] }

            /// Returns the denominator (the second number of the sequence).
            #[must_use] #[inline]
            pub const fn denominator(self) -> Int<$i> { self.0[1] }

            /// Retuns `true` if the fraction is valid `(denominator != 0)`.
            /// # Examples
            /// ```
            /// # use devela::num::{Frac, Int};
            #[doc = "assert![Frac([Int(2_" $i "), Int(1)]).is_valid()];"]
            #[doc = "assert![!Frac([Int(2_" $i "), Int(0)]).is_valid()];"]
            /// ```
            #[must_use] #[inline]
            pub const fn is_valid(self) -> bool { self.0[1].0 != 0 }

            /// Returns `true` if the fraction is proper
            /// `(numerator.abs() < denominator.abs())`.
            /// # Examples
            /// ```
            /// # use devela::num::{Frac, Int};
            #[doc = "assert![Frac([Int(2_" $i "), Int(3)]).is_proper()];"]
            #[doc = "assert![!Frac([Int(3_" $i "), Int(3)]).is_proper()];"]
            #[doc = "assert![!Frac([Int(4_" $i "), Int(3)]).is_proper()];"]
            /// ```
            pub const fn is_proper(self) -> bool { self.0[0].abs().0 < self.0[1].abs().0 }

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
impl_frac![];
