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

use crate::{iif, paste, Int};

#[doc = crate::doc_private!()]
/// $t:   the input/output type
/// $cap: the capability feature that enables the given implementation. E.g "_int_i8".
/// $d:  the doclink suffix for the method name
macro_rules! impl_int {
    () => {
        impl_int![signed
            i8:"_int_i8":"", i16:"_int_i16":"-1", i32:"_int_i32":"-2",
            i64:"_int_i64":"-3", i128:"_int_i128":"-4", isize:"_int_isize":"-5"
        ];
        impl_int![unsigned
            u8:"_int_u8":"-6", u16:"_int_u16":"-7", u32:"_int_u32":"-8",
            u64:"_int_u64":"-9", u128:"_int_u128":"-10", usize:"_int_usize":"-11"
        ];
    };

    (signed $( $t:ty : $cap:literal : $d:literal ),+) => {
        $( impl_int![@signed $t:$cap:$d]; )+
    };
    (unsigned $( $t:ty : $cap:literal : $d:literal ),+) => {
        $( impl_int![@unsigned $t:$cap:$d]; )+
    };

    // implements signed ops
    (@signed $t:ty : $cap:literal : $d:literal) => { paste! {
        #[doc = crate::doc_availability!(feature = $cap)]
        ///
        #[doc = "# Integer division related methods for `" $t "`\n\n"]
        #[doc = "- [div_rem](#method.div_rem" $d ")"]
        #[doc = "- [div_ceil](#method.div_ceil" $d ")"]
        #[doc = "- [div_floor](#method.div_floor" $d ")"]
        #[doc = "- [div_ties_away](#method.div_ties_away" $d ")"]
        #[doc = "- [div_ties_towards](#method.div_ties_towards" $d ")"]
        #[doc = "- [div_ties_even](#method.div_ties_even" $d ")"]
        #[doc = "- [div_ties_odd](#method.div_ties_odd" $d ")"]
        ///
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Int<$t> {
            /// Returns the truncated quotient and the remainder.
            #[must_use]
            pub const fn div_rem(self, b: $t) -> [Int<$t>; 2] {
                let a = self.0; [Int(a / b), Int(a % b)]
            }

            /// Returns the quotient, rounding the result towards positive infinity.
            /// # Notation
            /// $$ \large \left\lceil \frac{x}{y} \right\rceil $$
            /// # Examples
            /// ```
            /// # use devela::Int;
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
            // WAIT: [int_roundings](https://github.com/rust-lang/rust/issues/88581)
            #[must_use]
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
            /// # use devela::Int;
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
            // WAIT: [int_roundings](https://github.com/rust-lang/rust/issues/88581)
            #[must_use]
            pub const fn div_floor(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![(r > 0 && b < 0) || (r < 0 && b > 0); Int(d - 1); Int(d)]
            }

            /// Returns the quotient, rounding ties away from zero.
            /// # Examples
            /// ```
            /// # use devela::Int;
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
            #[must_use]
            pub const fn div_ties_away(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![2 * r.abs() >= b.abs();
                    iif![(a > 0) == (b > 0); Int(d + 1); Int(d - 1)]; Int(d)]
            }

            /// Returns the quotient, rounding ties towards zero.
            /// # Examples
            /// ```
            /// # use devela::Int;
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
            #[must_use]
            pub const fn div_ties_towards(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![2 * r.abs() > b.abs();
                    iif![(a > 0) == (b > 0); Int(d + 1); Int(d - 1)]; Int(d)]
            }

            /// Returns the quotient, rounding ties to the nearest even number.
            /// # Examples
            /// ```
            /// # use devela::Int;
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
            #[must_use]
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
            /// # use devela::Int;
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
            #[must_use]
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
    (@unsigned $t:ty : $cap:literal : $d:literal) => { paste! {
        #[doc = crate::doc_availability!(feature = $cap)]
        ///
        #[doc = "# Integer division related methods for `" $t "`\n\n"]
        #[doc = "- [div_rem](#method.div_rem" $d ")"]
        #[doc = "- [div_ceil](#method.div_ceil" $d ")"]
        #[doc = "- [div_floor](#method.div_floor" $d ")"]
        #[doc = "- [div_ties_away](#method.div_ties_away" $d ")"]
        #[doc = "- [div_ties_towards](#method.div_ties_towards" $d ")"]
        #[doc = "- [div_ties_even](#method.div_ties_even" $d ")"]
        #[doc = "- [div_ties_odd](#method.div_ties_odd" $d ")"]
        ///
        #[cfg(feature = $cap )]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Int<$t> {
            /* unsigned division */

            /// Returns the truncated quotient and the remainder.
            #[must_use]
            pub const fn div_rem(self, b: $t) -> [Int<$t>; 2] {
                let a = self.0; [Int(a / b), Int(a % b)]
            }

            /// Returns the quotient, rounding the result towards positive infinity.
            /// # Notation
            /// $$ \large \left\lceil \frac{x}{y} \right\rceil $$
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ceil(3), Int(3)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ceil(5), Int(2)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ceil(4), Int(2)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ceil(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ceil(2), Int(3)]; // == 2.5"]
            /// ```
            // unstable rust implementation for signed integers:
            // WAIT: [int_roundings](https://github.com/rust-lang/rust/issues/88581)
            #[must_use]
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
            /// # use devela::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_floor(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_floor(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_floor(4), Int(1)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_floor(5), Int(1)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_floor(2), Int(2)]; // == 2.5"]
            /// ```
            // unstable rust implementation for signed integers:
            // WAIT: [int_roundings](https://github.com/rust-lang/rust/issues/88581)
            #[must_use]
            pub const fn div_floor(self, b: $t) -> Int<$t> {
                Int(self.0 / b)
            }

            /// Returns the quotient, rounding ties away from zero.
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_away(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_away(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_away(4), Int(2)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_away(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_away(2), Int(3)]; // == 2.5"]
            /// ```
            #[must_use]
            pub const fn div_ties_away(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![2 * r >= b; iif![a > b; Int(d + 1); Int(d - 1)]; Int(d)]
            }

            /// Returns the quotient, rounding ties towards zero.
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_towards(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_towards(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_towards(4), Int(1)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_towards(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_towards(2), Int(2)]; // == 2.5"]
            /// ```
            #[must_use]
            pub const fn div_ties_towards(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                iif![2 * r > b; Int(d + 1); Int(d)]
            }

            /// Returns the quotient, rounding ties to the nearest even number.
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_even(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_even(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_even(4), Int(2)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_even(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_even(2), Int(2)]; // == 2.5"]
            /// ```
            #[must_use]
            pub const fn div_ties_even(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                // 1. If the remainder is zero or less than half of b, return the quotient.
                // 2. If the remainder is greater than half of b, return the quotient + 1.
                // 3. If the quotient is _EVEN_ return it, otherwise return the quotient + 1.
                iif![r == 0 || 2 * r < b; Int(d);
                    iif![2 * r > b; Int(d + 1);
                        iif![d % 2 == 0; Int(d); Int(d + 1)]]]
            }

            /// Returns the quotient, rounding ties to the nearest even number.
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(7_" $t ").div_ties_odd(3), Int(2)]; // == 2.33…"]
            ///
            #[doc = "assert_eq![Int(7_" $t ").div_ties_odd(5), Int(1)]; // == 1.4"]
            #[doc = "assert_eq![Int(6_" $t ").div_ties_odd(4), Int(1)]; // == 1.5"]
            #[doc = "assert_eq![Int(8_" $t ").div_ties_odd(5), Int(2)]; // == 1.6"]
            #[doc = "assert_eq![Int(5_" $t ").div_ties_odd(2), Int(3)]; // == 2.5"]
            /// ```
            #[must_use]
            pub const fn div_ties_odd(self, b: $t) -> Int<$t> {
                let a = self.0; let (d, r) = (a / b, a % b);
                // 1. If the remainder is zero or less than half of b, return the quotient.
                // 2. If the remainder is greater than half of b, return the quotient + 1.
                // 3. If the quotient is _ODD_ return it, otherwise return the quotient + 1.
                iif![r == 0 || 2 * r < b; Int(d);
                    iif![2 * r > b; Int(d + 1);
                        iif![d % 2 != 0; Int(d); Int(d + 1)]]]
            }
        }
    }};
}
impl_int!();
