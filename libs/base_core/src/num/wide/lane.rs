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
    #[derive(Clone, Copy)]
    #[allow(non_camel_case_types)]
    pub struct ExampleLane4_i32 pub lanes(4); unsigned(i32);
}

// consts for methods's docs
crate::CONST! { hidden macro_export,
_ADD_ASSIGN_SIMD = "Adds each lane of `rhs` to the corresponding lane in `self` using SIMD.\n\n
Performs elementwise addition with the semantics of the underlying element type.\n\n
Integer addition wraps on overflow; floating-point follows IEEE-754 rules.";
_SUB_ASSIGN_SIMD = "Subtracts each lane of `rhs` from the corresponding lane in `self` using SIMD.
\n\nPerforms elementwise subtraction with the semantics of the underlying element type.\n\n
Integer subtraction wraps on overflow; floating-point follows IEEE-754 rules.
";
_MUL_ASSIGN_SIMD = "Multiplies each lane of `self` by the corresponding lane of `rhs` using SIMD.
\n\nPerforms elementwise multiplication with the semantics of the underlying element type.\n\n
Integer multiplication wraps on overflow; floating-point follows IEEE-754 rules.";
// only floats
_DIV_ASSIGN_SIMD = "Divides each lane of `self` by the corresponding lane of `rhs` using SIMD.\n\n
Applies elementwise division following IEEE-754 semantics.
Not available for integer types.";
// _REM_ASSIGN_SIMD = "Applies elementwise remainder using SIMD.\n\n
// Uses the remainder semantics of the underlying element type.
// Availability depends on SIMD backend support.";
_NEG_ASSIGN_SIMD = "Negates each lane in `self` using SIMD.\n\n
Available for signed integer and floating-point element types.";
_ADD_SCALAR_ASSIGN_SIMD = "Adds the scalar `rhs` to every lane in `self` using SIMD.\n\n
Integer addition wraps on overflow; floating-point follows IEEE-754 rules.";
_SUB_SCALAR_ASSIGN_SIMD = "Subtracts the scalar `rhs` from every lane in `self` using SIMD.\n\n
Integer subtraction wraps on overflow; floating-point follows IEEE-754 rules.";
_MUL_SCALAR_ASSIGN_SIMD = "Multiplies every lane in `self` by the scalar `rhs` using SIMD.\n\n
Integer multiplication wraps on overflow; floating-point follows IEEE-754 rules.";
// only floats
_DIV_SCALAR_ASSIGN_SIMD = "Divides every lane in `self` by the scalar `rhs` using SIMD.\n\n
Follows IEEE-754 division semantics.";
/* only ints */
_BITAND_ASSIGN_SIMD = "Applies a bitwise AND between each lane of `self` and `rhs` using SIMD.";
_BITOR_ASSIGN_SIMD = "Applies a bitwise OR between each lane of `self` and `rhs` using SIMD.";
_BITXOR_ASSIGN_SIMD = "Applies a bitwise XOR between each lane of `self` and `rhs` using SIMD.";
_SHL_ASSIGN_SIMD = "Shifts each lane in `self` left by the scalar amount `rhs` using SIMD.\n\n
Uses the shift semantics of the underlying integer type.";
_SHR_ASSIGN_SIMD = "Shifts each lane in `self` right by the scalar amount `rhs` using SIMD.\n\n
Applies arithmetic right shift for signed integers and logical right shift for unsigned integers.";
/* other */
_SUM_SIMD = "Returns the sum of all lanes using SIMD acceleration.\n\n
Follows the addition semantics of the underlying element type.
Integer sums wrap on overflow; floating-point follows IEEE-754 rules.";
_MIN_SIMD = "Returns the minimum value across all lanes using SIMD acceleration.\n\n
Follows the comparison semantics of the underlying element type.";
_MAX_SIMD = "Returns the maximum value across all lanes using SIMD acceleration.\n\n
Follows the comparison semantics of the underlying element type.";
_CLAMP_ASSIGN_SIMD = "Clamps each lane of `self` into the inclusive range `[lo, hi]` using SIMD.\n\n
Follows the comparison semantics of the underlying element type.";
}

/// Defines a fixed-width lane type and/or attaches implementations for specific primitive types.
///
/// # Example
/// ```
/// # #![cfg_attr(nightly_doc, feature(doc_cfg))]
/// # #![cfg_attr(nightly_simd, feature(portable_simd))]
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
            /// Panics on integer overflow (debug and const only).
            pub const fn add_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] += rhs.0[i] }
            }
            #[doc = $crate::_ADD_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn add_assign_simd(&mut self, rhs: Self) {
                type Simd = $crate::Simd<$t, $L>;
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a + b).to_array();
            }
            $crate::_dep_wide_compile! { for ALL $t, $L;
                #[doc = $crate::_ADD_ASSIGN_SIMD!()]
                pub fn add_assign_simd_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a + b).into();
                }
            }

            /// Subtracts each lane of `rhs` from the corresponding lane in `self`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn sub_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] -= rhs.0[i] }
            }
            #[doc = $crate::_SUB_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn sub_assign_simd(&mut self, rhs: Self) {
                type Simd = $crate::Simd<$t, $L>;
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a - b).to_array();
            }
            $crate::_dep_wide_compile! { for ALL $t, $L;
                #[doc = $crate::_SUB_ASSIGN_SIMD!()]
                pub fn sub_assign_simd_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a + b).into();
                }
            }

            /// Multiplies each lane of `self` by the corresponding lane of `rhs`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn mul_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] *= rhs.0[i] }
            }
            #[doc = $crate::_MUL_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn mul_assign_simd(&mut self, rhs: Self) {
                type Simd = $crate::Simd<$t, $L>;
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a * b).to_array();
            }
            $crate::_dep_wide_compile! { for ALL $t, $L;
                #[doc = $crate::_MUL_ASSIGN_SIMD!()]
                pub fn mul_assign_simd_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a + b).into();
                }
            }

            /// Applies elementwise modular reduction.
            /// # Panics
            /// Panics if any divisor is zero.
            pub const fn rem_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] %= rhs.0[i] }
            }

            /// Divides each lane by the corresponding lane in `rhs` (truncating division).
            /// # Panics
            /// Panics if any divisor is zero or on signed integer overflow.
            // NOTE: simd version is only availabe for floating-point.
            pub const fn div_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] /= rhs.0[i] }
            }

            // NOTE: neg_assign has separated impls for integers and floats.

            /* arithmetic: scalar */

            /// Adds the scalar `rhs` to each lane in `self`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn add_scalar_assign(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] += rhs }
            }

            /// Subtracts the scalar `rhs` from each lane in `self`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            pub const fn sub_scalar_assign(&mut self, rhs: $t) {
                $crate::punroll! { $L |i| self.0[i] -= rhs }
            }

            /// Multiplies each lane by the scalar `rhs`.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
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
            /// Panics if `rhs == 0` or on signed integer overflow.
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

            /// Negates each lane in place.
            /// # Panics
            /// Panics on integer overflow (debug and const only).
            ///
            /// For signed integers, overflow occurs when the value is the minimum representable
            /// integer. Unsigned integers wrap according to two's complement semantics.
            pub const fn neg_assign(&mut self) {
                $crate::punroll! { $L |i| self.0[i] = (0 as $t).wrapping_sub(self.0[i]) }
            }

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

            /* alternative fast integer division */

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
            #[doc = $crate::_BITAND_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn bitand_assign_simd(&mut self, rhs: Self) {
                type Simd = $crate::Simd<$t, $L>;
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a & b).to_array();
            }
            $crate::_dep_wide_compile! { for INT $t, $L;
                #[doc = $crate::_BITAND_ASSIGN_SIMD!()]
                pub fn bitand_assign_simd_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a & b).into();
                }
            }

            /// Bitwise OR each lane with the corresponding lane in `rhs`.
            pub const fn bitor_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] |= rhs.0[i] }
            }
            #[doc = $crate::_BITOR_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn bitor_assign_simd(&mut self, rhs: Self) {
                type Simd = $crate::Simd<$t, $L>;
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a | b).to_array();
            }
            $crate::_dep_wide_compile! { for INT $t, $L;
                #[doc = $crate::_BITOR_ASSIGN_SIMD!()]
                pub fn bitor_assign_simd_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a | b).into();
                }
            }

            /// Bitwise XOR each lane with the corresponding lane in `rhs`.
            pub const fn bitxor_assign(&mut self, rhs: Self) {
                $crate::punroll! { $L |i| self.0[i] ^= rhs.0[i] }
            }
            #[doc = $crate::_BITXOR_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn bitxor_assign_simd(&mut self, rhs: Self) {
                type Simd = $crate::Simd<$t, $L>;
                let a = Simd::from_array(self.0); let b = Simd::from_array(rhs.0);
                self.0 = (a ^ b).to_array();
            }
            $crate::_dep_wide_compile! { for INT $t, $L;
                #[doc = $crate::_BITXOR_ASSIGN_SIMD!()]
                pub fn bitxor_assign_simd_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a ^ b).into();
                }
            }

            /* shifts */

            /// Shifts each lane left by `n`.
            pub const fn shl_assign(&mut self, n: $t) {
                $crate::punroll! { $L |i| self.0[i] <<= n }
            }
            #[doc = $crate::_SHL_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn shl_assign_simd(&mut self, n: $t) {
                type Simd = $crate::Simd<$t, $L>;
                let a = Simd::from_array(self.0);
                self.0 = (a << n).to_array();
            }
            $crate::_dep_wide_compile! { for SHIFT $t, $L;
                #[doc = $crate::_SHL_ASSIGN_SIMD!()]
                pub fn shl_assign_simd_wide(&mut self, n: $t) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0);
                    self.0 = (a << n).into();
                }
            }

            /// Shifts each lane right by `n`.
            ///
            /// Performs an arithmetic right shift for signed integers,
            /// and a logical right shift for unsigned integers.
            pub const fn shr_assign(&mut self, n: $t) {
                $crate::punroll! { $L |i| self.0[i] >>= n }
            }
            #[doc = $crate::_SHR_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn shr_assign_simd(&mut self, n: $t) {
                type Simd = $crate::Simd<$t, $L>;
                let a = Simd::from_array(self.0);
                self.0 = (a >> n).to_array();
            }
            $crate::_dep_wide_compile! { for SHIFT $t, $L;
                #[doc = $crate::_SHR_ASSIGN_SIMD!()]
                pub fn shr_assign_simd_wide(&mut self, n: $t) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0);
                    self.0 = (a >> n).into();
                }
            }
        }
    }};
    (%impl_float $name:ident : $L:literal for $t:ty) => { $crate::paste! {
        /// Methods for floating-point primitives.
        #[allow(dead_code)]
        impl $name<$t> {
            #[doc = $crate::_DIV_ASSIGN_SIMD!()]
            #[cfg(nightly_simd)]
            #[cfg_attr(nightly_doc, doc(cfg(nightly_simd)))]
            pub fn div_assign_simd(&mut self, rhs: Self) {
                let a = $crate::Simd::<$t, $L>::from_array(self.0);
                let b = $crate::Simd::<$t, $L>::from_array(rhs.0);
                self.0 = (a / b).to_array();
            }
            $crate::_dep_wide_compile! { for ALL $t, $L;
                #[doc = $crate::_DIV_ASSIGN_SIMD!()]
                pub fn div_assign_simd_wide(&mut self, rhs: Self) {
                    $crate::_dep_wide_use!($t, $L);
                    let a = Wide::new(self.0); let b = Wide::new(rhs.0);
                    self.0 = (a / b).into();
                }
            }

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
    /* definitions */

    define_lane! {
        #[doc = "doc"] #[derive(Copy, Debug, Clone)]
        pub struct TestLane4 pub lanes(4);
        signed(i32);
        float(f32);
    }

    /* define the rest supported by `dep_wide` */
    // NOTE: increases compile time a few secs

    define_lane! {
        pub struct TestLane2 pub lanes(2);
        unsigned(u64);
        signed(i64);
        float(f64);
    }
    // add extra implementations to an existing definition
    define_lane! {
        impl TestLane4 lanes(4);
        unsigned(u8, u16, u32, u64);
        signed(i8, i16, i64);
        float(f64);
    }
    define_lane! {
        pub struct TestLane8 pub lanes(8);
        unsigned(u16, u32, u64);
        signed(i16, i32, i64);
        float(f32, f64);
    }
    define_lane! {
        pub struct TestLane16 pub lanes(16);
        unsigned(u8, u16, u32);
        signed(i8, i16, i32);
        float(f32);
    }
    define_lane! {
        pub struct TestLane32 pub lanes(32);
        unsigned(u8, u16);
        signed(i8, i16);
    }

    /* tests */

    #[test]
    fn base() {
        let mut i1 = TestLane4::<i32>::splat(10);
        let i2 = TestLane4::<i32>::splat(20);
        i1.add_assign(i2);
        assert_eq![i1.0, [30, 30, 30, 30]];

        let mut f1 = TestLane4::<f32>::splat(10.);
        let f2 = TestLane4::<f32>::splat(5.);
        f1.add_assign(f2);
        assert_eq![f1.0, [15., 15., 15., 15.]];
    }
    #[test]
    #[cfg(nightly_simd)]
    fn simd_portable() {
        let mut i1 = TestLane4::<i32>::splat(10);
        let i2 = TestLane4::<i32>::splat(20);
        i1.add_assign_simd(i2);
        assert_eq![i1.0, [30, 30, 30, 30]];

        let mut f1 = TestLane4::<f32>::splat(10.);
        let f2 = TestLane4::<f32>::splat(5.);
        f1.div_assign_simd(f2);
        assert_eq![f1.0, [2., 2., 2., 2.]];
    }

    #[test]
    #[cfg(feature = "dep_wide")]
    // TODO: test systematically
    fn simd_wide() {
        /* 4 */

        let mut i1 = TestLane4::<i32>::splat(10);
        let i2 = TestLane4::<i32>::splat(20);
        i1.add_assign_simd_wide(i2);
        assert_eq![i1.0, [30, 30, 30, 30]];

        /* 32 */

        // let mut i1 = TestLane32::<u16>::splat(10);
        // let i2 = TestLane32::<u16>::splat(20);
        // i1.add_assign_simd_wide(i2);
    }
}
