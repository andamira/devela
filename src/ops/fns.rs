// devela::ops::fns
//
//! Functions for numeric operations.
//
// TOC
// - sint & uint
//   - gcd
//   - lcm
//   - div_rem
//   - div_ceil
//   - div_floor
//   - div_half_away
//   - div_half_towards
//   - div_half_even
//   - div_half_odd
//   - square_is
//   - sqrt_floor
//   - sqrt_ceil
//   - sqrt_round
//   - scale
//   - lerp
//
// - floating-point
//   - scale
//   - lerp

use crate::meta::{iif, paste};

// signed|unsigned
// $t:   the input/output type
// $ut:  the upcasted type to do the operations on (the ones that can overflow)
// $ft:  the floating-point type to do the operations on (for lerp)
macro_rules! impl_ops {
    (signed $( ($t:ty, $up:ty, $ft:ty) ),+) => { $( impl_ops![@signed($t, $up, $ft)]; )+ };
    (unsigned $( ($t:ty, $up:ty, $ft:ty) ),+) => { $( impl_ops![@unsigned($t, $up, $ft)]; )+ };
    (float $($t:ty ),+) => { $( impl_ops![@float($t)]; )+ };

    // implements signed ops
    (@signed($t:ty, $up:ty, $ft:ty) ) => { paste! {
        /* signed gcd, lcm */

        #[doc=r#"Returns the <abbr title="Greatest Common Divisor">GCD</abbr> of two [`"# $t "`]."]
        ///
        /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::gcd_" $t ";\n\n"]
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

        #[doc = r#"Returns the <abbr title="Least Common Multiple">LCM</abbr> of two [`"# $t "`]."]
        ///
        /// Returns `None` if the result would overflow.
        ///
        #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
        /// # Examples
        /// ```
        #[doc ="use devela::ops::lcm_" $t ";\n\n"]
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

        /* signed div */

        #[doc = "Returns an [` " $t " `] truncated quotient, and the remainder."]
        #[inline]
        #[must_use]
        pub const fn [<div_rem_ $t >](a: $t, b: $t) -> ($t, $t) {
            (a / b, a % b)
        }

        #[doc = "Returns an [` " $t " `] quotient, rounding the result towards positive infinity."]
        ///
        /// # Notation
        /// $$ \large \left\lceil \frac{x}{y} \right\rceil $$
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_ceil_" $t ";"]
        #[doc = "assert_eq![div_ceil_" $t "(7, 3), 3]; // 2.33…"]
        #[doc = "assert_eq![div_ceil_" $t "(7, -3), -2];"]
        #[doc = "assert_eq![div_ceil_" $t "(-7, 3), -2];"]
        #[doc = "assert_eq![div_ceil_" $t "(-7, -3), 3];"]
        ///
        #[doc = "assert_eq![div_ceil_" $t "(7, 5), 2]; // 1.4"]
        #[doc = "assert_eq![div_ceil_" $t "(6, 4), 2]; // 1.5"]
        #[doc = "assert_eq![div_ceil_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_ceil_" $t "(5, 2), 3]; // 2.5"]
        #[doc = "assert_eq![div_ceil_" $t "(-7, 5), -1]; // -1.4"]
        #[doc = "assert_eq![div_ceil_" $t "(-6, 4), -1]; // -1.5"]
        #[doc = "assert_eq![div_ceil_" $t "(-8, 5), -1]; // -1.6"]
        #[doc = "assert_eq![div_ceil_" $t "(-5, 2), -2]; // 2.5"]
        /// ```
        // unstable rust implementation for signed integers:
        // WAITING: https://github.com/rust-lang/rust/issues/88581
        #[inline]
        #[must_use]
        pub const fn [<div_ceil_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            iif![(r > 0 && b > 0) || (r < 0 && b < 0); d + 1; d]
        }
        // alternative implementation:
        // #[inline]
        // #[must_use]
        // pub const fn [<div_ceil_ $t _alt>](a: $t, b: $t) -> $t {
        //     iif![a > 0 && b > 0; ((a - 1) / b) + 1 ; a / b ]
        // }

        #[doc = "Returns an [` " $t " `] quotient, rounding the result towards negative infinity."]
        ///
        /// # Notation
        /// $$ \large \left\lfloor \frac{x}{y} \right\rfloor $$
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_floor_" $t ";"]
        #[doc = "assert_eq![div_floor_" $t "(7, 3), 2]; // 2.33…"]
        #[doc = "assert_eq![div_floor_" $t "(7, -3), -3];"]
        #[doc = "assert_eq![div_floor_" $t "(-7, 3), -3];"]
        #[doc = "assert_eq![div_floor_" $t "(-7, -3), 2];"]
        ///
        #[doc = "assert_eq![div_floor_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_floor_" $t "(6, 4), 1]; // 1.5"]
        #[doc = "assert_eq![div_floor_" $t "(8, 5), 1]; // 1.6"]
        #[doc = "assert_eq![div_floor_" $t "(5, 2), 2]; // 2.5"]
        #[doc = "assert_eq![div_floor_" $t "(-7, 5), -2]; // -1.4"]
        #[doc = "assert_eq![div_floor_" $t "(-6, 4), -2]; // -1.5"]
        #[doc = "assert_eq![div_floor_" $t "(-8, 5), -2]; // -1.6"]
        #[doc = "assert_eq![div_floor_" $t "(-5, 2), -3]; // -2.5"]
        /// ```
        // unstable rust implementation for signed integers:
        // WAITING: https://github.com/rust-lang/rust/issues/88581
        #[inline]
        #[must_use]
        pub const fn [<div_floor_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            iif![(r > 0 && b < 0) || (r < 0 && b > 0); d - 1; d]
        }

        /* div_half_away */

        #[doc = "Returns an [`" $t "`] quotient, rounding half away from zero."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_half_away_" $t ";"]
        #[doc = "assert_eq![div_half_away_" $t "(7, 3), 2]; // 2.33…"]
        #[doc = "assert_eq![div_half_away_" $t "(7, -3), -2];"]
        #[doc = "assert_eq![div_half_away_" $t "(-7, 3), -2];"]
        #[doc = "assert_eq![div_half_away_" $t "(-7, -3), 2];"]
        ///
        #[doc = "assert_eq![div_half_away_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_half_away_" $t "(6, 4), 2]; // 1.5"]
        #[doc = "assert_eq![div_half_away_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_half_away_" $t "(5, 2), 3]; // 2.5"]
        #[doc = "assert_eq![div_half_away_" $t "(-7, 5), -1]; // -1.4"]
        #[doc = "assert_eq![div_half_away_" $t "(-6, 4), -2]; // -1.5"]
        #[doc = "assert_eq![div_half_away_" $t "(-8, 5), -2]; // -1.6"]
        #[doc = "assert_eq![div_half_away_" $t "(-5, 2), -3]; // -2.5"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<div_half_away_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            iif![2 * r.abs() >= b.abs(); iif![(a > 0) == (b > 0); d + 1; d - 1]; d]
        }

        #[doc = "Returns an [`" $t "`] quotient, rounding half towards zero."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_half_towards_" $t ";"]
        #[doc = "assert_eq![div_half_towards_" $t "(7, 3), 2]; // 2.33…"]
        #[doc = "assert_eq![div_half_towards_" $t "(7, -3), -2];"]
        #[doc = "assert_eq![div_half_towards_" $t "(-7, 3), -2];"]
        #[doc = "assert_eq![div_half_towards_" $t "(-7, -3), 2];"]
        ///
        #[doc = "assert_eq![div_half_towards_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_half_towards_" $t "(6, 4), 1]; // 1.5"]
        #[doc = "assert_eq![div_half_towards_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_half_towards_" $t "(5, 2), 2]; // 2.5"]
        #[doc = "assert_eq![div_half_towards_" $t "(-7, 5), -1]; // -1.4"]
        #[doc = "assert_eq![div_half_towards_" $t "(-6, 4), -1]; // -1.5"]
        #[doc = "assert_eq![div_half_towards_" $t "(-8, 5), -2]; // -1.6"]
        #[doc = "assert_eq![div_half_towards_" $t "(-5, 2), -2]; // -2.5"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<div_half_towards_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            iif![2 * r.abs() > b.abs(); iif![(a > 0) == (b > 0); d + 1; d - 1]; d]
        }

        #[doc = "Returns an [`" $t "`] quotient, rounding half to the nearest odd number."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_half_even_" $t ";"]
        #[doc = "assert_eq![div_half_even_" $t "(7, 3), 2]; // 2.33…"]
        #[doc = "assert_eq![div_half_even_" $t "(7, -3), -2];"]
        #[doc = "assert_eq![div_half_even_" $t "(-7, 3), -2];"]
        #[doc = "assert_eq![div_half_even_" $t "(-7, -3), 2];"]
        ///
        #[doc = "assert_eq![div_half_even_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_half_even_" $t "(6, 4), 2]; // 1.5"]
        #[doc = "assert_eq![div_half_even_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_half_even_" $t "(5, 2), 2]; // 2.5"]
        #[doc = "assert_eq![div_half_even_" $t "(-7, 5), -1]; // -1.4"]
        #[doc = "assert_eq![div_half_even_" $t "(-6, 4), -2]; // -1.5"]
        #[doc = "assert_eq![div_half_even_" $t "(-8, 5), -2]; // -1.6"]
        #[doc = "assert_eq![div_half_even_" $t "(-6, 4), -2]; // -2.5"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<div_half_even_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            // If the remainder is zero or the |remainder| is less than half of
            // |b|, return the quotient.
            iif![r == 0 || 2 * r.abs() < b.abs(); d;
                // If the |remainder| is greater than half of b,
                // return the quotient + the sign of a × the sign of b.
                iif![2 * r.abs() > b.abs(); d + a.signum() * b.signum();
                    // If the quotient is even return it, otherwise return
                    // the quotient + the sign of a × the sign of b.
                    iif![d % 2 == 0; d; d + a.signum() * b.signum()] ] ]
        }

        #[doc = "Returns an [`" $t "`] quotient, rounding half to the nearest odd number."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_half_odd_" $t ";"]
        #[doc = "assert_eq![div_half_odd_" $t "(7, 3), 2]; // 2.33…"]
        #[doc = "assert_eq![div_half_odd_" $t "(7, -3), -2];"]
        #[doc = "assert_eq![div_half_odd_" $t "(-7, 3), -2];"]
        #[doc = "assert_eq![div_half_odd_" $t "(-7, -3), 2];"]
        ///
        #[doc = "assert_eq![div_half_odd_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_half_odd_" $t "(6, 4), 1]; // 1.5"]
        #[doc = "assert_eq![div_half_odd_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_half_odd_" $t "(5, 2), 3]; // 2.5"]
        #[doc = "assert_eq![div_half_odd_" $t "(-7, 5), -1]; // -1.4"]
        #[doc = "assert_eq![div_half_odd_" $t "(-6, 4), -1]; // -1.5"]
        #[doc = "assert_eq![div_half_odd_" $t "(-8, 5), -2]; // -1.6"]
        #[doc = "assert_eq![div_half_odd_" $t "(-5, 2), -3]; // -2.5"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<div_half_odd_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            // If the remainder is zero or the |remainder| is less than half of
            // |b|, return the quotient.
            iif![r == 0 || 2 * r.abs() < b.abs(); d;
                // If the |remainder| is greater than half of b,
                // return the quotient + the sign of a × the sign of b.
                iif![2 * r.abs() > b.abs(); d + a.signum() * b.signum();
                    // If the quotient is odd return it, otherwise return
                    // the quotient + the sign of a × the sign of b.
                    iif![d % 2 != 0; d; d + a.signum() * b.signum()] ] ]
        }

        /* signed sqrt */

        #[doc = "Returns `true` if an [`" $t "`] is a perfect square."]
        ///
        /// Returns `false` otherwise, which includes all negative values.
        ///
        /// # Algorithm
        /// $$ \large
        /// \text{is\textunderscore square}(n) = \begin{cases}
        /// \text{true} & \text{if } \left(\lfloor \sqrt{n} \rfloor\right)^2 = n \cr
        /// \text{false} & \text{if } \left(\lfloor \sqrt{n} \rfloor\right)^2 \neq n
        /// \end{cases}
        /// $$
        ///
        /// # Examples
        /// ```
        #[doc="use devela::ops::square_is_" $t ";"]
        ///
        #[doc="assert_eq![square_is_" $t "(12), false];"]
        #[doc="assert_eq![square_is_" $t "(13), false];"]
        #[doc="assert_eq![square_is_" $t "(16), true];"]
        #[doc="assert_eq![square_is_" $t "(20), false];"]
        #[doc="assert_eq![square_is_" $t "(21), false];"]
        #[doc="assert_eq![square_is_" $t "(-16), false];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<square_is_ $t>](a: $t) -> bool {
            iif![let Some(sqrt) = [<sqrt_floor_ $t>](a); sqrt * sqrt == a; false]
        }

        #[doc = "Returns an [`" $t "`] floored integer square root."]
        ///
        /// Returns `None` if `a` is negative.
        ///
        /// # Algorithm
        /// $$ \large \left\lfloor \sqrt{x} \right\rfloor = n_{k} $$
        ///
        /// Where $n_{k}$ is the result of a sequence of estimates that
        /// starts with an initial $n_{0} = x/2$ which is updated using
        /// [*Heron's method*](
        /// https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method):
        ///
        /// $$ \large
        /// n_{i+1} = n_{i} - ( n_{i}^{2} - x) / 2n_{i},
        /// \quad \small\text{for} \quad i = 0, 1, \ldots, k,
        /// $$
        ///
        /// Where $n_{i}$ is the current estimate, $n_{i+1}$ is the next
        /// estimate, $x$ is self, and $k$ is the number of iterations
        /// needed to converge to a solution, on the order of the number of
        /// bits of self, about $O(\log_2 b)$, which for e.g. 128 bits would
        /// be $ ±7 $ iterations.
        ///
        /// Hence, the function continues updating the estimate until
        /// reaching $n_{k}$, which provides the largest integer less than
        /// or equal to the square root of `x`.
        ///
        /// # Examples
        /// ```
        #[doc="use devela::ops::sqrt_floor_" $t ";"]
        ///
        #[doc="assert_eq![sqrt_floor_" $t "(12), Some(3)];"]
        #[doc="assert_eq![sqrt_floor_" $t "(13), Some(3)];"]
        #[doc="assert_eq![sqrt_floor_" $t "(16), Some(4)];"]
        #[doc="assert_eq![sqrt_floor_" $t "(20), Some(4)];"]
        #[doc="assert_eq![sqrt_floor_" $t "(21), Some(4)];"]
        #[doc="assert_eq![sqrt_floor_" $t "(-16), None];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sqrt_floor_ $t >](a: $t) -> Option<$t> {
            if a.is_negative() {
                None
            } else if a < 2 {
                Some(a)
            } else {
                let (mut x, mut y) = (a, (a + a / a) / 2);
                while y < x {
                    x = y;
                    y = (x + a / x) / 2;
                }
                Some(x)
            }
        }

        #[doc = "Returns an [`" $t "`] ceiled integer square root."]
        ///
        /// Returns `None` if `a` is negative.
        ///
        /// # Algorithm
        /// $$ \large
        /// \begin{align}
        /// \notag \left\lceil \sqrt{x} \thinspace\right\rceil = \begin{cases}
        /// n & \text{if } n^2 = x \cr
        /// n+1 & \text{if } n^2 < x \end{cases} \cr
        /// \notag \normalsize\text{where } n = \lfloor \sqrt{x} \rfloor &
        /// \end{align}
        /// $$
        ///
        /// # Examples
        /// ```
        #[doc="use devela::ops::sqrt_ceil_" $t ";"]
        ///
        #[doc="assert_eq![sqrt_ceil_" $t "(12), Some(4)];"]
        #[doc="assert_eq![sqrt_ceil_" $t "(13), Some(4)];"]
        #[doc="assert_eq![sqrt_ceil_" $t "(16), Some(4)];"]
        #[doc="assert_eq![sqrt_ceil_" $t "(20), Some(5)];"]
        #[doc="assert_eq![sqrt_ceil_" $t "(21), Some(5)];"]
        #[doc="assert_eq![sqrt_ceil_" $t "(-16), None];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sqrt_ceil_ $t>](a: $t) -> Option<$t> {
            if let Some(floor) = [<sqrt_floor_ $t>](a) {
                iif![floor * floor == a; Some(floor); Some(floor + 1)]
            } else {
                None
            }
        }

        #[doc = "Returns an [`" $t "`] rounded integer square root."]
        ///
        /// Returns `None` if `a` is negative.
        ///
        /// # Algorithm
        /// $$ \large
        /// \begin{align}
        /// \notag \left\lfloor\sqrt{x} \thinspace\right\rceil = \begin{cases}
        /// n & \text{if } x - n^2 < (n+1)^2 - x \cr
        /// n+1 & \text{if } x - n^2 \geq (n+1)^2 - x \end{cases} \cr
        /// \notag \normalsize\text{where } n = \lfloor \sqrt{x} \rfloor &
        /// \end{align}
        /// $$
        ///
        /// # Examples
        /// ```
        #[doc="use devela::ops::sqrt_round_" $t ";"]
        ///
        #[doc="assert_eq![sqrt_round_" $t "(12), Some(3)];"]
        #[doc="assert_eq![sqrt_round_" $t "(13), Some(4)];"]
        #[doc="assert_eq![sqrt_round_" $t "(16), Some(4)];"]
        #[doc="assert_eq![sqrt_round_" $t "(20), Some(4)];"]
        #[doc="assert_eq![sqrt_round_" $t "(21), Some(5)];"]
        #[doc="assert_eq![sqrt_round_" $t "(-16), None];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sqrt_round_ $t>](a: $t) -> Option<$t> {
            if a.is_negative() {
                None
            } else if a < 2 {
                Some(a)
            } else {
                // sqrt_floor
                let (mut x, mut y) = (a, (a + a / a) / 2);
                while y < x {
                    x = y;
                    y = (x + a / x) / 2;
                }
                // do we have to round up?
                iif![a - x * x >= (x + 1) * (x + 1) - a; Some(x + 1); Some(x)]
            }
        }

        /* signed scale */

        #[doc = "Returns a scaled [`" $t
            "`] `v`alue between `[min..=max]` to a new range `[a..=b]`.\n\n"]
        #[doc = "It upcasts internally to [`" $up "`] for the intermediate operations."]
        ///
        /// # Formula
        /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::scale_" $t ";\n\n"]
        #[doc = "assert_eq![scale_" $t "(60, 0, 120, 30, 50), 40];"]
        #[doc = "assert_eq![scale_" $t "(60, 0, 120, 30, 50), 40];"]
        /// ```
        pub const fn [<scale_ $t>](v: $t, min: $t, max: $t, a: $t, b: $t) -> $t {
            let (v, min, max, a, b) = (v as $up, min as $up, max as $up, a as $up, b as $up);
            ((b - a) * (v - min) / (max - min) + a) as $t
        }

        #[doc = "Returns an interpolated [`" $t "`] between `[a..=b]` with an [`" $ft
            "`] `pct` between `[0..=1]`.\n\n"]
        ///
        #[doc ="You can also use [`scale_" $t "`] for the same purpose."]
        /// Integer operations can have more precision for very large values.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::{lerp_" $t ", scale_" $t "};\n\n"]
        #[doc = "assert_eq![lerp_" $t "(0.5, 40, 80), 60];"]
        ///
        /// // equivalence using integer scaling:
        #[doc = "assert_eq![scale_" $t "(50, 0, 100, 40, 80), 60];"]
        /// ```
        pub fn [<lerp_ $t>](pct: $ft, a: $t, b: $t) -> $t {
            ((1.0 - pct) * (a as $ft) + pct * (b as $ft)) as $t
        }
    }};

    // implements unsigned ops
    (@unsigned($t:ty, $up:ty, $ft:ty) ) => { paste! {
        /* unsigned gcd, lcm */

        #[doc=r#"Returns the <abbr title="Greatest Common Divisor">GCD</abbr> of two [`"# $t "`]."]
        ///
        /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::gcd_" $t ";\n\n"]
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
        #[doc ="use devela::ops::lcm_" $t ";\n\n"]
        #[doc = "assert_eq![lcm_" $t "(12, 15), Some(60)];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<lcm_ $t >](a: $t, b: $t) -> Option<$t> {
            let (aup, bup) = (a as $up, b as $up);
            let res = aup * bup / [<gcd_ $t>](a, b) as $up;
            iif![res <= $t::MAX as $up; Some(res as $t); None]
        }

        /* unsigned div */

        #[doc = "Returns a [` " $t " `] truncated quotient, and the remainder."]
        #[inline]
        #[must_use]
        pub const fn [<div_rem_ $t >](a: $t, b: $t) -> ($t, $t) {
            (a / b, a % b)
        }

        #[doc = "Returns an [`" $t
            "`] quotient, rounding the result towards positive infinity."]
        ///
        /// # Notation
        /// $$ \large \left\lceil \frac{x}{y} \right\rceil $$
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_ceil_" $t ";"]
        #[doc = "assert_eq![div_ceil_" $t "(7, 3), 3]; // 2.33…"]
        ///
        #[doc = "assert_eq![div_ceil_" $t "(7, 5), 2]; // 1.4"]
        #[doc = "assert_eq![div_ceil_" $t "(6, 4), 2]; // 1.5"]
        #[doc = "assert_eq![div_ceil_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_ceil_" $t "(5, 2), 3]; // 2.5"]
        /// ```
        // rust implementation for unsigned integers, stabilized in 1.73.0:
        #[inline]
        #[must_use]
        pub const fn [<div_ceil_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            iif![r > 0 && b > 0; d + 1; d]
        }
        // alternative implementation:
        // #[inline]
        // #[must_use]
        // pub const fn [<div_ceil $t _alt>](a: u16, b: u16) -> u16 {
        //     iif![a > 0 && b > 0; ((a - 1) / b) + 1; a / b]
        // }

        #[doc = "Returns a [`" $t
            "`] quotient, rounding the result towards negative infinity."]
        ///
        /// This is the same as performing `a / b` for all unsigned integers.
        ///
        /// # Notation
        /// $$ \large \left\lfloor \frac{x}{y} \right\rfloor $$
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_floor_" $t ";"]
        #[doc = "assert_eq![div_floor_" $t "(7, 3), 2]; // 2.33…"]
        ///
        #[doc = "assert_eq![div_floor_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_floor_" $t "(6, 4), 1]; // 1.5"]
        #[doc = "assert_eq![div_floor_" $t "(8, 5), 1]; // 1.6"]
        #[doc = "assert_eq![div_floor_" $t "(5, 2), 2]; // 2.5"]
        /// ```
        // unstable rust implementation for unsigned integers.
        #[inline]
        #[must_use]
        pub const fn [<div_floor_ $t >](a: $t, b: $t) -> $t {
            a / b
        }

        #[doc = "Returns a [`" $t "`] quotient, rounding half away from zero."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_half_away_" $t ";"]
        #[doc = "assert_eq![div_half_away_" $t "(7, 3), 2]; // 2.33…"]
        ///
        #[doc = "assert_eq![div_half_away_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_half_away_" $t "(6, 4), 2]; // 1.5"]
        #[doc = "assert_eq![div_half_away_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_half_away_" $t "(5, 2), 3]; // 2.5"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<div_half_away_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            iif![2 * r >= b; iif![a > b; d + 1; d - 1]; d]
        }

        #[doc = "Returns a [`" $t "`] quotient, rounding half towards from zero."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_half_towards_" $t ";"]
        #[doc = "assert_eq![div_half_towards_" $t "(7, 3), 2]; // 2.33…"]
        ///
        #[doc = "assert_eq![div_half_towards_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_half_towards_" $t "(6, 4), 1]; // 1.5"]
        #[doc = "assert_eq![div_half_towards_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_half_towards_" $t "(5, 2), 2]; // 2.5"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<div_half_towards_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            iif![2 * r > b; d + 1; d]
        }

        #[doc = "Returns a [`" $t "`] quotient, rounding half to the nearest even number."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_half_even_" $t ";"]
        #[doc = "assert_eq![div_half_even_" $t "(7, 3), 2]; // 2.33…"]
        ///
        #[doc = "assert_eq![div_half_even_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_half_even_" $t "(6, 4), 2]; // 1.5"]
        #[doc = "assert_eq![div_half_even_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_half_even_" $t "(5, 2), 2]; // 2.5"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<div_half_even_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            // 1. If the remainder is zero or less than half of b, return the quotient.
            // 2. If the remainder is greater than half of b, return the quotient + 1.
            // 3. If the quotient is even return it, otherwise return the quotient + 1.
            iif![r == 0 || 2 * r < b; d; iif![2 * r > b; d + 1; iif![d % 2 == 0; d; d + 1]]]
        }

        #[doc = "Returns a [`" $t "`] quotient, rounding half to the nearest odd number."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::div_half_odd_" $t ";"]
        #[doc = "assert_eq![div_half_odd_" $t "(7, 3), 2]; // 2.33…"]
        ///
        #[doc = "assert_eq![div_half_odd_" $t "(7, 5), 1]; // 1.4"]
        #[doc = "assert_eq![div_half_odd_" $t "(6, 4), 1]; // 1.5"]
        #[doc = "assert_eq![div_half_odd_" $t "(8, 5), 2]; // 1.6"]
        #[doc = "assert_eq![div_half_odd_" $t "(5, 2), 3]; // 2.5"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<div_half_odd_ $t >](a: $t, b: $t) -> $t {
            let (d, r) = (a / b, a % b);
            // 1. If the remainder is zero or less than half of b, return the quotient.
            // 2. If the remainder is greater than half of b, return the quotient + 1.
            // 3. If the quotient is odd return it, otherwise return the quotient + 1.
            iif![r == 0 || 2 * r < b; d; iif![2 * r > b; d + 1; iif![d % 2 != 0; d; d + 1]]]
        }

        /* unsigned sqrt */

        #[doc = "Returns `true` if a [`" $t "`] is a perfect square."]
        ///
        /// Returns `false` otherwise, which includes all negative values.
        ///
        /// # Algorithm
        /// $$ \large
        /// \text{is\textunderscore square}(n) = \begin{cases}
        /// \text{true} & \text{if } \left(\lfloor \sqrt{n} \rfloor\right)^2 = n \cr
        /// \text{false} & \text{if } \left(\lfloor \sqrt{n} \rfloor\right)^2 \neq n
        /// \end{cases}
        /// $$
        ///
        /// # Examples
        /// ```
        #[doc="use devela::ops::square_is_" $t ";"]
        ///
        #[doc="assert_eq![square_is_" $t "(12), false];"]
        #[doc="assert_eq![square_is_" $t "(13), false];"]
        #[doc="assert_eq![square_is_" $t "(16), true];"]
        #[doc="assert_eq![square_is_" $t "(20), false];"]
        #[doc="assert_eq![square_is_" $t "(21), false];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<square_is_ $t>](a: $t) -> bool {
            let sqrt = [<sqrt_floor_ $t>](a);
            sqrt * sqrt == a
        }

        #[doc = "Returns a [`" $t "`] floored integer square root."]
        ///
        /// # Algorithm
        /// $$ \large \left\lfloor \sqrt{x} \right\rfloor = n_{k} $$
        ///
        /// Where $n_{k}$ is the result of a sequence of estimates that
        /// starts with an initial $n_{0} = x/2$ which is updated using
        /// [*Heron's method*](
        /// https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method):
        ///
        /// $$ \large
        /// n_{i+1} = n_{i} - ( n_{i}^{2} - x) / 2n_{i},
        /// \quad \small\text{for} \quad i = 0, 1, \ldots, k,
        /// $$
        ///
        /// Where $n_{i}$ is the current estimate, $n_{i+1}$ is the next
        /// estimate, $x$ is `a`, and $k$ is the number of iterations
        /// needed to converge to a solution, on the order of the number of
        /// bits of `a`, about $O(\log_2 b)$, which for e.g. 128 bits would
        /// be $ ±7 $ iterations.
        ///
        /// Hence, the function continues updating the estimate until
        /// reaching $n_{k}$, which provides the largest integer less than
        /// or equal to the square root of `x`.
        ///
        /// # Examples
        /// ```
        #[doc="use devela::ops::sqrt_floor_" $t ";"]
        ///
        #[doc="assert_eq![sqrt_floor_" $t "(12), 3];"]
        #[doc="assert_eq![sqrt_floor_" $t "(13), 3];"]
        #[doc="assert_eq![sqrt_floor_" $t "(16), 4];"]
        #[doc="assert_eq![sqrt_floor_" $t "(20), 4];"]
        #[doc="assert_eq![sqrt_floor_" $t "(21), 4];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sqrt_floor_ $t >](a: $t) -> $t {
            if a < 2 {
                a
            } else {
                let (mut x, mut y) = (a, (a + a / a) / 2);
                while y < x {
                    x = y;
                    y = (x + a / x) / 2;
                }
                x
            }
        }

        #[doc = "Returns an [`" $t "`] ceiled integer square root."]
        ///
        /// # Algorithm
        /// $$ \large
        /// \begin{align}
        /// \notag \left\lceil \sqrt{x} \thinspace\right\rceil = \begin{cases}
        /// n & \text{if } n^2 = x \cr
        /// n+1 & \text{if } n^2 < x \end{cases} \cr
        /// \notag \normalsize\text{where } n = \lfloor \sqrt{x} \rfloor &
        /// \end{align}
        /// $$
        ///
        /// # Examples
        /// ```
        #[doc="use devela::ops::sqrt_ceil_" $t ";"]
        ///
        #[doc="assert_eq![sqrt_ceil_" $t "(12), 4];"]
        #[doc="assert_eq![sqrt_ceil_" $t "(13), 4];"]
        #[doc="assert_eq![sqrt_ceil_" $t "(16), 4];"]
        #[doc="assert_eq![sqrt_ceil_" $t "(20), 5];"]
        #[doc="assert_eq![sqrt_ceil_" $t "(21), 5];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sqrt_ceil_ $t>](a: $t) -> $t {
            let floor = [<sqrt_floor_ $t>](a);
            iif![floor * floor == a; floor; floor + 1]
        }

        #[doc = "Returns an [`" $t "`] rounded integer square root."]
        ///
        /// # Algorithm
        /// $$ \large
        /// \begin{align}
        /// \notag \left\lfloor\sqrt{x} \thinspace\right\rceil = \begin{cases}
        /// n & \text{if } x - n^2 < (n+1)^2 - x \cr
        /// n+1 & \text{if } x - n^2 \geq (n+1)^2 - x \end{cases} \cr
        /// \notag \normalsize\text{where } n = \lfloor \sqrt{x} \rfloor &
        /// \end{align}
        /// $$
        ///
        /// # Examples
        /// ```
        #[doc="use devela::ops::sqrt_round_" $t ";"]
        ///
        #[doc="assert_eq![sqrt_round_" $t "(12), 3];"]
        #[doc="assert_eq![sqrt_round_" $t "(13), 4];"]
        #[doc="assert_eq![sqrt_round_" $t "(16), 4];"]
        #[doc="assert_eq![sqrt_round_" $t "(20), 4];"]
        #[doc="assert_eq![sqrt_round_" $t "(21), 5];"]
        /// ```
        #[must_use]
        pub const fn [<sqrt_round_ $t>](a: $t) -> $t {
            if a < 2 {
                a
            } else {
                // sqrt_floor
                let (mut x, mut y) = (a, (a + a / a) / 2);
                while y < x {
                    x = y;
                    y = (x + a / x) / 2;
                }
                // do we have to round up?
                iif![a - x * x >= (x + 1) * (x + 1) - a; x + 1; x]
            }
        }

        /* unsigned scale */

        #[doc = "Returns a scaled [`" $t
            "`] `v`alue between `[min..=max]` to a new range `[a..=b]`.\n\n"]
        #[doc = "It upcasts internally to [`" $up "`] for the intermediate operations."]
        ///
        /// # Formula
        /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::scale_" $t ";\n\n"]
        #[doc = "assert_eq![scale_" $t "(60, 0, 120, 30, 50), 40];"]
        /// ```
        pub const fn [<scale_ $t>](v: $t, min: $t, max: $t, a: $t, b: $t) -> $t {
            let (v, min, max, a, b) = (v as $up, min as $up, max as $up, a as $up, b as $up);
            ((b - a) * (v - min) / (max - min) + a) as $t
        }

        #[doc = "Returns an interpolated [`" $t "`] between `[a..=b]` with an [`" $ft
            "`] `pct` between `[0..=1]`.\n\n"]
        ///
        #[doc ="You can also use the [`scale_" $t "`] function for the same purpose,"]
        /// which can have more precision for large values.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::{lerp_" $t ", scale_" $t "};\n\n"]
        #[doc = "assert_eq![lerp_" $t "(0.5, 40, 80), 60];"]
        ///
        /// // equivalence using integer scaling:
        #[doc = "assert_eq![scale_" $t "(50, 0, 100, 40, 80), 60];"]
        /// ```
        pub fn [<lerp_ $t>](pct: $ft, a: $t, b: $t) -> $t {
            ((1.0 - pct) * (a as $ft) + pct * (b as $ft)) as $t
        }

        /* unsigned ratios */ // TODO:

        #[doc = r#"Returns the reduced [`"# $t "`] `r`atio."]
        /// # Examples
        /// ```
        #[doc ="use devela::ops::ratio_reduce_" $t ";\n\n"]
        #[doc = "assert_eq![ratio_reduce_" $t "(120, 42), [20, 7]];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<ratio_reduce_ $t >](a: $t, b: $t) -> [$t; 2] {
            let divisor = [<gcd_ $t>](a, b);
            [a / divisor, b / divisor]
        }
    }};

    (@float($t:ty) ) => { paste! {
        /* floating-point scale */

        #[doc = "Returns a scaled [` " $t
            " `] `v`alue between `[min..=max]` to a new range `[a..=b]`."]
        ///
        /// # Formula
        /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::ops::scale_" $t ";\n\n"]
        #[doc = "assert_eq![scale_" $t "(45., 0., 360., 0., 1.), 0.125];"]
        #[doc = "assert_eq![scale_" $t "(45., 0., 360., -1., 1.), -0.75];"]
        ///
        #[doc = "assert_eq![scale_" $t "(0.125, 0., 1., 0., 360.), 45.];"]
        #[doc = "assert_eq![scale_" $t "(-0.75, -1., 1., 0., 360.), 45.];"]
        /// ```
        #[inline]
        #[must_use]
        pub fn [<scale_ $t>](v: $t, min: $t, max: $t, a: $t, b: $t) -> $t {
            (b - a) * (v - min) / (max - min) + a
        }

        #[doc = "Returns an interpolated [`" $t
            "`] between `[a..=b]` with a `pct` between `[0..=1]`.\n\n"]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::lerp_" $t ";\n\n"]
        #[doc = "assert_eq![lerp_" $t "(0.5, 40., 80.), 60.];"]
        /// ```
        pub fn [<lerp_ $t>](pct: $t, a: $t, b: $t) -> $t {
            (1.0 - pct) * a + pct * b
        }
    }};

}
impl_ops![
    signed(i8, i16, f32),
    (i16, i32, f32),
    (i32, i64, f32),
    (i64, i128, f64),
    (i128, i128, f64),
    (isize, isize, fsize)
];
impl_ops![
    unsigned(u8, u16, f32),
    (u16, u32, f32),
    (u32, u64, f32),
    (u64, u128, f64),
    (u128, u128, f64),
    (usize, usize, fsize)
];
impl_ops![float f32, f64];

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
use crate::num::fsize;
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_ops![float fsize];
