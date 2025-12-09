// devela_base_core::num::wide::lane
//
//! Defines [`define_lane!`] macro.
//

#[cfg(doc)]
define_lane! {
    /// Example fixed-width pack of 4 × `i32` lanes.
    ///
    /// Generated with [`define_lane!`].
    #[derive(Clone, Copy)]
    #[allow(non_camel_case_types)]
    pub struct ExampleLane4_i32 pub lanes(4); unsigned(i32);
}

/// Defines a fixed-width lane type and/or attaches implementations for specific primitive types.
///
/// # Example
/// ```
/// # use devela_base_core::define_lane;
/// // 1. Auto lane definition
/// define_lane!(auto Lane4 lanes(4); signed(i32); float(f32););
///
/// // 2. Custom lane definition
/// define_lane! {
///     /// Doc comments.
///     #[repr(transparent)]
///     #[derive(Copy, Clone, Debug)]
///     pub struct Lane8 pub(crate) lanes(8);
///     unsigned(u8, u16);
/// }
///
/// // 3. Implementation-only (attach new primitive impls to a previews definition)
/// define_lane![impl Lane8 lanes(8); signed(i64);];
/// ```
///
/// See also [`ExampleLane4_i32`] for the exact methods implementations.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! define_lane {
    (
    /* public macro arms */

        // 1. auto definition & impls
        auto $name:ident lanes($L:literal) ; $($rest:tt)*
    ) => {
        $crate::define_lane! {
            #[doc = concat!("A fixed-width pack of ", stringify!($L),
            " numeric lanes for parallel elementwise operations.")]
            #[repr(transparent)]
            #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name pub lanes($L);

            $($rest)*
        }
    };
    (
        // 2. custom struct definition & impls
        $(#[$attr:meta])*
        $vis:vis struct $name:ident $vis_inner:vis lanes($L:literal);
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        $vis struct $name<T>($vis_inner [T; $L]);

        impl<T: $crate::ConstInitCore> $crate::ConstInitCore for $name<T> {
            const INIT: Self = Self([T::INIT; $L]);
        }

        $crate::define_lane!(%impls $name : $L ; $($rest)*);
    };
    (
        // 3. only impls
        impl $name:ident lanes($L:literal) ; $($rest:tt)*

    /* private macro arms */

    ) => {
        $crate::define_lane!(%impls $name : $L ; $($rest)*);
    };
    //% impl group dispatch
    (%impls $name:ident : $L:literal ;) => {};
    (%impls $name:ident : $L:literal ; signed($($tys:ty),*); $($rest:tt)*) => {
        $crate::define_lane!(%$name : $L signed($($tys),*));
        $crate::define_lane!(%impls $name : $L ; $($rest)*);
    };
    (%impls $name:ident : $L:literal ; unsigned($($tys:ty),*); $($rest:tt)*) => {
        $crate::define_lane!(%$name : $L unsigned($($tys),*));
        $crate::define_lane!(%impls $name : $L ; $($rest)*);
    };
    (%impls $name:ident : $L:literal ; float($($tys:ty),*); $($rest:tt)*) => {
        $crate::define_lane!(%$name : $L float($($tys),*));
        $crate::define_lane!(%impls $name : $L ; $($rest)*);
    };
    //% type category dispatch
    (%$name:ident : $L:literal signed($($t:ty),+)) => {
        $(
            $crate::define_lane!(%impl_common $name : $L for $t);
            $crate::define_lane!(%impl_int $name : $L for $t);
            $crate::define_lane!(%impl_signed $name : $L for $t);
        )+
    };
    (%$name:ident : $L:literal unsigned($($t:ty),+)) => {
        $(
            $crate::define_lane!(%impl_common $name : $L for $t);
            $crate::define_lane!(%impl_int $name : $L for $t);
        )+
    };
    (%$name:ident : $L:literal float($($t:ty),+)) => {
        $(
            $crate::define_lane!(%impl_common $name : $L for $t);
            // $crate::define_lane!(%impl_float $name : $L for $t); // future?
            $crate::define_lane!(%impl_signed $name : $L for $t);
        )+
    };
    //% impl blocks for single types
    (%impl_common $name:ident : $L:literal for $t:ty) => { $crate::paste! {
        #[allow(dead_code)]
        /// Common methods for all integers and floating-point primitives.
        impl $name<$t> {
            /* broadcasting / constructing */

            #[doc = "Builds a lane pack from the first " $L " bytes of a byte slice."]
            /// # Panics
            #[doc = "Panics if `bytes.len() < " $L "`."]
            pub const fn from_bytes(bytes: &[u8]) -> Self {
                Self( $crate::punroll! { $L[] |i| bytes[i] as $t })
            }

            #[doc = "Builds a lane pack from the first " $L " elements of a slice."]
            /// # Panics
            #[doc = "Panics if `slice.len() < " $L "`."]
            pub const fn from_slice(slice: &[$t]) -> Self {
                Self( $crate::punroll! { $L[] |i| slice[i] })
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
                $crate::punroll! { $L |i| self.0[i] += rhs.0[i] }
            }

            /// Subtracts each lane of `rhs` from the corresponding lane in `self`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn sub_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] -= rhs.0[i] }
            }

            /// Multiplies each lane of `self` by the corresponding lane of `rhs`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn mul_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] *= rhs.0[i] }
            }

            /// Applies elementwise modular reduction.
            /// # Panics
            /// Panics if any divisor is zero.
            pub const fn rem_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] %= rhs.0[i] }
            }

            /// Divides each lane by the corresponding lane in `rhs` (truncating division).
            /// # Panics
            /// Panics if any divisor is zero or if signed division overflows.
            pub const fn div_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] /= rhs.0[i] }
            }

            /* arithmetic: scalar */

            /// Adds the scalar `rhs` to each lane in `self`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn add_scalar_assign(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] += rhs }
            }

            /// Subtracts the scalar `rhs` from each lane in `self`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn sub_scalar_assign(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] -= rhs }
            }

            /// Multiplies each lane by the scalar `rhs`.
            /// # Panics
            /// Panics on arithmetic overflow in debug and const evaluation.
            pub const fn mul_scalar_assign(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] *= rhs }
            }

            /// Applies scalar modular reduction to each lane.
            /// # Panics
            /// Panics if `rhs == 0`.
            pub const fn rem_scalar_assign(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] %= rhs }
            }

            /// Divides each lane by the scalar `rhs` (truncating division).
            /// # Panics
            /// Panics if `rhs == 0` or if signed division overflows.
            pub const fn div_scalar_assign(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] /= rhs }
            }

            /* comparisons and clamping */

            /// Returns the minimum lane value.
            pub const fn min(&self) -> $t {
                let mut m = self.0[0];
                $crate::punroll! { $L |i| if i != 0 && self.0[i] < m { m = self.0[i]; } }
                m
            }

            /// Returns the maximum lane value.
            pub const fn max(&self) -> $t {
                let mut m = self.0[0];
                $crate::punroll! { $L |i| if i != 0 && self.0[i] > m { m = self.0[i]; } }
                m
            }

            /// Clamps each lane to the inclusive `[lo, hi]` range.
            pub const fn clamp_assign(&mut self, lo: $t, hi: $t) {
                $crate::punroll! { $L |i| {
                    let x = self.0[i];
                    self.0[i] = if x < lo { lo } else if x > hi { hi } else { x };
                }}
            }

            /* reductions */

            /// Returns the sum of all lanes.
            pub const fn sum(&self) -> $t {
                let mut acc = self.0[0];
                $crate::punroll! { $L |i| if i != 0 { acc += self.0[i]; } }
                acc
            }
        }
        impl $crate::AddAssign<Self> for $name<$t> {
            fn add_assign(&mut self, rhs: Self) { self.add_assign(rhs) } }
        impl $crate::SubAssign<Self> for $name<$t> {
            fn sub_assign(&mut self, rhs: Self) { self.sub_assign(rhs) } }
        impl $crate::MulAssign<Self> for $name<$t> {
            fn mul_assign(&mut self, rhs: Self) { self.mul_assign(rhs) } }
        impl $crate::RemAssign<Self> for $name<$t> {
            fn rem_assign(&mut self, rhs: Self) { self.rem_assign(rhs) } }
        impl $crate::AddAssign<$t> for $name<$t> {
            fn add_assign(&mut self, rhs: $t) { self.add_scalar_assign(rhs) } }
        impl $crate::SubAssign<$t> for $name<$t> {
            fn sub_assign(&mut self, rhs: $t) { self.sub_scalar_assign(rhs) } }
        impl $crate::MulAssign<$t> for $name<$t> {
            fn mul_assign(&mut self, rhs: $t) { self.mul_scalar_assign(rhs) } }
        impl $crate::RemAssign<$t> for $name<$t> {
            fn rem_assign(&mut self, rhs: $t) { self.rem_scalar_assign(rhs) } }
    }};
    (%impl_int $name:ident : $L:literal for $t:ty) => { $crate::paste! {
        #[allow(dead_code)]
        /// Methods for integer primitives.
        impl $name<$t> {
            /* arithmetic: elementwise */

            /// Adds each lane of `rhs` to the corresponding lane in `self`,
            /// saturating on overflow.
            pub const fn saturating_add_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].saturating_add(rhs.0[i]) }
            }
            /// Adds each lane of `rhs` to the corresponding lane in `self`,
            /// wrapping on overflow.
            pub const fn wrapping_add_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].wrapping_add(rhs.0[i]) }
            }

            /// Subtracts each lane of `rhs` from the corresponding lane in `self`,
            /// saturating on overflow.
            pub const fn saturating_sub_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].saturating_sub(rhs.0[i]) }
            }
            /// Subtracts each lane of `rhs` from the corresponding lane in `self`,
            /// wrapping on overflow.
            pub const fn wrapping_sub_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].wrapping_sub(rhs.0[i]) }
            }

            /// Multiplies each lane of `rhs` by the corresponding lane in `self`,
            /// wrapping on overflow.
            pub const fn wrapping_mul_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].wrapping_mul(rhs.0[i]) }
            }

            /// Divides each lane by the scalar `rhs`, using an optimized
            /// [`Divisor`][crate::Divisor].
            /// # Panics
            /// Panics if `rhs == 0` or if signed division overflows.
            pub const fn div_scalar_fast_assign(&mut self, rhs: $t) {
                let d = $crate::Divisor::<$t>::new(rhs).expect("Divisor::new(0) is invalid");
                $crate::punroll! { $L |i| self.0[i] = d.div_of(self.0[i]) }
            }

            /* bitwise operations */

            /// Bitwise AND each lane with the corresponding lane in `rhs`.
            pub const fn bitand_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] &= rhs.0[i] }
            }

            /// Bitwise OR each lane with the corresponding lane in `rhs`.
            pub const fn bitor_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] |= rhs.0[i] }
            }

            /// Bitwise XOR each lane with the corresponding lane in `rhs`.
            pub const fn bitxor_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] ^= rhs.0[i] }
            }

            /* shifts */

            /// Shifts each lane left by `n`.
            pub const fn shl_assign(&mut self, n: u32) {
                $crate::punroll! { $L |i| self.0[i] <<= n }
            }

            /// Shifts each lane right by `n`.
            ///
            /// Performs an arithmetic right shift for signed integers,
            /// and a logical right shift for unsigned integers.
            pub const fn shr_assign(&mut self, n: u32) {
                $crate::punroll! { $L |i| self.0[i] >>= n }
            }
        }
    }};
    (%impl_signed $name:ident : $L:literal for $t:ty) => { $crate::paste! {
        #[allow(dead_code)]
        /// Methods for signed primitives (signed integers or floating-point);
        impl $name<$t> {
            /// Negates each lane in place.
            pub const fn neg_assign(&mut self) {
                $crate::punroll! { $L |i| self.0[i] = -self.0[i] }
            }
        }
    }};
}
#[doc(inline)]
pub use define_lane;

#[cfg(test)]
mod tests {
    #[test]
    fn definition() {
        define_lane! {
            /// Test docs.
            #[derive(Copy, Debug, Clone)]
            pub struct TestLane4 pub lanes(4);

            signed(i32, i64);
            unsigned(u32);
            float(f32);
        }
        // additional impls
        define_lane! {
            impl TestLane4 lanes(4);
            signed(i8, i16);
            unsigned(u8);
            float(f64);
        }

        let _l = TestLane4::<f32>::splat(45.0);
    }
}
