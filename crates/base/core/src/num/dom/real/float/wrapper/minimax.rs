// devela_base_core::num::dom::real::float::wrapper::minimax
//
//! Shared methods implemented using Horner minimax.
//
// TODO: f16, 128
// NOTE: this file is symlinked from devela_base_std::num::dom::real::float::wrapper

use super::definition::Float;
use crate::{_FloatInternals, is};

/// Implements methods independently of any features
///
/// $f:   the floating-point type.
/// $uf:  unsigned int type with the same bit-size.
/// $ue:  unsigned int type used for integer exponentiation and number of terms (u32).
macro_rules! impl_float_shared_minimax {
    () => {
        impl_float_shared_minimax![(f32:u32, u32), (f64:u64, u32)];
    };
    ($( ($f:ty:$uf:ty, $ue:ty)),+ $(,)?) => {
        $( impl_float_shared_minimax![@$f:$uf, $ue]; )+
    };
    (@$f:ty:$uf:ty, $ue:ty) => {
        ///
        /// # *Common methods implemented using Taylor series*
        impl Float<$f> {
            /// The sine calculated using a minimax polynomial evaluated by Horner's method.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_SIN_MINIMAX!()]
            ///
            /// This approximation uses a degree-N minimax polynomial over the range
            /// [-π/2, π/2], combined with argument reduction. It provides higher accuracy
            /// than the Taylor series for the same number of terms and avoids divisions.
            pub const fn sin_minimax(self) -> Float<$f> {
                let (x, _flip) = self.reduce_half_pi();
                x.sin_minimax_from_reduced()
            }

            /// The cosine calculated using a minimax polynomial evaluated by Horner's method.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_COS_MINIMAX!()]
            ///
            /// This approximation uses a degree-N minimax polynomial over the range
            /// [-π/2, π/2], combined with argument reduction. It provides higher accuracy
            /// than the Taylor series for the same number of terms and avoids divisions.
            pub const fn cos_minimax(self) -> Float<$f> {
                let (x, flip) = self.reduce_half_pi();
                x.cos_minimax_from_reduced(flip)
            }

            /// Computes the sine and the cosine
            /// using a minimax polynomial evaluated by Horner's method.
            ///
            /// The current implementation computes separately the sine and the cosine.
            #[inline(always)]
            pub const fn sin_cos_minimax(self) -> (Float<$f>, Float<$f>) {
                let (x, flip) = self.reduce_half_pi();
                (x.sin_minimax_from_reduced(), x.cos_minimax_from_reduced(flip))
            }

            /* private helpers */

            #[inline(always)]
            const fn sin_minimax_from_reduced(self) -> Float<$f> {
                let x2 = self.0 * self.0;
                let poly = Float(x2).eval_poly_const::<{_FloatInternals::<$f>::SIN_COS_DEGREE}>(
                    &_FloatInternals::<$f>::SIN_COEFFS
                ).0;
                Float(self.0 + self.0 * x2 * poly)
            }

            #[inline(always)]
            const fn cos_minimax_from_reduced(self, flip: bool) -> Float<$f> {
                let x2 = self.0 * self.0;
                let poly = Float(x2).eval_poly_const::<{_FloatInternals::<$f>::SIN_COS_DEGREE}>(
                    &_FloatInternals::<$f>::COS_COEFFS
                ).0;
                let c = 1.0 + x2 * poly;
                Float(is![flip, -c, c])
            }

            #[inline(always)]
            const fn reduce_half_pi(self) -> (Float<$f>, bool) {
                use crate::FloatConst;
                // first stage: reduce into [-PI, PI]
                let mut xr = self.0 - <$f>::TAU * Float(self.0 / <$f>::TAU).round().0;

                // second stage: fold into [-PI/2, PI/2]
                let mut flip = false;
                if xr > <$f>::FRAC_PI_2 {
                    xr = <$f>::PI - xr;
                    flip = true;
                } else if xr < - <$f>::FRAC_PI_2 {
                    xr = - <$f>::PI - xr;
                    flip = true;
                }
                (Float(xr), flip)
            }

        }
    };
}
impl_float_shared_minimax!();
