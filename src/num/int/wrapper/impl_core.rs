// devela::num::int::wrapper::impl_core
//
//! implements core integer functions
//
// TOC
// - signed|unsigned
//   - abs
//   - is_even
//   - is_odd
//   - gcd
//   - lcm
// - signed only:
//   - gcd_ext
//   - gcd_ext_euc

use super::Int;
use crate::code::{iif, paste};
use crate::num::{NumErrors, NumResult as Result};
use NumErrors::Overflow;

// $t:   the input/output type
// $up:  the upcasted type to do the operations on (for lcm)
// $dl:  the doclink suffix for the method name
macro_rules! impl_core {
    (signed $( $t:ty : $up:ty : $dl:literal ),+) => { $( impl_core![@signed $t:$up:$dl]; )+ };
    (unsigned $( $t:ty : $up:ty : $dl:literal ),+) => { $( impl_core![@unsigned $t:$up:$dl]; )+ };

    // implements signed ops
    (@signed $t:ty : $up:ty : $dl:literal) => { paste! {
        #[doc = "# Integer core methods for `" $t "`\n\n"]
        #[doc = "- [abs](#method.abs" $dl ")"]
        #[doc = "- [is_even](#method.is_even" $dl ")"]
        #[doc = "- [is_odd](#method.is_odd" $dl ")"]
        #[doc = "- [gcd](#method.gcd" $dl ")"]
        #[doc = "- [gcd_ext](#method.gcd_ext" $dl ")"]
        #[doc = "- [gcd_ext_euc](#method.gcd_ext_euc" $dl ")"]
        #[doc = "- [lcm](#method.lcm" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /// Returns the absolute value of `self`.
            #[inline] #[must_use]
            pub const fn abs(self) -> $t { self.0.abs() }

            /// Returns `true` if `self` is an even number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(2_" $t ").is_even()];"]
            #[doc = "assert![Int(-2_" $t ").is_even()];"]
            #[doc = "assert![!Int(3_" $t ").is_even()];"]
            #[doc = "assert![Int(0_" $t ").is_even()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn is_even(self) -> bool { self.0 & 1 == 0 }

            /// Returns `true` if `self` is an odd number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(3_" $t ").is_odd()];"]
            #[doc = "assert![Int(-3_" $t ").is_odd()];"]
            #[doc = "assert![!Int(2_" $t ").is_odd()];"]
            #[doc = "assert![!Int(0_" $t ").is_odd()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn is_odd(self) -> bool { self.0 & 1 == 1 }

            /* signed gcd, lcm */

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
            ///
            /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(4), Int(64_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(4), Int(-64_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(4), Int(64_" $t ").gcd(-36)];"]
            #[doc = "assert_eq![Int(4), Int(-64_" $t ").gcd(-36)];"]
            #[doc = "assert_eq![Int(36), Int(0_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(64), Int(64_" $t ").gcd(0)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn gcd(self, b: $t) -> Int<$t> {
                let [mut a, mut b] = [self.0.abs(), b.abs()];
                iif![a == 0; return Int(b)];
                iif![b == 0; return Int(a)];
                // Let k be the greatest power of 2 dividing both a and b:
                let k = (a | b).trailing_zeros();
                // Divide a and b by 2 until they become odd:
                a >>= a.trailing_zeros();
                b >>= b.trailing_zeros();
                // Break when a == GCD of a / 2^k:
                while b != 0 {
                    b >>= b.trailing_zeros();
                    // ensure b >= a before substraction:
                    iif![a > b; {let swp = a; a = b; b = swp }; b -= a];
                }
                Int(a << k)

                // Euclid's algorithm:
                // while a != b { iif![a > b; a -= b; b -= a] }; a
            }

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>
            /// and the Bézout coeficients.
            ///
            /// This version uses the extended Stein's algorithm which is much more
            /// efficient to compute than Euclid's. It uses only simple arithmetic
            /// operations and works by dividing the inputs by 2 until they are odd,
            /// and then subtracting the smaller number from the larger one.
            ///
            /// There's no unsigned version of this function, since the coeficients can be negative.
            ///
            /// The Bézout's coefficients are not unique, and different algorithms
            /// can yield different coefficients that all satisfy Bézout's identity.
            ///
            /// Bézout's identity states that for any two integers a and b,
            /// there exist integers x and y such that $ax + by = gcd(a, b)$.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "let [gcd, x, y] = Int(32_" $t ").gcd_ext(36);"]
            /// assert_eq!(gcd.0, 4);
            /// assert_eq!(x.0 * 32 + y.0 * 36, gcd.0);
            /// ```
            #[inline] #[must_use]
            pub const fn gcd_ext(self, b: $t) -> [Int<$t>; 3] {
                let [mut a, mut b] = [self.0.abs(), b.abs()];
                if a == 0 { return [Int(b), Int(0), Int(1)]; }
                if b == 0 { return [Int(a), Int(1), Int(0)]; }

                let mut k = 0;
                while ((a | b) & 1) == 0 {
                    a >>= 1;
                    b >>= 1;
                    k += 1;
                }
                let (a0, b0, mut sa, mut sb, mut ta, mut tb) = (a, b, 1, 0, 0, 1);

                while (a & 1) == 0 {
                    if (sa & 1) != 0 || (sb & 1) != 0 {
                        sa -= b0;
                        sb += a0;
                    }
                    a >>= 1;
                    sa >>= 1;
                    sb >>= 1;
                }
                while b != 0 {
                    while (b & 1) == 0 {
                        if (ta & 1) != 0 || (tb & 1) != 0 {
                            ta -= b0;
                            tb += a0;
                        }
                        b >>= 1;
                        ta >>= 1;
                        tb >>= 1;
                    }
                    if a > b {
                        let swp = a; a = b; b = swp;
                        let swp = sa; sa = ta; ta = swp;
                        let swp = sb; sb = tb; tb = swp;
                    }
                    b -= a;
                    ta -= sa;
                    tb -= sb;
                }
                [Int(a << k), Int(sa), Int(sb)]
            }

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>
            /// and the Bézout coeficients.
            ///
            /// This version uses the extended Euclids's algorithm, which uses a
            /// series of euclidean divisions and works by subtracting multiples
            /// of the smaller number from the larger one.
            ///
            /// There's no unsigned version of this function, since the coeficients can be negative.
            ///
            /// The Bézout's coefficients are not unique, and different algorithms
            /// can yield different coefficients that all satisfy Bézout's identity.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "let [gcd, x, y] = Int(32_" $t ").gcd_ext_euc(36);"]
            /// assert_eq!(gcd.0, 4);
            /// assert_eq!(x.0 * 32 + y.0 * 36, gcd.0);
            /// ```
            #[inline] #[must_use]
            pub const fn gcd_ext_euc(self, b: $t) -> [Int<$t>; 3] {
                let a = self.0;
                if a == 0 {
                    [Int(b), Int(0), Int(1)]
                } else {
                    let [g, x, y] = Int(b % a).gcd_ext_euc(a);
                    [g, Int(y.0 - (b / a) * x.0), x] // IMPROVE impl ops
                }
            }

            /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            /// # Errors
            /// Can [`Overflow`].
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(12_" $t ").lcm(15), Ok(Int(60))];"]
            #[doc = "assert_eq![Int(-12_" $t ").lcm(15), Ok(Int(60))];"]
            #[doc = "assert_eq![Int(12_" $t ").lcm(-15), Ok(Int(60))];"]
            /// ```
            #[inline]
            pub const fn lcm(self, b: $t) -> Result<Int<$t>> {
                let (aup, bup) = (self.0 as $up, b as $up);
                let res = (aup * bup).abs() / self.gcd(b).0 as $up;
                iif![res <= $t::MAX as $up; Ok(Int(res as $t)); Err(Overflow)]
            }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $up:ty : $dl:literal) => { paste! {
        #[doc = "# Integer core methods for `" $t "`\n\n"]
        #[doc = "- [abs](#method.abs" $dl ")"]
        #[doc = "- [is_even](#method.is_even" $dl ")"]
        #[doc = "- [is_odd](#method.is_odd" $dl ")"]
        #[doc = "- [gcd](#method.gcd" $dl ")"]
        #[doc = "- [lcm](#method.lcm" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /// Returns the absolute value of `self` (no-op).
            #[inline] #[must_use]
            pub const fn abs(self) -> $t { self.0 }

            /// Returns `true` if `self` is an even number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(2_" $t ").is_even()];"]
            #[doc = "assert![!Int(3_" $t ").is_even()];"]
            #[doc = "assert![Int(0_" $t ").is_even()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn is_even(self) -> bool { self.0 & 1 == 0 }

            /// Returns `true` if `self` is an odd number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(3_" $t ").is_odd()];"]
            #[doc = "assert![!Int(2_" $t ").is_odd()];"]
            #[doc = "assert![!Int(0_" $t ").is_odd()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn is_odd(self) -> bool { self.0 & 1 == 1 }

            /* unsigned gcd, lcm */

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
            ///
            /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(4), Int(64_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(36), Int(0_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(64), Int(64_" $t ").gcd(0)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn gcd(self, mut b: $t) -> Int<$t> {
                let mut a = self.0;
                iif![a == 0; return Int(b)];
                iif![b == 0; return self];
                // Let k be the greatest power of 2 dividing both a and b:
                let k = (a | b).trailing_zeros();
                // Divide a and b by 2 until they become odd:
                a >>= a.trailing_zeros();
                b >>= b.trailing_zeros();
                // Break when a == GCD of a / 2^k:
                while b != 0 {
                    b >>= b.trailing_zeros();
                    // ensure b >= a before substraction:
                    iif![a > b; {let swp = a; a = b; b = swp }; b -= a];
                }
                Int(a << k)

                // Euclid's algorithm:
                // while a != b { iif![a > b; a -= b; b -= a] }; a
            }

            /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            /// # Errors
            /// Can [`Overflow`].
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(12_" $t ").lcm(15), Ok(Int(60))];"]
            /// ```
            #[inline]
            pub const fn lcm(self, b: $t) -> Result<Int<$t>> {
                let (aup, bup) = (self.0 as $up, b as $up);
                let res = aup * bup / self.gcd(b).0 as $up;
                iif![res <= $t::MAX as $up; Ok(Int(res as $t)); Err(Overflow)]
            }
        }
    }};
}
impl_core![signed
i8:i16:"", i16:i32:"-1", i32:i64:"-2", i64:i128:"-3", i128:i128:"-4", isize:isize:"-5"];
impl_core![unsigned
u8:u16:"-6", u16:u32:"-7", u32:u64:"-8", u64:u128:"-9", u128:u128:"-10", usize:usize:"-11"];
