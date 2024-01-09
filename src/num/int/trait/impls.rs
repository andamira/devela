// devela::num::int::trait::impls
//
//!
//

use crate::code::paste;
use crate::num::{Int, NumErrors as E, NumInt, NumResult as Result};

impl_int![];
macro_rules! impl_int {
    [] => {
        impl_int![i i8, i16, i32, i64, i128, isize];
        impl_int![u u8, u16, u32, u64, u128, usize];
    };

    // Implements `NumInt` for signed integer types
    // --------------------------------------------------------------------------------------------
    (i $($p:ident),+) => { $( impl_int![@i $p]; )+ };
    (@i $p:ident) => { paste! {
        // i*
        impl NumInt for $p {
            impl_int![common_body_iu];

            /* square root */

            #[inline]
            fn int_sqrt_ceil(self) -> Result<Self::Out> { Int(self).sqrt_ceil().map(|n|n.0) }
            #[inline]
            fn int_sqrt_floor(self) -> Result<Self::Out> { Int(self).sqrt_floor().map(|n|n.0) }
            #[inline]
            fn int_sqrt_round(self) -> Result<Self::Out> { Int(self).sqrt_round().map(|n|n.0) }

            /* gcd & lcm */

            #[inline]
            fn int_gcd_ext(self, other: Self::Rhs) -> Result<[Self::Out; 3]> {
                let [gcd, b1, b2] = Int(self).gcd_ext(other);
                Ok([gcd.0, b1.0, b2.0]) }
            #[inline]
            fn int_ref_gcd_ext(&self, other: &Self::Rhs) -> Result<[Self::Out; 3]> {
                let [gcd, b1, b2] = Int(*self).gcd_ext(*other);
                Ok([gcd.0, b1.0, b2.0]) }
        }
    }};

    // Implements `Num` for unsigned integer types
    // --------------------------------------------------------------------------------------------
    (u $($p:ident),+) => { $( impl_int![@u $p]; )+ };
    (@u $p:ident) => { paste! {
        // u*
        impl NumInt for $p {
            impl_int![common_body_iu];

            /* square root */

            #[inline]
            fn int_sqrt_ceil(self) -> Result<Self::Out> { Ok(Int(self).sqrt_ceil().0) }
            #[inline]
            fn int_sqrt_floor(self) -> Result<Self::Out> { Ok(Int(self).sqrt_floor().0) }
            #[inline]
            fn int_sqrt_round(self) -> Result<Self::Out> { Ok(Int(self).sqrt_round().0) }

            /* gcd & lcm */

            #[inline]
            fn int_gcd_ext(self, _: Self::Rhs) -> Result<[Self::Out; 3]> { E::ns() }
            #[inline]
            fn int_ref_gcd_ext(&self, _: &Self::Rhs) -> Result<[Self::Out; 3]> { E::ns() }
        }
    }};

    // Inner helpers for repeated common bodies for signed and unsigned
    // ============================================================================================
    (common_body_iu) => {
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

        /* square root */

        #[inline]
        fn int_is_square(self) -> Result<bool> { Ok(Int(self).is_square()) }

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

        /* digital root */

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

        /* digits */

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

        /* gcd & lcm */

        #[inline]
        fn int_gcd(self, other: Self::Rhs) -> Result<Self::Out> {
            Ok(Int(self).gcd(other).0) }
        #[inline]
        fn int_ref_gcd(&self, other: &Self::Rhs) -> Result<Self::Out> {
            Ok(Int(*self).gcd(*other).0) }

        #[inline]
        fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> {
            if let Some(res) = Int(self).lcm(other) { Ok(res.0) } else { Err(E::Overflow) } }
        #[inline]
        fn int_ref_lcm(&self, other: &Self::Rhs) -> Result<Self::Out> {
            if let Some(res) = Int(*self).lcm(*other) { Ok(res.0) } else { Err(E::Overflow) } }
    };
}
use impl_int;
