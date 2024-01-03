// devela::num::int::wrapper::impl_gcd_lcm
//
//! implements gcd & lcm related functions
//
// TOC
// - signed:
//   - gcd
//   - gcd_ext
//   - gcd_ext_euc
//   - lcm
// - unsigned:
//   - gcd
//   - lcm

use super::Int;
use crate::code::{iif, paste};

// $t:   the input/output type
// $up:  the upcasted type to do the operations on (for lcm)
// $dl:  the doclink suffix for the method name
macro_rules! impl_gcd_lcm {
    (signed $( $t:ty : $up:ty : $dl:literal ),+) => {
        $( impl_gcd_lcm![@signed $t:$up:$dl]; )+
    };
    (unsigned $( $t:ty : $up:ty : $dl:literal ),+) => {
        $( impl_gcd_lcm![@unsigned $t:$up:$dl]; )+
    };

    // implements signed ops
    (@signed $t:ty : $up:ty : $dl:literal) => { paste! {
        /* signed count_digits */

        #[doc = "# Numeric <abbr title='Greatest Common Divisor'>GCD</abbr> and "
        "<abbr title='Least Common Multiple'>LCM</abbr> related methods for `" $t "`\n\n"]
        #[doc = "- [gcd](#method.gcd" $dl ")"]
        #[doc = "- [gcd_ext](#method.gcd_ext" $dl ")"]
        #[doc = "- [gcd_ext_euc](#method.gcd_ext_euc" $dl ")"]
        #[doc = "- [lcm](#method.lcm" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /* signed gcd, lcm */

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
            ///
            /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![4, Int(64_" $t ").gcd(36)];"]
            #[doc = "assert_eq![4, Int(-64_" $t ").gcd(36)];"]
            #[doc = "assert_eq![4, Int(64_" $t ").gcd(-36)];"]
            #[doc = "assert_eq![4, Int(-64_" $t ").gcd(-36)];"]
            #[doc = "assert_eq![36, Int(0_" $t ").gcd(36)];"]
            #[doc = "assert_eq![64, Int(64_" $t ").gcd(0)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn gcd(self, b: $t) -> $t {
                let [mut a, mut b] = [self.0.abs(), b.abs()];
                iif![a == 0; return b];
                iif![b == 0; return a];
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
                a << k

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
            /// assert_eq!(gcd, 4);
            /// assert_eq!(x * 32 + y * 36, gcd);
            /// ```
            #[inline] #[must_use]
            pub const fn gcd_ext(self, b: $t) -> [$t; 3] {
                let [mut a, mut b] = [self.0.abs(), b.abs()];
                if a == 0 { return [b, 0, 1]; }
                if b == 0 { return [a, 1, 0]; }

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

                [a << k, sa, sb]
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
            /// assert_eq!(gcd, 4);
            /// assert_eq!(x * 32 + y * 36, gcd);
            /// ```
            #[inline] #[must_use]
            pub const fn gcd_ext_euc(self, b: $t) -> [$t; 3] {
                if self.0 == 0 {
                    [b, 0, 1]
                } else {
                    let [g, x, y] = Int(b % self.0).gcd_ext_euc(self.0);
                    [g, y - (b / self.0) * x, x]
                }
            }

            /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
            ///
            /// Returns `None` if the result would overflow.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(12_" $t ").lcm(15), Some(60)];"]
            #[doc = "assert_eq![Int(-12_" $t ").lcm(15), Some(60)];"]
            #[doc = "assert_eq![Int(12_" $t ").lcm(-15), Some(60)];"]
            /// ```
            // TODO:CHECK overflow for i128
            #[inline] #[must_use]
            pub const fn lcm(self, b: $t) -> Option<$t> {
                let (aup, bup) = (self.0 as $up, b as $up);
                let res = (aup * bup).abs() / self.gcd(b) as $up;
                iif![res <= $t::MAX as $up; Some(res as $t); None]
            }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $up:ty : $dl:literal) => { paste! {
        #[doc = "# Numeric <abbr title='Greatest Common Divisor'>GCD</abbr> and "
        "<abbr title='Least Common Multiple'>LCM</abbr> related methods for `" $t "`\n\n"]
        #[doc = "- [gcd](#method.gcd" $dl ")"]
        #[doc = "- [gcd_ext](#method.gcd_ext" $dl ")"]
        #[doc = "- [gcd_ext_euc](#method.gcd_ext_euc" $dl ")"]
        #[doc = "- [lcm](#method.lcm" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /* unsigned gcd, lcm */

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
            ///
            /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![4, Int(64_" $t ").gcd(36)];"]
            #[doc = "assert_eq![36, Int(0_" $t ").gcd(36)];"]
            #[doc = "assert_eq![64, Int(64_" $t ").gcd(0)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn gcd(mut self, mut b: $t) -> $t {
                iif![self.0 == 0; return b];
                iif![b == 0; return self.0];
                // Let k be the greatest power of 2 dividing both self.0 and b:
                let k = (self.0 | b).trailing_zeros();
                // Divide a and b by 2 until they become odd:
                self.0 >>= self.0.trailing_zeros();
                b >>= b.trailing_zeros();
                // Break when self.0 == GCD of self.0 / 2^k:
                while b != 0 {
                    b >>= b.trailing_zeros();
                    // ensure b >= self.0 before substraction:
                    iif![self.0 > b; {let swp = self.0; self.0 = b; b = swp }; b -= self.0];
                }
                self.0 << k

                // Euclid's algorithm:
                // while a != b { iif![a > b; a -= b; b -= a] }; a
            }

            /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(12_" $t ").lcm(15), Some(60)];"]
            /// ```
            // TODO:CHECK overflow for u128
            #[inline] #[must_use]
            pub const fn lcm(self, b: $t) -> Option<$t> {
                let (aup, bup) = (self.0 as $up, b as $up);
                let res = aup * bup / self.gcd(b) as $up;
                iif![res <= $t::MAX as $up; Some(res as $t); None]
            }
        }
    }};
}
impl_gcd_lcm![signed
i8:i16:"", i16:i32:"-1", i32:i64:"-2", i64:i128:"-3", i128:i128:"-4", isize:isize:"-5"];
impl_gcd_lcm![unsigned
u8:u16:"-6", u16:u32:"-7", u32:u64:"-8", u64:u128:"-9", u128:u128:"-10", usize:usize:"-11"];
