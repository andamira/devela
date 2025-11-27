// devela::num::float::wrapper::consts
//
//! Constants
//!
//! - <https://en.wikipedia.org/wiki/List_of_mathematical_constants>
//
// NOTE: In sync with num::float::constants

#![allow(clippy::excessive_precision, reason = "constants are defined with 81 decimals")]
#![allow(dead_code, reason = "WIP f16,f128")]

use crate::{Float, FloatConst};
#[cfg(nightly_float)]
use ::core::{f16, f128};

/// impl mathematical constants
///
/// $f: the floating-point type.
macro_rules! float_const_impls {
    () => {
        float_const_impls![f32, f64];
        #[cfg(nightly_float)]
        float_const_impls![f16, f128];
    };
    ($( $f:ty),+) => { $( float_const_impls![@$f]; )+ };
    (@$f:ty) => {
        /// # *Mathematical constants*.
        ///
        /// See [`FloatConst`].
        impl Float<$f> {
            #[doc = crate::_FLOAT_CONST_ONE!()]
            pub const ONE: Float<$f> = Float(<$f>::ONE);
            #[doc = crate::_FLOAT_CONST_ZERO!()]
            pub const ZERO: Float<$f> = Float(<$f>::ZERO);
            #[doc = crate::_FLOAT_CONST_NEG_ONE!()]
            pub const NEG_ONE: Float<$f> = Float(<$f>::NEG_ONE);
            #[doc = crate::_FLOAT_CONST_NEG_ZERO!()]
            pub const NEG_ZERO: Float<$f> = Float(<$f>::NEG_ZERO);

            // ...

            #[doc = crate::_FLOAT_CONST_NAN!()]
            pub const NAN: Float<$f> = Float(<$f>::NAN);
            #[doc = crate::_FLOAT_CONST_INFINITY!()]
            pub const INFINITY: Float<$f> = Float(<$f>::INFINITY);
            #[doc = crate::_FLOAT_CONST_NEG_INFINITY!()]
            pub const NEG_INFINITY: Float<$f> = Float(<$f>::NEG_INFINITY);

            #[doc = crate::_FLOAT_CONST_EPSILON!()]
            pub const EPSILON: Float<$f> = Float(<$f>::EPSILON);

            #[doc = crate::_FLOAT_CONST_LOW_MARGIN!()]
            pub const LOW_MARGIN: Float<$f> = Float(<$f>::LOW_MARGIN);
            #[doc = crate::_FLOAT_CONST_MEDIUM_MARGIN!()]
            pub const MEDIUM_MARGIN: Float<$f> = Float(<$f>::MEDIUM_MARGIN);
            #[doc = crate::_FLOAT_CONST_HIGH_MARGIN!()]
            pub const HIGH_MARGIN: Float<$f> = Float(<$f>::HIGH_MARGIN);

            #[doc = crate::_FLOAT_CONST_RADIX!()]
            pub const RADIX: u32 = <$f>::RADIX;
            #[doc = crate::_FLOAT_CONST_DIGITS!()]
            pub const DIGITS: u32 = <$f>::DIGITS;
            #[doc = crate::_FLOAT_CONST_MANTISSA_DIGITS!()]
            pub const MANTISSA_DIGITS: u32 = <$f>::MANTISSA_DIGITS;

            #[doc = crate::_FLOAT_CONST_MIN!()]
            pub const MIN: Float<$f> = Float(<$f>::MIN);
            #[doc = crate::_FLOAT_CONST_MIN_POSITIVE!()]
            pub const MIN_POSITIVE: Float<$f> = Float(<$f>::MIN_POSITIVE);
            #[doc = crate::_FLOAT_CONST_MAX!()]
            pub const MAX: Float<$f> = Float(<$f>::MAX);

            #[doc = crate::_FLOAT_CONST_MIN_EXP!()]
            pub const MIN_EXP: i32 = <$f>::MIN_EXP;
            #[doc = crate::_FLOAT_CONST_MAX_EXP!()]
            pub const MAX_EXP: i32 = <$f>::MAX_EXP;

            #[doc = crate::_FLOAT_CONST_MIN_10_EXP!()]
            pub const MIN_10_EXP: i32 = <$f>::MIN_10_EXP;
            #[doc = crate::_FLOAT_CONST_MAX_10_EXP!()]
            pub const MAX_10_EXP: i32 = <$f>::MAX_10_EXP;

            /* Mathematical constants related to Pi (π) */

            #[doc = crate::_FLOAT_CONST_PI!()]
            pub const PI: Float<$f> = Float(<$f>::PI);
            #[doc = crate::_FLOAT_CONST_FRAC_PI_2!()]
            pub const FRAC_PI_2: Float<$f> = Float(<$f>::FRAC_PI_2);
            #[doc = crate::_FLOAT_CONST_FRAC_PI_3!()]
            pub const FRAC_PI_3: Float<$f> = Float(<$f>::FRAC_PI_3);
            #[doc = crate::_FLOAT_CONST_FRAC_PI_4!()]
            pub const FRAC_PI_4: Float<$f> = Float(<$f>::FRAC_PI_4);
            #[doc = crate::_FLOAT_CONST_FRAC_PI_6!()]
            pub const FRAC_PI_6: Float<$f> = Float(<$f>::FRAC_PI_6);
            #[doc = crate::_FLOAT_CONST_FRAC_PI_8!()]
            pub const FRAC_PI_8: Float<$f> = Float(<$f>::FRAC_PI_8);
            #[doc = crate::_FLOAT_CONST_SQRT_PI!()]
            pub const SQRT_PI: Float<$f> = Float(<$f>::SQRT_PI);
            #[doc = crate::_FLOAT_CONST_FRAC_1_PI!()]
            pub const FRAC_1_PI: Float<$f> = Float(<$f>::FRAC_1_PI);
            #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_PI!()]
            pub const FRAC_1_SQRT_PI: Float<$f> = Float(<$f>::FRAC_1_SQRT_PI);
            #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_2PI!()]
            pub const FRAC_1_SQRT_2PI: Float<$f> = Float(<$f>::FRAC_1_SQRT_2PI);
            #[doc = crate::_FLOAT_CONST_FRAC_2_PI!()]
            pub const FRAC_2_PI: Float<$f> = Float(<$f>::FRAC_2_PI);
            #[doc = crate::_FLOAT_CONST_FRAC_2_SQRT_PI!()]
            pub const FRAC_2_SQRT_PI: Float<$f> = Float(<$f>::FRAC_2_SQRT_PI);

            /* Mathematical constants related to Tau (τ) */

            #[doc = crate::_FLOAT_CONST_TAU!()]
            pub const TAU: Float<$f> = Float(<$f>::TAU);
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_2!()]
            pub const FRAC_TAU_2: Float<$f> = Self::PI;
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_3!()]
            pub const FRAC_TAU_3: Float<$f> = Float(<$f>::FRAC_TAU_3);
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_4!()]
            pub const FRAC_TAU_4: Float<$f> = Self::FRAC_PI_2;
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_5!()]
            pub const FRAC_TAU_5: Float<$f> = Float(<$f>::FRAC_TAU_5);
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_6!()]
            pub const FRAC_TAU_6: Float<$f> = Self::FRAC_PI_3;
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_8!()]
            pub const FRAC_TAU_8: Float<$f> = Self::FRAC_PI_4;
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_9!()]
            pub const FRAC_TAU_9: Float<$f> = Float(<$f>::FRAC_TAU_9);
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_12!()]
            pub const FRAC_TAU_12: Float<$f> = Self::FRAC_PI_6;
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_16!()]
            pub const FRAC_TAU_16: Float<$f> = Self::FRAC_PI_8;
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_24!()]
            pub const FRAC_TAU_24: Float<$f> = Float(<$f>::FRAC_TAU_24);
            #[doc = crate::_FLOAT_CONST_FRAC_TAU_72!()]
            pub const FRAC_TAU_72: Float<$f> = Float(<$f>::FRAC_TAU_72);
            #[doc = crate::_FLOAT_CONST_FRAC_360_TAU!()]
            pub const FRAC_360_TAU: Float<$f> = Float(<$f>::FRAC_360_TAU);
            #[doc = crate::_FLOAT_CONST_SQRT_TAU!()]
            pub const SQRT_TAU: Float<$f> = Float(<$f>::SQRT_TAU);
            #[doc = crate::_FLOAT_CONST_FRAC_1_TAU!()]
            pub const FRAC_1_TAU: Float<$f> = Float(<$f>::FRAC_1_TAU);
            #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_TAU!()]
            pub const FRAC_1_SQRT_TAU: Float<$f> = Float(<$f>::FRAC_1_SQRT_TAU);
            #[doc = crate::_FLOAT_CONST_FRAC_2_TAU!()]
            pub const FRAC_2_TAU: Float<$f> = Self::FRAC_1_PI;
            #[doc = crate::_FLOAT_CONST_FRAC_2_SQRT_TAU!()]
            pub const FRAC_2_SQRT_TAU: Float<$f> = Float(<$f>::FRAC_2_SQRT_TAU);

            /* Degrees */

            #[doc = crate::_FLOAT_CONST_ARC_DEGREE!()]
            pub const ARC_DEGREE: Float<$f> = Float(<$f>::ARC_DEGREE);
            #[doc = crate::_FLOAT_CONST_ARC_MINUTE!()]
            pub const ARC_MINUTE: Float<$f> = Float(<$f>::ARC_MINUTE);
            #[doc = crate::_FLOAT_CONST_ARC_SECOND!()]
            pub const ARC_SECOND: Float<$f> = Float(<$f>::ARC_SECOND);

            /* Mathematical constants related to Phi (φ) */

            #[doc = crate::_FLOAT_CONST_PHI!()]
            pub const PHI: Float<$f> = Float(<$f>::PHI);
            #[doc = crate::_FLOAT_CONST_SQ_PHI!()]
            pub const SQ_PHI: Float<$f> = Float(<$f>::SQ_PHI);
            #[doc = crate::_FLOAT_CONST_FRAC_1_PHI!()]
            pub const FRAC_1_PHI: Float<$f> = Float(<$f>::FRAC_1_PHI);
            #[doc = crate::_FLOAT_CONST_NEG_FRAC_1_PHI!()]
            pub const NEG_FRAC_1_PHI: Float<$f> = Float(<$f>::NEG_FRAC_1_PHI);
            #[doc = crate::_FLOAT_CONST_SQRT_PHI!()]
            pub const SQRT_PHI: Float<$f> = Float(<$f>::SQRT_PHI);
            #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_PHI!()]
            pub const FRAC_1_SQRT_PHI: Float<$f> = Float(<$f>::FRAC_1_SQRT_PHI);
            #[doc = crate::_FLOAT_CONST_TRIBONACCI!()]
            pub const TRIBONACCI: Float<$f> = Float(<$f>::TRIBONACCI);

            /* Mathematical constants related to integer roots */

            #[doc = crate::_FLOAT_CONST_SQRT_2!()]
            pub const SQRT_2: Float<$f> = Float(<$f>::SQRT_2);
            #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_2!()]
            pub const FRAC_1_SQRT_2: Float<$f> = Float(<$f>::FRAC_1_SQRT_2);
            #[doc = crate::_FLOAT_CONST_SQRT_3!()]
            pub const SQRT_3: Float<$f> = Float(<$f>::SQRT_3);
            #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_3!()]
            pub const FRAC_1_SQRT_3: Float<$f> = Float(<$f>::FRAC_1_SQRT_3);
            #[doc = crate::_FLOAT_CONST_SQRT_5!()]
            pub const SQRT_5: Float<$f> = Float(<$f>::SQRT_5);
            #[doc = crate::_FLOAT_CONST_SQRT_6!()]
            pub const SQRT_6: Float<$f> = Float(<$f>::SQRT_6);
            #[doc = crate::_FLOAT_CONST_SQRT_7!()]
            pub const SQRT_7: Float<$f> = Float(<$f>::SQRT_7);
            #[doc = crate::_FLOAT_CONST_SQRT_8!()]
            pub const SQRT_8: Float<$f> = Float(<$f>::SQRT_8);
            #[doc = crate::_FLOAT_CONST_SQRT_10!()]
            pub const SQRT_10: Float<$f> = Float(<$f>::SQRT_10);
            #[doc = crate::_FLOAT_CONST_SQRT_11!()]
            pub const SQRT_11: Float<$f> = Float(<$f>::SQRT_11);
            #[doc = crate::_FLOAT_CONST_SQRT_12!()]
            pub const SQRT_12: Float<$f> = Float(<$f>::SQRT_12);
            #[doc = crate::_FLOAT_CONST_SQRT_2!()]
            pub const CBRT_2: Float<$f> = Float(<$f>::CBRT_2);
            #[doc = crate::_FLOAT_CONST_CBRT_3!()]
            pub const CBRT_3: Float<$f> = Float(<$f>::CBRT_3);
            #[doc = crate::_FLOAT_CONST_FRAC_1_CBRT_3!()]
            pub const FRAC_1_CBRT_3: Float<$f> = Float(<$f>::FRAC_1_CBRT_3);

            /* Other mathematical constants */

            #[doc = crate::_FLOAT_CONST_E!()]
            pub const E: Float<$f> = Float(<$f>::E);
            #[doc = crate::_FLOAT_CONST_EGAMMA!()]
            pub const EGAMMA: Float<$f> = Float(<$f>::EGAMMA);
            #[doc = crate::_FLOAT_CONST_LOG2_E!()]
            pub const LOG2_E: Float<$f> = Float(<$f>::LOG2_E);
            #[doc = crate::_FLOAT_CONST_LOG2_10!()]
            pub const LOG2_10: Float<$f> = Float(<$f>::LOG2_10);
            #[doc = crate::_FLOAT_CONST_LOG10_E!()]
            pub const LOG10_E: Float<$f> = Float(<$f>::LOG10_E);
            #[doc = crate::_FLOAT_CONST_LOG10_2!()]
            pub const LOG10_2: Float<$f> = Float(<$f>::LOG10_2);
            #[doc = crate::_FLOAT_CONST_LN_2!()]
            pub const LN_2: Float<$f> = Float(<$f>::LN_2);
            #[doc = crate::_FLOAT_CONST_LN_10!()]
            pub const LN_10: Float<$f> = Float(<$f>::LN_10);
        }
    };
}
float_const_impls!();
