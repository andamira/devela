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
            fn int_gcd_ext(self, other: Self::Rhs) -> Result<[Self::Out; 3]> {
                let [gcd, b1, b2] = Int(self).gcd_ext(other);
                Ok([gcd.0, b1.0, b2.0]) }
            #[inline]
            fn int_ref_gcd_ext(&self, other: &Self::Rhs) -> Result<[Self::Out; 3]> {
                let [gcd, b1, b2] = Int(*self).gcd_ext(*other);
                Ok([gcd.0, b1.0, b2.0]) }
            #[inline]
            fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(self).lcm(other) { Ok(res.0) } else { Err(E::Overflow) } }
            #[inline]
            fn int_ref_lcm(&self, other: &Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(*self).lcm(*other) { Ok(res.0) } else { Err(E::Overflow) } }
        }
    }};

    // Implements `Num` for unsigned integer types
    // --------------------------------------------------------------------------------------------
    (u $($p:ident),+) => { $( impl_int![@u $p]; )+ };
    (@u $p:ident) => { paste! {
        // u*
        impl NumInt for $p {
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
            fn int_gcd_ext(self, _: Self::Rhs) -> Result<[Self::Out; 3]> { E::ns() }
            #[inline]
            fn int_ref_gcd_ext(&self, _: &Self::Rhs) -> Result<[Self::Out; 3]> { E::ns() }

            #[inline]
            fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(self).lcm(other) { Ok(res.0) } else { Err(E::Overflow) } }
            #[inline]
            fn int_ref_lcm(&self, other: &Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(*self).lcm(*other) { Ok(res.0) } else { Err(E::Overflow) } }
        }
    }};
}
use impl_int;
