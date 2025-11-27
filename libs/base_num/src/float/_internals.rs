// devela_base_num::float::internals
//
//!
//
// TODO: make crate-private once possible

use crate::PhantomData;

/// Private helper struct to define manual, type-dependendent constants.
#[derive(Debug)]
pub struct _FloatInternals<T> {
    _marker: PhantomData<T>,
}

macro_rules! impl_float_internals {
    () => {
        // ~3–4 decimal digits of precision.
        // Uses a half-precision magic number found by brute-force
        #[cfg(nightly_float)]
        impl_float_internals![f16:u16; (1e-4, 1e-3, 1e-2)
            [5, 0x59b9, 1e-3]
        ];
        // ~7 decimal digits of precision.
        // Chris Lomont's single precision magic number for fisqrt: 0x5f37_59df
        // Uses Matthew Robertson's single precision magic number for fisqrt:
        impl_float_internals![f32:u32; (1e-7, 1e-6, 1e-5)
            [8, 0x5f375a86, 1e-6]
        ];
        // ~15–16 decimal digits of precision.
        // Chris Lomont's double precision magic number: 0x5fe6_eb50_c7b5_37aa
        // Uses Matthew Robertson's double precision magic number for fisqrt:
        impl_float_internals![f64:u64; (1e-12, 1e-9, 1e-6)
            [11, 0x5fe6_eb50_c7b5_37a9, 1e-12]
        ];
        // ~33–34 decimal digits of precision.
        // Uses Matthew Robertson's quadruple precision magic number:
        #[cfg(nightly_float)]
        impl_float_internals![f128:u128; (1e-30, 1e-27, 1e-24)
            [15, 0x5ffe_6eb5_0c7b_537a_9cd9_f02e_504f_cfbf, 1e-30]
        ];
    };
    (
    // $f:    the floating-point type
    // $u:    unsigned integer type with the same bit-size
    // (
    // $lm:   low margin of tolerance
    // $mm:   medium margin of tolerance
    // $hm:   high margin of tolerance
    // )
    // [
    // $ebit: bits for the exponent
    // $fisr: magic fisr constant
    // $nrt:  newton-rapson-tolerance for sqrt()
    // ]
    $f:ty:$u:ty;
    ($lm:literal, $mm:literal, $hm:literal)
    [$ebit:literal, $fisr:literal, $nrt:literal] ) => {
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
        }
    };
}
impl_float_internals![];
