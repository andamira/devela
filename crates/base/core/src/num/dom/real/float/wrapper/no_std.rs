// devela_base_core::num::dom::real::float::wrapper::no_std
//
//! Defines all the `no_std` public methods for `Float`.
//

use crate::Float;

/// $f:   the floating-point type.
/// $uf:  unsigned int type with the same bit-size.
/// $ie:  the integer type for integer exponentiation.
macro_rules! impl_float_no_std {
    () => {
        impl_float_no_std![(f32, u32, i32), (f64, u64, i32)];
    };
    ($( ($f:ty, $uf:ty, $ie:ty)),+) => {
        $( impl_float_no_std![@$f, $uf, $ie]; )+
    };
    (@$f:ty, $uf:ty, $ie:ty) => {
        /// # *Implementations without `std`
        impl Float<$f> {
            /// The largest integer less than or equal to itself.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_FLOOR!()]
            pub const fn floor(self) -> Float<$f> { self.const_floor() }

            /// The smallest integer greater than or equal to itself.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_CEIL!()]
            pub const fn ceil(self) -> Float<$f> { self.const_ceil() }

            /// The nearest integer to itself, default rounding
            ///
            /// This is the default [`round_ties_away`][Self::round_ties_away] implementation.
            pub const fn round(self) -> Float<$f> { self.const_round() }

            /// The nearest integer to itself, rounding ties away from `0.0`.
            ///
            /// This is the default [`round`][Self::round] implementation.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_ROUND_TIES_AWAY!()]
            pub const fn round_ties_away(self) -> Float<$f> {self.const_round_ties_away() }

            /// Returns the nearest integer to `x`, rounding ties to the nearest even integer.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_ROUND_TIES_EVEN!()]
            pub const fn round_ties_even(self) -> Float<$f> { self.const_round_ties_even() }

            /// The integral part.
            /// This means that non-integer numbers are always truncated towards zero.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_TRUNC!()]
            ///
            /// This implementation uses bitwise manipulation to remove the fractional part
            /// of the floating-point number. The exponent is extracted, and a mask is
            /// created to remove the fractional part. The new bits are then used to create
            /// the truncated floating-point number.
            pub const fn trunc(self) -> Float<$f> { self.const_trunc() }

            /// The fractional part.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_FRACT!()]
            pub const fn fract(self) -> Float<$f> { self.const_fract() }

            /// The integral and fractional parts.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_SPLIT!()]
            pub const fn split(self) -> (Float<$f>, Float<$f>) { self.const_split() }

            /// Raises itself to the `p` integer power.
            pub const fn powi(self, p: $ie) -> Float<$f> { self.const_powi(p) }
        }
    };
}
impl_float_no_std!();
