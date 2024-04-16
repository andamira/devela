// devela::num::float::wrapper::consts
//
//! Constants
//!
//! - <https://en.wikipedia.org/wiki/List_of_mathematical_constants>
//

// constants are defined with 37 digits, usually 1 integer and 36 decimals.
#![allow(clippy::excessive_precision)]
#![allow(missing_docs)]

use crate::num::{ExtFloatConst, Float};

// impl technical constants
//
// $f: the floating-point type.
// $u: unsigned integer type with the same bit-size.
macro_rules! technical_const_impls {
    ($( $f:ty:$u:ty
        [$bias:literal, $exp:literal, $sig:literal, $fisr:literal, $nrt:literal] ),+) => {
        $( technical_const_impls![@$f:$u[$bias, $exp, $sig, $fisr, $nrt]]; )+
    };
    (@$f:ty:$u:ty
     [$bias:literal, $exp:literal, $sig:literal, $fisr:literal, $nrt:literal]
     ) => {
        #[allow(unused)]
        impl Float<$f> {
            // Bias value used in the exponent to allow representation of both positive
            // and negative exponents.
            pub(super) const BIAS: u32 = $bias;
            // Number of bits used to represent the exponent.
            pub(super) const EXPONENT_BITS: u32 = $exp;
            // Number of explicit bits used to represent the significand (or mantissa).
            //
            // Note that std's `MANTISSA_DIGITS` floating-point constant equals
            // `SIGNIFICAND_BITS + 1` since it accounts for an additional implicit leading bit,
            // which is not stored but assumed in the standard floating-point representation.
            pub(super) const SIGNIFICAND_BITS: u32 = $sig;
            pub(super) const FISR_MAGIC: $u = $fisr;
            // Tolerances for the difference between successive guesses using the
            // Newton-Raphson method for square root calculation:
            pub(super) const NR_TOLERANCE: $f = $nrt;
        }
    };
}
technical_const_impls![
    // Uses Lomont's single precision magic number for fisqrt
    f32:u32[127, 8, 23, 0x5f37_59df, 1e-6],
    // Uses Lomont's double precision magic number for fisqrt
    // f64[1023, 11, 52, 0x5fe6_eb50_c7b5_37a9, 1e-15],
    // Uses Matthew Robertson's double precision magic number
    f64:u64[1023, 11, 52, 0x5fe6_eb50_c7b5_37a9, 1e-14]
    // Matthew Robertson's quadruple precision magic number
    // f128:u128[, , , 0x5ffe_6eb5_0c7b_537a_9cd9_f02e_504f_cfbf, ]
];

// impl mathematical constants
//
// $f: the floating-point type.
macro_rules! math_const_impls {
    ($( $f:ty),+) => { $( math_const_impls![@$f]; )+ };
    (@$f:ty) => {
        /// # *Mathematical constants*.
        ///
        /// See [`ExtFloatConst`].
        impl Float<$f> {
            pub const ONE: Float<$f> = Float(<$f>::ONE);
            pub const ZERO: Float<$f> = Float(<$f>::ZERO);
            pub const NEG_ONE: Float<$f> = Float(<$f>::NEG_ONE);
            pub const NEG_ZERO: Float<$f> = Float(<$f>::NEG_ZERO);

            // ...

            pub const NAN: Float<$f> = Float(<$f>::NAN);
            pub const INFINITY: Float<$f> = Float(<$f>::INFINITY);
            pub const NEG_INFINITY: Float<$f> = Float(<$f>::NEG_INFINITY);

            pub const EPSILON: Float<$f> = Float(<$f>::EPSILON);

            pub const RADIX: u32 = <$f>::RADIX;
            pub const DIGITS: u32 = <$f>::DIGITS;
            pub const MANTISSA_DIGITS: u32 = <$f>::MANTISSA_DIGITS;

            pub const MIN: Float<$f> = Float(<$f>::MIN);
            pub const MIN_POSITIVE: Float<$f> = Float(<$f>::MIN_POSITIVE);
            pub const MAX: Float<$f> = Float(<$f>::MAX);

            pub const MIN_EXP: i32 = <$f>::MIN_EXP;
            pub const MAX_EXP: i32 = <$f>::MAX_EXP;

            pub const MIN_10_EXP: i32 = <$f>::MIN_10_EXP;
            pub const MAX_10_EXP: i32 = <$f>::MAX_10_EXP;

            /* Mathematical constants related to Pi (π) */

            pub const PI: Float<$f> = Float(<$f>::PI);
            pub const FRAC_PI_2: Float<$f> = Float(<$f>::FRAC_PI_2);
            pub const FRAC_PI_3: Float<$f> = Float(<$f>::FRAC_PI_3);
            pub const FRAC_PI_4: Float<$f> = Float(<$f>::FRAC_PI_4);
            pub const FRAC_PI_6: Float<$f> = Float(<$f>::FRAC_PI_6);
            pub const FRAC_PI_8: Float<$f> = Float(<$f>::FRAC_PI_8);
            pub const SQRT_PI: Float<$f> = Float(<$f>::SQRT_PI);
            pub const FRAC_1_PI: Float<$f> = Float(<$f>::FRAC_1_PI);
            pub const FRAC_1_SQRT_PI: Float<$f> = Float(<$f>::FRAC_1_SQRT_PI);
            pub const FRAC_2_PI: Float<$f> = Float(<$f>::FRAC_2_PI);
            pub const FRAC_2_SQRT_PI: Float<$f> = Float(<$f>::FRAC_2_SQRT_PI);

            /* Mathematical constants related to Tau (τ) */

            pub const TAU: Float<$f> = Float(<$f>::TAU);
            pub const FRAC_TAU_2: Float<$f> = Self::PI;
            pub const FRAC_TAU_3: Float<$f> = Float(<$f>::FRAC_TAU_3);
            pub const FRAC_TAU_4: Float<$f> = Self::FRAC_PI_2;
            pub const FRAC_TAU_5: Float<$f> = Float(<$f>::FRAC_TAU_5);
            pub const FRAC_TAU_6: Float<$f> = Self::FRAC_PI_3;
            pub const FRAC_TAU_8: Float<$f> = Self::FRAC_PI_4;
            pub const FRAC_TAU_9: Float<$f> = Float(<$f>::FRAC_TAU_9);
            pub const FRAC_TAU_12: Float<$f> = Self::FRAC_PI_6;
            pub const FRAC_TAU_16: Float<$f> = Self::FRAC_PI_8;
            pub const FRAC_TAU_24: Float<$f> = Float(<$f>::FRAC_TAU_24);
            pub const FRAC_TAU_72: Float<$f> = Float(<$f>::FRAC_TAU_72);
            pub const FRAC_360_TAU: Float<$f> = Float(<$f>::FRAC_360_TAU);
            pub const SQRT_TAU: Float<$f> = Float(<$f>::SQRT_TAU);
            pub const FRAC_1_TAU: Float<$f> = Float(<$f>::FRAC_1_TAU);
            pub const FRAC_1_SQRT_TAU: Float<$f> = Float(<$f>::FRAC_1_SQRT_TAU);
            pub const FRAC_2_TAU: Float<$f> = Self::FRAC_1_PI;
            pub const FRAC_2_SQRT_TAU: Float<$f> = Float(<$f>::FRAC_2_SQRT_TAU);

            /* Degrees */

            pub const ARC_DEGREE: Float<$f> = Float(<$f>::ARC_DEGREE);
            pub const ARC_MINUTE: Float<$f> = Float(<$f>::ARC_MINUTE);
            pub const ARC_SECOND: Float<$f> = Float(<$f>::ARC_SECOND);

            /* Mathematical constants related to Phi (φ) */

            pub const PHI: Float<$f> = Float(<$f>::PHI);
            pub const SQ_PHI: Float<$f> = Float(<$f>::SQ_PHI);
            pub const FRAC_1_PHI: Float<$f> = Float(<$f>::FRAC_1_PHI);
            pub const NEG_FRAC_1_PHI: Float<$f> = Float(<$f>::NEG_FRAC_1_PHI);
            pub const SQRT_PHI: Float<$f> = Float(<$f>::SQRT_PHI);
            pub const FRAC_1_SQRT_PHI: Float<$f> = Float(<$f>::FRAC_1_SQRT_PHI);
            pub const TRIBONACCI: Float<$f> = Float(<$f>::TRIBONACCI);

            /* Mathematical constants related to integer roots */

            pub const SQRT_2: Float<$f> = Float(<$f>::SQRT_2);
            pub const FRAC_1_SQRT_2: Float<$f> = Float(<$f>::FRAC_1_SQRT_2);
            pub const SQRT_3: Float<$f> = Float(<$f>::SQRT_3);
            pub const FRAC_1_SQRT_3: Float<$f> = Float(<$f>::FRAC_1_SQRT_3);
            pub const SQRT_5: Float<$f> = Float(<$f>::SQRT_5);
            pub const SQRT_6: Float<$f> = Float(<$f>::SQRT_6);
            pub const SQRT_7: Float<$f> = Float(<$f>::SQRT_7);
            pub const SQRT_8: Float<$f> = Float(<$f>::SQRT_8);
            pub const SQRT_10: Float<$f> = Float(<$f>::SQRT_10);
            pub const SQRT_11: Float<$f> = Float(<$f>::SQRT_11);
            pub const SQRT_12: Float<$f> = Float(<$f>::SQRT_12);
            pub const CBRT_2: Float<$f> = Float(<$f>::CBRT_2);
            pub const CBRT_3: Float<$f> = Float(<$f>::CBRT_3);
            pub const FRAC_1_CBRT_3: Float<$f> = Float(<$f>::FRAC_1_CBRT_3);

            /* Other mathematical constants */

            pub const E: Float<$f> = Float(<$f>::E);
            pub const EGAMMA: Float<$f> = Float(<$f>::EGAMMA);
            pub const LOG2_E: Float<$f> = Float(<$f>::LOG2_E);
            pub const LOG2_10: Float<$f> = Float(<$f>::LOG2_10);
            pub const LOG10_E: Float<$f> = Float(<$f>::LOG10_E);
            pub const LOG10_2: Float<$f> = Float(<$f>::LOG10_2);
            pub const LN_2: Float<$f> = Float(<$f>::LN_2);
            pub const LN_10: Float<$f> = Float(<$f>::LN_10);
        }
    };
}
math_const_impls![f32, f64];
