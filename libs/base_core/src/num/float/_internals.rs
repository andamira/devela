// devela_base_core::num::float::internals
//
//! Defines [`_FloatInternals`].
//
// NOTE: It's also used from devela_base_std for FloatStd.

#![allow(clippy::excessive_precision)]

use crate::PhantomData;
#[cfg(nightly_float)]
use ::core::{f16, f128};

/// Private helper struct to define manual, type-dependendent constants.
#[doc(hidden)]
#[derive(Debug)]
pub struct _FloatInternals<F> {
    _f: PhantomData<F>,
}

macro_rules! impl_float_internals {
    () => {
        // ~3–4 decimal digits of precision.
        // Uses a half-precision magic number found by brute-force
        #[cfg(nightly_float)]
        impl_float_internals![f16,u16; (1e-4, 1e-3, 1e-2)
            [5, 0x59b9, 1e-3]
            // TEMP
            { 4, [
                -0.1666666716337204,
                 0.008333347737789154,
                -0.00019840903785611612,
                 0.000002752556381357731
            ], [
                -0.4999999701976776,
                 0.041666641652584076,
                -0.001388836889460682,
                 0.00002476049791497401
            ] }
        ];
        // ~7 decimal digits of precision.
        // Chris Lomont's single precision magic number for fisqrt: 0x5f37_59df
        // Uses Matthew Robertson's single precision magic number for fisqrt:
        impl_float_internals![f32,u32; (1e-7, 1e-6, 1e-5)
            [8, 0x5f375a86, 1e-6]
            { 4, [
                -0.1666666716337204,
                 0.008333347737789154,
                -0.00019840903785611612,
                 0.000002752556381357731
            ], [
                -0.4999999701976776,
                 0.041666641652584076,
                -0.001388836889460682,
                 0.00002476049791497401
            ] }
        ];
        // ~15–16 decimal digits of precision.
        // Chris Lomont's double precision magic number: 0x5fe6_eb50_c7b5_37aa
        // Uses Matthew Robertson's double precision magic number for fisqrt:
        impl_float_internals![f64,u64; (1e-12, 1e-9, 1e-6)
            [11, 0x5fe6_eb50_c7b5_37a9, 1e-12]
            { 8, [
                -1.66666666666666324348e-01,
                 8.33333333332248946124e-03,
                -1.98412698298579493134e-04,
                 2.75573137070700676789e-06,
                -2.50507602534068634195e-08,
                 1.58969099521155010221e-10,
                -5.58974346219658380687e-13,
                 8.37308034031243904303e-16
            ], [
                -0.5,
                 4.16666666666665929218e-02,
                -1.38888888888730564116e-03,
                 2.48015872894767294178e-05,
                -2.75573143513906633035e-07,
                 2.08757232129817482790e-09,
                -1.13596475577881948265e-11,
                 2.08767569878680989792e-14
            ] }
        ];
        // ~33–34 decimal digits of precision.
        // Uses Matthew Robertson's quadruple precision magic number:
        #[cfg(nightly_float)]
        impl_float_internals![f128,u128; (1e-30, 1e-27, 1e-24)
            [15, 0x5ffe_6eb5_0c7b_537a_9cd9_f02e_504f_cfbf, 1e-30]
            // TEMP, same as f64, but should be a better set of 12
            { 8, [
                -1.66666666666666324348e-01,
                 8.33333333332248946124e-03,
                -1.98412698298579493134e-04,
                 2.75573137070700676789e-06,
                -2.50507602534068634195e-08,
                 1.58969099521155010221e-10,
                -5.58974346219658380687e-13,
                 8.37308034031243904303e-16
            ], [
                -0.5,
                 4.16666666666665929218e-02,
                -1.38888888888730564116e-03,
                 2.48015872894767294178e-05,
                -2.75573143513906633035e-07,
                 2.08757232129817482790e-09,
                -1.13596475577881948265e-11,
                 2.08767569878680989792e-14
            ] }
        ];
    };
    (
    // $f:     the floating-point type
    // $u:     unsigned integer type with the same bit-size
    // (
    // $lm:    low margin of tolerance
    // $mm:    medium margin of tolerance
    // $hm:    high margin of tolerance
    // )
    // [
    // $ebit:  bits for the exponent
    // $fisr:  magic fisr constant
    // $nrt:   newton-rapson-tolerance for sqrt()
    // ]
    // {
    // $deg:   number of polynomial coefficients for sine/cosine
    // [$sin]: minimax polynomial coefficients for sine
    // [$cos]: minimax polynomial coefficients for cosine
    $f:ty, $u:ty;
    ($lm:literal, $mm:literal, $hm:literal)
    [$ebit:literal, $fisr:literal, $nrt:literal]
    { $deg:literal, [$($sin:expr),*], [$($cos:expr),*] } ) => {
        impl _FloatInternals<$f> {
            /// Low practical margin of error.
            pub const LOW_MARGIN: $f = $lm;
            /// Medium practical margin of error.
            pub const MEDIUM_MARGIN: $f = $mm;
            /// High practical margin of error.
            pub const HIGH_MARGIN: $f = $hm;

            #[doc = crate::_FLOAT_CONST_SIGNIFICAND_BITS!()]
            pub const SIGNIFICAND_BITS: u32 = <$f>::MANTISSA_DIGITS -1;
            #[doc = crate::_FLOAT_CONST_EXPONENT_BIAS!()]
            pub const EXPONENT_BIAS: u32 = <$f>::MAX_EXP as u32 - 1;
            #[doc = crate::_FLOAT_CONST_EXPONENT_BITS!()]
            pub const EXPONENT_BITS: u32 = $ebit;

            /// Fast inverse square root magic constant.
            pub const FISR_MAGIC: $u = $fisr;

            /// Tolerances for the difference between successive guesses using the
            /// Newton-Raphson method for square root calculation:
            pub const NR_TOLERANCE: $f = $nrt;

            /// Polynomial degree for minimax sine and cosine.
            pub const SIN_COS_DEGREE: usize = $deg;

            /// Minimax polynomial coefficients for sine.
            ///
            /// `sin(x) ≈ x + x³·p1 + x⁵·p2 + x⁷·p3 + x⁹·p4`
            pub const SIN_COEFFS: [$f; $deg] = [ $($sin),* ];

            /// Minimax polynomial coefficients for cosine.
            ///
            /// `cos(x) ≈ 1 + x²·c1 + x⁴·c2 + x⁶·c3 + x⁸·c4`
            pub const COS_COEFFS: [$f; $deg] = [ $($cos),* ];
        }
    };
}
impl_float_internals![];
