// devela_base_core::num::wide::lane
//
//! Defines [`define_lane!`] macro.
//
// TOC
// - struct ExampleLn4_i32
// - consts for method's docs
// - macro define_lane!
// - tests

#[cfg(feature = "_docs_min")]
define_lane! {
    /// Example fixed-width pack of 4 × `i32` lanes.
    ///
    /// Generated with [`define_lane!`].
    #[derive(Clone, Copy, Debug)]
    #[allow(non_camel_case_types)]
    pub struct ExampleLane4_i32 pub lanes(4); unsigned(i32);
}

/// Defines a fixed-width lane type and/or attaches implementations for specific primitive types.
///
/// It offers parallel APIs, depending on the method suffix.
/// - `_plain`: A plain, compile-time friendly, unrolled fallback.
/// - `_simd`:  A nightly-only portable SIMD implementation, that will stabilize some day.
/// - `_wide`:  A stable SIMD implementation, leveraging the `wide` crate.
// And the method without a suffix chooses from the available XXX
// in the following order of preference, from highe to lowest: simd → wide → plain.
///
/// # Example
/// ```
/// # #![cfg_attr(nightly_doc, feature(doc_cfg))]
/// # #![cfg_attr(nightly_simd, feature(portable_simd))]
/// # use devela_base_core::define_lane;
/// // 1. Auto lane definition
/// define_lane!(auto Lane4 lanes(4); signed(i32); float(f32););
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
            $crate::define_lane!(%impl_float $name : $L for $t);
        )+
    };
    //% impl blocks for single types
    (%impl_common $name:ident : $L:literal for $t:ty) => { $crate::paste! {
        #[allow(dead_code)]
        #[allow(unexpected_cfgs)]
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
            // TODO: pub const fn splat_plain(v: $t) -> Self {}

            /* arithmetic: elementwise */

            #[inline(always)]
            /// Adds each lane of `rhs` to the corresponding lane in `self`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn add_assign(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(self, add_assign(rhs));
            }
            /// Adds each lane of `rhs` to the corresponding lane in `self`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn add_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] += rhs.0[i] }
            }
            #[doc = $crate::_ADD_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn add_assign_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a + b).to_array();
            }
            $crate::_dep_wide_compile! { for ALL_OR_ELSE $t, $L;
                #[doc = $crate::_ADD_ASSIGN_SIMD!()]
                pub fn add_assign_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a + b).into();
                } else {
                    $crate::punroll! { $L |i| self.0[i] += rhs.0[i] }
                }
            }

            #[inline(always)]
            /// Subtracts each lane of `rhs` from the corresponding lane in `self`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn sub_assign(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(self, sub_assign(rhs));
            }
            /// Subtracts each lane of `rhs` from the corresponding lane in `self`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn sub_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] -= rhs.0[i] }
            }
            #[doc = $crate::_SUB_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn sub_assign_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a - b).to_array();
            }
            $crate::_dep_wide_compile! { for ALL_OR_ELSE $t, $L;
                #[doc = $crate::_SUB_ASSIGN_SIMD!()]
                pub fn sub_assign_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a + b).into();
                } else {
                    $crate::punroll! { $L |i| self.0[i] -= rhs.0[i] }
                }
            }

            #[inline(always)]
            /// Multiplies each lane of `self` by the corresponding lane of `rhs`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn mul_assign(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(self, mul_assign(rhs));
            }
            /// Multiplies each lane of `self` by the corresponding lane of `rhs`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn mul_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] *= rhs.0[i] }
            }
            #[doc = $crate::_MUL_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn mul_assign_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a * b).to_array();
            }
            $crate::_dep_wide_compile! { for ALL_OR_ELSE $t, $L;
                #[doc = $crate::_MUL_ASSIGN_SIMD!()]
                pub fn mul_assign_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a + b).into();
                } else {
                    $crate::punroll! { $L |i| self.0[i] *= rhs.0[i] }
                }
            }

            #[inline(always)]
            /// Applies elementwise modular reduction.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn rem_assign(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(plain: self, mul_assign(rhs)); // IMPROVE
            }
            /// Applies elementwise modular reduction.
            /// # Panics
            /// Panics if any divisor is zero.
            pub const fn rem_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] %= rhs.0[i] }
            }

            // NOTE: div_assign is separated between int and floats
            /// Divides each lane by the corresponding lane in `rhs` (truncating division).
            /// # Panics
            /// Panics if any divisor is zero or on signed integer overflow.
            pub const fn div_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] /= rhs.0[i] }
            }
            // NOTE: div_assign_wide only supports floats
            #[doc = $crate::_DIV_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn div_assign_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a / b).to_array();
            }

            #[inline(always)]
            /// Negates each lane in place.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn neg_assign(&mut self) {
                $crate::__lane_dispatch!(plain: self, neg_assign()); // IMPROVE
            }
            // NOTE: neg_assign_plain has separated impls for integers and floats.

            /* arithmetic: scalar */

            // TODO: add scalar SIMD impls
            #[inline(always)]
            /// Adds the scalar `rhs` to each lane in `self`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn add_scalar_assign(&mut self, rhs: $t) {
                $crate::__lane_dispatch!(plain: self, add_scalar_assign(rhs)); // IMPROVE
            }
            /// Adds the scalar `rhs` to each lane in `self`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn add_scalar_assign_plain(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] += rhs }
            }

            #[inline(always)]
            /// Subtracts the scalar `rhs` from each lane in `self`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn sub_scalar_assign(&mut self, rhs: $t) {
                $crate::__lane_dispatch!(plain: self, sub_scalar_assign(rhs)); // IMPROVE
            }
            /// Subtracts the scalar `rhs` from each lane in `self`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn sub_scalar_assign_plain(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] -= rhs }
            }

            #[inline(always)]
            /// Multiplies each lane by the scalar `rhs`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn mul_scalar_assign(&mut self, rhs: $t) {
                $crate::__lane_dispatch!(plain: self, mul_scalar_assign(rhs)); // IMPROVE
            }
            /// Multiplies each lane by the scalar `rhs`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn mul_scalar_assign_plain(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] *= rhs }
            }

            #[inline(always)]
            /// Applies scalar modular reduction to each lane.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn rem_scalar_assign(&mut self, rhs: $t) {
                $crate::__lane_dispatch!(plain: self, rem_scalar_assign(rhs)); // IMPROVE
            }
            /// Applies scalar modular reduction to each lane.
            /// # Panics
            /// Panics if `rhs == 0`.
            pub const fn rem_scalar_assign_plain(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] %= rhs }
            }

            #[inline(always)]
            /// Divides each lane by the scalar `rhs` (truncating division).
            #[doc = $crate::_LANE_AUTO!()]
            pub fn div_scalar_assign(&mut self, rhs: $t) {
                $crate::__lane_dispatch!(plain: self, div_scalar_assign(rhs)); // IMPROVE
            }
            /// Divides each lane by the scalar `rhs` (truncating division).
            /// # Panics
            /// Panics if `rhs == 0` or on signed integer overflow.
            pub const fn div_scalar_assign_plain(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] /= rhs }
            }

            /* comparisons and clamping */

            #[inline(always)]
            /// Returns the minimum lane value.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn min(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(no_wide: self, min(rhs)); // BUG
            }
            /// Returns the minimum lane value.
            pub const fn min_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| if rhs.0[i] < self.0[i] { self.0[i] = rhs.0[i]; } }
            }
            // NOTE: min_simd has separate implementation for integer and floats
            $crate::_dep_wide_compile! { for ALL_OR_ELSE $t, $L;
                #[doc = $crate::_MIN_SIMD!()]
                pub fn min_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = a.min(b).into();
                } else {
                    $crate::punroll! { $L |i| if rhs.0[i] < self.0[i] { self.0[i] = rhs.0[i]; } }
                }
            }

            #[inline(always)]
            /// Returns the maximum lane value.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn max(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(no_wide: self, max(rhs)); // BUG
            }
            /// Returns the maximum lane value.
            pub const fn max_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| if rhs.0[i] > self.0[i] { self.0[i] = rhs.0[i]; } }
            }
            // NOTE: max_simd has separate implementation for integer and floats
            $crate::_dep_wide_compile! { for ALL_OR_ELSE $t, $L;
                #[doc = $crate::_MAX_SIMD!()]
                pub fn max_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = a.max(b).into();
                } else {
                    $crate::punroll! { $L |i| if rhs.0[i] > self.0[i] { self.0[i] = rhs.0[i]; } }
                }
            }

            #[inline(always)]
            /// Clamps each lane to the inclusive `[lo, hi]` range.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn clamp_assign(&mut self, lo: $t, hi: $t) {
                $crate::__lane_dispatch!(plain: self, clamp_assign(lo, hi)); // IMPROVE
            }
            /// Clamps each lane to the inclusive `[lo, hi]` range.
            pub const fn clamp_assign_plain(&mut self, lo: $t, hi: $t) {
                $crate::punroll! { $L |i| {
                    let x = self.0[i];
                    self.0[i] = if x < lo { lo } else if x > hi { hi } else { x };
                }}
            }

            /* reductions */

            /// Returns the horizontal minimum of all lanes.
            pub const fn min_reduce_plain(&self) -> $t {
                let mut m = self.0[0];
                $crate::punroll! { $L |i| if i != 0 && self.0[i] < m { m = self.0[i]; } }
                m
            }
            // TODO: max_reduce_simd → reduce_min (separate for int|float)
            // CHECK: max_reduce_wide

            /// Returns the horizontal maximum of all lanes.
            pub const fn max_reduce_plain(&self) -> $t {
                let mut m = self.0[0];
                $crate::punroll! { $L |i| if i != 0 && self.0[i] > m { m = self.0[i]; } }
                m
            }
            // TODO: min_reduce_simd → reduce_min (separate for int|float)
            // CHECK: min_reduce_wide

            #[inline(always)]
            /// Returns the sum of all lanes.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn sum(&mut self) {
                $crate::__lane_dispatch!(plain: self, sum()); // IMPROVE
            }
            /// Returns the sum of all lanes.
            pub const fn sum_plain(&self) -> $t {
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
    }}; // %impl_common
    (%impl_int $name:ident : $L:literal for $t:ty) => { $crate::paste! {
        #[allow(dead_code)]
        #[allow(unexpected_cfgs)]
        /// Methods for integer primitives.
        impl $name<$t> {
            /* arithmetic: elementwise */

            #[inline(always)]
            /// Divides each lane by the corresponding lane in `rhs` (truncating division).
            #[doc = $crate::_LANE_AUTO!()]
            pub fn div_assign(&mut self, rhs: Self) {
                // NOTE: div_assign_wide impl does not support integers
                $crate::__lane_dispatch!(no_wide: self, div_assign(rhs));
            }

            /// Negates each lane in place.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            ///
            /// For signed integers, overflow occurs when the value is the minimum representable
            /// integer. Unsigned integers wrap according to two's complement semantics.
            // NOTE: separate implementations for floating-point and integers.
            pub const fn neg_assign_plain(&mut self) {
                $crate::punroll! { $L |i| self.0[i] = (0 as $t).wrapping_sub(self.0[i]) }
            }

            /* saturating and wrapping (no simd impls) */

            /// Adds each lane of `rhs` to the corresponding lane in `self`,
            /// saturating on overflow.
            pub const fn saturating_add_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].saturating_add(rhs.0[i]) }
            }
            /// Adds each lane of `rhs` to the corresponding lane in `self`,
            /// wrapping on overflow.
            pub const fn wrapping_add_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].wrapping_add(rhs.0[i]) }
            }

            /// Subtracts each lane of `rhs` from the corresponding lane in `self`,
            /// saturating on overflow.
            pub const fn saturating_sub_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].saturating_sub(rhs.0[i]) }
            }
            /// Subtracts each lane of `rhs` from the corresponding lane in `self`,
            /// wrapping on overflow.
            pub const fn wrapping_sub_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].wrapping_sub(rhs.0[i]) }
            }

            /// Multiplies each lane of `rhs` by the corresponding lane in `self`,
            /// wrapping on overflow.
            pub const fn wrapping_mul_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] = self.0[i].wrapping_mul(rhs.0[i]) }
            }

            /* alternative fast integer division */

            /// Divides each lane by the scalar `rhs`, using an optimized
            /// [`Divisor`][crate::Divisor].
            /// # Panics
            /// Panics if `rhs == 0` or if signed division overflows.
            pub const fn div_scalar_fast_assign_plain(&mut self, rhs: $t) {
                let d = $crate::Divisor::<$t>::new(rhs).expect("Divisor::new(0) is invalid");
                $crate::punroll! { $L |i| self.0[i] = d.div_of(self.0[i]) }
            }

            /* bitwise operations */

            #[inline(always)]
            /// Bitwise AND each lane with the corresponding lane in `rhs`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn bitand_assign(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(self, bitand_assign(rhs));
            }
            /// Bitwise AND each lane with the corresponding lane in `rhs`.
            pub const fn bitand_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] &= rhs.0[i] }
            }
            #[doc = $crate::_BITAND_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn bitand_assign_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a & b).to_array();
            }
            $crate::_dep_wide_compile! { for INT_OR_ELSE $t, $L;
                #[doc = $crate::_BITAND_ASSIGN_SIMD!()]
                pub fn bitand_assign_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a & b).into();
                } else {
                    $crate::punroll! { $L |i| self.0[i] &= rhs.0[i] }
                }
            }

            #[inline(always)]
            /// Bitwise OR each lane with the corresponding lane in `rhs`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn bitor_assign(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(self, bitor_assign(rhs));
            }
            /// Bitwise OR each lane with the corresponding lane in `rhs`.
            pub const fn bitor_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] |= rhs.0[i] }
            }
            #[doc = $crate::_BITOR_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn bitor_assign_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a | b).to_array();
            }
            $crate::_dep_wide_compile! { for INT_OR_ELSE $t, $L;
                #[doc = $crate::_BITOR_ASSIGN_SIMD!()]
                pub fn bitor_assign_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a | b).into();
                } else {
                    $crate::punroll! { $L |i| self.0[i] |= rhs.0[i] }
                }
            }

            #[inline(always)]
            /// Bitwise XOR each lane with the corresponding lane in `rhs`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn bitxor_assign(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(self, bitxor_assign(rhs));
            }
            /// Bitwise XOR each lane with the corresponding lane in `rhs`.
            pub const fn bitxor_assign_plain(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] ^= rhs.0[i] }
            }
            #[doc = $crate::_BITXOR_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn bitxor_assign_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a ^ b).to_array();
            }
            $crate::_dep_wide_compile! { for INT_OR_ELSE $t, $L;
                #[doc = $crate::_BITXOR_ASSIGN_SIMD!()]
                pub fn bitxor_assign_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a ^ b).into();
                } else {
                    $crate::punroll! { $L |i| self.0[i] ^= rhs.0[i] }
                }
            }

            /* shifts */

            #[inline(always)]
            /// Shifts each lane left by `n`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn shl_assign(&mut self, n: $t) {
                $crate::__lane_dispatch!(self, shl_assign(n));
            }
            /// Shifts each lane left by `n`.
            pub const fn shl_assign_plain(&mut self, n: $t) {
                $crate::punroll! { $L |i| self.0[i] <<= n }
            }
            #[doc = $crate::_SHL_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn shl_assign_simd(&mut self, n: $t) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0);
                self.0 = (a << n).to_array();
            }
            $crate::_dep_wide_compile! { for SHIFT_OR_ELSE $t, $L;
                #[doc = $crate::_SHL_ASSIGN_SIMD!()]
                pub fn shl_assign_wide(&mut self, n: $t) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0);
                    self.0 = (a << n).into();
                } else {
                    $crate::punroll! { $L |i| self.0[i] <<= n }
                }
            }

            #[inline(always)]
            /// Shifts each lane right by `n`.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn shr_assign(&mut self, n: $t) {
                $crate::__lane_dispatch!(self, shr_assign(n));
            }
            /// Shifts each lane right by `n`.
            ///
            /// Performs an arithmetic right shift for signed integers,
            /// and a logical right shift for unsigned integers.
            pub const fn shr_assign_plain(&mut self, n: $t) {
                $crate::punroll! { $L |i| self.0[i] >>= n }
            }
            #[doc = $crate::_SHR_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn shr_assign_simd(&mut self, n: $t) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0);
                self.0 = (a >> n).to_array();
            }
            $crate::_dep_wide_compile! { for SHIFT_OR_ELSE $t, $L;
                #[doc = $crate::_SHR_ASSIGN_SIMD!()]
                pub fn shr_assign_wide(&mut self, n: $t) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0);
                    self.0 = (a >> n).into();
                } else {
                    $crate::punroll! { $L |i| self.0[i] >>= n }
                }
            }

            /* comparison, reduction */

            // NOTE: separate implementations for floating-point and integers.
            #[doc = $crate::_MIN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn min_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = a.min(b).to_array();
            }

            // NOTE: separate implementations for floating-point and integers.
            #[doc = $crate::_MAX_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn max_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = a.max(b).to_array();
            }
        }
    }}; // %impl_int
    (%impl_float $name:ident : $L:literal for $t:ty) => { $crate::paste! {
        /// Methods for floating-point primitives.
        #[allow(dead_code)]
        #[allow(unexpected_cfgs)]
        impl $name<$t> {
            /// Divides each lane by the corresponding lane in `rhs` (truncating division).
            ///
            /// This method is only available for floating-points.
            #[doc = $crate::_LANE_AUTO!()]
            pub fn div_assign(&mut self, rhs: Self) {
                $crate::__lane_dispatch!(self, div_assign(rhs));
            }
            // NOTE: div_assign_wide only support floats
            $crate::_dep_wide_compile! { for ALL_OR_ELSE $t, $L;
                #[doc = $crate::_DIV_ASSIGN_SIMD!()]
                pub fn div_assign_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a / b).into();
                } else {
                    $crate::punroll! { $L |i| self.0[i] /= rhs.0[i] }
                }
            }

            /// Negates each lane in place.
            // NOTE: separate implementations for floating-point and integers.
            pub const fn neg_assign_plain(&mut self) {
                $crate::punroll! { $L |i| self.0[i] = -self.0[i] }
            }

            /* comparison, reduction */

            // NOTE: separate implementations for floating-point and integers.
            #[doc = $crate::_MIN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn min_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = a.simd_min(b).to_array();
            }
            // NOTE: separate implementations for floating-point and integers.
            #[doc = $crate::_MAX_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn max_simd(&mut self, rhs: Self) {
                $crate::_simd_use!($t, $L);
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = a.simd_max(b).to_array();
            }
        }
    }}; // %impl_float
}
#[doc(inline)]
pub use define_lane;
