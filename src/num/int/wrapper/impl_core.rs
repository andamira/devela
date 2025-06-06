// devela::num::int::wrapper::impl_core
//
//! Implements core methods for [`Int`].
//
// TOC
// - signed|unsigned
//   - abs
//   - is_even
//   - is_odd
//   - gcd
//   - gcd_ext
//   - gcd_ext_euc
//   - lcm
//   - scale
//   - scale_wrap
//   - midpoint

use super::super::shared_docs::*;
#[cfg(feature = "cast")]
use crate::Cast;
#[cfg(any(feature = "_int_isize", feature = "_int_usize"))]
use crate::isize_up;
#[cfg(feature = "_int_usize")]
use crate::usize_up;
#[allow(unused_imports)]
use crate::{GcdReturn, Int, NumError::Overflow, NumResult as Result, cswap, is, paste, unwrap};

/// Implements core methods for [`Int`].
///
/// # Args
/// $t:    the input/output type
/// $cap:  the capability feature enabling the given implementation. E.g "_int_u8"
///
/// $ut:   the unsigned type of the same size as $t (only for signed)
/// $ucap: the feature enabling some methods related to `$ut` (signed midpoint)
///
/// $up:   the upcasted type to do the operations on (for lcm). E.g u8
///
/// $iup:  the signed upcasted type for some methods (gcd_ext). E.g. i16 (only for unsigned)
/// $icap: the feature enabling some methods related to `$iup`. E.g "_int_i16" (only for unsigned)
///
/// $d:    the doclink suffix for the method name
macro_rules! impl_core {
    () => {
        impl_core![signed
            // $t :$cap         :$ut   :$ucap        |$up      |$d
            i8    :"_int_i8"    :u8    :"_int_u8"    |i16      |"",
            i16   :"_int_i16"   :u16   :"_int_u16"   |i32      |"-1",
            i32   :"_int_i32"   :u32   :"_int_u32"   |i64      |"-2",
            i64   :"_int_i64"   :u64   :"_int_u64"   |i128     |"-3",
            i128  :"_int_i128"  :u128  :"_int_u128"  |i128     |"-4",
            isize :"_int_isize" :usize :"_int_usize" |isize_up |"-5"
        ];
        impl_core![unsigned
            // $t :$cap         :$up      |$iup     :$iucap          |$d
            u8    :"_int_u8"    :u16      |i16      :"_int_i16"      |"-6",
            u16   :"_int_u16"   :u32      |i32      :"_int_i32"      |"-7",
            u32   :"_int_u32"   :u64      |i64      :"_int_i64"      |"-8",
            u64   :"_int_u64"   :u128     |i128     :"_int_i128"     |"-9",
            u128  :"_int_u128"  :u128     |i128     :"_int_i128"     |"-10"
          //usize :"_int_usize" :usize_up |isize_up :"_int_isize_up" |"-11"]; // MAYBE
        ];
        #[cfg(target_pointer_width = "32")]
        impl_core![unsigned usize :"_int_usize" :usize_up |isize_up :"_int_i64"  |"-11"];
        #[cfg(target_pointer_width = "64")]
        impl_core![unsigned usize :"_int_usize" :usize_up |isize_up :"_int_i128" |"-11"];
    };
    (signed $( $t:ty : $cap:literal : $ut:ty : $ucap:literal | $up:ty |$d:literal ),+) => {
        $( impl_core![@signed   $t :$cap :$ut :$ucap :$up |$d]; )+
    };
    (unsigned $( $t:ty : $cap:literal : $up:ty | $iup:ty : $icap:literal |$d:literal ),+) => {
        $( impl_core![@unsigned $t :$cap :$up |$iup :$icap |$d]; )+
    };
    (
    // implements signed ops
    @signed $t:ty : $cap:literal : $ut:ty : $ucap:literal : $up:ty |$d:literal) => { paste! {
        #[doc = crate::doc_availability!(feature = $cap)]
        ///
        #[doc = "# Integer core methods for `" $t "`\n\n"]
        #[doc = "- [abs](#method.abs" $d ")"]
        #[doc = "- [is_even](#method.is_even" $d ")"]
        #[doc = "- [is_odd](#method.is_odd" $d ")"]
        #[doc = "- [gcd](#method.gcd" $d ")"]
        #[doc = "- [gcd_ext](#method.gcd_ext" $d ")"]
        #[doc = "- [gcd_ext_euc](#method.gcd_ext_euc" $d ")"]
        #[doc = "- [lcm](#method.lcm" $d ")"]
        #[doc = "- [scale](#method.scale" $d ")"]
        #[doc = "- [scale_wrap](#method.scale_wrap" $d ")"]
        #[doc = "- [midpoint](#method.midpoint" $d ")"]
        ///
        #[cfg(feature = $cap )]
        impl Int<$t> {
            /// Returns the absolute value of `self`.
            pub const fn abs(self) -> Int<$t> { Int(self.0.abs()) }

            /// Returns `true` if `self` is an even number.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert![Int(2_" $t ").is_even()];"]
            #[doc = "assert![Int(-2_" $t ").is_even()];"]
            #[doc = "assert![!Int(3_" $t ").is_even()];"]
            #[doc = "assert![Int(0_" $t ").is_even()];"]
            /// ```
            #[must_use]
            pub const fn is_even(self) -> bool { self.0 & 1 == 0 }

            /// Returns `true` if `self` is an odd number.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert![Int(3_" $t ").is_odd()];"]
            #[doc = "assert![Int(-3_" $t ").is_odd()];"]
            #[doc = "assert![!Int(2_" $t ").is_odd()];"]
            #[doc = "assert![!Int(0_" $t ").is_odd()];"]
            /// ```
            #[must_use]
            pub const fn is_odd(self) -> bool { self.0 & 1 == 1 }

            /* signed gcd, lcm */

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
            ///
            /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(4), Int(64_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(4), Int(-64_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(4), Int(64_" $t ").gcd(-36)];"]
            #[doc = "assert_eq![Int(4), Int(-64_" $t ").gcd(-36)];"]
            #[doc = "assert_eq![Int(36), Int(0_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(64), Int(64_" $t ").gcd(0)];"]
            /// ```
            pub const fn gcd(self, b: $t) -> Int<$t> {
                let [mut a, mut b] = [self.0.abs(), b.abs()];
                is![a == 0; return Int(b)];
                is![b == 0; return Int(a)];
                // Let k be the greatest power of 2 dividing both a and b:
                let k = (a | b).trailing_zeros();
                // Divide a and b by 2 until they become odd:
                a >>= a.trailing_zeros();
                b >>= b.trailing_zeros();
                // Break when a == GCD of a / 2^k:
                while b != 0 {
                    b >>= b.trailing_zeros();
                    // ensure b >= a before subtraction:
                    is![a > b; cswap!(mut: a, b); b -= a];
                }
                Int(a << k)

                // Euclid's algorithm:
                // while a != b { is![a > b; a -= b; b -= a] }; a
            }

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>
            /// and the Bézout coeficients.
            ///
            /// This version uses the extended Stein's algorithm which is much more
            /// efficient to compute than Euclid's. It uses only simple arithmetic
            /// operations and works by dividing the inputs by 2 until they are odd,
            /// and then subtracting the smaller number from the larger one.
            ///
            /// The Bézout's coefficients are not unique, and different algorithms
            /// can yield different coefficients that all satisfy Bézout's identity.
            ///
            /// Bézout's identity states that for any two integers a and b,
            /// there exist integers x and y such that $ax + by = gcd(a, b)$.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "let (gcd, x, y) = Int(32_" $t ").gcd_ext(36).as_tuple();"]
            /// assert_eq!(gcd.0, 4);
            /// assert_eq!(x.0 * 32 + y.0 * 36, gcd.0);
            /// ```
            pub const fn gcd_ext(self, b: $t) -> GcdReturn<Int<$t>, Int<$t>> {
                let [mut a, mut b] = [self.0.abs(), b.abs()];
                if a == 0 { return GcdReturn::new(Int(b), Int(0), Int(1)); }
                if b == 0 { return GcdReturn::new(Int(a), Int(1), Int(0)); }

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
                        cswap![mut: a, b];
                        cswap![mut: sa, ta];
                        cswap![mut: sb, tb];
                    }
                    b -= a;
                    ta -= sa;
                    tb -= sb;
                }
                GcdReturn::new(Int(a << k), Int(sa), Int(sb))
            }

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>
            /// and the Bézout coeficients.
            ///
            /// This version uses the extended Euclids's algorithm, which uses a
            /// series of euclidean divisions and works by subtracting multiples
            /// of the smaller number from the larger one.
            ///
            /// The Bézout's coefficients are not unique, and different algorithms
            /// can yield different coefficients that all satisfy Bézout's identity.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "let (gcd, x, y) = Int(32_" $t ").gcd_ext_euc(36).as_tuple();"]
            /// assert_eq!(gcd.0, 4);
            /// assert_eq!(x.0 * 32 + y.0 * 36, gcd.0);
            /// ```
            pub const fn gcd_ext_euc(self, b: $t) -> GcdReturn<Int<$t>, Int<$t>> {
                let a = self.0;
                if a == 0 {
                    GcdReturn::new(Int(b), Int(0), Int(1))
                } else {
                    let (g, x, y) = Int(b % a).gcd_ext_euc(a).as_tuple_copy();
                    GcdReturn::new(g, Int(y.0 - (b / a) * x.0), x)
                }
            }

            /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
            ///
            #[doc = "Performs operations internally as [`" $up "`] for the inner operations."]
            ///
            /// # Errors
            /// Can [`Overflow`].
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(12_" $t ").lcm(15), Ok(Int(60))];"]
            #[doc = "assert_eq![Int(-12_" $t ").lcm(15), Ok(Int(60))];"]
            #[doc = "assert_eq![Int(12_" $t ").lcm(-15), Ok(Int(60))];"]
            /// ```
            pub const fn lcm(self, b: $t) -> Result<Int<$t>> {
                let (aup, bup) = (self.0 as $up, b as $up);
                let res = (aup * bup).abs() / self.gcd(b).0 as $up;
                is![res <= $t::MAX as $up; Ok(Int(res as $t)); Err(Overflow(None))]
            }

            /// Returns a scaled value between `[min..=max]` to a new range `[a..=b]`.
            ///
            #[doc = "Performs operations internally as [`" $up "`] for the checked operations."]
            ///
            /// If the value lies outside of `[min..=max]` it will result in extrapolation.
            ///
            /// # Errors
            /// Can [`Overflow`] for extrapolated values that can't fit the type,
            /// and for overflowing arithmetic operations in the following formula:
            ///
            /// # Formula
            #[doc = FORMULA_SCALE!()]
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Ok(Int(40)), Int(60_" $t ").scale(0, 120, 30, 50)]; // interpolate"]
            #[doc = "assert_eq![Ok(Int(112)), Int(100_" $t ").scale(0, 80, 0, 90)]; // extrapolate"]
            /// # #[cfg(feature = "_int_i8")]
            /// assert![Int(100_i8).scale(0, 50, 0, 90).is_err()]; // extrapolate and overflow
            /// ```
            #[cfg(feature = "cast")]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "cast")))]
            pub const fn scale(self, min: $t, max: $t, a: $t, b: $t) -> Result<Int<$t>> {
                let v = self.0 as $up;
                let (min, max, a, b) = (min as $up, max as $up, a as $up, b as $up);
                let b_a = is![let Some(n) = b.checked_sub(a); n; return Err(Overflow(None))];
                let v_min = is![let Some(n) = v.checked_sub(min); n; return Err(Overflow(None))];
                let mul = is![let Some(n) = b_a.checked_mul(v_min); n; return Err(Overflow(None))];
                let max_min = is![let Some(n) = max.checked_sub(min); n; return Err(Overflow(None))];
                let div = is![let Some(n) = mul.checked_div(max_min); n; return Err(Overflow(None))];
                let sum = is![let Some(n) = div.checked_add(a); n; return Err(Overflow(None))];
                match Cast(sum).[<checked_cast_to_ $t>]() {
                    Ok(n) => Ok(Int(n)),
                    Err(e) => Err(e),
                }
            }

            /// Returns a scaled value between `[min..=max]` to a new range `[a..=b]`.
            ///
            #[doc = "Performs operations internally as [`" $up "`]."]
            ///
            /// If the value lies outside of `[min..=max]` it will result in extrapolation, and
            /// if the value doesn't fit in the type it will wrap around its boundaries.
            ///
            /// # Panics
            /// Could panic for large values of `i128` or `u128`.
            ///
            /// # Formula
            #[doc = FORMULA_SCALE!()]
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(40), Int(60_" $t ").scale_wrap(0, 120, 30, 50)]; // interpolate"]
            #[doc = "assert_eq![Int(112), Int(100_" $t ").scale_wrap(0, 80, 0, 90)]; // extrapolate"]
            /// # #[cfg(feature = "_int_i8")]
            /// assert_eq![Int(-76), Int(100_i8).scale_wrap(0, 50, 0, 90)]; // extrapolate and wrap
            /// ```
            pub const fn scale_wrap(self, min: $t, max: $t, a: $t, b: $t) -> Int<$t> {
                let v = self.0 as $up;
                let (min, max, a, b) = (min as $up, max as $up, a as $up, b as $up);
                Int(((b - a) * (v - min) / (max - min) + a) as $t)
            }
            // MAYBE: scale_saturate

            /// Calculates the middle point of `self` and `other`.
            ///
            /// The result is always rounded towards negative infinity.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = concat!("assert_eq![Int(0_", stringify!($t), ").midpoint(4), 2];")]
            #[doc = concat!("assert_eq![Int(0_", stringify!($t), ").midpoint(-1), -1];")]
            #[doc = concat!("assert_eq![Int(-1_", stringify!($t), ").midpoint(0), -1];")]
            /// ```
            // WAIT: [num_midpoint](https://github.com/rust-lang/rust/issues/110840)
            // NOTE: based on Rust's std implementation.
            #[cfg(feature = $ucap )]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $ucap)))]
            pub const fn midpoint(self, other: $t) -> Int<$t> {
                const U: $ut = <$t>::MIN.unsigned_abs();

                // Map a $t to a $ut
                // ex: i8 [-128; 127] to [0; 255]
                const fn map(a: $t) -> $ut { (a as $ut) ^ U }

                // Map a $ut to a $t
                // ex: u8 [0; 255] to [-128; 127]
                const fn demap(a: $ut) -> $t { (a ^ U) as $t }

                Int(demap(Int(map(self.0)).midpoint(map(other)).0))
            }
        }
    }};
    (
    // implements unsigned ops
    @unsigned $t:ty : $cap:literal : $up:ty | $iup:ty : $icap:literal |$d:literal) => { paste! {
        #[doc = crate::doc_availability!(feature = $cap)]
        ///
        #[doc = "# Integer core methods for `" $t "`\n\n"]
        #[doc = "- [abs](#method.abs" $d ")"]
        #[doc = "- [is_even](#method.is_even" $d ")"]
        #[doc = "- [is_odd](#method.is_odd" $d ")"]
        #[doc = "- [gcd](#method.gcd" $d ")"]
        #[doc = "- [gcd_ext](#method.gcd_ext" $d ")"]
        #[doc = "- [gcd_ext_euc](#method.gcd_ext_euc" $d ")"]
        #[doc = "- [lcm](#method.lcm" $d ")"]
        #[doc = "- [scale](#method.scale" $d ")"]
        #[doc = "- [scale_wrap](#method.scale_wrap" $d ")"]
        #[doc = "- [midpoint](#method.midpoint" $d ")"]
        ///
        #[cfg(feature = $cap )]
        impl Int<$t> {
            /// Returns the absolute value of `self` (no-op).
            pub const fn abs(self) -> Int<$t> { self }

            /// Returns `true` if `self` is an even number.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert![Int(2_" $t ").is_even()];"]
            #[doc = "assert![!Int(3_" $t ").is_even()];"]
            #[doc = "assert![Int(0_" $t ").is_even()];"]
            /// ```
            #[must_use]
            pub const fn is_even(self) -> bool { self.0 & 1 == 0 }

            /// Returns `true` if `self` is an odd number.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert![Int(3_" $t ").is_odd()];"]
            #[doc = "assert![!Int(2_" $t ").is_odd()];"]
            #[doc = "assert![!Int(0_" $t ").is_odd()];"]
            /// ```
            #[must_use]
            pub const fn is_odd(self) -> bool { self.0 & 1 == 1 }

            /* unsigned gcd, lcm */

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
            ///
            /// Uses Stein's algorithm which is much more efficient to compute than Euclid's.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(4), Int(64_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(36), Int(0_" $t ").gcd(36)];"]
            #[doc = "assert_eq![Int(64), Int(64_" $t ").gcd(0)];"]
            /// ```
            pub const fn gcd(self, mut b: $t) -> Int<$t> {
                let mut a = self.0;
                is![a == 0; return Int(b)];
                is![b == 0; return self];
                // Let k be the greatest power of 2 dividing both a and b:
                let k = (a | b).trailing_zeros();
                // Divide a and b by 2 until they become odd:
                a >>= a.trailing_zeros();
                b >>= b.trailing_zeros();
                // Break when a == GCD of a / 2^k:
                while b != 0 {
                    b >>= b.trailing_zeros();
                    // ensure b >= a before subtraction:
                    is![a > b; cswap![mut: a, b]; b -= a];
                }
                Int(a << k)

                // Euclid's algorithm:
                // while a != b { is![a > b; a -= b; b -= a] }; a
            }

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>
            /// and the Bézout coeficients.
            ///
            #[doc = "Performs inner operations and returns coefficients as [`" $iup "`]."]
            ///
            /// This version uses the extended Stein's algorithm which is much more
            /// efficient to compute than Euclid's. It uses only simple arithmetic
            /// operations and works by dividing the inputs by 2 until they are odd,
            /// and then subtracting the smaller number from the larger one.
            ///
            /// The Bézout's coefficients are not unique, and different algorithms
            /// can yield different coefficients that all satisfy Bézout's identity.
            ///
            /// Bézout's identity states that for any two integers a and b,
            /// there exist integers x and y such that $ax + by = gcd(a, b)$.
            ///
            /// # Errors
            /// Can return [`Overflow`] for `u128`.
            ///
            /// # Examples
            /// ```
            /// # use devela::{Int, isize_up};
            #[doc = "let (gcd, x, y) = Int(32_" $t ").gcd_ext(36).unwrap().as_tuple();"]
            /// assert_eq!(gcd.0, 4);
            #[doc = "assert_eq![x.0 * 32 + y.0 * 36, gcd.0 as " $iup "];"]
            /// ```
            #[cfg(all(feature = $icap, feature = "cast"))]
            #[cfg_attr(nightly_doc, doc(cfg(all(feature = $icap, feature = "cast"))))]
            pub const fn gcd_ext(self, b: $t) -> Result<GcdReturn<Int<$t>, Int<$iup>>> {
                if self.0 == 0 { return Ok(GcdReturn::new(Int(b), Int(0), Int(1))); }
                if b == 0 { return Ok(GcdReturn::new(self, Int(1), Int(0))); }

                let mut a = unwrap![ok? Cast(self.0).[<checked_cast_to_ $iup>]()];
                let mut b = unwrap![ok? Cast(b).[<checked_cast_to_ $iup>]()];

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
                        cswap![mut: a, b];
                        cswap![mut: sa, ta];
                        cswap![mut: sb, tb];
                    }
                    b -= a;
                    ta -= sa;
                    tb -= sb;
                }
                Ok(GcdReturn::new(Int((a << k) as $t), Int(sa), Int(sb)))
            }

            /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>
            /// and the Bézout coeficients.
            ///
            #[doc = "Performs inner operations and returns coefficients as [`" $iup "`]."]
            ///
            /// This version uses the extended Euclids's algorithm, which uses a
            /// series of euclidean divisions and works by subtracting multiples
            /// of the smaller number from the larger one.
            ///
            /// The Bézout's coefficients are not unique, and different algorithms
            /// can yield different coefficients that all satisfy Bézout's identity.
            ///
            /// # Errors
            /// Can return [`Overflow`] for `u128`.
            ///
            /// # Examples
            /// ```
            /// # use devela::{Int, isize_up};
            #[doc = "let (gcd, x, y) = Int(32_" $t ").gcd_ext_euc(36).unwrap().as_tuple();"]
            /// assert_eq!(gcd.0, 4);
            #[doc = "assert_eq![x.0 * 32 + y.0 * 36, gcd.0 as " $iup "];"]
            /// ```
            #[cfg(all(feature = $icap, feature = "cast"))]
            #[cfg_attr(nightly_doc, doc(cfg(all(feature = $icap, feature = "cast"))))]
            pub const fn gcd_ext_euc(self, b: $t) -> Result<GcdReturn<Int<$t>, Int<$iup>>> {
                let a = unwrap![ok? Cast(self.0).[<checked_cast_to_ $iup>]()];
                let b = unwrap![ok? Cast(b).[<checked_cast_to_ $iup>]()];

                if a == 0 {
                    Ok(GcdReturn::new(Int(b as $t), Int(0), Int(1)))
                } else {
                    let (g, x, y) = Int(b % a).gcd_ext_euc(a).as_tuple_copy();
                    Ok(GcdReturn::new(Int(g.0 as $t), Int(y.0 - (b / a) * x.0), x))
                }
            }

            /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
            ///
            #[doc = "Performs operations internally as [`" $up "`] for the inner operations."]
            ///
            /// # Errors
            /// Can [`Overflow`].
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(12_" $t ").lcm(15), Ok(Int(60))];"]
            /// ```
            pub const fn lcm(self, b: $t) -> Result<Int<$t>> {
                let (aup, bup) = (self.0 as $up, b as $up);
                let res = aup * bup / self.gcd(b).0 as $up;
                is![res <= $t::MAX as $up; Ok(Int(res as $t)); Err(Overflow(None))]
            }

            /// Returns a scaled value between `[min..=max]` to a new range `[a..=b]`.
            ///
            #[doc = "Performs operations internally as [`" $up "`] for the checked operations."]
            ///
            /// If the value lies outside of `[min..=max]` it will result in extrapolation.
            ///
            /// # Errors
            /// Can [`Overflow`] for extrapolated values that can't fit the type,
            /// and for overflowing arithmetic operations in the following formula:
            ///
            /// # Formula
            #[doc = FORMULA_SCALE!()]
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc ="assert_eq![Ok(Int(40)), Int(60_" $t ").scale(0, 120, 30, 50)]; // interpolate"]
            #[doc ="assert_eq![Ok(Int(112)), Int(100_" $t ").scale(0, 80, 0, 90)]; // extrapolate"]
            /// # #[cfg(feature = "_int_u8")]
            /// assert![Int(200_u8).scale(0, 50, 0, 90).is_err()]; // extrapolate and overflow
            /// ```
            #[cfg(feature = "cast")]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "cast")))]
            pub const fn scale(self, min: $t, max: $t, a: $t, b: $t) -> Result<Int<$t>> {
                let v = self.0 as $up;
                let (min, max, a, b) = (min as $up, max as $up, a as $up, b as $up);
                let b_a = is![let Some(n) = b.checked_sub(a); n; return Err(Overflow(None))];
                let v_min = is![let Some(n) = v.checked_sub(min); n; return Err(Overflow(None))];
                let mul = is![let Some(n) = b_a.checked_mul(v_min); n; return Err(Overflow(None))];
                let max_min = is![let Some(n) = max.checked_sub(min); n; return Err(Overflow(None))];
                let div = is![let Some(n) = mul.checked_div(max_min); n; return Err(Overflow(None))];
                let sum = is![let Some(n) = div.checked_add(a); n; return Err(Overflow(None))];
                match Cast(sum).[<checked_cast_to_ $t>]() {
                    Ok(n) => Ok(Int(n)),
                    Err(e) => Err(e),
                }
            }

            /// Returns a scaled value between `[min..=max]` to a new range `[a..=b]`.
            ///
            #[doc = "Performs operations internally as [`" $up "`]."]
            ///
            /// If the value lies outside of `[min..=max]` it will result in extrapolation, and
            /// if the value doesn't fit in the type it will wrap around its boundaries.
            ///
            /// # Panics
            /// Could panic for large values of `i128` or `u128`.
            ///
            /// # Formula
            #[doc = FORMULA_SCALE!()]
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = "assert_eq![Int(40), Int(60_" $t ").scale_wrap(0, 120, 30, 50)]; // interpolate"]
            #[doc = "assert_eq![Int(112), Int(100_" $t ").scale_wrap(0, 80, 0, 90)]; // extrapolate"]
            /// # #[cfg(feature = "_int_u8")]
            /// assert_eq![Int(104), Int(200_u8).scale_wrap(0, 50, 0, 90)]; // extrapolate and wrap
            /// ```
            pub const fn scale_wrap(self, min: $t, max: $t, a: $t, b: $t) -> Int<$t> {
                let v = self.0 as $up;
                let (min, max, a, b) = (min as $up, max as $up, a as $up, b as $up);
                Int(((b - a) * (v - min) / (max - min) + a) as $t)
            }
            // MAYBE: scale_saturate

            /// Calculates the middle point of `self` and `other`.
            ///
            /// The result is always rounded towards negative infinity.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc = concat!("assert_eq![Int(0_", stringify!($t), ").midpoint(4), 2];")]
            #[doc = concat!("assert_eq![Int(1_", stringify!($t), ").midpoint(4), 2];")]
            /// ```
            // WAIT: [num_midpoint](https://github.com/rust-lang/rust/pull/131784)
            // NOTE: based on Rust's std implementation.
            pub const fn midpoint(self, other: $t) -> Int<$t> {
                // Use the well known branchless algorithm from Hacker's Delight to compute
                // `(a + b) / 2` without overflowing: `((a ^ b) >> 1) + (a & b)`.
                Int(((self.0 ^ other) >> 1) + (self.0 & other))
            }
        }
    }};
}
impl_core!();
