// devela::math::fns::div_sqrt
//
//! Functions for numeric operations.
//
// TOC
// - sint|uint:
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

use crate::meta::{iif, paste};

// $t:   the input/output type
macro_rules! impl_ops {
    (signed $( $t:ty ),+) => { $( impl_ops![@signed $t]; )+ };
    (unsigned $( $t:ty ),+) => { $( impl_ops![@unsigned $t]; )+ };

    // implements signed ops
    (@signed $t:ty ) => { paste! {
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
        #[doc = "use devela::math::div_ceil_" $t ";"]
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
        #[doc = "use devela::math::div_floor_" $t ";"]
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
        #[doc = "use devela::math::div_half_away_" $t ";"]
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
        #[doc = "use devela::math::div_half_towards_" $t ";"]
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
        #[doc = "use devela::math::div_half_even_" $t ";"]
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
        #[doc = "use devela::math::div_half_odd_" $t ";"]
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
        #[doc="use devela::math::square_is_" $t ";"]
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
        #[doc="use devela::math::sqrt_floor_" $t ";"]
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
        #[doc="use devela::math::sqrt_ceil_" $t ";"]
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
        #[doc="use devela::math::sqrt_round_" $t ";"]
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
    }};

    // implements unsigned ops
    (@unsigned $t:ty) => { paste! {
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
        #[doc = "use devela::math::div_ceil_" $t ";"]
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
        #[doc = "use devela::math::div_floor_" $t ";"]
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
        #[doc = "use devela::math::div_half_away_" $t ";"]
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
        #[doc = "use devela::math::div_half_towards_" $t ";"]
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
        #[doc = "use devela::math::div_half_even_" $t ";"]
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
        #[doc = "use devela::math::div_half_odd_" $t ";"]
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
        #[doc="use devela::math::square_is_" $t ";"]
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
        #[doc="use devela::math::sqrt_floor_" $t ";"]
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
        #[doc="use devela::math::sqrt_ceil_" $t ";"]
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
        #[doc="use devela::math::sqrt_round_" $t ";"]
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
    }};
}
impl_ops![signed i8, i16, i32, i64, i128, isize];
impl_ops![unsigned u8, u16, u32, u64, u128, usize];
