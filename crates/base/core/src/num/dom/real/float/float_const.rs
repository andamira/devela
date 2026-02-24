// devela_base_core::num::dom::real::float::float_const
//
//! Defines [`FloatConst`] and implements it for floating-point primitives.
//
// TOC
// - trait FloatConst
// - macro impl_float_const!
//
// NOTE: In sync with float:wrapper::consts

#![allow(clippy::excessive_precision, reason = "using constants with many decimals")]

use crate::_FloatInternals;

#[cfg(nightly_float)]
use ::core::{f16, f128};

#[doc = crate::_tags!(num namespace)] // by association with Float
/// Extension trait for floating-point types. Associated constants.
#[doc = crate::_doc_location!("num/dom/real")]
///
/// # Constants
///
/// - Identities:
/// [`ONE`], [`ZERO`], [`NEG_ONE`], [`NEG_ZERO`].
///
/// - Representation, precision and computational bounds:
/// [`NAN`], [`INFINITY`], [`NEG_INFINITY`], [`MIN`], [`MIN_POSITIVE`], [`MAX`], [`MIN_EXP`],
/// [`MAX_EXP`], [`MIN_10_EXP`], [`MAX_10_EXP`], [`EPSILON`], [`RADIX`], [`DIGITS`],
/// [`MANTISSA_DIGITS`], [`SIGNIFICAND_BITS`], [`EXPONENT_BIAS`], [`EXPONENT_BITS`].
///
/// - Arc degrees: [`ARC_DEGREE`], [`ARC_MINUTE`], [`ARC_SECOND`].
///
/// - Pi (π) related:
/// <big>
/// [π], [π/2], [π/3], [π/4], [π/6], [π/8], [√π], [1/π], [1/√π], [2/π], [2/√π].
/// </big>
///
/// - Tau (τ) related:
/// <big>
/// [τ], [τ/2], [τ/3], [τ/4], [τ/5], [τ/6], [τ/8], [τ/9], [τ/12], [τ/16], [τ/24], [τ/72],
/// [τ/360], [360/τ], [√τ], [1/τ], [1/√τ], [2/τ], [2/√τ].
/// </big>
///
/// - Phi (φ) related:
/// <big>[φ], [φ²], [1/φ], [-1/φ], [√φ], [1/√φ],</big>
/// [`TRIBONACCI`].
///
/// - Related to integer roots:
/// <big>
/// [√2], [1/√2], [√3], [1/√3], [√5], [√6], [√7], [√8], [√10], [√11], [√12], [∛2], [∛3], [1/∛3].
/// </big>
///
/// - Other constants:
/// [`E`], [`EGAMMA`], [`LOG2_E`], [`LOG2_10`], [`LOG10_E`], [`LOG10_2`], [`LN_2`], [`LN_10`].
///
// ---------------------
/// [`ONE`]: Self::ONE
/// [`ZERO`]: Self::ZERO
/// [`NEG_ONE`]: Self::NEG_ONE
/// [`NEG_ZERO`]: Self::NEG_ZERO
///
/// [`NAN`]: Self::NAN
/// [`INFINITY`]: Self::INFINITY
/// [`NEG_INFINITY`]: Self::NEG_INFINITY
/// [`MIN`]: Self::MIN
/// [`MIN_POSITIVE`]: Self::MIN_POSITIVE
/// [`MAX`]: Self::MAX
/// [`MIN_EXP`]: Self::MIN_EXP
/// [`MAX_EXP`]: Self::MAX_EXP
/// [`MIN_10_EXP`]: Self::MIN_10_EXP
/// [`MAX_10_EXP`]: Self::MAX_10_EXP
/// [`EPSILON`]: Self::EPSILON
/// [`RADIX`]: Self::RADIX
/// [`DIGITS`]: Self::DIGITS
/// [`MANTISSA_DIGITS`]: Self::MANTISSA_DIGITS
/// [`SIGNIFICAND_BITS`]: Self::SIGNIFICAND_BITS
/// [`EXPONENT_BIAS`]: Self::EXPONENT_BIAS
/// [`EXPONENT_BITS`]: Self::EXPONENT_BITS
///
/// [`ARC_DEGREE`]: Self::ARC_DEGREE
/// [`ARC_MINUTE`]: Self::ARC_MINUTE
/// [`ARC_SECOND`]: Self::ARC_SECOND
///
/// [π]: Self::PI
/// [π/2]: Self::FRAC_PI_2
/// [π/3]: Self::FRAC_PI_3
/// [π/4]: Self::FRAC_PI_4
/// [π/6]: Self::FRAC_PI_6
/// [π/8]: Self::FRAC_PI_8
/// [√π]: Self::SQRT_PI
/// [1/π]: Self::FRAC_1_PI
/// [1/√π]: Self::FRAC_1_SQRT_PI
/// [1/√2π]: Self::FRAC_1_SQRT_2PI
/// [2/π]: Self::FRAC_2_PI
/// [2/√π]: Self::FRAC_2_SQRT_PI
///
/// [τ]: Self::TAU
/// [τ/2]: Self::FRAC_TAU_2
/// [τ/3]: Self::FRAC_TAU_3
/// [τ/4]: Self::FRAC_TAU_4
/// [τ/5]: Self::FRAC_TAU_5
/// [τ/6]: Self::FRAC_TAU_6
/// [τ/8]: Self::FRAC_TAU_8
/// [τ/9]: Self::FRAC_TAU_9
/// [τ/12]: Self::FRAC_TAU_12
/// [τ/16]: Self::FRAC_TAU_16
/// [τ/24]: Self::FRAC_TAU_24
/// [τ/72]: Self::FRAC_TAU_72
/// [τ/360]: Self::FRAC_TAU_360
/// [360/τ]: Self::FRAC_360_TAU
/// [√τ]: Self::SQRT_TAU
/// [1/τ]: Self::FRAC_1_TAU
/// [1/√τ]: Self::FRAC_1_SQRT_TAU
/// [2/τ]: Self::FRAC_2_TAU
/// [2/√τ]: Self::FRAC_2_SQRT_TAU
///
/// [φ]: Self::PHI
/// [φ²]: Self::SQ_PHI
/// [1/φ]: Self::FRAC_1_PHI
/// [-1/φ]: Self::NEG_FRAC_1_PHI
/// [√φ]: Self::SQRT_PHI
/// [1/√φ]: Self::FRAC_1_SQRT_PHI
/// [`TRIBONACCI`]: Self::TRIBONACCI
///
/// [√2]: Self::SQRT_2
/// [1/√2]: Self::FRAC_1_SQRT_2
/// [√3]: Self::SQRT_3
/// [1/√3]: Self::FRAC_1_SQRT_3
/// [√5]: Self::SQRT_5
/// [√6]: Self::SQRT_6
/// [√7]: Self::SQRT_7
/// [√8]: Self::SQRT_8
/// [√10]: Self::SQRT_10
/// [√11]: Self::SQRT_11
/// [√12]: Self::SQRT_12
/// [∛2]: Self::CBRT_2
/// [∛3]: Self::CBRT_3
/// [1/∛3]: Self::FRAC_1_CBRT_3
///
/// [`E`]: Self::E
/// [`EGAMMA`]: Self::EGAMMA
/// [`LOG2_E`]: Self::LOG2_E
/// [`LOG2_10`]: Self::LOG2_10
/// [`LOG10_E`]: Self::LOG10_E
/// [`LOG10_2`]: Self::LOG10_2
/// [`LN_2`]: Self::LN_2
/// [`LN_10`]: Self::LN_10
#[rustfmt::skip]
pub trait FloatConst: Sized {
    // identities
    #[doc = crate::_FLOAT_CONST_ONE!()]                 const ONE: Self;
    #[doc = crate::_FLOAT_CONST_ZERO!()]                const ZERO: Self;
    #[doc = crate::_FLOAT_CONST_NEG_ONE!()]             const NEG_ONE: Self;
    #[doc = crate::_FLOAT_CONST_NEG_ZERO!()]            const NEG_ZERO: Self;
    // representation, precision and range
    #[doc = crate::_FLOAT_CONST_NAN!()]                 const NAN: Self;
    #[doc = crate::_FLOAT_CONST_INFINITY!()]            const INFINITY: Self;
    #[doc = crate::_FLOAT_CONST_NEG_INFINITY!()]        const NEG_INFINITY: Self;
    #[doc = crate::_FLOAT_CONST_MIN!()]                 const MIN: Self;
    #[doc = crate::_FLOAT_CONST_MIN_POSITIVE!()]        const MIN_POSITIVE: Self;
    #[doc = crate::_FLOAT_CONST_MAX!()]                 const MAX: Self;
    #[doc = crate::_FLOAT_CONST_MIN_EXP!()]             const MIN_EXP: i32;
    #[doc = crate::_FLOAT_CONST_MAX_EXP!()]             const MAX_EXP: i32;
    #[doc = crate::_FLOAT_CONST_MIN_10_EXP!()]          const MIN_10_EXP: i32;
    #[doc = crate::_FLOAT_CONST_MAX_10_EXP!()]          const MAX_10_EXP: i32;
    #[doc = crate::_FLOAT_CONST_EPSILON!()]             const EPSILON: Self;
    #[doc = crate::_FLOAT_CONST_LOW_MARGIN!()]          const LOW_MARGIN: Self;
    #[doc = crate::_FLOAT_CONST_MEDIUM_MARGIN!()]       const MEDIUM_MARGIN: Self;
    #[doc = crate::_FLOAT_CONST_HIGH_MARGIN!()]         const HIGH_MARGIN: Self;
    #[doc = crate::_FLOAT_CONST_RADIX!()]               const RADIX: u32;
    #[doc = crate::_FLOAT_CONST_DIGITS!()]              const DIGITS: u32;
    #[doc = crate::_FLOAT_CONST_MANTISSA_DIGITS!()]     const MANTISSA_DIGITS: u32;
    #[doc = crate::_FLOAT_CONST_SIGNIFICAND_BITS!()]    const SIGNIFICAND_BITS: u32;
    #[doc = crate::_FLOAT_CONST_EXPONENT_BIAS!()]       const EXPONENT_BIAS: u32;
    #[doc = crate::_FLOAT_CONST_EXPONENT_BITS!()]       const EXPONENT_BITS: u32;
    // pi
    #[doc = crate::_FLOAT_CONST_PI!()]                  const PI: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_PI_2!()]           const FRAC_PI_2: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_PI_3!()]           const FRAC_PI_3: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_PI_4!()]           const FRAC_PI_4: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_PI_6!()]           const FRAC_PI_6: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_PI_8!()]           const FRAC_PI_8: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_PI!()]             const SQRT_PI: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_PI!()]           const FRAC_1_PI: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_PI!()]      const FRAC_1_SQRT_PI: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_2PI!()]     const FRAC_1_SQRT_2PI: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_2_PI!()]           const FRAC_2_PI: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_2_SQRT_PI!()]      const FRAC_2_SQRT_PI: Self;
    // tau
    #[doc = crate::_FLOAT_CONST_TAU!()]                 const TAU: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_2!()]          const FRAC_TAU_2: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_3!()]          const FRAC_TAU_3: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_4!()]          const FRAC_TAU_4: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_5!()]          const FRAC_TAU_5: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_6!()]          const FRAC_TAU_6: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_8!()]          const FRAC_TAU_8: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_9!()]          const FRAC_TAU_9: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_12!()]         const FRAC_TAU_12: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_16!()]         const FRAC_TAU_16: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_24!()]         const FRAC_TAU_24: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_72!()]         const FRAC_TAU_72: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_TAU_360!()]        const FRAC_TAU_360: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_360_TAU!()]        const FRAC_360_TAU: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_TAU!()]            const SQRT_TAU: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_TAU!()]          const FRAC_1_TAU: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_TAU!()]     const FRAC_1_SQRT_TAU: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_2_TAU!()]          const FRAC_2_TAU: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_2_SQRT_TAU!()]     const FRAC_2_SQRT_TAU: Self;
    // arc degrees
    #[doc = crate::_FLOAT_CONST_ARC_DEGREE!()]          const ARC_DEGREE: Self;
    #[doc = crate::_FLOAT_CONST_ARC_MINUTE!()]          const ARC_MINUTE: Self;
    #[doc = crate::_FLOAT_CONST_ARC_SECOND!()]          const ARC_SECOND: Self;
    // phi
    #[doc = crate::_FLOAT_CONST_PHI!()]                 const PHI: Self;
    #[doc = crate::_FLOAT_CONST_PHI!()]                 const GOLDEN_RATIO: Self; // std
    #[doc = crate::_FLOAT_CONST_SQ_PHI!()]              const SQ_PHI: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_PHI!()]          const FRAC_1_PHI: Self;
    #[doc = crate::_FLOAT_CONST_NEG_FRAC_1_PHI!()]      const NEG_FRAC_1_PHI: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_PHI!()]            const SQRT_PHI: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_PHI!()]     const FRAC_1_SQRT_PHI: Self;
    #[doc = crate::_FLOAT_CONST_TRIBONACCI!()]          const TRIBONACCI: Self;
    // sqrt
    #[doc = crate::_FLOAT_CONST_SQRT_2!()]              const SQRT_2: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_2!()]       const FRAC_1_SQRT_2: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_3!()]              const SQRT_3: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_SQRT_3!()]       const FRAC_1_SQRT_3: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_5!()]              const SQRT_5: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_6!()]              const SQRT_6: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_7!()]              const SQRT_7: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_8!()]              const SQRT_8: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_10!()]             const SQRT_10: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_11!()]             const SQRT_11: Self;
    #[doc = crate::_FLOAT_CONST_SQRT_12!()]             const SQRT_12: Self;
    #[doc = crate::_FLOAT_CONST_CBRT_2!()]              const CBRT_2: Self;
    #[doc = crate::_FLOAT_CONST_CBRT_3!()]              const CBRT_3: Self;
    #[doc = crate::_FLOAT_CONST_FRAC_1_CBRT_3!()]       const FRAC_1_CBRT_3: Self;
    // other
    #[doc = crate::_FLOAT_CONST_E!()]                   const E: Self;
    #[doc = crate::_FLOAT_CONST_E!()]                   const EULER: Self;
    #[doc = crate::_FLOAT_CONST_EGAMMA!()]              const EGAMMA: Self;
    #[doc = crate::_FLOAT_CONST_EGAMMA!()]              const EULER_GAMMA: Self; // std
    #[doc = crate::_FLOAT_CONST_LOG2_E!()]              const LOG2_E: Self;
    #[doc = crate::_FLOAT_CONST_LOG2_10!()]             const LOG2_10: Self;
    #[doc = crate::_FLOAT_CONST_LOG10_E!()]             const LOG10_E: Self;
    #[doc = crate::_FLOAT_CONST_LOG10_2!()]             const LOG10_2: Self;
    #[doc = crate::_FLOAT_CONST_LN_2!()]                const LN_2: Self;
    #[doc = crate::_FLOAT_CONST_LN_10!()]               const LN_10: Self;
}

/// impl mathematical constants
///
/// $f: the floating-point type.
macro_rules! impl_float_const {
    ($( $(#[$attrs:meta])* $f:ty ),+) => { $( impl_float_const![@$(#[$attrs])* $f]; )+ };
    (@$(#[$attrs:meta])* $f:ty) => {
        /// # *Mathematical constants*.
        $(#[$attrs])*
        impl FloatConst for $f {
            // identities
            const ONE: $f = 1.0;
            const ZERO: $f = 0.0;
            const NEG_ONE: $f = -1.0;
            const NEG_ZERO: $f = -0.0;

            // representation, precision and range
            const NAN: $f = <$f>::NAN;
            const INFINITY: $f = <$f>::INFINITY;
            const NEG_INFINITY: $f = <$f>::NEG_INFINITY;
            const EPSILON: $f = <$f>::EPSILON;
            const LOW_MARGIN: $f = _FloatInternals::<$f>::LOW_MARGIN;
            const MEDIUM_MARGIN: $f = _FloatInternals::<$f>::MEDIUM_MARGIN;
            const HIGH_MARGIN: $f = _FloatInternals::<$f>::HIGH_MARGIN;
            const RADIX: u32 = <$f>::RADIX;
            const DIGITS: u32 = <$f>::DIGITS;
            const MANTISSA_DIGITS: u32 = <$f>::MANTISSA_DIGITS;
            const SIGNIFICAND_BITS: u32 = _FloatInternals::<$f>::SIGNIFICAND_BITS;
            const EXPONENT_BIAS: u32 = _FloatInternals::<$f>::EXPONENT_BIAS;
            const EXPONENT_BITS: u32 = _FloatInternals::<$f>::EXPONENT_BITS;
            const MIN: $f = <$f>::MIN;
            const MIN_POSITIVE: $f = <$f>::MIN_POSITIVE;
            const MAX: $f = <$f>::MAX;
            const MIN_EXP: i32 = <$f>::MIN_EXP;
            const MAX_EXP: i32 = <$f>::MAX_EXP;
            const MIN_10_EXP: i32 = <$f>::MIN_10_EXP;
            const MAX_10_EXP: i32 = <$f>::MAX_10_EXP;

            // pi
            const PI: $f = crate::PI!();
            const FRAC_PI_2: $f = crate::FRAC_PI_2!();
            const FRAC_PI_3: $f = crate::FRAC_PI_3!();
            const FRAC_PI_4: $f = crate::FRAC_PI_4!();
            const FRAC_PI_6: $f = crate::FRAC_PI_6!();
            const FRAC_PI_8: $f = crate::FRAC_PI_8!();
            const SQRT_PI: $f = crate::SQRT_PI!();
            const FRAC_1_PI: $f = crate::FRAC_1_PI!();
            const FRAC_1_SQRT_PI: $f = crate::FRAC_1_SQRT_PI!();
            const FRAC_1_SQRT_2PI: $f = crate::FRAC_1_SQRT_2PI!();
            const FRAC_2_PI: $f = crate::FRAC_2_PI!();
            const FRAC_2_SQRT_PI: $f = crate::FRAC_2_SQRT_PI!();

            // tau
            const TAU: $f = crate::TAU!();
            const FRAC_TAU_2: $f = crate::FRAC_TAU_2!();
            const FRAC_TAU_3: $f = crate::FRAC_TAU_3!();
            const FRAC_TAU_4: $f = crate::FRAC_TAU_4!();
            const FRAC_TAU_5: $f = crate::FRAC_TAU_5!();
            const FRAC_TAU_6: $f = crate::FRAC_TAU_6!();
            const FRAC_TAU_8: $f = crate::FRAC_TAU_8!();
            const FRAC_TAU_9: $f = crate::FRAC_TAU_9!();
            const FRAC_TAU_12: $f = crate::FRAC_TAU_12!();
            const FRAC_TAU_16: $f = crate::FRAC_TAU_16!();
            const FRAC_TAU_24: $f = crate::FRAC_TAU_24!();
            const FRAC_TAU_72: $f = crate::FRAC_TAU_72!();
            const FRAC_TAU_360: $f = crate::FRAC_TAU_360!();
            const FRAC_360_TAU: $f = crate::FRAC_360_TAU!();
            const SQRT_TAU: $f = crate::SQRT_TAU!();
            const FRAC_1_TAU: $f = crate::FRAC_1_TAU!();
            const FRAC_1_SQRT_TAU: $f = crate::FRAC_1_SQRT_TAU!();
            const FRAC_2_TAU: $f = crate::FRAC_2_TAU!();
            const FRAC_2_SQRT_TAU: $f = crate::FRAC_2_SQRT_TAU!();

            // arc degrees
            const ARC_DEGREE: $f = crate::ARC_DEGREE!();
            const ARC_MINUTE: $f = crate::ARC_MINUTE!();
            const ARC_SECOND: $f = crate::ARC_SECOND!();

            // phi
            const PHI: $f = crate::PHI!();
            const GOLDEN_RATIO: $f = crate::PHI!(); // std
            const SQ_PHI: $f = crate::SQ_PHI!();
            const FRAC_1_PHI: $f = crate::FRAC_1_PHI!();
            const NEG_FRAC_1_PHI: $f = crate::NEG_FRAC_1_PHI!();
            const SQRT_PHI: $f = crate::SQRT_PHI!();
            const FRAC_1_SQRT_PHI: $f = crate::FRAC_1_SQRT_PHI!();
            const TRIBONACCI: $f = crate::TRIBONACCI!();

            // integer roots
            const SQRT_2: $f = crate::SQRT_2!();
            const FRAC_1_SQRT_2: $f = crate::FRAC_1_SQRT_2!();
            const SQRT_3: $f = crate::SQRT_3!();
            const FRAC_1_SQRT_3: $f = crate::FRAC_1_SQRT_3!();
            const SQRT_5: $f = crate::SQRT_5!();
            const SQRT_6: $f = crate::SQRT_6!();
            const SQRT_7: $f = crate::SQRT_7!();
            const SQRT_8: $f = crate::SQRT_8!();
            const SQRT_10: $f = crate::SQRT_10!();
            const SQRT_11: $f = crate::SQRT_11!();
            const SQRT_12: $f = crate::SQRT_12!();
            const CBRT_2: $f = crate::CBRT_2!();
            const CBRT_3: $f = crate::CBRT_3!();
            const FRAC_1_CBRT_3: $f = crate::FRAC_1_CBRT_3!();

            // other
            const E: $f = crate::E!();
            const EULER: $f = crate::E!();
            const EGAMMA: $f = crate::EGAMMA!();
            const EULER_GAMMA: $f = crate::EGAMMA!(); // std
            const LOG2_E: $f = crate::LOG2_E!();
            const LOG2_10: $f = crate::LOG2_10!();
            const LOG10_E: $f = crate::LOG10_E!();
            const LOG10_2: $f = crate::LOG10_2!();
            const LN_2: $f = crate::LN_2!();
            const LN_10: $f = crate::LN_10!();
        }
    };
}
impl_float_const![f32, f64];

#[cfg(nightly_float)]
impl_float_const![
    #[cfg_attr(nightly_doc, doc(cfg(nightly_float)))]
    f16,
    #[cfg_attr(nightly_doc, doc(cfg(nightly_float)))]
    f128
];
