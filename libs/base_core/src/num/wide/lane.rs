// devela_base_core::num::wide::lane
//
//! Defines [`Lane4`], [`Lane8`], [`Lane16`], [`Lane32`].
//

use crate::{ConstInitCore, Divisor, punroll};

macro_rules! impl_lanes {
    () => {
        impl_lanes![lanes: 4, 8];
        // impl_lanes![lanes: 16, 32]; // NOTE: adds noticiable compile time
    };
    (lanes: $( $L:literal ),+) => { $crate::paste! {
        $( impl_lanes![define_lane: [<Lane $L>] + $L]; )+
    }};
    (define_lane: $name: ident + $L:literal) => {
        $crate::paste! {
            #[doc = "A fixed-width pack of " $L
            " numeric lanes for parallel elementwise operations."]
            #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name<T>(pub [T; $L]);
        }
        impl<T: ConstInitCore> ConstInitCore for $name<T> {
            const INIT: Self = Self([T::INIT; $L]);
        }
        impl_lanes![all_types:
            $name + $L for u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];
        impl_lanes![signed_types:
            $name + $L for i8, i16, i32, i64, i128, f32, f64];
        impl_lanes![int_types:
            $name + $L for u8, u16, u32, u64, u128, i8, i16, i32, i64, i128];
    };
    (all_types: $name:ident + $L:literal for $( $t:ty ),+) => {
        $( impl_lanes![one_type: $name + $L for $t]; )+
    };
    (one_type: $name:ident + $L:literal for $t:ty) => { $crate::paste! {
        impl $name<$t> {
            /* broadcasting / constructing */

            #[doc = "Builds a lane pack from the first " $L " bytes of a byte slice."]
            /// # Panics
            #[doc = "Panics if `bytes.len() < " $L "`."]
            pub const fn from_bytes(bytes: &[u8]) -> Self {
                Self( punroll! { $L[] |i| bytes[i] as $t })
            }

            #[doc = "Builds a lane pack from the first " $L " elements of a slice."]
            /// # Panics
            #[doc = "Panics if `slice.len() < " $L "`."]
            pub const fn from_slice(slice: &[$t]) -> Self {
                Self( punroll! { $L[] |i| slice[i] })
            }

            /// Fills all lanes with the same value.
            pub const fn splat(v: $t) -> Self {
                Self([v; $L])
            }

            /* arithmetic: elementwise */

            /// Adds each lane of `rhs` to the corresponding lane in `self`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn add_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] += rhs.0[i] }
            }

            /// Subtracts each lane of `rhs` from the corresponding lane in `self`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn sub_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] -= rhs.0[i] }
            }

            /// Multiplies each lane of `self` by the corresponding lane of `rhs`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn mul_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] *= rhs.0[i] }
            }

            /// Applies elementwise modular reduction.
            /// # Panics
            /// Panics if any divisor is zero.
            pub const fn rem_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] %= rhs.0[i] }
            }

            /// Divides each lane by the corresponding lane in `rhs` (truncating division).
            /// # Panics
            /// Panics if any divisor is zero or if signed division overflows.
            pub const fn div_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] /= rhs.0[i] }
            }

            /* arithmetic: scalar */

            /// Adds the scalar `rhs` to each lane in `self`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn add_scalar_assign(&mut self, rhs: $t) {
                punroll! { $L |i| self.0[i] += rhs }
            }

            /// Subtracts the scalar `rhs` from each lane in `self`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn sub_scalar_assign(&mut self, rhs: $t) {
                punroll! { $L |i| self.0[i] -= rhs }
            }

            /// Multiplies each lane by the scalar `rhs`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn mul_scalar_assign(&mut self, rhs: $t) {
                punroll! { $L |i| self.0[i] *= rhs }
            }

            /// Applies scalar modular reduction to each lane.
            /// # Panics
            /// Panics if `rhs == 0`.
            pub const fn rem_scalar_assign(&mut self, rhs: $t) {
                punroll! { $L |i| self.0[i] %= rhs }
            }

            /// Divides each lane by the scalar `rhs` (truncating division).
            /// # Panics
            /// Panics if `rhs == 0` or if signed division overflows.
            pub const fn div_scalar_assign(&mut self, rhs: $t) {
                punroll! { $L |i| self.0[i] /= rhs }
            }

            /* comparisons and clamping */

            /// Returns the minimum lane value.
            pub const fn min(&self) -> $t {
                let mut m = self.0[0];
                punroll! { $L |i| if i != 0 && self.0[i] < m { m = self.0[i]; } }
                m
            }

            /// Returns the maximum lane value.
            pub const fn max(&self) -> $t {
                let mut m = self.0[0];
                punroll! { $L |i| if i != 0 && self.0[i] > m { m = self.0[i]; } }
                m
            }

            /// Clamps each lane to the inclusive `[lo, hi]` range.
            pub const fn clamp_assign(&mut self, lo: $t, hi: $t) {
                punroll! { $L |i| {
                    let x = self.0[i];
                    self.0[i] = if x < lo { lo } else if x > hi { hi } else { x };
                }}
            }

            /* reductions */

            /// Returns the sum of all lanes.
            pub const fn sum(&self) -> $t {
                let mut acc = self.0[0];
                punroll! { $L |i| if i != 0 { acc += self.0[i]; } }
                acc
            }
        }
        impl crate::AddAssign<Self> for $name<$t> {
            fn add_assign(&mut self, rhs: Self) { self.add_assign(rhs) } }
        impl crate::SubAssign<Self> for $name<$t> {
            fn sub_assign(&mut self, rhs: Self) { self.sub_assign(rhs) } }
        impl crate::MulAssign<Self> for $name<$t> {
            fn mul_assign(&mut self, rhs: Self) { self.mul_assign(rhs) } }
        impl crate::RemAssign<Self> for $name<$t> {
            fn rem_assign(&mut self, rhs: Self) { self.rem_assign(rhs) } }
        impl crate::AddAssign<$t> for $name<$t> {
            fn add_assign(&mut self, rhs: $t) { self.add_scalar_assign(rhs) } }
        impl crate::SubAssign<$t> for $name<$t> {
            fn sub_assign(&mut self, rhs: $t) { self.sub_scalar_assign(rhs) } }
        impl crate::MulAssign<$t> for $name<$t> {
            fn mul_assign(&mut self, rhs: $t) { self.mul_scalar_assign(rhs) } }
        impl crate::RemAssign<$t> for $name<$t> {
            fn rem_assign(&mut self, rhs: $t) { self.rem_scalar_assign(rhs) } }
    }};
    (signed_types: $name:ident + $L:literal for $( $t:ty ),+) => {
        $( impl_lanes![one_signed_type: $name + $L for $t]; )+
    };
    (one_signed_type: $name:ident + $L:literal for $t:ty) => { $crate::paste! {
        /// Methods for signed primitives.
        impl $name<$t> {
            /// Negates each lane in place.
            pub const fn neg_assign(&mut self) {
                punroll! { $L |i| self.0[i] = -self.0[i] }
            }
        }
    }};
    (int_types: $name:ident + $L:literal for $( $t:ty ),+) => {
        $( impl_lanes![one_int_type: $name + $L for $t]; )+
    };
    (one_int_type: $name:ident + $L:literal for $t:ty) => { $crate::paste! {
        /// Methods for integer primitives.
        impl $name<$t> {
            /* arithmetic: elementwise */

            /// Adds each lane of `rhs` to the corresponding lane in `self`,
            /// saturating on overflow.
            pub const fn saturating_add_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] = self.0[i].saturating_add(rhs.0[i]) }
            }
            /// Adds each lane of `rhs` to the corresponding lane in `self`,
            /// wrapping on overflow.
            pub const fn wrapping_add_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] = self.0[i].wrapping_add(rhs.0[i]) }
            }

            /// Subtracts each lane of `rhs` from the corresponding lane in `self`,
            /// saturating on overflow.
            pub const fn saturating_sub_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] = self.0[i].saturating_sub(rhs.0[i]) }
            }
            /// Subtracts each lane of `rhs` from the corresponding lane in `self`,
            /// wrapping on overflow.
            pub const fn wrapping_sub_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] = self.0[i].wrapping_sub(rhs.0[i]) }
            }

            /// Multiplies each lane of `rhs` by the corresponding lane in `self`,
            /// wrapping on overflow.
            pub const fn wrapping_mul_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] = self.0[i].wrapping_mul(rhs.0[i]) }
            }

            /// Divides each lane by the scalar `rhs`, using an optimized [`Divisor`].
            /// # Panics
            /// Panics if `rhs == 0` or if signed division overflows.
            pub const fn div_scalar_fast_assign(&mut self, rhs: $t) {
                let d = Divisor::<$t>::new(rhs).expect("Divisor::new(0) is invalid");
                punroll! { $L |i| self.0[i] = d.div_of(self.0[i]) }
            }

            /* bitwise operations */

            /// Bitwise AND each lane with the corresponding lane in `rhs`.
            pub const fn bitand_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] &= rhs.0[i] }
            }

            /// Bitwise OR each lane with the corresponding lane in `rhs`.
            pub const fn bitor_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] |= rhs.0[i] }
            }

            /// Bitwise XOR each lane with the corresponding lane in `rhs`.
            pub const fn bitxor_assign(&mut self, rhs: Self) {
                punroll! { $L |i| self.0[i] ^= rhs.0[i] }
            }

            /* shifts */

            /// Shifts each lane left by `n`.
            pub const fn shl_assign(&mut self, n: u32) {
                punroll! { $L |i| self.0[i] <<= n }
            }

            /// Shifts each lane right by `n`.
            ///
            /// Performs an arithmetic right shift for signed integers,
            /// and a logical right shift for unsigned integers.
            pub const fn shr_assign(&mut self, n: u32) {
                punroll! { $L |i| self.0[i] >>= n }
            }
        }
    }};
}
impl_lanes![];
