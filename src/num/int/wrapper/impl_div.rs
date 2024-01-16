// devela::num::int::wrapper::impl_div
//
//! implements division related functions
//
// TOC
// - signed|unsigned:
//   - div_rem
//   - div_ceil
//   - div_floor
//   - div_ties_away
//   - div_ties_towards
//   - div_ties_even
//   - div_ties_odd

use super::Int;
use crate::code::{iif, paste};

// $t:   the input/output type
// $dl:  the doclink suffix for the method name
macro_rules! impl_div {
    (signed $( $t:ty : $dl:literal ),+) => { $( impl_div![@signed $t:$dl]; )+ };
    (unsigned $( $t:ty : $dl:literal ),+) => { $( impl_div![@unsigned $t:$dl]; )+ };

    // implements signed ops
    (@signed $t:ty : $dl:literal) => { paste! {
        /* signed division */

        #[doc = "# Integer division related methods for `" $t "`\n\n"]
        #[doc = "- [div_rem](#method.div_rem" $dl ")"]
        #[doc = "- [div_ceil](#method.div_ceil" $dl ")"]
        #[doc = "- [div_floor](#method.div_floor" $dl ")"]
        #[doc = "- [div_ties_away](#method.div_ties_away" $dl ")"]
        #[doc = "- [div_ties_towards](#method.div_ties_towards" $dl ")"]
        #[doc = "- [div_ties_even](#method.div_ties_even" $dl ")"]
        #[doc = "- [div_ties_odd](#method.div_ties_odd" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /// Returns the truncated quotient and the remainder.
            #[inline] #[must_use]
            pub const fn div_rem(self, b: $t) -> [Int<$t>; 2] {
                let a = self.0; [Int(a / b), Int(a % b)]
            }

            /// Returns the quotient, rounding the result towards positive infinity.
            /// # Notation
            /// $$ \large \left\lceil \frac{x}{y} \right\rceil $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ceil(3), Int(3)]; // == 2.33…"]
            #[doc = "assert_eq![Int(7_" $t ").div_ceil(-3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ceil(3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ceil(-3), Int(3)];"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ceil(5), Int(2)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ceil(4), Int(2)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ceil(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ceil(2), Int(3)]; // == 2.5"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ceil(5), Int(-1)]; // == -1.4"]
            #[doc = "assert_eq![Int(-6_" $t ").div_ceil(4), Int(-1)]; // == -1.5"]
            #[doc = "assert_eq![Int(-8_" $t ").div_ceil(5), Int(-1)]; // == -1.6"]
            #[doc = "assert_eq![Int(-5_" $t ").div_ceil(2), Int(-2)]; // == -2.5"]
            /// ```
            // unstable rust implementation for signed integers:
            // WAITING: https://github.com/rust-lang/rust/issues/88581
            #[inline] #[must_use]
            pub const fn div_ceil(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![(r > 0 && b > 0) || (r < 0 && b < 0); Int(d + 1); Int(d)]
            }
            // alternative implementation:
            // pub const fn div_ceil(self, b: $t) -> $t {
            //     let a = self.0; iif![a > 0 && b > 0; ((a - 1) / b) + 1 ; a / b ]
            // }

            /// Returns the quotient, rounding the result towards negative infinity.
            /// # Notation
            /// $$ \large \left\lfloor \frac{x}{y} \right\rfloor $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_floor(3), Int(2)]; // == 2.33…"]
            #[doc = "assert_eq![Int(7_" $t ").div_floor(-3), Int(-3)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_floor(3), Int(-3)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_floor(-3), Int(2)];"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_floor(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_floor(4), Int(1)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_floor(5), Int(1)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_floor(2), Int(2)]; // == 2.5"]
            #[doc = "assert_eq![Int(-7_" $t ").div_floor(5), Int(-2)]; // == -1.4"]
            #[doc = "assert_eq![Int(-6_" $t ").div_floor(4), Int(-2)]; // == -1.5"]
            #[doc = "assert_eq![Int(-8_" $t ").div_floor(5), Int(-2)]; // == -1.6"]
            #[doc = "assert_eq![Int(-5_" $t ").div_floor(2), Int(-3)]; // == -2.5"]
            /// ```
            // unstable rust implementation for signed integers:
            // WAITING: https://github.com/rust-lang/rust/issues/88581
            #[inline] #[must_use]
            pub const fn div_floor(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![(r > 0 && b < 0) || (r < 0 && b > 0); Int(d - 1); Int(d)]
            }

            /// Returns the quotient, rounding ties away from zero.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_away(3), Int(2)]; // == 2.33…"]
            #[doc = "assert_eq![Int(7_" $t ").div_ties_away(-3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_away(3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_away(-3), Int(2)];"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_away(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_away(4), Int(2)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_away(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_away(2), Int(3)]; // == 2.5"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_away(5), Int(-1)]; // == -1.4"]
            #[doc = "assert_eq![Int(-6_" $t ").div_ties_away(4), Int(-2)]; // == -1.5"]
            #[doc = "assert_eq![Int(-8_" $t ").div_ties_away(5), Int(-2)]; // == -1.6"]
            #[doc = "assert_eq![Int(-5_" $t ").div_ties_away(2), Int(-3)]; // == -2.5"]
            /// ```
            #[inline] #[must_use]
            pub const fn div_ties_away(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![2 * r.abs() >= b.abs();
                    iif![(a > 0) == (b > 0); Int(d + 1); Int(d - 1)]; Int(d)]
            }

            /// Returns the quotient, rounding ties towards zero.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_towards(3), Int(2)]; // == 2.33…"]
            #[doc = "assert_eq![Int(7_" $t ").div_ties_towards(-3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_towards(3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_towards(-3), Int(2)];"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_towards(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_towards(4), Int(1)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_towards(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_towards(2), Int(2)]; // == 2.5"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_towards(5), Int(-1)]; // == -1.4"]
            #[doc = "assert_eq![Int(-6_" $t ").div_ties_towards(4), Int(-1)]; // == -1.5"]
            #[doc = "assert_eq![Int(-8_" $t ").div_ties_towards(5), Int(-2)]; // == -1.6"]
            #[doc = "assert_eq![Int(-5_" $t ").div_ties_towards(2), Int(-2)]; // == -2.5"]
            /// ```
            #[inline] #[must_use]
            pub const fn div_ties_towards(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![2 * r.abs() > b.abs();
                    iif![(a > 0) == (b > 0); Int(d + 1); Int(d - 1)]; Int(d)]
            }

            /// Returns the quotient, rounding ties to the nearest even number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_even(3), Int(2)]; // == 2.33…"]
            #[doc = "assert_eq![Int(7_" $t ").div_ties_even(-3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_even(3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_even(-3), Int(2)];"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_even(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_even(4), Int(2)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_even(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_even(2), Int(2)]; // == 2.5"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_even(5), Int(-1)]; // == -1.4"]
            #[doc = "assert_eq![Int(-6_" $t ").div_ties_even(4), Int(-2)]; // == -1.5"]
            #[doc = "assert_eq![Int(-8_" $t ").div_ties_even(5), Int(-2)]; // == -1.6"]
            #[doc = "assert_eq![Int(-5_" $t ").div_ties_even(2), Int(-2)]; // == -2.5"]
            /// ```
            #[inline] #[must_use]
            pub const fn div_ties_even(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                // If the remainder is zero or the |remainder| is less than half of
                // |b|, return the quotient.
                iif![r == 0 || 2 * r.abs() < b.abs(); Int(d);
                    // If the |remainder| is greater than half of b,
                    // return the quotient + the sign of a × the sign of b.
                    iif![2 * r.abs() > b.abs(); Int(d + a.signum() * b.signum());
                        // If the quotient is even return it, otherwise return
                        // the quotient + the sign of a × the sign of b.
                        iif![d % 2 == 0; Int(d); Int(d + a.signum() * b.signum())] ] ]
            }

            /// Returns the quotient, rounding ties to the nearest odd number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_odd(3), Int(2)]; // == 2.33…"]
            #[doc = "assert_eq![Int(7_" $t ").div_ties_odd(-3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_odd(3), Int(-2)];"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_odd(-3), Int(2)];"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_odd(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_odd(4), Int(1)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_odd(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_odd(2), Int(3)]; // == 2.5"]
            #[doc = "assert_eq![Int(-7_" $t ").div_ties_odd(5), Int(-1)]; // == -1.4"]
            #[doc = "assert_eq![Int(-6_" $t ").div_ties_odd(4), Int(-1)]; // == -1.5"]
            #[doc = "assert_eq![Int(-8_" $t ").div_ties_odd(5), Int(-2)]; // == -1.6"]
            #[doc = "assert_eq![Int(-5_" $t ").div_ties_odd(2), Int(-3)]; // == -2.5"]
            /// ```
            #[inline] #[must_use]
            pub const fn div_ties_odd(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                // If the remainder is zero or the |remainder| is less than half of
                // |b|, return the quotient.
                iif![r == 0 || 2 * r.abs() < b.abs(); Int(d);
                    // If the |remainder| is greater than half of b,
                    // return the quotient + the sign of a × the sign of b.
                    iif![2 * r.abs() > b.abs(); Int(d + a.signum() * b.signum());
                        // If the quotient is odd return it, otherwise return
                        // the quotient + the sign of a × the sign of b.
                        iif![d % 2 != 0; Int(d); Int(d + a.signum() * b.signum())] ] ]
            }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $dl:literal) => { paste! {
        #[doc = "# Integer division related methods for `" $t "`\n\n"]
        #[doc = "- [div_rem](#method.div_rem" $dl ")"]
        #[doc = "- [div_ceil](#method.div_ceil" $dl ")"]
        #[doc = "- [div_floor](#method.div_floor" $dl ")"]
        #[doc = "- [div_ties_away](#method.div_ties_away" $dl ")"]
        #[doc = "- [div_ties_towards](#method.div_ties_towards" $dl ")"]
        #[doc = "- [div_ties_even](#method.div_ties_even" $dl ")"]
        #[doc = "- [div_ties_odd](#method.div_ties_odd" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /* unsigned division */

            /// Returns the truncated quotient and the remainder.
            #[inline] #[must_use]
            pub const fn div_rem(self, b: $t) -> [Int<$t>; 2] {
                let a = self.0; [Int(a / b), Int(a % b)]
            }

            /// Returns the quotient, rounding the result towards positive infinity.
            /// # Notation
            /// $$ \large \left\lceil \frac{x}{y} \right\rceil $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ceil(3), Int(3)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ceil(5), Int(2)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ceil(4), Int(2)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ceil(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ceil(2), Int(3)]; // == 2.5"]
            /// ```
            // unstable rust implementation for signed integers:
            // WAITING: https://github.com/rust-lang/rust/issues/88581
            #[inline] #[must_use]
            pub const fn div_ceil(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![r > 0 && b > 0; Int(d + 1); Int(d)]
            }
            // alternative implementation:
            // pub const fn div_ceil(self, b: $t) -> $t {
            //     let a = self.0; iif![a > 0 && b > 0; ((a - 1) / b) + 1; a / b ]
            // }

            /// Returns the quotient, rounding the result towards negative infinity.
            /// # Notation
            /// $$ \large \left\lfloor \frac{x}{y} \right\rfloor $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_floor(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_floor(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_floor(4), Int(1)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_floor(5), Int(1)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_floor(2), Int(2)]; // == 2.5"]
            /// ```
            // unstable rust implementation for signed integers:
            // WAITING: https://github.com/rust-lang/rust/issues/88581
            #[inline] #[must_use]
            pub const fn div_floor(self, b: $t) -> Int<$t> {
                Int(self.0 / b)
            }

            /// Returns the quotient, rounding ties away from zero.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_away(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_away(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_away(4), Int(2)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_away(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_away(2), Int(3)]; // == 2.5"]
            /// ```
            #[inline] #[must_use]
            pub const fn div_ties_away(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![2 * r >= b; iif![a > b; Int(d + 1); Int(d - 1)]; Int(d)]
            }

            /// Returns the quotient, rounding ties towards zero.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_towards(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_towards(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_towards(4), Int(1)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_towards(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_towards(2), Int(2)]; // == 2.5"]
            /// ```
            #[inline] #[must_use]
            pub const fn div_ties_towards(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![2 * r > b; Int(d + 1); Int(d)]
            }

            /// Returns the quotient, rounding ties to the nearest even number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_even(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_even(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_even(4), Int(2)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_even(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_even(2), Int(2)]; // == 2.5"]
            /// ```
            #[inline] #[must_use]
            pub const fn div_ties_even(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                // 1. If the remainder is zero or less than half of b, return the quotient.
                // 2. If the remainder is greater than half of b, return the quotient + 1.
                // 3. If the quotient is even return it, otherwise return the quotient + 1.
                iif![r == 0 || 2 * r < b; Int(d);
                    iif![2 * r > b; Int(d + 1);
                        iif![d % 2 == 0; Int(d); Int(d + 1)]]]
            }

            /// Returns the quotient, rounding ties to the nearest even number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_odd(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_odd(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_odd(4), Int(1)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_odd(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_odd(2), Int(3)]; // == 2.5"]
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_ties_odd(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                // 1. If the remainder is zero or less than half of b, return the quotient.
                // 2. If the remainder is greater than half of b, return the quotient + 1.
                // 3. If the quotient is odd return it, otherwise return the quotient + 1.
                iif![r == 0 || 2 * r < b; Int(d);
                    iif![2 * r > b; Int(d + 1);
                        iif![d % 2 != 0; Int(d); Int(d + 1)]]]
            }
        }
    }};
}
impl_div![signed i8:"", i16:"-1", i32:"-2", i64:"-3", i128:"-4", isize:"-5"];
impl_div![unsigned u8:"-6", u16:"-7", u32:"-8", u64:"-9", u128:"-10", usize:"-11"];