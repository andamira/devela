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
//   - scale
// - signed only:
//   - gcd_ext
//   - gcd_ext_euc

use {
    crate::{
        code::{iif, paste},
        mem::cswap,
        num::{isize_up, usize_up, Cast, GcdExt, Int, NumError, NumResult as Result},
    },
    NumError::Overflow,
};

// $t:   the input/output type
// $up:  the upcasted type to do the operations on (for lcm)
// $d:  the doclink suffix for the method name
macro_rules! impl_int {
    (signed $( $t:ty : $up:ty : $d:literal ),+) => { $( impl_int![@signed $t:$up:$d]; )+ };
    (unsigned $( $t:ty : $up:ty : $d:literal ),+) => { $( impl_int![@unsigned $t:$up:$d]; )+ };

    // implements signed ops
    (@signed $t:ty : $up:ty : $d:literal) => { paste! {
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
        impl Int<$t> {
            /// Returns the absolute value of `self`.
            #[inline] #[must_use]
            pub const fn abs(self) -> Int<$t> { Int(self.0.abs()) }

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
                    iif![a > b; cswap![a, b]; b -= a];
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
            #[doc = "let (gcd, x, y) = Int(32_" $t ").gcd_ext(36).as_tuple();"]
            /// assert_eq!(gcd.0, 4);
            /// assert_eq!(x.0 * 32 + y.0 * 36, gcd.0);
            /// ```
            #[inline]
            pub const fn gcd_ext(self, b: $t) -> GcdExt<Int<$t>, Int<$t>> {
                let [mut a, mut b] = [self.0.abs(), b.abs()];
                if a == 0 { return GcdExt::new(Int(b), Int(0), Int(1)); }
                if b == 0 { return GcdExt::new(Int(a), Int(1), Int(0)); }

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
                        cswap![a, b];
                        cswap![sa, ta];
                        cswap![sb, tb];
                    }
                    b -= a;
                    ta -= sa;
                    tb -= sb;
                }
                GcdExt::new(Int(a << k), Int(sa), Int(sb))
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
            #[doc = "let (gcd, x, y) = Int(32_" $t ").gcd_ext_euc(36).as_tuple();"]
            /// assert_eq!(gcd.0, 4);
            /// assert_eq!(x.0 * 32 + y.0 * 36, gcd.0);
            /// ```
            #[inline]
            pub const fn gcd_ext_euc(self, b: $t) -> GcdExt<Int<$t>, Int<$t>> {
                let a = self.0;
                if a == 0 {
                    GcdExt::new(Int(b), Int(0), Int(1))
                } else {
                    let (g, x, y) = Int(b % a).gcd_ext_euc(a).as_tuple_const();
                    GcdExt::new(g, Int(y.0 - (b / a) * x.0), x) // IMPROVE impl ops
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
                iif![res <= $t::MAX as $up; Ok(Int(res as $t)); Err(Overflow(None))]
            }

            /// Returns a scaled value between `[min..=max]` to a new range `[a..=b]`.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the checked operations."]
            ///
            /// If the value lies outside of `[min..=max]` it will result in extrapolation.
            /// # Errors
            /// Can [`Overflow`] for extrapolated values that can't fit the type,
            /// and for overflowing arithmetic operations in the following formula:
            /// # Formula
            /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(40)), Int(60_" $t ").scale(0, 120, 30, 50)]; // interpolate"]
            #[doc = "assert_eq![Ok(Int(112)), Int(100_" $t ").scale(0, 80, 0, 90)]; // extrapolate"]
            /// assert![Int(100_i8).scale(0, 50, 0, 90).is_err()]; // extrapolate and overflow"]
            /// ```
            pub const fn scale(self, min: $t, max: $t, a: $t, b: $t) -> Result<Int<$t>> {
                let v = self.0 as $up;
                let (min, max, a, b) = (min as $up, max as $up, a as $up, b as $up);
                let b_a = iif![let Some(n) = b.checked_sub(a); n; return Err(Overflow(None))];
                let v_min = iif![let Some(n) = v.checked_sub(min); n; return Err(Overflow(None))];
                let mul = iif![let Some(n) = b_a.checked_mul(v_min); n; return Err(Overflow(None))];
                let max_min = iif![let Some(n) = max.checked_sub(min); n; return Err(Overflow(None))];
                let div = iif![let Some(n) = mul.checked_div(max_min); n; return Err(Overflow(None))];
                let sum = iif![let Some(n) = div.checked_add(a); n; return Err(Overflow(None))];
                match Cast(sum).[<checked_cast_to_ $t>]() {
                    Ok(n) => Ok(Int(n)),
                    Err(e) => Err(e),
                }
            }

            /// Returns a scaled value between `[min..=max]` to a new range `[a..=b]`.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the intermediate operations."]
            ///
            /// If the value lies outside of `[min..=max]` it will result in extrapolation, and
            /// if the value doesn't fit in the type it will wrap around its boundaries.
            /// # Panics
            /// Could panic for large values of `i128` or `u128`.
            /// # Formula
            /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(40), Int(60_" $t ").scale_wrap(0, 120, 30, 50)]; // interpolate"]
            #[doc = "assert_eq![Int(112), Int(100_" $t ").scale_wrap(0, 80, 0, 90)]; // extrapolate"]
            /// assert_eq![Int(-76), Int(100_i8).scale_wrap(0, 50, 0, 90)]; // extrapolate and wrap"]
            /// ```
            pub const fn scale_wrap(self, min: $t, max: $t, a: $t, b: $t) -> Int<$t> {
                let v = self.0 as $up;
                let (min, max, a, b) = (min as $up, max as $up, a as $up, b as $up);
                Int(((b - a) * (v - min) / (max - min) + a) as $t)
            }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $up:ty : $d:literal) => { paste! {
        #[doc = "# Integer core methods for `" $t "`\n\n"]
        #[doc = "- [abs](#method.abs" $d ")"]
        #[doc = "- [is_even](#method.is_even" $d ")"]
        #[doc = "- [is_odd](#method.is_odd" $d ")"]
        #[doc = "- [gcd](#method.gcd" $d ")"]
        #[doc = "- [lcm](#method.lcm" $d ")"]
        #[doc = "- [scale](#method.scale" $d ")"]
        #[doc = "- [scale_wrap](#method.scale_wrap" $d ")"]
        impl Int<$t> {
            /// Returns the absolute value of `self` (no-op).
            #[inline] #[must_use]
            pub const fn abs(self) -> Int<$t> { self }

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
                    iif![a > b; cswap![a, b]; b -= a];
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
                iif![res <= $t::MAX as $up; Ok(Int(res as $t)); Err(Overflow(None))]
            }

            /// Returns a scaled value between `[min..=max]` to a new range `[a..=b]`.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the checked operations."]
            ///
            /// If the value lies outside of `[min..=max]` it will result in extrapolation.
            /// # Errors
            /// Can [`Overflow`] for extrapolated values that can't fit the type,
            /// and for overflowing arithmetic operations in the following formula:
            /// # Formula
            /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(40)), Int(60_" $t ").scale(0, 120, 30, 50)]; // interpolate"]
            #[doc = "assert_eq![Ok(Int(112)), Int(100_" $t ").scale(0, 80, 0, 90)]; // extrapolate"]
            /// assert![Int(100_i8).scale(0, 50, 0, 90).is_err()]; // extrapolate and overflow"]
            /// ```
            pub const fn scale(self, min: $t, max: $t, a: $t, b: $t) -> Result<Int<$t>> {
                let v = self.0 as $up;
                let (min, max, a, b) = (min as $up, max as $up, a as $up, b as $up);
                let b_a = iif![let Some(n) = b.checked_sub(a); n; return Err(Overflow(None))];
                let v_min = iif![let Some(n) = v.checked_sub(min); n; return Err(Overflow(None))];
                let mul = iif![let Some(n) = b_a.checked_mul(v_min); n; return Err(Overflow(None))];
                let max_min = iif![let Some(n) = max.checked_sub(min); n; return Err(Overflow(None))];
                let div = iif![let Some(n) = mul.checked_div(max_min); n; return Err(Overflow(None))];
                let sum = iif![let Some(n) = div.checked_add(a); n; return Err(Overflow(None))];
                match Cast(sum).[<checked_cast_to_ $t>]() {
                    Ok(n) => Ok(Int(n)),
                    Err(e) => Err(e),
                }
            }

            /// Returns a scaled value between `[min..=max]` to a new range `[a..=b]`.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the intermediate operations."]
            ///
            /// If the value lies outside of `[min..=max]` it will result in extrapolation, and
            /// if the value doesn't fit in the type it will wrap around its boundaries.
            /// # Panics
            /// Could panic for large values of `i128` or `u128`.
            /// # Formula
            /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(40), Int(60_" $t ").scale_wrap(0, 120, 30, 50)]; // interpolate"]
            #[doc = "assert_eq![Int(112), Int(100_" $t ").scale_wrap(0, 80, 0, 90)]; // extrapolate"]
            /// assert_eq![Int(104), Int(200_u8).scale_wrap(0, 50, 0, 90)]; // extrapolate and wrap"]
            /// ```
            pub const fn scale_wrap(self, min: $t, max: $t, a: $t, b: $t) -> Int<$t> {
                let v = self.0 as $up;
                let (min, max, a, b) = (min as $up, max as $up, a as $up, b as $up);
                Int(((b - a) * (v - min) / (max - min) + a) as $t)
            }
        }
    }};

    // $n:  the niche type name prefix (e.g. NonRange)
    // $t:  the niche inner type (the associated primitive integer) (e.g. u8)
    // $($g)*: an optional list of const generics (e.g. RMIN, RMAX)
    // $d:  the doclink suffix for the method name
    // $dt: the doclink suffix for the associated method name implemented for the inner primitive
    (niche $( $n:ident : $t:ident <$($g:ident),*> : $d:literal : $dt: literal),+ $(,)? ) => {
        $( impl_int![@niche $n:$t <$($g),*> : $d:$dt ]; )+
    };
    (@niche $n:ident : $t:ident <$($g:ident),*> : $d:literal : $dt: literal) => { paste! {
        #[doc = "# Integer core methods for `" $t "`\n\n"]
        #[doc = "- [abs](#method.abs" $d ")"]
        #[doc = "- [is_even](#method.is_even" $d ")"]
        #[doc = "- [is_odd](#method.is_odd" $d ")"]
        #[doc = "- [gcd](#method.gcd" $d ")"]
        // IMPROVE: not in unsigned, and for now not in niche signed either
        // #[doc = "- [gcd_ext](#method.gcd_ext" $d ")"]
        // #[doc = "- [gcd_ext_euc](#method.gcd_ext_euc" $d ")"]
        #[doc = "- [lcm](#method.lcm" $d ")"]
        #[doc = "- [scale](#method.scale" $d ")"]
        #[doc = "- [scale_wrap](#method.scale_wrap" $d ")"]
        impl<$(const $g:$t,)*> Int<[<$n$t:camel>]<$($g,)*>> {
            num_niche_impls![Int $n:$t:$dt<$($g),*>, +const abs, self];
            num_niche_impls![Int=>bool: $n:$t:$dt<$($g),*>, +const is_even, self];
            num_niche_impls![Int=>bool: $n:$t:$dt<$($g),*>, +const is_odd, self];
            num_niche_impls![Int $n:$t:$dt<$($g),*>, +const gcd, self, r: $t];
            // ...
            num_niche_impls![Int=>res $n:$t:$dt<$($g),*>, +const lcm, self, r: $t];
            num_niche_impls![Int=>res $n:$t:$dt<$($g),*>, +const scale, self,
                min: $t, max: $t, a: $t, b: $t];
            num_niche_impls![Int $n:$t:$dt<$($g),*>, +const scale_wrap, self,
                min: $t, max: $t, a: $t, b: $t];
        }
    }};
}
impl_int![signed
i8:i16:"", i16:i32:"-1", i32:i64:"-2", i64:i128:"-3", i128:i128:"-4", isize:isize_up:"-5"];
impl_int![unsigned
u8:u16:"-6", u16:u32:"-7", u32:u64:"-8", u64:u128:"-9", u128:u128:"-10", usize:usize_up:"-11"];

#[cfg(feature = "num_niche_impls")]
use crate::num::{niche::*, num_niche_impls};
#[cfg(feature = "num_niche_impls")]
num_niche_impls![impl_int niche];
