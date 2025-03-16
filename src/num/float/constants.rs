// devela::num::float::constants
//
//! Defines [`FloatConst`] and implements it for floating-point primitives.
//
// TOC
// - trait FloatConst
// - CONST shared doc strings
// - macro impl_ext_float_const!
// - struct TempFloat
//
// WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
// - SQRT_3, FRAC_1_SQRT_3, FRAC_1_SQRT_PI, PHI, EGAMMA…
//
// NOTE: In sync with num::float:wrapper::consts

#![allow(clippy::excessive_precision)]

use crate::Float; // because of float_technical_const_impls! consts

#[cfg(nightly_float)]
use ::core::{f128, f16};

#[doc = crate::TAG_NUM!()]
#[doc = crate::TAG_NAMESPACE!()]
/// Extension trait for floating-point types. Associated constants.
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
    #[doc = ONE!()]                 const ONE: Self;
    #[doc = ZERO!()]                const ZERO: Self;
    #[doc = NEG_ONE!()]             const NEG_ONE: Self;
    #[doc = NEG_ZERO!()]            const NEG_ZERO: Self;
    // representation, precision and range
    #[doc = NAN!()]                 const NAN: Self;
    #[doc = INFINITY!()]            const INFINITY: Self;
    #[doc = NEG_INFINITY!()]        const NEG_INFINITY: Self;
    #[doc = MIN!()]                 const MIN: Self;
    #[doc = MIN_POSITIVE!()]        const MIN_POSITIVE: Self;
    #[doc = MAX!()]                 const MAX: Self;
    #[doc = MIN_EXP!()]             const MIN_EXP: i32;
    #[doc = MAX_EXP!()]             const MAX_EXP: i32;
    #[doc = MIN_10_EXP!()]          const MIN_10_EXP: i32;
    #[doc = MAX_10_EXP!()]          const MAX_10_EXP: i32;
    #[doc = EPSILON!()]             const EPSILON: Self;
    #[doc = LOW_MARGIN!()]          const LOW_MARGIN: Self;
    #[doc = MEDIUM_MARGIN!()]       const MEDIUM_MARGIN: Self;
    #[doc = HIGH_MARGIN!()]         const HIGH_MARGIN: Self;
    #[doc = RADIX!()]               const RADIX: u32;
    #[doc = DIGITS!()]              const DIGITS: u32;
    #[doc = MANTISSA_DIGITS!()]     const MANTISSA_DIGITS: u32;
    #[doc = SIGNIFICAND_BITS!()]    const SIGNIFICAND_BITS: u32;
    #[doc = EXPONENT_BIAS!()]       const EXPONENT_BIAS: u32;
    #[doc = EXPONENT_BITS!()]       const EXPONENT_BITS: u32;
    // pi
    #[doc = PI!()]                  const PI: Self;
    #[doc = FRAC_PI_2!()]           const FRAC_PI_2: Self;
    #[doc = FRAC_PI_3!()]           const FRAC_PI_3: Self;
    #[doc = FRAC_PI_4!()]           const FRAC_PI_4: Self;
    #[doc = FRAC_PI_6!()]           const FRAC_PI_6: Self;
    #[doc = FRAC_PI_8!()]           const FRAC_PI_8: Self;
    #[doc = SQRT_PI!()]             const SQRT_PI: Self;
    #[doc = FRAC_1_PI!()]           const FRAC_1_PI: Self;
    #[doc = FRAC_1_SQRT_PI!()]      const FRAC_1_SQRT_PI: Self;
    #[doc = FRAC_1_SQRT_2PI!()]     const FRAC_1_SQRT_2PI: Self;
    #[doc = FRAC_2_PI!()]           const FRAC_2_PI: Self;
    #[doc = FRAC_2_SQRT_PI!()]      const FRAC_2_SQRT_PI: Self;
    // tau
    #[doc = TAU!()]                 const TAU: Self;
    #[doc = FRAC_TAU_2!()]          const FRAC_TAU_2: Self;
    #[doc = FRAC_TAU_3!()]          const FRAC_TAU_3: Self;
    #[doc = FRAC_TAU_4!()]          const FRAC_TAU_4: Self;
    #[doc = FRAC_TAU_5!()]          const FRAC_TAU_5: Self;
    #[doc = FRAC_TAU_6!()]          const FRAC_TAU_6: Self;
    #[doc = FRAC_TAU_8!()]          const FRAC_TAU_8: Self;
    #[doc = FRAC_TAU_9!()]          const FRAC_TAU_9: Self;
    #[doc = FRAC_TAU_12!()]         const FRAC_TAU_12: Self;
    #[doc = FRAC_TAU_16!()]         const FRAC_TAU_16: Self;
    #[doc = FRAC_TAU_24!()]         const FRAC_TAU_24: Self;
    #[doc = FRAC_TAU_72!()]         const FRAC_TAU_72: Self;
    #[doc = FRAC_TAU_360!()]        const FRAC_TAU_360: Self;
    #[doc = FRAC_360_TAU!()]        const FRAC_360_TAU: Self;
    #[doc = SQRT_TAU!()]            const SQRT_TAU: Self;
    #[doc = FRAC_1_TAU!()]          const FRAC_1_TAU: Self;
    #[doc = FRAC_1_SQRT_TAU!()]     const FRAC_1_SQRT_TAU: Self;
    #[doc = FRAC_2_TAU!()]          const FRAC_2_TAU: Self;
    #[doc = FRAC_2_SQRT_TAU!()]     const FRAC_2_SQRT_TAU: Self;
    // arc degrees
    #[doc = ARC_DEGREE!()]          const ARC_DEGREE: Self;
    #[doc = ARC_MINUTE!()]          const ARC_MINUTE: Self;
    #[doc = ARC_SECOND!()]          const ARC_SECOND: Self;
    // phi
    #[doc = PHI!()]                 const PHI: Self;
    #[doc = SQ_PHI!()]              const SQ_PHI: Self;
    #[doc = FRAC_1_PHI!()]          const FRAC_1_PHI: Self;
    #[doc = NEG_FRAC_1_PHI!()]      const NEG_FRAC_1_PHI: Self;
    #[doc = SQRT_PHI!()]            const SQRT_PHI: Self;
    #[doc = FRAC_1_SQRT_PHI!()]     const FRAC_1_SQRT_PHI: Self;
    #[doc = TRIBONACCI!()]          const TRIBONACCI: Self;
    // sqrt
    #[doc = SQRT_2!()]              const SQRT_2: Self;
    #[doc = FRAC_1_SQRT_2!()]       const FRAC_1_SQRT_2: Self;
    #[doc = SQRT_3!()]              const SQRT_3: Self;
    #[doc = FRAC_1_SQRT_3!()]       const FRAC_1_SQRT_3: Self;
    #[doc = SQRT_5!()]              const SQRT_5: Self;
    #[doc = SQRT_6!()]              const SQRT_6: Self;
    #[doc = SQRT_7!()]              const SQRT_7: Self;
    #[doc = SQRT_8!()]              const SQRT_8: Self;
    #[doc = SQRT_10!()]             const SQRT_10: Self;
    #[doc = SQRT_11!()]             const SQRT_11: Self;
    #[doc = SQRT_12!()]             const SQRT_12: Self;
    #[doc = CBRT_2!()]              const CBRT_2: Self;
    #[doc = CBRT_3!()]              const CBRT_3: Self;
    #[doc = FRAC_1_CBRT_3!()]       const FRAC_1_CBRT_3: Self;
    // other
    #[doc = E!()]                   const E: Self;
    #[doc = EGAMMA!()]              const EGAMMA: Self;
    #[doc = LOG2_E!()]              const LOG2_E: Self;
    #[doc = LOG2_10!()]             const LOG2_10: Self;
    #[doc = LOG10_E!()]             const LOG10_E: Self;
    #[doc = LOG10_2!()]             const LOG10_2: Self;
    #[doc = LN_2!()]                const LN_2: Self;
    #[doc = LN_10!()]               const LN_10: Self;
}

// Define shared doc strings
crate::CONST! { pub(in crate::num::float),
    // identities
    ONE = r#"The multiplicative identity 1."#;
    ZERO = r#"The additive identity 0."#;
    NEG_ONE = r#"The negative of the multiplicative identity -1."#;
    NEG_ZERO = r#"The negative of the additive identity -0."#;

    // representation, precision and range
    NAN = r#"Not a Number (NaN)."#;
    INFINITY = r#"Infinity (∞)."#;
    NEG_INFINITY = r#"Negative infinity (-∞)."#;
    MIN = r#"Smallest finite value."#;
    MIN_POSITIVE = r#"Smallest positive normal value."#;
    MAX = r#"Largest finite value."#;
    MIN_EXP = r#"One greater than the minimum possible normal power of 2 exponent."#;
    MAX_EXP = r#"Maximum possible power of 2 exponent."#;
    MIN_10_EXP = r#"Minimum *x* for which 10<sup>*x*</sup> is normal."#;
    MAX_10_EXP = r#"Maximum *x* for which 10<sup>*x*</sup> is normal."#;
    EPSILON = r#"Machine epsilon value.
    <p>This is the smallest difference detectable between 1.0 and the next
    representable number in the floating-point format.</p>"#;
    LOW_MARGIN = r#"Allows for minimal deviation; use for high precision needs.."#;
    MEDIUM_MARGIN = r#"Accommodates moderate deviation; balances precision and flexibility."#;
    HIGH_MARGIN = r#"Permits generous deviation; suitable for less precise scenarios."#;
    RADIX = r#"The radix or base of the internal representation."#;
    DIGITS = r#"Approximate number of significant digits in base 10."#;
    MANTISSA_DIGITS = r#"Number of significant digits in base 2."#;
    SIGNIFICAND_BITS = "Number of explicit bits used to represent the significand (or mantissa).";
    EXPONENT_BIAS = r#"Exponent bias for representing both positive and negative exponents."#;
    EXPONENT_BITS = r#"Number of bits used to represent the exponent."#;

    // pi
    PI = r#"$ π = \frac{1}{2} τ = 180º $
    ([A000796](https://oeis.org/A000796/constant))
    `≈ 3.14159265…`
    <p>*The ratio of the circumference to the diameter, a half-turn*.</p>"#;
    FRAC_PI_2 = r#"$ π/2 = τ/4 = 90º $
    ([A019669](https://oeis.org/A019669/constant))
    `≈ 1.57079632…`"#;
    FRAC_PI_3 = r#"$ π/3 = τ/6 = 60º $
    ([A019670](https://oeis.org/A019670/constant))
    `≈ 1.04719755…`"#;
    FRAC_PI_4 = r#"$ π/4 = τ/8 = 45º $
    ([A003881](https://oeis.org/A003881/constant))
    `≈ 0.78539816…`"#;
    FRAC_PI_6 = r#"$ π/6 = τ/12 = 30º $
    ([A019673](https://oeis.org/A019673/constant))
    `≈ 0.52359877…`"#;
    FRAC_PI_8 = r#"$ π/8 = τ/16 = 22.5º $
    ([A019675](https://oeis.org/A019675/constant))
    `≈ 0.39269908…`"#;
    SQRT_PI = r#"$ \sqrt{π} = \sqrt{\frac{1}{2} τ} $
    ([A002161](https://oeis.org/A002161/constant))
    `≈ 1.77245385…`"#;
    FRAC_1_PI = r#"$ 1/π = 2/τ $
    ([A049541](https://oeis.org/A049541/constant))
    `≈ 0.31830988…`"#;
    FRAC_1_SQRT_PI = r#"$ 1/\sqrt{π} = 1/\sqrt{τ/2} $
    ([A087197](https://oeis.org/A087197/constant))
    `≈ 0.56418958…`"#;
    FRAC_1_SQRT_2PI = r#"$ 1/\sqrt{2π} = 1/\sqrt{τ} $
    ([A231863](https://oeis.org/A231863/constant))
    `≈ 0.39894228…`"#;
    FRAC_2_PI = r#"$ 2/π $
    ([A060294](https://oeis.org/A060294/constant))
    `≈ 0.63661977…`
    <p>*Buffon's constant*.</p>"#;
    FRAC_2_SQRT_PI = r#"$ 2/\sqrt{π} $
    ([A190732](https://oeis.org/A190732/constant))
    `≈ 1.12837916…`"#;

    // tau
    TAU = r#"$ τ = 2π = 360º $
    ([A019692](https://oeis.org/A019692/constant))
    `≈ 6.28318530…`
    <p>*The ratio of the circumference to the radius, a full-turn*.</p>"#;
    FRAC_TAU_2 = r#"$ τ/2 = π = 180º $
    ([A000796](https://oeis.org/A000796/constant))
    `≈ 3.14159265…`"#;
    FRAC_TAU_3 = r#"$ τ/3  = 2π/3 = 120º $
    ([A019693](https://oeis.org/A019693/constant))
    `≈ 2.09439510…`"#;
    FRAC_TAU_4 = r#"$ τ/4 = π/2 = 90º $
    ([A019693](https://oeis.org/A019693/constant))
    `≈ 1.57079632…`"#;
    FRAC_TAU_5 = r#"$ τ/5 = 2π/5 = 72º $
    ([A019694](https://oeis.org/A019694/constant))
    `≈ 1.25663706…`"#;
    FRAC_TAU_6 = r#"$ τ/6 = π/3 = 60º $
    ([A019670](https://oeis.org/A019670/constant))
    `≈ 1.04719755…`"#;
    FRAC_TAU_8 = r#"$ τ/8 = π/4 = 45º $
    ([A003881](https://oeis.org/A003881/constant))
    `≈ 0.78539816…`"#;
    FRAC_TAU_9 = r#"$ τ/9 = 2π/9 = 40º $
    ([A019696](https://oeis.org/A019696/constant))
    `≈ 0.69813170…`"#;
    FRAC_TAU_12 = r#"$ τ/12 = π/6 = 30º $
    ([A019673](https://oeis.org/A019673/constant))
    `≈ 0.52359877…`"#;
    FRAC_TAU_16 = r#"$ τ/16 = π/8 = 22.5º $
    ([A019675](https://oeis.org/A019675/constant))
    `≈ 0.39269908…`"#;
    FRAC_TAU_24 = r#"$ τ/24 = π/12 = 15º $
    ([A019679](https://oeis.org/A019679/constant))
    `≈ 0.26179938…`"#;
    FRAC_TAU_72 = r#"$ τ/72 = π/36 = 5º $
    `≈ 0.08726646…`"#;
    FRAC_TAU_360 = r#"$ τ/360 = π/180 = 1º $ *arc degree*
    ([A019685](https://oeis.org/A019685),
    [wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
    `≈ 0.01745329…`"#;
    FRAC_360_TAU = r#"$ 360/τ = 180/π $
    ([A072097](https://oeis.org/A072097/constant))
    `≈ 57.2957795…`"#;
    SQRT_TAU = r#"$ \sqrt{τ} = \sqrt{2π} $
    ([A019727](https://oeis.org/A019727/constant))
    `≈ 2.50662827…`"#;
    FRAC_1_TAU = r#"$ 1/τ = 1/2π $
    ([A086201](https://oeis.org/A086201/constant))
    `≈ 0.15915494…`"#;
    FRAC_1_SQRT_TAU = r#"$ 1/\sqrt{τ} = 1/\sqrt{2π} $
    ([A231863](https://oeis.org/A231863/constant))
    `≈ 0.39894228…`"#;
    FRAC_2_TAU = r#"$ 2/τ = 1/π $
    ([A049541](https://oeis.org/A049541/constant))
    `≈ 0.31830988…`"#;
    FRAC_2_SQRT_TAU = r#"$ 2/\sqrt{τ} = \sqrt{2/π} $
    ([A076668](https://oeis.org/A076668/constant))
    `≈ 0.79788456…`"#;

    // arc degrees
    ARC_DEGREE = r#"$ τ/360 = π/180 = 1º $ *arc degree*
    ([A019685](https://oeis.org/A019685),
    [wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
    `≈ 0.01745329…`"#;
    ARC_MINUTE = r#"$ τ/(360*60) = 1' $ *arc minute*
    ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
    `≈ 0.00029088…`"#;
    ARC_SECOND = r#"$ τ/(360 * 60 * 60) = 1'' $ *arc second*
    ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
    `≈ 0.00000484…`"#;

    // phi
    PHI = r#"$ φ  = (1+\sqrt{5})/2 $
    ([A001622](https://oeis.org/A001622/constant))
    `≈ 1.61803398…`
    <p>*The golden ratio*.</p>
    <p>Continued fraction: $ [1;1,1,1,…] $</p>"#;
    SQ_PHI = r#"$ φ^2 = φ+1 = (3+\sqrt{5})/2 $
    ([A104457](https://oeis.org/A104457/constant))
    `≈ 2.61803398…`"#;
    FRAC_1_PHI = r#"$ 1/φ = φ-1 $
    ([A094214](https://oeis.org/A094214/constant))
    `≈ 0.61803398…`
    <p>*The reciprocal of [φ][Self#PHI]*.</p>"#;
    NEG_FRAC_1_PHI = r#"$ -1/φ = 1-φ $
    `≈ -0.61803398…`
    <p>*The negative reciprocal of [φ][Self#PHI] and its conjugate in $ x^2-x-1 $*.</p>"#;
    SQRT_PHI = r#"$ \sqrt{φ} $
    ([A139339](https://oeis.org/A139339/constant))
    `≈ 1.27201964…`"#;
    FRAC_1_SQRT_PHI = r#"$ 1/\sqrt{φ} = \sqrt{φ/φ^2} = \sqrt{φ^2-2} $
    ([A197762](https://oeis.org/A197762/constant))
    `≈ 0.78615137…`"#;
    TRIBONACCI = r#"([A058265](https://oeis.org/A058265/constant))
    `≈ 1.83928675…`
    <p>*The tribonacci constant*.</p>"#;

    // integer roots
    SQRT_2 = r#"$ \sqrt{2} $
    ([A002193](https://oeis.org/A002193/constant),
    [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2))
    `≈ 1.41421356…`"#;
    FRAC_1_SQRT_2 = r#"$ 1/\sqrt{2} = \sqrt{1/2} $
    ([A010503](https://oeis.org/A010503/constant),
    [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2#Multiplicative_inverse))
    `≈ 0.70710678…`"#;
    SQRT_3 = r#"$ \sqrt{3} $
    ([A002194](https://oeis.org/A002194/constant),
    [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_3))
    `≈ 1.73205080…`"#;
    FRAC_1_SQRT_3 = r#"$ 1/\sqrt{3} = \sqrt{1/3} $
    ([A020760](https://oeis.org/A002194/constant),
    `≈ 0.57735026…`"#;
    SQRT_5 = r#"$ \sqrt{5} $
    ([A002163](https://oeis.org/A002163/constant),
    [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_5))
    `≈ 2.23606797…`"#;
    SQRT_6 = r#"$ \sqrt{6} $
    ([A010464](https://oeis.org/A010464/constant))
    `≈ 2.44948974…`"#;
    SQRT_7 = r#"$ \sqrt{7} $
    ([A010465](https://oeis.org/A010465/constant))
    `≈ 2.64575131…`"#;
    SQRT_8 = r#"$ \sqrt{8} $
    ([A010466](https://oeis.org/A010466/constant))
    `≈ 2.82842712…`"#;
    SQRT_10 = r#"$ \sqrt{10} $
    ([A010467](https://oeis.org/A010467/constant))
    `≈ 3.16227766…`"#;
    SQRT_11 = r#"$ \sqrt{11} $
    ([A010468](https://oeis.org/A010468/constant))
    `≈ 3.31662479…`"#;
    SQRT_12 = r#"$ \sqrt{12} $
    ([A010469](https://oeis.org/A010469/constant))
    `≈ 3.46410161…`"#;
    CBRT_2 = r#"$ \sqrt[\small 3]{2} $
    ([A002580](https://oeis.org/A002580/constant),
    [wikipedia](https://en.wikipedia.org/wiki/Doubling_the_cube))
    `≈ 1.25992104…`"#;
    CBRT_3 = r#"$ \sqrt[\small 3]{3} $
    ([A002581](https://oeis.org/A002581/constant))
    `≈ 1.44224957…`"#;
    FRAC_1_CBRT_3 = r#"$ 1/\sqrt[\small 3]{3} = (\normalsize\frac{1}{3})^{\small\frac{1}{3}} $
    ([A072365](https://oeis.org/A072365/constant))
    `≈ 0.69336127…`"#;

    // other
    E = r#"$ e $
    ([A001113](https://oeis.org/A001113/constant))
    `≈ 2.71828182…`
    <p>*The Euler number or Napier's constant*.</p>
    <p>Continuous fraction: $ [2;1,2,1,1,4,1,1,6,1,…,1,2n,1,…] $</p>"#;
    EGAMMA = r#"$ γ $
    ([A001620](https://oeis.org/A001620/constant))
    `≈ 0.57721566…`
    <p>*Gamma, or the Euler-Mascheroni constant.*</p>"#;
    LOG2_E = r#"$ \log_2{e} $
    ([A007525](https://oeis.org/A007525/constant))
    `≈ 1.44269504…`"#;
    LOG2_10 = r#"log<sub>2</sub>(10)
    ([A020862](https://oeis.org/A020862/constant))
    `≈ 3.32192809…`"#;
    LOG10_E = r#"log<sub>10</sub>(e)
    ([A002285](https://oeis.org/A002285/constant))
    `≈ 0.43429448…`"#;
    LOG10_2 = r#"log<sub>10</sub>(2)
    ([A007524](https://oeis.org/A007524/constant))
    `≈ 0.30102999…`"#;
    LN_2 = r#"ln(2)
    ([A002162](https://oeis.org/A002162/constant))
    `≈ 0.69314718…`"#;
    LN_10 = r#"ln(10)
    ([A002392](https://oeis.org/A002392/constant))
    `≈ 2.30258509…`"#;
}

/// impl mathematical constants
///
/// $f: the floating-point type.
macro_rules! impl_ext_float_const {
    ($( $(#[$attrs:meta])* $f:ty ),+) => { $( impl_ext_float_const![@$(#[$attrs])* $f]; )+ };
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
            const LOW_MARGIN: $f = TempFloat::<$f>::LOW_MARGIN;
            const MEDIUM_MARGIN: $f = TempFloat::<$f>::MEDIUM_MARGIN;
            const HIGH_MARGIN: $f = TempFloat::<$f>::HIGH_MARGIN;
            const RADIX: u32 = <$f>::RADIX;
            const DIGITS: u32 = <$f>::DIGITS;
            const MANTISSA_DIGITS: u32 = <$f>::MANTISSA_DIGITS;
            const SIGNIFICAND_BITS: u32 = Float::<$f>::SIGNIFICAND_BITS;
            const EXPONENT_BIAS: u32 = Float::<$f>::EXPONENT_BIAS;
            const EXPONENT_BITS: u32 = Float::<$f>::EXPONENT_BITS;
            const MIN: $f = <$f>::MIN;
            const MIN_POSITIVE: $f = <$f>::MIN_POSITIVE;
            const MAX: $f = <$f>::MAX;
            const MIN_EXP: i32 = <$f>::MIN_EXP;
            const MAX_EXP: i32 = <$f>::MAX_EXP;
            const MIN_10_EXP: i32 = <$f>::MIN_10_EXP;
            const MAX_10_EXP: i32 = <$f>::MAX_10_EXP;

            // pi
            const PI: $f =
             3.14159265358979323846264338327950288419716939937510582097494459230781640628620899;
            const FRAC_PI_2: $f =
             1.57079632679489661923132169163975144209858469968755291048747229615390820314310449;
            const FRAC_PI_3: $f =
             1.04719755119659774615421446109316762806572313312503527365831486410260546876206966;
            const FRAC_PI_4: $f =
             0.78539816339744830961566084581987572104929234984377645524373614807695410157155224;
            const FRAC_PI_6: $f =
             0.52359877559829887307710723054658381403286156656251763682915743205130273438103483;
            const FRAC_PI_8: $f =
             0.39269908169872415480783042290993786052464617492188822762186807403847705078577612;
            const SQRT_PI: $f =
             1.77245385090551602729816748334114518279754945612238712821380778985291128459103218;
            const FRAC_1_PI: $f =
             0.31830988618379067153776752674502872406891929148091289749533468811779359526845307;
            const FRAC_1_SQRT_PI: $f =
             0.56418958354775628694807945156077258584405062932899885684408572171064246844149341;
            const FRAC_1_SQRT_2PI: $f = Self::FRAC_1_SQRT_TAU;
            const FRAC_2_PI: $f =
             0.63661977236758134307553505349005744813783858296182579499066937623558719053690614;
            const FRAC_2_SQRT_PI: $f =
             1.12837916709551257389615890312154517168810125865799771368817144342128493688298683;

            // tau
            const TAU: $f =
             6.28318530717958647692528676655900576839433879875021164194988918461563281257241799;
            const FRAC_TAU_2: $f = Self::PI;
            const FRAC_TAU_3: $f =
             2.09439510239319549230842892218633525613144626625007054731662972820521093752413933;
            const FRAC_TAU_4: $f = Self::FRAC_PI_2;
            const FRAC_TAU_5: $f =
             1.25663706143591729538505735331180115367886775975004232838997783692312656251448359;
            const FRAC_TAU_6: $f = Self::FRAC_PI_3;
            const FRAC_TAU_8: $f = Self::FRAC_PI_4;
            const FRAC_TAU_9: $f =
             0.69813170079773183076947630739544508537714875541669018243887657606840364584137977;
            const FRAC_TAU_12: $f = Self::FRAC_PI_6;
            const FRAC_TAU_16: $f = Self::FRAC_PI_8;
            const FRAC_TAU_24: $f =
             0.26179938779914943653855361527329190701643078328125881841457871602565136719051741;
            const FRAC_TAU_72: $f =
             0.08726646259971647884618453842443063567214359442708627280485957200855045573017247;
            const FRAC_TAU_360: $f =
             0.01745329251994329576923690768488612713442871888541725456097191440171009114603449;
            const FRAC_360_TAU: $f =
            57.29577951308232087679815481410517033240547246656432154916024386120284714832155263;
            const SQRT_TAU: $f =
             2.50662827463100050241576528481104525300698674060993831662992357634229365460784197;
            const FRAC_1_TAU: $f =
             0.15915494309189533576888376337251436203445964574045644874766734405889679763422653;
            const FRAC_1_SQRT_TAU: $f =
             0.39894228040143267793994605993438186847585863116493465766592582967065792589930183;
            const FRAC_2_TAU: $f = Self::FRAC_1_PI;
            const FRAC_2_SQRT_TAU: $f =
             0.79788456080286535587989211986876373695171726232986931533185165934131585179860367;

            // arc degrees
            const ARC_DEGREE: $f = Self::FRAC_TAU_360;
            const ARC_MINUTE: $f =
             0.00029088820866572159615394846141476878557381198142362090934953190669516818576724;
            const ARC_SECOND: $f =
             0.00000484813681109535993589914102357947975956353302372701515582553177825280309612;

            // phi
            const PHI: $f =
             1.61803398874989484820458683436563811772030917980576286213544862270526046281890244;
            const SQ_PHI: $f =
             2.61803398874989484820458683436563811772030917980576286213544862270526046281890244;
            const FRAC_1_PHI: $f =
             0.61803398874989484820458683436563811772030917980576286213544862270526046281890244;
            const NEG_FRAC_1_PHI: $f =
            -0.61803398874989484820458683436563811772030917980576286213544862270526046281890244;
            const SQRT_PHI: $f =
             1.27201964951406896425242246173749149171560804184009624861664038253929757553606801;
            const FRAC_1_SQRT_PHI: $f =
             0.78615137775742328606955858584295892952312205783772323766490197010118204762231091;
            const TRIBONACCI: $f =
             1.83928675521416113255185256465328660042417874609759224677875863940420322208196642;

            // integer roots
            const SQRT_2: $f =
             1.41421356237309504880168872420969807856967187537694807317667973799073247846210703;
            const FRAC_1_SQRT_2: $f =
             0.70710678118654752440084436210484903928483593768847403658833986899536623923105351;
            const SQRT_3: $f =
             1.73205080756887729352744634150587236694280525381038062805580697945193301690880003;
            const FRAC_1_SQRT_3: $f =
             0.57735026918962576450914878050195745564760175127012687601860232648397767230293334;
            const SQRT_5: $f =
             2.23606797749978969640917366873127623544061835961152572427089724541052092563780489;
            const SQRT_6: $f =
             2.44948974278317809819728407470589139196594748065667012843269256725096037745731502;
            const SQRT_7: $f =
             2.64575131106459059050161575363926042571025918308245018036833445920106882323028362;
            const SQRT_8: $f =
             2.82842712474619009760337744841939615713934375075389614635335947598146495692421407;
            const SQRT_10: $f =
             3.16227766016837933199889354443271853371955513932521682685750485279259443863923822;
            const SQRT_11: $f =
             3.31662479035539984911493273667068668392708854558935359705868214611648464260904384;
            const SQRT_12: $f =
             3.46410161513775458705489268301174473388561050762076125611161395890386603381760007;
            const CBRT_2: $f =
             1.25992104989487316476721060727822835057025146470150798008197511215529967651395948;
            const CBRT_3: $f =
             1.44224957030740838232163831078010958839186925349935057754641619454168759682999733;
            const FRAC_1_CBRT_3: $f =
             0.69336127435063470484335227478596179544593511345775403656586369340003543713242292;

            // other
            const E: $f =
             2.71828182845904523536028747135266249775724709369995957496696762772407663035354759;
            const EGAMMA: $f =
             0.57721566490153286060651209008240243104215933593992359880576723488486772677766467;
            const LOG2_E: $f =
             1.44269504088896340735992468100189213742664595415298593413544940693110921918118507;
            const LOG2_10: $f =
             3.32192809488736234787031942948939017586483139302458061205475639581593477660862521;
            const LOG10_E: $f =
             0.43429448190325182765112891891660508229439700580366656611445378316586464920887077;
            const LOG10_2: $f =
             0.30102999566398119521373889472449302676818988146210854131042746112710818927442450;
            const LN_2: $f =
             0.69314718055994530941723212145817656807550013436025525412068000949339362196969471;
            const LN_10: $f =
             2.30258509299404568401799145468436420760110148862877297603332790096757260967735248;
        }
    };
}
impl_ext_float_const![f32, f64];
#[cfg(nightly_float)]
impl_ext_float_const![
    #[cfg_attr(nightly_doc, doc(cfg(nightly_float)))]
    f16,
    #[cfg_attr(nightly_doc, doc(cfg(nightly_float)))]
    f128
];

/// Private helper struct to define manual, type-dependendent constants.
struct TempFloat<T> {
    _marker: ::core::marker::PhantomData<T>,
}
macro_rules! impl_temp_float {
    () => {
        #[cfg(nightly_float)]
        impl_temp_float![f16: 1e-4, 1e-3, 1e-2]; // ~3–4 decimal digits of precision.
        impl_temp_float![f32: 1e-7, 1e-6, 1e-5]; // ~7 decimal digits of precision.
        impl_temp_float![f64: 1e-12, 1e-9, 1e-6]; // ~15–16 decimal digits of precision.
        #[cfg(nightly_float)]
        impl_temp_float![f128: 1e-30, 1e-27, 1e-24]; // ~33–34 decimal digits of precision.
    };
    ($f:ty: $lm:literal, $mm:literal, $hm:literal) => {
        impl TempFloat<$f> {
            // Practical margins of error.
            pub const LOW_MARGIN: $f = $lm;
            pub const MEDIUM_MARGIN: $f = $mm;
            pub const HIGH_MARGIN: $f = $hm;
        }
    };
}
impl_temp_float![];
