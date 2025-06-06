// devela::num::int::num_trait::impls
//
//! Implementations of `NumInt` for primitives.
//

#[allow(unused_imports)]
use crate::GcdReturn;
#[cfg(feature = "alloc")]
use crate::Vec;
#[cfg(feature = "_int_usize")]
use crate::isize_up;
use crate::{Int, NumInt, NumResult as Result, ValueQuant};

/// $t:     the primitive type
/// $cap:   the capability feature that enables the given implementation. E.g "_int_i8".
///
/// $ut:    the unsigned type of the same size as $t, only for signed (used for midpoint).
/// $ucap:  the feature that enables some methods related to `$ut`. E.g "_int_i8". (only for signed)
///
/// $io:    the signed output primitive type (upcasted for unsigned, same as $t for signed).
/// $iocap: the capability feature that enables some ops with signed output primitive type.
///         also corresponds to $iup in impl_modulo, for example.
macro_rules! impl_int {
    () => {
        impl_int![signed
            i8:"_int_i8"|u8:"_int_u8",
            i16:"_int_i16"|u16:"_int_u16",
            i32:"_int_i32"|u32:"_int_u32",
            i64:"_int_i64"|u64:"_int_u64",
            i128:"_int_i128"|u128:"_int_u128",
            isize:"_int_isize"|usize:"_int_usize"
        ];
        impl_int![unsigned
            u8:"_int_u8"|i16:"_int_i16",
            u16:"_int_u16"|i32:"_int_i32",
            u32:"_int_u32"|i64:"_int_i64",
            u64:"_int_u64"|i128:"_int_i128",
            u128:"_int_u128"|i128:"_int_i128"
        ];
        #[cfg(target_pointer_width = "32")]
        impl_int![unsigned usize:"_int_usize"|isize_up:"_int_i64"];
        #[cfg(target_pointer_width = "64")]
        impl_int![unsigned usize:"_int_usize"|isize_up:"_int_i128"];
    };

    // Implements `NumInt` for signed integer types
    // --------------------------------------------------------------------------------------------
    (signed $($t:ident : $cap:literal | $ut:ident : $ucap:literal),+) => {
        $( impl_int![@signed $t:$cap | $ut:$ucap]; )+
    };
    (@signed $t:ident : $cap:literal | $ut:ident:$ucap:literal) => { $crate::paste! {
        #[cfg(feature = $cap )]
        #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))]
        impl NumInt for $t {
            type OutI = $t;

            impl_int![common_body_iu];

            /* core */

            fn int_gcd_ext(self, other: Self::Rhs)
                -> Result<GcdReturn<Self::Out, Self::OutI>> {
                match Int(self).gcd_ext(other) {
                    GcdReturn { gcd, x, y } => Ok(GcdReturn { gcd: gcd.0, x: x.0, y: y.0 }) }}
            fn int_ref_gcd_ext(&self, other: &Self::Rhs)
                -> Result<GcdReturn<Self::Out, Self::OutI>> {
                match Int(*self).gcd_ext(*other) {
                    GcdReturn { gcd, x, y } => Ok(GcdReturn { gcd: gcd.0, x: x.0, y: y.0 }) }}

            #[cfg(feature = $ucap )]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $ucap)))]
            fn int_midpoint(self, other: Self::Rhs) -> Result<Self::Out> {
                Ok(Int(self).midpoint(other).0) }
            #[cfg(feature = $ucap )]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $ucap)))]
            fn int_ref_midpoint(&self, other: &Self::Rhs) -> Result<Self::Out> {
                Ok(Int(*self).midpoint(*other).0) }

            /* modulo */

            fn int_modulo_mul_inv(self, modulus: Self) -> Result<Self> {
                Int(self).modulo_mul_inv(modulus).map(|n|n.0) }
            fn int_ref_modulo_mul_inv(&self, modulus: &Self) -> Result<Self> {
                Int(*self).modulo_mul_inv(*modulus).map(|n|n.0) }

            fn int_modulo_div(self, other: Self, modulus: Self) -> Result<Self> {
                Int(self).modulo_div(other, modulus).map(|n|n.0) }
            fn int_ref_modulo_div(&self, other: &Self, modulus: &Self) -> Result<Self> {
                Int(*self).modulo_div(*other, *modulus).map(|n|n.0) }

            /* sqrt roots */

            fn int_sqrt_ceil(self) -> Result<Self::Out> {
                Int(self).sqrt_ceil().map(|n|n.0) }
            fn int_ref_sqrt_ceil(&self) -> Result<Self::Out> {
                Int(*self).sqrt_ceil().map(|n|n.0) }

            fn int_sqrt_floor(self) -> Result<Self::Out> {
                Int(self).sqrt_floor().map(|n|n.0) }
            fn int_ref_sqrt_floor(&self) -> Result<Self::Out> {
                Int(*self).sqrt_floor().map(|n|n.0) }

            fn int_sqrt_round(self) -> Result<Self::Out> {
                Int(self).sqrt_round().map(|n|n.0) }
            fn int_ref_sqrt_round(&self) -> Result<Self::Out> {
                Int(*self).sqrt_round().map(|n|n.0) }
        }
    }};

    // Implements `Num` for unsigned integer types
    // --------------------------------------------------------------------------------------------
    (unsigned $($t:ident : $cap:literal | $io:ident : $iocap:literal),+) => {
        $( impl_int![@unsigned $t:$cap | $io:$iocap]; )+
    };
    (@unsigned $t:ident : $cap:literal | $io:ident : $iocap:literal) => { $crate::paste! {
        #[cfg(feature = $cap )]
        #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))]
        impl NumInt for $t {
            type OutI = $io;

            impl_int![common_body_iu];

            /* core */

            #[cfg(all(feature = $iocap, feature = "cast"))]
            #[cfg_attr(nightly_doc, doc(cfg(all(feature = $iocap, feature = "cast"))))]
            fn int_gcd_ext(self, other: Self::Rhs)
                -> Result<GcdReturn<Self::Out, Self::OutI>> {
                Int(self).gcd_ext(other)
                    .map(|res| GcdReturn { gcd: res.gcd.0, x: res.x.0, y: res.y.0 }) }
            #[cfg(all(feature = $iocap, feature = "cast"))]
            #[cfg_attr(nightly_doc, doc(cfg(all(feature = $iocap, feature = "cast"))))]
            fn int_ref_gcd_ext(&self, other: &Self::Rhs)
                -> Result<GcdReturn<Self::Out, Self::OutI>> {
                Int(*self).gcd_ext(*other)
                    .map(|res| GcdReturn { gcd: res.gcd.0, x: res.x.0, y: res.y.0 }) }

            fn int_midpoint(self, other: Self::Rhs) -> Result<Self::Out> {
                Ok(Int(self).midpoint(other).0) }
            fn int_ref_midpoint(&self, other: &Self::Rhs) -> Result<Self::Out> {
                Ok(Int(*self).midpoint(*other).0) }

            /* modulo */

            #[cfg(all(feature = $iocap, feature = "cast"))]
            #[cfg_attr(nightly_doc, doc(cfg(all(feature = $iocap, feature = "cast"))))]
            fn int_modulo_mul_inv(self, modulus: Self) -> Result<Self> {
                Int(self).modulo_mul_inv(modulus).map(|n|n.0) }
            #[cfg(all(feature = $iocap, feature = "cast"))]
            #[cfg_attr(nightly_doc, doc(cfg(all(feature = $iocap, feature = "cast"))))]
            fn int_ref_modulo_mul_inv(&self, modulus: &Self) -> Result<Self> {
                Int(*self).modulo_mul_inv(*modulus).map(|n|n.0) }

            #[cfg(all(feature = $iocap, feature = "cast"))]
            #[cfg_attr(nightly_doc, doc(cfg(all(feature = $iocap, feature = "cast"))))]
            fn int_modulo_div(self, other: Self, modulus: Self) -> Result<Self> {
                Int(self).modulo_div(other, modulus).map(|n|n.0) }
            #[cfg(all(feature = $iocap, feature = "cast"))]
            #[cfg_attr(nightly_doc, doc(cfg(all(feature = $iocap, feature = "cast"))))]
            fn int_ref_modulo_div(&self, other: &Self, modulus: &Self) -> Result<Self> {
                Int(*self).modulo_div(*other, *modulus).map(|n|n.0) }

            /* sqrt roots */

            fn int_sqrt_ceil(self) -> Result<Self::Out> {
                Ok(Int(self).sqrt_ceil().0) }
            fn int_ref_sqrt_ceil(&self) -> Result<Self::Out> {
                Ok(Int(*self).sqrt_ceil().0) }

            fn int_sqrt_floor(self) -> Result<Self::Out> {
                Ok(Int(self).sqrt_floor().0) }
            fn int_ref_sqrt_floor(&self) -> Result<Self::Out> {
                Ok(Int(*self).sqrt_floor().0) }

            fn int_sqrt_round(self) -> Result<Self::Out> {
                Int(self).sqrt_round().map(|n|n.0) }
            fn int_ref_sqrt_round(&self) -> Result<Self::Out> {
                Int(*self).sqrt_round().map(|n|n.0) }
        }
    }};

    // Inner helpers for repeated common bodies for signed and unsigned
    // ============================================================================================
    (common_body_iu) => {
        /* base */

        fn int_digital_root(self) -> Result<Self::Out> {
            Ok(Int(self).digital_root().0) }
        fn int_ref_digital_root(&self) -> Result<Self::Out> {
            Ok(Int(*self).digital_root().0) }
        fn int_digital_root_base(self, base: Self::Rhs) -> Result<Self::Out> {
            Ok(Int(self).digital_root_base(base).0) }
        fn int_ref_digital_root_base(&self, base: &Self::Rhs) -> Result<Self::Out> {
            Ok(Int(*self).digital_root_base(*base).0) }

        fn int_digits(self) -> Result<Self::Out> {
            Ok(Int(self).digits().0) }
        fn int_ref_digits(&self) -> Result<Self::Out> {
            Ok(Int(*self).digits().0) }
        fn int_digits_sign(self) -> Result<Self::Out> {
            Ok(Int(self).digits_sign().0) }
        fn int_ref_digits_sign(&self) -> Result<Self::Out> {
            Ok(Int(*self).digits_sign().0) }
        fn int_digits_base(self, base: Self::Rhs) -> Result<Self::Out> {
            Ok(Int(self).digits_base(base).0) }
        fn int_ref_digits_base(&self, base: &Self::Rhs) -> Result<Self::Out> {
            Ok(Int(*self).digits_base(*base).0) }
        fn int_digits_base_sign(self, base: Self::Rhs) -> Result<Self::Out> {
            Ok(Int(self).digits_base_sign(base).0) }
        fn int_ref_digits_base_sign(&self, base: &Self::Rhs) -> Result<Self::Out> {
            Ok(Int(*self).digits_base_sign(*base).0) }

        /* core */

        fn int_abs(self) -> Result<Self::Out> { Ok(Int(self).abs().0) }
        fn int_ref_abs(&self) -> Result<Self::Out> { Ok(Int(*self).abs().0) }

        fn int_is_even(self) -> Result<bool> { Ok(Int(self).is_even()) }
        fn int_ref_is_even(&self) -> Result<bool> { Ok(Int(*self).is_even()) }
        fn int_is_odd(self) -> Result<bool> { Ok(Int(self).is_odd()) }
        fn int_ref_is_odd(&self) -> Result<bool> { Ok(Int(*self).is_odd()) }

        fn int_gcd(self, other: Self::Rhs) -> Result<Self::Out> {
            Ok(Int(self).gcd(other).0) }
        fn int_ref_gcd(&self, other: &Self::Rhs) -> Result<Self::Out> {
            Ok(Int(*self).gcd(*other).0) }
        // NOTE: the rest are sign-specific

        fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> {
            match Int(self).lcm(other) { Ok(res) => Ok(res.0), Err(e) => Err(e) } }
        fn int_ref_lcm(&self, other: &Self::Rhs) -> Result<Self::Out> {
            match Int(*self).lcm(*other) { Ok(res) => Ok(res.0), Err(e) => Err(e) } }

        #[cfg(feature = "cast")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "cast")))]
        fn int_scale(self, min: Self::Rhs, max: Self::Rhs, a: Self::Rhs, b: Self::Rhs)
            -> Result<Self::Out> where Self: Sized { Int(self).scale(min, max, a, b).map(|n|n.0) }
        #[cfg(feature = "cast")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "cast")))]
        fn int_ref_scale(&self, min: &Self::Rhs, max: &Self::Rhs, a: &Self::Rhs, b: &Self::Rhs)
            -> Result<Self::Out> { Int(*self).scale(*min, *max, *a, *b).map(|n|n.0) }

        fn int_scale_wrap(self, min: Self::Rhs, max: Self::Rhs, a: Self::Rhs, b: Self::Rhs)
            -> Result<Self::Out> where Self: Sized { Ok(Int(self).scale_wrap(min, max, a, b).0) }
        fn int_ref_scale_wrap(&self, min: &Self::Rhs, max: &Self::Rhs, a: &Self::Rhs, b: &Self::Rhs)
            -> Result<Self::Out> { Ok(Int(*self).scale_wrap(*min, *max, *a, *b).0) }

        /* combinatorics */

        fn int_factorial(self) -> Result<Self::Out> {
            Int(self).factorial().map(|n|n.0) }
        fn int_ref_factorial(&self) -> Result<Self::Out> {
            Int(*self).factorial().map(|n|n.0) }
        fn int_subfactorial(self) -> Result<Self::Out> {
            Int(self).subfactorial().map(|n|n.0) }
        fn int_ref_subfactorial(&self) -> Result<Self::Out> {
            Int(*self).subfactorial().map(|n|n.0) }

        fn int_permute(self, r: Self) -> Result<Self::Out> {
            Int(self).permute(r).map(|n|n.0) }
        fn int_ref_permute(&self, r: &Self) -> Result<Self::Out> {
            Int(*self).permute(*r).map(|n|n.0) }
        #[cfg(feature = "cast")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "cast")))]
        fn int_permute_rep(self, r: Self) -> Result<Self::Out> {
            Int(self).permute_rep(r).map(|n|n.0) }
        #[cfg(feature = "cast")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "cast")))]
        fn int_ref_permute_rep(&self, r: &Self) -> Result<Self::Out> {
            Int(*self).permute_rep(*r).map(|n|n.0) }

        fn int_combine(self, r: Self) -> Result<Self::Out> {
            Int(self).combine(r).map(|n|n.0) }
        fn int_ref_combine(&self, r: &Self) -> Result<Self::Out> {
            Int(*self).combine(*r).map(|n|n.0) }
        fn int_combine_rep(self, r: Self) -> Result<Self::Out> {
            Int(self).combine_rep(r).map(|n|n.0) }
        fn int_ref_combine_rep(&self, r: &Self) -> Result<Self::Out> {
            Int(*self).combine_rep(*r).map(|n|n.0) }

        /* division */

        fn int_div_rem(self, b: Self::Rhs) -> Result<[Self::Out; 2]> {
            let [d, r] = Int(self).div_rem(b); Ok([d.0, r.0]) }
        fn int_div_ceil(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ceil(b).0) }
        fn int_div_floor(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_floor(b).0) }
        fn int_div_ties_away(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ties_away(b).0) }
        fn int_div_ties_towards(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ties_towards(b).0) }
        fn int_div_ties_even(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ties_even(b).0) }
        fn int_div_ties_odd(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ties_odd(b).0) }

        /* factors (allocating) */

        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        fn int_factors(self) -> Result<Vec<Self::Out>> { Ok(Int(self).factors()) }
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        fn int_ref_factors(&self) -> Result<Vec<Self::Out>> { Ok(Int(*self).factors()) }
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        fn int_factors_proper(self) -> Result<Vec<Self::Out>> { Ok(Int(self).factors_proper()) }
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        fn int_ref_factors_proper(&self) -> Result<Vec<Self::Out>> {
            Ok(Int(*self).factors_proper()) }
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        fn int_factors_prime(self) -> Result<Vec<Self::Out>> { Ok(Int(self).factors_prime()) }
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        fn int_ref_factors_prime(&self) -> Result<Vec<Self::Out>> {
            Ok(Int(*self).factors_prime()) }
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        fn int_factors_prime_unique(self) -> Result<Vec<Self::Out>> {
            Ok(Int(self).factors_prime_unique()) }
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        fn int_ref_factors_prime_unique(&self) -> Result<Vec<Self::Out>> {
            Ok(Int(*self).factors_prime_unique()) }

        /* factors (non-allocating) */

        fn int_factors_buf(self, fbuf: &mut [Self::Out], upfbuf: &mut [Self::Out])
            -> Result<(usize, usize)> { Int(self).factors_buf(fbuf, upfbuf) }
        fn int_ref_factors_buf(&self, fbuf: &mut [Self::Out], upfbuf: &mut [Self::Out])
            -> Result<(usize, usize)> { Int(*self).factors_buf(fbuf, upfbuf) }

        fn int_factors_proper_buf(self, fbuf: &mut [Self], upfbuf: &mut [Self])
            -> Result<(usize, usize)> { Int(self).factors_proper_buf(fbuf, upfbuf) }
        fn int_ref_factors_proper_buf(&self, fbuf: &mut [Self::Out], upfbuf: &mut [Self::Out])
            -> Result<(usize, usize)> { Int(*self).factors_proper_buf(fbuf, upfbuf) }

        fn int_factors_prime_buf(self, buffer: &mut [Self])
            -> Result<usize> { Int(self).factors_prime_buf(buffer) }
        fn int_ref_factors_prime_buf(&self, buffer: &mut [Self::Out])
         -> Result<usize> { Int(*self).factors_prime_buf(buffer) }

        fn int_factors_prime_unique_buf(self, buffer: &mut [Self])
            -> Result<usize> { Int(self).factors_prime_unique_buf(buffer) }
        fn int_ref_factors_prime_unique_buf(&self, buffer: &mut [Self::Out])
         -> Result<usize> { Int(*self).factors_prime_unique_buf(buffer) }

        /* modulo */

        fn int_modulo(self, modulus: Self) -> Result<Self> {
            Int(self).modulo(modulus).map(|n|n.0) }
        fn int_ref_modulo(&self, modulus: &Self) -> Result<Self> {
            Int(*self).modulo(*modulus).map(|n|n.0) }

        fn int_modulo_add(self, other: Self, modulus: Self) -> Result<Self> {
            Int(self).modulo_add(other, modulus).map(|n|n.0) }
        fn int_ref_modulo_add(&self, other: &Self, modulus: &Self) -> Result<Self> {
            Int(*self).modulo_add(*other, *modulus).map(|n|n.0) }
        fn int_modulo_add_cycles(self, other: Self, modulus: Self)
            -> Result<ValueQuant<Self, Self>> {
            Int(self).modulo_add_cycles(other, modulus)
                .map(|res| ValueQuant { v: res.v.0, q: res.q.0 }) }
        fn int_ref_modulo_add_cycles(&self, other: &Self, modulus: &Self)
            -> Result<ValueQuant<Self, Self>> {
            Int(*self).modulo_add_cycles(*other, *modulus)
                .map(|res| ValueQuant { v: res.v.0, q: res.q.0 }) }
        fn int_modulo_add_inv(self, modulus: Self) -> Result<Self> {
            Int(self).modulo_add_inv(modulus).map(|n|n.0) }
        fn int_ref_modulo_add_inv(&self, modulus: &Self) -> Result<Self> {
            Int(*self).modulo_add_inv(*modulus).map(|n|n.0) }

        fn int_modulo_sub(self, other: Self, modulus: Self) -> Result<Self> {
            Int(self).modulo_sub(other, modulus).map(|n|n.0) }
        fn int_ref_modulo_sub(&self, other: &Self, modulus: &Self) -> Result<Self> {
            Int(*self).modulo_sub(*other, *modulus).map(|n|n.0) }
        fn int_modulo_sub_cycles(self, other: Self, modulus: Self)
            -> Result<ValueQuant<Self, Self>> {
            Int(self).modulo_sub_cycles(other, modulus)
                .map(|res| ValueQuant { v: res.v.0, q: res.q.0 }) }
        fn int_ref_modulo_sub_cycles(&self, other: &Self, modulus: &Self)
            -> Result<ValueQuant<Self, Self>> {
            Int(*self).modulo_sub_cycles(*other, *modulus)
                .map(|res| ValueQuant { v: res.v.0, q: res.q.0 }) }

        fn int_modulo_mul(self, other: Self, modulus: Self) -> Result<Self> {
            Int(self).modulo_mul(other, modulus).map(|n|n.0) }
        fn int_ref_modulo_mul(&self, other: &Self, modulus: &Self) -> Result<Self> {
            Int(*self).modulo_mul(*other, *modulus).map(|n|n.0) }
        fn int_modulo_mul_cycles(self, other: Self, modulus: Self)
            -> Result<ValueQuant<Self, Self>> {
            Int(self).modulo_mul_cycles(other, modulus)
                .map(|res| ValueQuant { v: res.v.0, q: res.q.0 }) }
        fn int_ref_modulo_mul_cycles(&self, other: &Self, modulus: &Self)
            -> Result<ValueQuant<Self, Self>> {
            Int(*self).modulo_mul_cycles(*other, *modulus)
                .map(|res| ValueQuant { v: res.v.0, q: res.q.0 }) }

        /* primes */

        fn int_is_prime(self) -> Result<bool> { Ok(Int(self).is_prime()) }
        fn int_ref_is_prime(&self) -> Result<bool> { Ok(Int(*self).is_prime()) }

        fn int_prime_nth(self) -> Result<Self::Out> { Int(self).prime_nth().map(|n|n.0) }
        fn int_ref_prime_nth(&self) -> Result<Self::Out> { Int(*self).prime_nth().map(|n|n.0) }

        fn int_prime_pi(self) -> Result<usize> { Ok(Int(self).prime_pi()) }
        fn int_ref_prime_pi(&self) -> Result<usize> { Ok(Int(*self).prime_pi()) }

        fn int_totient(self) -> Result<Self::Out> { Ok(Int(self).totient().0) }
        fn int_ref_totient(&self) -> Result<Self::Out> { Ok(Int(*self).totient().0) }

        /* sqrt roots */

        fn int_is_square(self) -> Result<bool> { Ok(Int(self).is_square()) }
        fn int_ref_is_square(&self) -> Result<bool> { Ok(Int(*self).is_square()) }
        // NOTE: the rest are sign-specific

        /* roots */

        fn int_root_ceil(self, nth: u32) -> Result<Self::Out> {
            Int(self).root_ceil(nth).map(|n|n.0) }
        fn int_ref_root_ceil(&self, nth: u32) -> Result<Self::Out> {
            Int(*self).root_ceil(nth).map(|n|n.0) }

        fn int_root_floor(self, nth: u32) -> Result<Self::Out> {
            Int(self).root_floor(nth).map(|n|n.0) }
        fn int_ref_root_floor(&self, nth: u32) -> Result<Self::Out> {
            Int(*self).root_floor(nth).map(|n|n.0) }
    };
}
impl_int!();
