// devela::num::int::trait::impls
//
//!
//

#[cfg(not(feature = "std"))]
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
            /* gcd & lcm */

            #[inline]
            fn int_gcd(self, other: Self::Rhs) -> Result<Self::Out> {
                Ok(Int(self).gcd(other)) }
            #[inline]
            fn int_gcd_ref(self, other: &Self::Rhs) -> Result<Self::Out> {
                Ok(Int(self).gcd(*other)) }
            #[inline]
            fn int_ref_gcd(&self, other: Self::Rhs) -> Result<Self::Out> {
                Ok(Int(*self).gcd(other)) }
            #[inline]
            fn int_ref_gcd_ref(&self, other: &Self::Rhs) -> Result<Self::Out> {
                Ok(Int(*self).gcd(*other)) }

            #[inline]
            fn int_gcd_ext(self, other: Self::Rhs) -> Result<[Self::Out; 3]> {
                Ok(Int(self).gcd_ext(other)) }
            #[inline]
            fn int_gcd_ext_ref(self, other: &Self::Rhs) -> Result<[Self::Out; 3]> {
                Ok(Int(self).gcd_ext(*other)) }
            #[inline]
            fn int_ref_gcd_ext(&self, other: Self::Rhs) -> Result<[Self::Out; 3]> {
                Ok(Int(*self).gcd_ext(other)) }
            #[inline]
            fn int_ref_gcd_ext_ref(&self, other: &Self::Rhs) -> Result<[Self::Out; 3]> {
                Ok(Int(*self).gcd_ext(*other)) }

            #[inline]
            fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(self).lcm(other) { Ok(res) } else { Err(E::Overflow) } }
            #[inline]
            fn int_lcm_ref(self, other: &Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(self).lcm(*other) { Ok(res) } else { Err(E::Overflow) } }
            #[inline]
            fn int_ref_lcm(&self, other: Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(*self).lcm(other) { Ok(res) } else { Err(E::Overflow) } }
            #[inline]
            fn int_ref_lcm_ref(&self, other: &Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(*self).lcm(*other) { Ok(res) } else { Err(E::Overflow) } }
        }
    }};

    // Implements `Num` for unsigned integer types
    // --------------------------------------------------------------------------------------------
    (u $($p:ident),+) => { $( impl_int![@u $p]; )+ };
    (@u $p:ident) => { paste! {
        // u*
        impl NumInt for $p {
            #[inline]
            fn int_gcd(self, other: Self::Rhs) -> Result<Self::Out> {
                Ok(Int(self).gcd(other)) }
            #[inline]
            fn int_gcd_ref(self, other: &Self::Rhs) -> Result<Self::Out> {
                Ok(Int(self).gcd(*other)) }
            #[inline]
            fn int_ref_gcd(&self, other: Self::Rhs) -> Result<Self::Out> {
                Ok(Int(*self).gcd(other)) }
            #[inline]
            fn int_ref_gcd_ref(&self, other: &Self::Rhs) -> Result<Self::Out> {
                Ok(Int(*self).gcd(*other)) }

            #[inline]
            fn int_gcd_ext(self, _: Self::Rhs) -> Result<[Self::Out; 3]> { E::ns() }
            #[inline]
            fn int_gcd_ext_ref(self, _: &Self::Rhs) -> Result<[Self::Out; 3]> { E::ns() }
            #[inline]
            fn int_ref_gcd_ext(&self, _: Self::Rhs) -> Result<[Self::Out; 3]> { E::ns() }
            #[inline]
            fn int_ref_gcd_ext_ref(&self, _: &Self::Rhs) -> Result<[Self::Out; 3]> { E::ns() }

            #[inline]
            fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(self).lcm(other) { Ok(res) } else { Err(E::Overflow) } }
            #[inline]
            fn int_lcm_ref(self, other: &Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(self).lcm(*other) { Ok(res) } else { Err(E::Overflow) } }
            #[inline]
            fn int_ref_lcm(&self, other: Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(*self).lcm(other) { Ok(res) } else { Err(E::Overflow) } }
            #[inline]
            fn int_ref_lcm_ref(&self, other: &Self::Rhs) -> Result<Self::Out> {
                if let Some(res) = Int(*self).lcm(*other) { Ok(res) } else { Err(E::Overflow) } }
        }
    }};
}
use impl_int;
