// devela::num::float::wrapper::consts
//
//! Constants
//!
//! - <https://en.wikipedia.org/wiki/List_of_mathematical_constants>
//
// NOTE: In sync with num::float::constants

#![allow(clippy::excessive_precision, reason = "constants are defined with 81 decimals")]
#![allow(dead_code, reason = "WIP f16,f128")]

use super::super::constants::{ExtFloatConst, *};
use crate::Float;
#[cfg(feature = "nightly_float")]
use ::core::{f128, f16};

/// impl technical constants
///
/// $f: the floating-point type.
/// $u: unsigned integer type with the same bit-size.
macro_rules! float_technical_const_impls {
    () => {
        float_technical_const_impls![
            // Uses Lomont's single precision magic number for fisqrt
            f32:u32[127, 8, 23, 0x5f37_59df, 1e-6],

            // Uses Lomont's double precision magic number for fisqrt
            // f64[1023, 11, 52, 0x5fe6_eb50_c7b5_37a9, 1e-15],
            //
            // Uses Matthew Robertson's double precision magic number
            f64:u64[1023, 11, 52, 0x5fe6_eb50_c7b5_37a9, 1e-12]
        ];
        #[cfg(feature = "nightly_float")]
        float_technical_const_impls![
            // Uses a half-precision magic number found by brute-force
            f16:u16[15, 5, 10, 0x59b9, 1e-3],
            // Uses Matthew Robertson's quadruple precision magic number
            f128:u128[16383, 15, 112, 0x5ffe_6eb5_0c7b_537a_9cd9_f02e_504f_cfbf, 1e-30]
        ];
    };
    ($( $f:ty:$u:ty
        [$bias:literal, $exp:literal, $sig:literal, $fisr:literal, $nrt:literal] ),+) => {
        $( float_technical_const_impls![@$f:$u[$bias, $exp, $sig, $fisr, $nrt]]; )+
    };
    (@$f:ty:$u:ty
        [$bias:literal, $exp:literal, $sig:literal, $fisr:literal, $nrt:literal] ) => {
        // #[allow(unused)]
        impl Float<$f> {
            /// Bias value used in the exponent to allow representation of both positive
            /// and negative exponents.
            pub(super) const BIAS: u32 = $bias;
            /// Number of bits used to represent the exponent.
            pub(super) const EXPONENT_BITS: u32 = $exp;
            /// Number of explicit bits used to represent the significand (or mantissa).
            ///
            /// Note that std's `MANTISSA_DIGITS` floating-point constant equals
            /// `SIGNIFICAND_BITS + 1` since it accounts for an additional implicit leading bit,
            /// which is not stored but assumed in the standard floating-point representation.
            pub(super) const SIGNIFICAND_BITS: u32 = $sig;
            pub(super) const FISR_MAGIC: $u = $fisr;
            /// Tolerances for the difference between successive guesses using the
            /// Newton-Raphson method for square root calculation:
            pub(super) const NR_TOLERANCE: $f = $nrt;
        }
    };
}
float_technical_const_impls![];

/// impl mathematical constants
///
/// $f: the floating-point type.
macro_rules! float_const_impls {
    () => {
        float_const_impls![f32, f64];
        #[cfg(feature = "nightly_float")]
        float_const_impls![f16, f128];
    };
    ($( $f:ty),+) => { $( float_const_impls![@$f]; )+ };
    (@$f:ty) => {
        /// # *Mathematical constants*.
        ///
        /// See [`ExtFloatConst`].
        impl Float<$f> {
            #[doc = ONE!()]
            pub const ONE: Float<$f> = Float(<$f>::ONE);
            #[doc = ZERO!()]
            pub const ZERO: Float<$f> = Float(<$f>::ZERO);
            #[doc = NEG_ONE!()]
            pub const NEG_ONE: Float<$f> = Float(<$f>::NEG_ONE);
            #[doc = NEG_ZERO!()]
            pub const NEG_ZERO: Float<$f> = Float(<$f>::NEG_ZERO);

            // ...

            #[doc = NAN!()]
            pub const NAN: Float<$f> = Float(<$f>::NAN);
            #[doc = INFINITY!()]
            pub const INFINITY: Float<$f> = Float(<$f>::INFINITY);
            #[doc = NEG_INFINITY!()]
            pub const NEG_INFINITY: Float<$f> = Float(<$f>::NEG_INFINITY);

            #[doc = EPSILON!()]
            pub const EPSILON: Float<$f> = Float(<$f>::EPSILON);

            #[doc = LOW_MARGIN!()]
            pub const LOW_MARGIN: Float<$f> = Float(<$f>::LOW_MARGIN);
            #[doc = MEDIUM_MARGIN!()]
            pub const MEDIUM_MARGIN: Float<$f> = Float(<$f>::MEDIUM_MARGIN);
            #[doc = HIGH_MARGIN!()]
            pub const HIGH_MARGIN: Float<$f> = Float(<$f>::HIGH_MARGIN);

            #[doc = RADIX!()]
            pub const RADIX: u32 = <$f>::RADIX;
            #[doc = DIGITS!()]
            pub const DIGITS: u32 = <$f>::DIGITS;
            #[doc = MANTISSA_DIGITS!()]
            pub const MANTISSA_DIGITS: u32 = <$f>::MANTISSA_DIGITS;

            #[doc = MIN!()]
            pub const MIN: Float<$f> = Float(<$f>::MIN);
            #[doc = MIN_POSITIVE!()]
            pub const MIN_POSITIVE: Float<$f> = Float(<$f>::MIN_POSITIVE);
            #[doc = MAX!()]
            pub const MAX: Float<$f> = Float(<$f>::MAX);

            #[doc = MIN_EXP!()]
            pub const MIN_EXP: i32 = <$f>::MIN_EXP;
            #[doc = MAX_EXP!()]
            pub const MAX_EXP: i32 = <$f>::MAX_EXP;

            #[doc = MIN_10_EXP!()]
            pub const MIN_10_EXP: i32 = <$f>::MIN_10_EXP;
            #[doc = MAX_10_EXP!()]
            pub const MAX_10_EXP: i32 = <$f>::MAX_10_EXP;

            /* Mathematical constants related to Pi (π) */

            #[doc = PI!()]
            pub const PI: Float<$f> = Float(<$f>::PI);
            #[doc = FRAC_PI_2!()]
            pub const FRAC_PI_2: Float<$f> = Float(<$f>::FRAC_PI_2);
            #[doc = FRAC_PI_3!()]
            pub const FRAC_PI_3: Float<$f> = Float(<$f>::FRAC_PI_3);
            #[doc = FRAC_PI_4!()]
            pub const FRAC_PI_4: Float<$f> = Float(<$f>::FRAC_PI_4);
            #[doc = FRAC_PI_6!()]
            pub const FRAC_PI_6: Float<$f> = Float(<$f>::FRAC_PI_6);
            #[doc = FRAC_PI_8!()]
            pub const FRAC_PI_8: Float<$f> = Float(<$f>::FRAC_PI_8);
            #[doc = SQRT_PI!()]
            pub const SQRT_PI: Float<$f> = Float(<$f>::SQRT_PI);
            #[doc = FRAC_1_PI!()]
            pub const FRAC_1_PI: Float<$f> = Float(<$f>::FRAC_1_PI);
            #[doc = FRAC_1_SQRT_PI!()]
            pub const FRAC_1_SQRT_PI: Float<$f> = Float(<$f>::FRAC_1_SQRT_PI);
            #[doc = FRAC_1_SQRT_2PI!()]
            pub const FRAC_1_SQRT_2PI: Float<$f> = Float(<$f>::FRAC_1_SQRT_2PI);
            #[doc = FRAC_2_PI!()]
            pub const FRAC_2_PI: Float<$f> = Float(<$f>::FRAC_2_PI);
            #[doc = FRAC_2_SQRT_PI!()]
            pub const FRAC_2_SQRT_PI: Float<$f> = Float(<$f>::FRAC_2_SQRT_PI);

            /* Mathematical constants related to Tau (τ) */

            #[doc = TAU!()]
            pub const TAU: Float<$f> = Float(<$f>::TAU);
            #[doc = FRAC_TAU_2!()]
            pub const FRAC_TAU_2: Float<$f> = Self::PI;
            #[doc = FRAC_TAU_3!()]
            pub const FRAC_TAU_3: Float<$f> = Float(<$f>::FRAC_TAU_3);
            #[doc = FRAC_TAU_4!()]
            pub const FRAC_TAU_4: Float<$f> = Self::FRAC_PI_2;
            #[doc = FRAC_TAU_5!()]
            pub const FRAC_TAU_5: Float<$f> = Float(<$f>::FRAC_TAU_5);
            #[doc = FRAC_TAU_6!()]
            pub const FRAC_TAU_6: Float<$f> = Self::FRAC_PI_3;
            #[doc = FRAC_TAU_8!()]
            pub const FRAC_TAU_8: Float<$f> = Self::FRAC_PI_4;
            #[doc = FRAC_TAU_9!()]
            pub const FRAC_TAU_9: Float<$f> = Float(<$f>::FRAC_TAU_9);
            #[doc = FRAC_TAU_12!()]
            pub const FRAC_TAU_12: Float<$f> = Self::FRAC_PI_6;
            #[doc = FRAC_TAU_16!()]
            pub const FRAC_TAU_16: Float<$f> = Self::FRAC_PI_8;
            #[doc = FRAC_TAU_24!()]
            pub const FRAC_TAU_24: Float<$f> = Float(<$f>::FRAC_TAU_24);
            #[doc = FRAC_TAU_72!()]
            pub const FRAC_TAU_72: Float<$f> = Float(<$f>::FRAC_TAU_72);
            #[doc = FRAC_360_TAU!()]
            pub const FRAC_360_TAU: Float<$f> = Float(<$f>::FRAC_360_TAU);
            #[doc = SQRT_TAU!()]
            pub const SQRT_TAU: Float<$f> = Float(<$f>::SQRT_TAU);
            #[doc = FRAC_1_TAU!()]
            pub const FRAC_1_TAU: Float<$f> = Float(<$f>::FRAC_1_TAU);
            #[doc = FRAC_1_SQRT_TAU!()]
            pub const FRAC_1_SQRT_TAU: Float<$f> = Float(<$f>::FRAC_1_SQRT_TAU);
            #[doc = FRAC_2_TAU!()]
            pub const FRAC_2_TAU: Float<$f> = Self::FRAC_1_PI;
            #[doc = FRAC_2_SQRT_TAU!()]
            pub const FRAC_2_SQRT_TAU: Float<$f> = Float(<$f>::FRAC_2_SQRT_TAU);

            /* Degrees */

            #[doc = ARC_DEGREE!()]
            pub const ARC_DEGREE: Float<$f> = Float(<$f>::ARC_DEGREE);
            #[doc = ARC_MINUTE!()]
            pub const ARC_MINUTE: Float<$f> = Float(<$f>::ARC_MINUTE);
            #[doc = ARC_SECOND!()]
            pub const ARC_SECOND: Float<$f> = Float(<$f>::ARC_SECOND);

            /* Mathematical constants related to Phi (φ) */

            #[doc = PHI!()]
            pub const PHI: Float<$f> = Float(<$f>::PHI);
            #[doc = SQ_PHI!()]
            pub const SQ_PHI: Float<$f> = Float(<$f>::SQ_PHI);
            #[doc = FRAC_1_PHI!()]
            pub const FRAC_1_PHI: Float<$f> = Float(<$f>::FRAC_1_PHI);
            #[doc = NEG_FRAC_1_PHI!()]
            pub const NEG_FRAC_1_PHI: Float<$f> = Float(<$f>::NEG_FRAC_1_PHI);
            #[doc = SQRT_PHI!()]
            pub const SQRT_PHI: Float<$f> = Float(<$f>::SQRT_PHI);
            #[doc = FRAC_1_SQRT_PHI!()]
            pub const FRAC_1_SQRT_PHI: Float<$f> = Float(<$f>::FRAC_1_SQRT_PHI);
            #[doc = TRIBONACCI!()]
            pub const TRIBONACCI: Float<$f> = Float(<$f>::TRIBONACCI);

            /* Mathematical constants related to integer roots */

            #[doc = SQRT_2!()]
            pub const SQRT_2: Float<$f> = Float(<$f>::SQRT_2);
            #[doc = FRAC_1_SQRT_2!()]
            pub const FRAC_1_SQRT_2: Float<$f> = Float(<$f>::FRAC_1_SQRT_2);
            #[doc = SQRT_3!()]
            pub const SQRT_3: Float<$f> = Float(<$f>::SQRT_3);
            #[doc = FRAC_1_SQRT_3!()]
            pub const FRAC_1_SQRT_3: Float<$f> = Float(<$f>::FRAC_1_SQRT_3);
            #[doc = SQRT_5!()]
            pub const SQRT_5: Float<$f> = Float(<$f>::SQRT_5);
            #[doc = SQRT_6!()]
            pub const SQRT_6: Float<$f> = Float(<$f>::SQRT_6);
            #[doc = SQRT_7!()]
            pub const SQRT_7: Float<$f> = Float(<$f>::SQRT_7);
            #[doc = SQRT_8!()]
            pub const SQRT_8: Float<$f> = Float(<$f>::SQRT_8);
            #[doc = SQRT_10!()]
            pub const SQRT_10: Float<$f> = Float(<$f>::SQRT_10);
            #[doc = SQRT_11!()]
            pub const SQRT_11: Float<$f> = Float(<$f>::SQRT_11);
            #[doc = SQRT_12!()]
            pub const SQRT_12: Float<$f> = Float(<$f>::SQRT_12);
            #[doc = SQRT_2!()]
            pub const CBRT_2: Float<$f> = Float(<$f>::CBRT_2);
            #[doc = CBRT_3!()]
            pub const CBRT_3: Float<$f> = Float(<$f>::CBRT_3);
            #[doc = FRAC_1_CBRT_3!()]
            pub const FRAC_1_CBRT_3: Float<$f> = Float(<$f>::FRAC_1_CBRT_3);

            /* Other mathematical constants */

            #[doc = E!()]
            pub const E: Float<$f> = Float(<$f>::E);
            #[doc = EGAMMA!()]
            pub const EGAMMA: Float<$f> = Float(<$f>::EGAMMA);
            #[doc = LOG2_E!()]
            pub const LOG2_E: Float<$f> = Float(<$f>::LOG2_E);
            #[doc = LOG2_10!()]
            pub const LOG2_10: Float<$f> = Float(<$f>::LOG2_10);
            #[doc = LOG10_E!()]
            pub const LOG10_E: Float<$f> = Float(<$f>::LOG10_E);
            #[doc = LOG10_2!()]
            pub const LOG10_2: Float<$f> = Float(<$f>::LOG10_2);
            #[doc = LN_2!()]
            pub const LN_2: Float<$f> = Float(<$f>::LN_2);
            #[doc = LN_10!()]
            pub const LN_10: Float<$f> = Float(<$f>::LN_10);
        }
    };
}
float_const_impls!();
