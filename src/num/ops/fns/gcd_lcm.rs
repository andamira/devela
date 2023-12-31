// devela::num::fns::gcd_lcm
//
//! greatest common divisor and least common multiple functions
//
// TOC
// - sint|uint
//   - gcd
//   - gcd_ext
//   - gcd_ext_euc
//   - lcm

use crate::code::{iif, paste};

// signed|unsigned
// $t:   the input/output type
// $up:  the upcasted type to do the operations on (for lcm)
macro_rules! impl_ops {
    (signed $( ($t:ty, $up:ty) ),+) => { $( impl_ops![@signed($t, $up)]; )+ };
    (unsigned $( ($t:ty, $up:ty) ),+) => { $( impl_ops![@unsigned($t, $up)]; )+ };

    // implements signed ops
    (@signed($t:ty, $up:ty) ) => { paste! {
        /* signed gcd, lcm */

        #[doc=r#"Returns the <abbr title="Greatest Common Divisor">GCD</abbr> of two [`"# $t "`]."]
        ///
        /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::gcd_" $t ";\n\n"]
        #[doc = "assert_eq![gcd_" $t "(64, 36), 4];"]
        #[doc ="assert_eq![gcd_" $t "(-64, 36), 4];"]
        #[doc ="assert_eq![gcd_" $t "(64, -36), 4];"]
        #[doc ="assert_eq![gcd_" $t "(-64, -36), 4];"]
        #[doc = "assert_eq![gcd_" $t "(0, 36), 36];"]
        #[doc = "assert_eq![gcd_" $t "(64, 0), 64];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<gcd_ $t >](a: $t, b: $t) -> $t {
            let [mut a, mut b] = [a.abs(), b.abs()];
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

        #[doc=r#"Returns the <abbr title="Greatest Common Divisor">GCD</abbr> of two
            [`"# $t "`], and the Bézout coeficients."]
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
        #[doc ="use devela::num::gcd_ext_" $t ";\n\n"]
        #[doc = "let [gcd, x, y] = gcd_ext_" $t "(32, 36);"]
        #[doc = "assert_eq!(gcd, 4);"]
        #[doc = "assert_eq!(x * 32 + y * 36, gcd);"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<gcd_ext_ $t>](a: $t, b: $t) -> [$t; 3] {
            let [mut a, mut b] = [a.abs(), b.abs()];
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

        #[doc=r#"Returns the <abbr title="Greatest Common Divisor">GCD</abbr> of two
            [`"# $t "`], and the Bézout coeficients."]
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
        #[doc ="use devela::num::gcd_ext_euc_" $t ";\n\n"]
        #[doc = "let [gcd, x, y] = gcd_ext_euc_" $t "(32, 36);"]
        #[doc = "assert_eq!(gcd, 4);"]
        #[doc = "assert_eq!(x * 32 + y * 36, gcd);"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<gcd_ext_euc_ $t>](a: $t, b: $t) -> [$t; 3] {
            if a == 0 {
                [b, 0, 1]
            } else {
                let [g, x, y] = [<gcd_ext_euc_ $t>](b % a, a);
                [g, y - (b / a) * x, x]
            }
        }

        #[doc = r#"Returns the <abbr title="Least Common Multiple">LCM</abbr> of two [`"# $t "`]."]
        ///
        /// Returns `None` if the result would overflow.
        ///
        #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
        /// # Examples
        /// ```
        #[doc ="use devela::num::lcm_" $t ";\n\n"]
        #[doc = "assert_eq![lcm_" $t "(12, 15), Some(60)];"]
        #[doc = "assert_eq![lcm_" $t "(-12, 15), Some(60)];"]
        #[doc = "assert_eq![lcm_" $t "(12, -15), Some(60)];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<lcm_ $t >](a: $t, b: $t) -> Option<$t> {
            let (aup, bup) = (a as $up, b as $up);
            let res = (aup * bup).abs() / [<gcd_ $t>](a, b) as $up;
            iif![res <= $t::MAX as $up; Some(res as $t); None]
        }
    }};

    // implements unsigned ops
    (@unsigned($t:ty, $up:ty) ) => { paste! {
        /* unsigned gcd, lcm */

        #[doc=r#"Returns the <abbr title="Greatest Common Divisor">GCD</abbr> of two [`"# $t "`]."]
        ///
        /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::gcd_" $t ";\n\n"]
        #[doc = "assert_eq![gcd_" $t "(64, 36), 4];"]
        #[doc = "assert_eq![gcd_" $t "(0, 36), 36];"]
        #[doc = "assert_eq![gcd_" $t "(64, 0), 64];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<gcd_ $t >](mut a: $t, mut b: $t) -> $t {
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

        #[doc = r#"Returns the <abbr title="Least Common Multiple">LCM</abbr> of two [`"# $t "`]."]
        ///
        #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
        /// # Examples
        /// ```
        #[doc ="use devela::num::lcm_" $t ";\n\n"]
        #[doc = "assert_eq![lcm_" $t "(12, 15), Some(60)];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<lcm_ $t >](a: $t, b: $t) -> Option<$t> {
            let (aup, bup) = (a as $up, b as $up);
            let res = aup * bup / [<gcd_ $t>](a, b) as $up;
            iif![res <= $t::MAX as $up; Some(res as $t); None]
        }
    }};
}
impl_ops![
    signed(i8, i16),
    (i16, i32),
    (i32, i64),
    (i64, i128),
    (i128, i128),
    (isize, isize)
];
impl_ops![
    unsigned(u8, u16),
    (u16, u32),
    (u32, u64),
    (u64, u128),
    (u128, u128),
    (usize, usize)
];
