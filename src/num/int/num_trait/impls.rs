// devela::num::int::num_trait::impls
//
//!
//

#[cfg(feature = "alloc")]
use crate::_liballoc::vec::Vec;
#[cfg(feature = "_int_usize")]
use crate::num::isize_up;
#[allow(unused_imports)]
use crate::num::GcdExt;
use crate::num::{Int, NumInt, NumResult as Result};

// $t:     the primitive type
// $cap:   the capability feature that enables the given implementation. E.g "_int_i8".
//
// $ut:    the unsigned type of the same size as $t, only for signed (used for midpoint).
// $ucap:  the feature that enables some methods related to `$ut`. E.g "_int_i8". (only for signed)
//
// $io:    the signed output primitive type (upcasted for unsigned, same as $t for signed).
// $iocap: the capability feature that enables some ops with signed output primitive type.
macro_rules! impl_int {
    () => {
        impl_int![signed
            i8:"_int_i8"|u8:"_int_u8", i16:"_int_i16"|u16:"_int_u16",
            i32:"_int_i32"|u32:"_int_u32", i64:"_int_i64"|u64:"_int_u64",
            i128:"_int_i128"|u128:"_int_u128", isize:"_int_isize"|usize:"_int_usize"
        ];
        impl_int![unsigned
            u8:"_int_u8"|i16:"_int_i16", u16:"_int_u16"|i32:"_int_i32",
            u32:"_int_u32"|i64:"_int_i64", u64:"_int_u64"|i128:"_int_i128",
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
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl NumInt for $t {
            type OutI = $t;

            impl_int![common_body_iu];

            /* core */

            #[inline]
            fn int_gcd_ext(self, other: Self::Rhs) -> Result<GcdExt<Self::Out, Self::OutI>> {
                let g = Int(self).gcd_ext(other);
                Ok(GcdExt::new(g.gcd.0, g.x.0, g.y.0)) }
            #[inline]
            fn int_ref_gcd_ext(&self, other: &Self::Rhs) -> Result<GcdExt<Self::Out, Self::OutI>> {
                let g = Int(*self).gcd_ext(*other);
                Ok(GcdExt::new(g.gcd.0, g.x.0, g.y.0)) }

            #[inline]
            #[cfg(feature = $ucap )]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $ucap)))]
            fn int_midpoint(self, other: Self::Rhs) -> Result<Self::Out> {
                Ok(Int(self).midpoint(other).0) }
            #[inline]
            #[cfg(feature = $ucap )]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $ucap)))]
            fn int_ref_midpoint(&self, other: &Self::Rhs) -> Result<Self::Out> {
                Ok(Int(*self).midpoint(*other).0) }

            /* sqrt roots */

            #[inline]
            fn int_sqrt_ceil(self) -> Result<Self::Out> {
                Int(self).sqrt_ceil().map(|n|n.0) }
            #[inline]
            fn int_ref_sqrt_ceil(&self) -> Result<Self::Out> {
                Int(*self).sqrt_ceil().map(|n|n.0) }

            #[inline]
            fn int_sqrt_floor(self) -> Result<Self::Out> {
                Int(self).sqrt_floor().map(|n|n.0) }
            #[inline]
            fn int_ref_sqrt_floor(&self) -> Result<Self::Out> {
                Int(*self).sqrt_floor().map(|n|n.0) }

            #[inline]
            fn int_sqrt_round(self) -> Result<Self::Out> {
                Int(self).sqrt_round().map(|n|n.0) }
            #[inline]
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
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl NumInt for $t {
            type OutI = $io;

            impl_int![common_body_iu];

            /* core */

            #[inline]
            #[cfg(feature = $iocap )]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $iocap)))]
            fn int_gcd_ext(self, other: Self::Rhs) -> Result<GcdExt<Self::Out, Self::OutI>> {
                let g = Int(self).gcd_ext(other)?;
                Ok(GcdExt::new(g.gcd.0, g.x.0, g.y.0)) }
            #[inline]
            #[cfg(feature = $iocap )]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $iocap)))]
            fn int_ref_gcd_ext(&self, other: &Self::Rhs) -> Result<GcdExt<Self::Out, Self::OutI>> {
                let g = Int(*self).gcd_ext(*other)?;
                Ok(GcdExt::new(g.gcd.0, g.x.0, g.y.0)) }

            #[inline]
            fn int_midpoint(self, other: Self::Rhs) -> Result<Self::Out> {
                Ok(Int(self).midpoint(other).0) }
            #[inline]
            fn int_ref_midpoint(&self, other: &Self::Rhs) -> Result<Self::Out> {
                Ok(Int(*self).midpoint(*other).0) }

            /* sqrt roots */

            #[inline]
            fn int_sqrt_ceil(self) -> Result<Self::Out> {
                Ok(Int(self).sqrt_ceil().0) }
            #[inline]
            fn int_ref_sqrt_ceil(&self) -> Result<Self::Out> {
                Ok(Int(*self).sqrt_ceil().0) }

            #[inline]
            fn int_sqrt_floor(self) -> Result<Self::Out> {
                Ok(Int(self).sqrt_floor().0) }
            #[inline]
            fn int_ref_sqrt_floor(&self) -> Result<Self::Out> {
                Ok(Int(*self).sqrt_floor().0) }

            #[inline]
            fn int_sqrt_round(self) -> Result<Self::Out> {
                Int(self).sqrt_round().map(|n|n.0) }
            #[inline]
            fn int_ref_sqrt_round(&self) -> Result<Self::Out> {
                Int(*self).sqrt_round().map(|n|n.0) }
        }
    }};

    // Inner helpers for repeated common bodies for signed and unsigned
    // ============================================================================================
    (common_body_iu) => {
        /* base */

        #[inline]
        fn int_digital_root(self) -> Result<Self::Out> {
            Ok(Int(self).digital_root().0) }
        #[inline]
        fn int_ref_digital_root(&self) -> Result<Self::Out> {
            Ok(Int(*self).digital_root().0) }
        #[inline]
        fn int_digital_root_base(self, base: Self::Rhs) -> Result<Self::Out> {
            Ok(Int(self).digital_root_base(base).0) }
        #[inline]
        fn int_ref_digital_root_base(&self, base: &Self::Rhs) -> Result<Self::Out> {
            Ok(Int(*self).digital_root_base(*base).0) }

        #[inline]
        fn int_digits(self) -> Result<Self::Out> {
            Ok(Int(self).digits().0) }
        #[inline]
        fn int_ref_digits(&self) -> Result<Self::Out> {
            Ok(Int(*self).digits().0) }
        #[inline]
        fn int_digits_sign(self) -> Result<Self::Out> {
            Ok(Int(self).digits_sign().0) }
        #[inline]
        fn int_ref_digits_sign(&self) -> Result<Self::Out> {
            Ok(Int(*self).digits_sign().0) }
        #[inline]
        fn int_digits_base(self, base: Self::Rhs) -> Result<Self::Out> {
            Ok(Int(self).digits_base(base).0) }
        #[inline]
        fn int_ref_digits_base(&self, base: &Self::Rhs) -> Result<Self::Out> {
            Ok(Int(*self).digits_base(*base).0) }
        #[inline]
        fn int_digits_base_sign(self, base: Self::Rhs) -> Result<Self::Out> {
            Ok(Int(self).digits_base_sign(base).0) }
        #[inline]
        fn int_ref_digits_base_sign(&self, base: &Self::Rhs) -> Result<Self::Out> {
            Ok(Int(*self).digits_base_sign(*base).0) }

        /* core */

        #[inline]
        fn int_abs(self) -> Result<Self::Out> { Ok(Int(self).abs().0) }
        #[inline]
        fn int_ref_abs(&self) -> Result<Self::Out> { Ok(Int(*self).abs().0) }

        #[inline]
        fn int_is_even(self) -> Result<bool> { Ok(Int(self).is_even()) }
        #[inline]
        fn int_ref_is_even(&self) -> Result<bool> { Ok(Int(*self).is_even()) }
        #[inline]
        fn int_is_odd(self) -> Result<bool> { Ok(Int(self).is_odd()) }
        #[inline]
        fn int_ref_is_odd(&self) -> Result<bool> { Ok(Int(*self).is_odd()) }

        #[inline]
        fn int_gcd(self, other: Self::Rhs) -> Result<Self::Out> {
            Ok(Int(self).gcd(other).0) }
        #[inline]
        fn int_ref_gcd(&self, other: &Self::Rhs) -> Result<Self::Out> {
            Ok(Int(*self).gcd(*other).0) }
        // NOTE: the rest are sign-specific

        #[inline]
        fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> {
            match Int(self).lcm(other) { Ok(res) => Ok(res.0), Err(e) => Err(e) } }
        #[inline]
        fn int_ref_lcm(&self, other: &Self::Rhs) -> Result<Self::Out> {
            match Int(*self).lcm(*other) { Ok(res) => Ok(res.0), Err(e) => Err(e) } }

        #[inline]
        fn int_scale(self, min: Self::Rhs, max: Self::Rhs, a: Self::Rhs, b: Self::Rhs)
            -> Result<Self::Out> where Self: Sized { Int(self).scale(min, max, a, b).map(|n|n.0) }
        #[inline]
        fn int_ref_scale(&self, min: &Self::Rhs, max: &Self::Rhs, a: &Self::Rhs, b: &Self::Rhs)
            -> Result<Self::Out> { Int(*self).scale(*min, *max, *a, *b).map(|n|n.0) }

        #[inline]
        fn int_scale_wrap(self, min: Self::Rhs, max: Self::Rhs, a: Self::Rhs, b: Self::Rhs)
            -> Result<Self::Out> where Self: Sized { Ok(Int(self).scale_wrap(min, max, a, b).0) }
        #[inline]
        fn int_ref_scale_wrap(&self, min: &Self::Rhs, max: &Self::Rhs, a: &Self::Rhs, b: &Self::Rhs)
            -> Result<Self::Out> { Ok(Int(*self).scale_wrap(*min, *max, *a, *b).0) }

        /* combinatorics */

        #[inline]
        fn int_factorial(self) -> Result<Self::Out> {
            Int(self).factorial().map(|n|n.0) }
        #[inline]
        fn int_ref_factorial(&self) -> Result<Self::Out> {
            Int(*self).factorial().map(|n|n.0) }
        #[inline]
        fn int_subfactorial(self) -> Result<Self::Out> {
            Int(self).subfactorial().map(|n|n.0) }
        #[inline]
        fn int_ref_subfactorial(&self) -> Result<Self::Out> {
            Int(*self).subfactorial().map(|n|n.0) }

        #[inline]
        fn int_permute(self, r: Self) -> Result<Self::Out> {
            Int(self).permute(r).map(|n|n.0) }
        #[inline]
        fn int_ref_permute(&self, r: &Self) -> Result<Self::Out> {
            Int(*self).permute(*r).map(|n|n.0) }
        #[inline]
        fn int_permute_rep(self, r: Self) -> Result<Self::Out> {
            Int(self).permute_rep(r).map(|n|n.0) }
        #[inline]
        fn int_ref_permute_rep(&self, r: &Self) -> Result<Self::Out> {
            Int(*self).permute_rep(*r).map(|n|n.0) }

        #[inline]
        fn int_combine(self, r: Self) -> Result<Self::Out> {
            Int(self).combine(r).map(|n|n.0) }
        #[inline]
        fn int_ref_combine(&self, r: &Self) -> Result<Self::Out> {
            Int(*self).combine(*r).map(|n|n.0) }
        #[inline]
        fn int_combine_rep(self, r: Self) -> Result<Self::Out> {
            Int(self).combine_rep(r).map(|n|n.0) }
        #[inline]
        fn int_ref_combine_rep(&self, r: &Self) -> Result<Self::Out> {
            Int(*self).combine_rep(*r).map(|n|n.0) }

        /* division */

        #[inline]
        fn int_div_rem(self, b: Self::Rhs) -> Result<[Self::Out; 2]> {
            let [d, r] = Int(self).div_rem(b); Ok([d.0, r.0]) }
        #[inline]
        fn int_div_ceil(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ceil(b).0) }
        #[inline]
        fn int_div_floor(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_floor(b).0) }
        #[inline]
        fn int_div_ties_away(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ties_away(b).0) }
        #[inline]
        fn int_div_ties_towards(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ties_towards(b).0) }
        #[inline]
        fn int_div_ties_even(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ties_even(b).0) }
        #[inline]
        fn int_div_ties_odd(self, b: Self) -> Result<Self::Out> {
            Ok(Int(self).div_ties_odd(b).0) }

        /* factors (allocating) */

        #[inline] #[cfg(feature = "alloc")]
        fn int_factors(self) -> Result<Vec<Self::Out>> { Ok(Int(self).factors()) }
        #[inline] #[cfg(feature = "alloc")]
        fn int_ref_factors(&self) -> Result<Vec<Self::Out>> { Ok(Int(*self).factors()) }
        #[inline] #[cfg(feature = "alloc")]
        fn int_factors_proper(self) -> Result<Vec<Self::Out>> { Ok(Int(self).factors_proper()) }
        #[inline] #[cfg(feature = "alloc")]
        fn int_ref_factors_proper(&self) -> Result<Vec<Self::Out>> {
            Ok(Int(*self).factors_proper()) }
        #[inline] #[cfg(feature = "alloc")]
        fn int_factors_prime(self) -> Result<Vec<Self::Out>> { Ok(Int(self).factors_prime()) }
        #[inline] #[cfg(feature = "alloc")]
        fn int_ref_factors_prime(&self) -> Result<Vec<Self::Out>> {
            Ok(Int(*self).factors_prime()) }
        #[inline] #[cfg(feature = "alloc")]
        fn int_factors_prime_unique(self) -> Result<Vec<Self::Out>> {
            Ok(Int(self).factors_prime_unique()) }
        #[inline] #[cfg(feature = "alloc")]
        fn int_ref_factors_prime_unique(&self) -> Result<Vec<Self::Out>> {
            Ok(Int(*self).factors_prime_unique()) }

        /* factors (non-allocating) */

        #[inline]
        fn int_factors_buf(self, fbuf: &mut [Self::Out], upfbuf: &mut [Self::Out])
            -> Result<(usize, usize)> { Int(self).factors_buf(fbuf, upfbuf) }
        #[inline]
        fn int_ref_factors_buf(&self, fbuf: &mut [Self::Out], upfbuf: &mut [Self::Out])
            -> Result<(usize, usize)> { Int(*self).factors_buf(fbuf, upfbuf) }

        #[inline]
        fn int_factors_proper_buf(self, fbuf: &mut [Self], upfbuf: &mut [Self])
            -> Result<(usize, usize)> { Int(self).factors_proper_buf(fbuf, upfbuf) }
        #[inline]
        fn int_ref_factors_proper_buf(&self, fbuf: &mut [Self::Out], upfbuf: &mut [Self::Out])
            -> Result<(usize, usize)> { Int(*self).factors_proper_buf(fbuf, upfbuf) }

        #[inline]
        fn int_factors_prime_buf(self, buffer: &mut [Self])
            -> Result<usize> { Int(self).factors_prime_buf(buffer) }
        #[inline]
        fn int_ref_factors_prime_buf(&self, buffer: &mut [Self::Out])
         -> Result<usize> { Int(*self).factors_prime_buf(buffer) }

        #[inline]
        fn int_factors_prime_unique_buf(self, buffer: &mut [Self])
            -> Result<usize> { Int(self).factors_prime_unique_buf(buffer) }
        #[inline]
        fn int_ref_factors_prime_unique_buf(&self, buffer: &mut [Self::Out])
         -> Result<usize> { Int(*self).factors_prime_unique_buf(buffer) }

        /* primes */

        #[inline]
        fn int_is_prime(self) -> Result<bool> { Ok(Int(self).is_prime()) }
        #[inline]
        fn int_ref_is_prime(&self) -> Result<bool> { Ok(Int(*self).is_prime()) }

        #[inline]
        fn int_prime_nth(self) -> Result<Self::Out> { Int(self).prime_nth().map(|n|n.0) }
        #[inline]
        fn int_ref_prime_nth(&self) -> Result<Self::Out> { Int(*self).prime_nth().map(|n|n.0) }

        #[inline]
        fn int_prime_pi(self) -> Result<usize> { Ok(Int(self).prime_pi()) }
        #[inline]
        fn int_ref_prime_pi(&self) -> Result<usize> { Ok(Int(*self).prime_pi()) }

        #[inline]
        fn int_totient(self) -> Result<Self::Out> { Ok(Int(self).totient().0) }
        #[inline]
        fn int_ref_totient(&self) -> Result<Self::Out> { Ok(Int(*self).totient().0) }

        /* sqrt roots */

        #[inline]
        fn int_is_square(self) -> Result<bool> { Ok(Int(self).is_square()) }
        #[inline]
        fn int_ref_is_square(&self) -> Result<bool> { Ok(Int(*self).is_square()) }
        // NOTE: the rest are sign-specific

        /* roots */

        #[inline]
        fn int_root_ceil(self, nth: u32) -> Result<Self::Out> {
            Int(self).root_ceil(nth).map(|n|n.0) }
        #[inline]
        fn int_ref_root_ceil(&self, nth: u32) -> Result<Self::Out> {
            Int(*self).root_ceil(nth).map(|n|n.0) }

        #[inline]
        fn int_root_floor(self, nth: u32) -> Result<Self::Out> {
            Int(self).root_floor(nth).map(|n|n.0) }
        #[inline]
        fn int_ref_root_floor(&self, nth: u32) -> Result<Self::Out> {
            Int(*self).root_floor(nth).map(|n|n.0) }
    };
}
impl_int!();
