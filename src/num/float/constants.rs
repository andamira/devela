// devela::num::float::ext_trait
//
//! Extention trait for floating-point constants.
//

#![allow(clippy::excessive_precision)]

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
/// [`MANTISSA_DIGITS`].
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
pub trait ExtFloatConst: Sized {
    /* identities */

    /// The multiplicative identity 1.
    const ONE: Self;
    /// The additive identity 0.
    const ZERO: Self;
    /// The negative of the multiplicative identity -1.
    const NEG_ONE: Self;
    /// The negative of the additive identity -0.
    const NEG_ZERO: Self;

    /* representation, precision and range */

    /// Not a Number (NaN).
    const NAN: Self;
    /// Infinity (∞).
    const INFINITY: Self;
    /// Negative infinity (-∞).
    const NEG_INFINITY: Self;

    /// Smallest finite value.
    const MIN: Self;
    /// Smallest positive normal value.
    const MIN_POSITIVE: Self;
    /// Largest finite value.
    const MAX: Self;

    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32;
    /// Maximum possible power of 2 exponent.
    const MAX_EXP: i32;

    /// Minimum *x* for which 10<sup>*x*</sup> is normal.
    const MIN_10_EXP: i32;
    /// Maximum *x* for which 10<sup>*x*</sup> is normal.
    const MAX_10_EXP: i32;

    /// Machine epsilon value.
    ///
    /// This is the smallest difference detectable between 1.0 and the next
    /// representable number in the floating-point format.
    const EPSILON: Self;
    /// Allows for minimal deviation; use for high precision needs.
    const LOW_MARGIN: Self;
    /// Accommodates moderate deviation; balances precision and flexibility.
    const MEDIUM_MARGIN: Self;
    /// Permits generous deviation; suitable for less precise scenarios.
    const HIGH_MARGIN: Self;

    /// The radix or base of the internal representation.
    const RADIX: u32;
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32;
    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32;

    /* consts: pi */

    /// $ π = \frac{1}{2} τ = 180º $
    /// ([A000796](https://oeis.org/A000796/constant))
    /// `≈ 3.14159265…`
    ///
    /// *The ratio of the circumference to the diameter, a half-turn*.
    const PI: Self;

    /// $ π/2 = τ/4 = 90º $
    /// ([A019669](https://oeis.org/A019669/constant))
    /// `≈ 1.57079632…`
    const FRAC_PI_2: Self;

    /// $ π/3 = τ/6 = 60º $
    /// ([A019670](https://oeis.org/A019670/constant))
    /// `≈ 1.04719755…`
    const FRAC_PI_3: Self;

    /// $ π/4 = τ/8 = 45º $
    /// ([A003881](https://oeis.org/A003881/constant))
    /// `≈ 0.78539816…`
    const FRAC_PI_4: Self;

    /// $ π/6 = τ/12 = 30º $
    /// ([A019673](https://oeis.org/A019673/constant))
    /// `≈ 0.52359877…`
    const FRAC_PI_6: Self;

    /// $ π/8 = τ/16 = 22.5º $
    /// ([A019675](https://oeis.org/A019675/constant))
    /// `≈ 0.39269908…`
    const FRAC_PI_8: Self;

    /// $ \sqrt{π} = \sqrt{\frac{1}{2} τ} $
    /// ([A002161](https://oeis.org/A002161/constant))
    /// `≈ 1.77245385…`
    const SQRT_PI: Self;

    /// $ 1/π = 2/τ $
    /// ([A049541](https://oeis.org/A049541/constant))
    /// `≈ 0.31830988…`
    const FRAC_1_PI: Self;

    /// $ 1/\sqrt{π} = 1/\sqrt{τ/2} $
    /// ([A087197](https://oeis.org/A087197/constant))
    /// `≈ 0.56418958…`
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const FRAC_1_SQRT_PI: Self;

    /// $ 2/π $
    /// ([A060294](https://oeis.org/A060294/constant))
    /// `≈ 0.63661977…`
    ///
    /// *Buffon's constant*.
    const FRAC_2_PI: Self;

    /// $ 2/\sqrt{π} $
    /// ([A190732](https://oeis.org/A190732/constant))
    /// `≈ 1.12837916…`
    const FRAC_2_SQRT_PI: Self;

    /* consts: TAU */

    /// $ τ = 2π = 360º $
    /// ([A019692](https://oeis.org/A019692/constant))
    /// `≈ 6.28318530…`
    ///
    /// *The ratio of the circumference to the radius, a full-turn*.
    const TAU: Self;

    /// $ τ/2 = π = 180º $
    /// ([A000796](https://oeis.org/A000796/constant))
    /// `≈ 3.14159265…`
    const FRAC_TAU_2: Self;

    /// $ τ/3  = 2π/3 = 120º $
    /// ([A019693](https://oeis.org/A019693/constant))
    /// `≈ 2.09439510…`
    const FRAC_TAU_3: Self;

    /// $ τ/4 = π/2 = 90º $
    /// ([A019693](https://oeis.org/A019693/constant))
    /// `≈ 1.57079632…`
    const FRAC_TAU_4: Self;

    /// $ τ/5 = 2π/5 = 72º $
    /// ([A019694](https://oeis.org/A019694/constant))
    /// `≈ 1.25663706…`
    const FRAC_TAU_5: Self;

    /// $ τ/6 = π/3 = 60º $
    /// ([A019670](https://oeis.org/A019670/constant))
    /// `≈ 1.04719755…`
    const FRAC_TAU_6: Self;

    /// $ τ/8 = π/4 = 45º $
    /// ([A003881](https://oeis.org/A003881/constant))
    /// `≈ 0.78539816…`
    const FRAC_TAU_8: Self;

    /// $ τ/9 = 2π/9 = 40º $
    /// ([A019696](https://oeis.org/A019696/constant))
    /// `≈ 0.69813170…`
    const FRAC_TAU_9: Self;

    /// $ τ/12 = π/6 = 30º $
    /// ([A019673](https://oeis.org/A019673/constant))
    /// `≈ 0.52359877…`
    const FRAC_TAU_12: Self;

    /// $ τ/16 = π/8 = 22.5º $
    /// ([A019675](https://oeis.org/A019675/constant))
    /// `≈ 0.39269908…`
    const FRAC_TAU_16: Self;

    /// $ τ/24 = π/12 = 15º $
    /// ([A019679](https://oeis.org/A019679/constant))
    /// `≈ 0.26179938…`
    const FRAC_TAU_24: Self;

    /// $ τ/72 = π/36 = 5º $
    /// `≈ 0.08726646…`
    const FRAC_TAU_72: Self;

    /// $ τ/360 = π/180 = 1º $ *arc degree*
    /// ([A019685](https://oeis.org/A019685),
    /// [wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
    /// `≈ 0.01745329…`
    const FRAC_TAU_360: Self;

    /// $ 360/τ = 180/π $
    /// ([A072097](https://oeis.org/A072097/constant))
    /// `≈ 57.2957795…`
    const FRAC_360_TAU: Self;

    /// $ \sqrt{τ} = \sqrt{2π} $
    /// ([A019727](https://oeis.org/A019727/constant))
    /// `≈ 2.50662827…`
    const SQRT_TAU: Self;

    /// $ 1/τ = 1/2π $
    /// ([A086201](https://oeis.org/A086201/constant))
    /// `≈ 0.15915494…`
    const FRAC_1_TAU: Self;

    /// $ 1/\sqrt{τ} = 1/\sqrt{2π} $
    /// ([A231863](https://oeis.org/A231863/constant))
    /// `≈ 0.39894228…`
    const FRAC_1_SQRT_TAU: Self;

    /// $ 2/τ = 1/π $
    /// ([A049541](https://oeis.org/A049541/constant))
    /// `≈ 0.31830988…`
    const FRAC_2_TAU: Self;

    /// $ 2/\sqrt{τ} = \sqrt{2/π} $
    /// ([A231863](https://oeis.org/A231863/constant))
    /// `≈ 0.79788456…`
    const FRAC_2_SQRT_TAU: Self;

    /* degrees */

    /// $ τ/360 = π/180 = 1º $ *arc degree*
    /// ([A019685](https://oeis.org/A019685),
    /// [wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
    /// `≈ 0.01745329…`
    const ARC_DEGREE: Self;

    /// $ τ/(360*60) = 1' $ *arc minute*
    /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
    /// `≈ 0.00029088…`
    const ARC_MINUTE: Self;

    /// $ τ/(360 * 60 * 60) = 1'' $ *arc second*
    /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
    /// `≈ 0.00000484…`
    const ARC_SECOND: Self;
    /* consts: PHI */

    /// $ φ  = (1+\sqrt{5})/2 $
    /// ([A001622](https://oeis.org/A001622/constant))
    /// `≈ 1.61803398…`
    ///
    /// *The golden ratio*.
    ///
    /// Continued fraction: $ [1;1,1,1,…] $
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const PHI: Self;

    /// $ φ^2 = φ+1 = (3+\sqrt{5})/2 $
    /// ([A104457](https://oeis.org/A104457/constant))
    /// `≈ 2.61803398…`
    const SQ_PHI: Self;

    /// $ 1/φ = φ-1 $
    /// ([A094214](https://oeis.org/A094214/constant))
    /// `≈ 0.61803398…`
    ///
    /// *The reciprocal of [φ][Self#PHI]*.
    const FRAC_1_PHI: Self;

    /// $ -1/φ = 1-φ $
    /// `≈ -0.61803398…`
    ///
    /// *The negative reciprocal of [φ][Self#PHI] and its conjugate in $ x^2-x-1 $*.
    const NEG_FRAC_1_PHI: Self;

    /// $ \sqrt{φ} $
    /// ([A139339](https://oeis.org/A139339/constant))
    /// `≈ 1.27201964…`
    const SQRT_PHI: Self;

    /// $ 1/\sqrt{φ} = \sqrt{φ/φ^2} = \sqrt{φ^2-2} $
    /// ([A197762](https://oeis.org/A197762/constant))
    /// `≈ 0.78615137…`
    const FRAC_1_SQRT_PHI: Self;

    /// ([A058265](https://oeis.org/A058265/constant))
    /// `≈ 1.83928675…`
    ///
    /// *The tribonacci constant*.
    const TRIBONACCI: Self;

    /* consts: sqrt */

    /// $ \sqrt{2} $
    /// ([A002193](https://oeis.org/A002193/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2))
    /// `≈ 1.41421356…`
    const SQRT_2: Self;

    /// $ 1/\sqrt{2} = \sqrt{1/2} $
    /// ([A010503](https://oeis.org/A010503/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2#Multiplicative_inverse))
    /// `≈ 0.70710678…`
    const FRAC_1_SQRT_2: Self;

    /// $ \sqrt{3} $
    /// ([A002194](https://oeis.org/A002194/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_3))
    /// `≈ 1.73205080…`
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const SQRT_3: Self;

    /// $ 1/\sqrt{3} = \sqrt{1/3} $
    /// `≈ 0.57735026…`
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const FRAC_1_SQRT_3: Self;

    /// $ \sqrt{5} $
    /// ([A002163](https://oeis.org/A002163/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_5))
    /// `≈ 2.23606797…`
    const SQRT_5: Self;

    /// $ \sqrt{6} $
    /// ([A010464](https://oeis.org/A010464/constant))
    /// `≈ 2.44948974…`
    const SQRT_6: Self;

    /// $ \sqrt{7} $
    /// ([A010465](https://oeis.org/A010465/constant))
    /// `≈ 2.64575131…`
    const SQRT_7: Self;

    /// $ \sqrt{8} $
    /// ([A010466](https://oeis.org/A010466/constant))
    /// `≈ 2.82842712…`
    const SQRT_8: Self;

    /// $ \sqrt{10} $
    /// ([A010467](https://oeis.org/A010467/constant))
    /// `≈ 3.16227766…`
    const SQRT_10: Self;

    /// $ \sqrt{11} $
    /// ([A010468](https://oeis.org/A010468/constant))
    /// `≈ 3.31662479…`
    const SQRT_11: Self;

    /// $ \sqrt{12} $
    /// ([A010469](https://oeis.org/A010469/constant))
    /// `≈ 3.46410161…`
    const SQRT_12: Self;

    /* consts: cbrt */

    /// $ \sqrt[\small 3]{2} $
    /// ([A002580](https://oeis.org/A002580/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Doubling_the_cube))
    /// `≈ 1.25992104…`
    const CBRT_2: Self;

    /// $ \sqrt[\small 3]{3} $
    /// ([A002581](https://oeis.org/A002581/constant))
    /// `≈ 1.44224957…`
    const CBRT_3: Self;

    /// $ 1/\sqrt[\small 3]{3} = (\normalsize\frac{1}{3})^{\small\frac{1}{3}} $
    /// ([A072365](https://oeis.org/A072365/constant))
    /// `≈ 0.69336127…`
    const FRAC_1_CBRT_3: Self;

    /* consts: other */

    /// $ e $
    /// ([A001113](https://oeis.org/A001113/constant))
    /// `≈ 2.71828182…`
    ///
    /// *The Euler number or Napier's constant*.
    ///
    /// Continuous fraction: $ [2;1,2,1,1,4,1,1,6,1,…,1,2n,1,…] $
    const E: Self;

    /// $ γ $
    /// ([A001620](https://oeis.org/A001620/constant))
    /// `≈ 0.57721566…`
    ///
    /// *Gamma, or the Euler-Mascheroni constant*.
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const EGAMMA: Self;

    /// $ \log_2{e} $
    /// ([A007525](https://oeis.org/A007525/constant))
    /// `≈ 1.44269504…`
    const LOG2_E: Self;

    /// log<sub>2</sub>(10)
    /// ([A020862](https://oeis.org/A020862/constant))
    /// `≈ 3.32192809…`
    const LOG2_10: Self;

    /// log<sub>10</sub>(e)
    /// ([A002285](https://oeis.org/A002285/constant))
    /// `≈ 0.43429448…`
    const LOG10_E: Self;

    /// log<sub>10</sub>(2)
    /// ([A007524](https://oeis.org/A007524/constant))
    /// `≈ 0.30102999…`
    const LOG10_2: Self;

    /// ln(2)
    /// ([A002162](https://oeis.org/A002162/constant))
    /// `≈ 0.69314718…`
    const LN_2: Self;

    /// ln(10)
    /// ([A002392](https://oeis.org/A002392/constant))
    /// `≈ 2.30258509…`
    const LN_10: Self;
}

// Private helper struct to define manual, type-dependendent constants.
struct TempFloat<T> {
    _marker: core::marker::PhantomData<T>,
}
macro_rules! impl_temp_float {
    () => {
        impl_temp_float![f32: 1e-7, 1e-6, 1e-5];
        impl_temp_float![f64: 1e-12, 1e-9, 1e-6];
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

// impl mathematical constants
//
// $f: the floating-point type.
macro_rules! math_const_impls {
    ($( $f:ty),+) => { $( math_const_impls![@$f]; )+ };
    (@$f:ty) => {
        /// # *Mathematical constants*.
        impl ExtFloatConst for $f {
            const ONE: $f = 1.0;
            const ZERO: $f = 0.0;
            const NEG_ONE: $f = -1.0;
            const NEG_ZERO: $f = -0.0;

            // ...

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

            const MIN: $f = <$f>::MIN;
            const MIN_POSITIVE: $f = <$f>::MIN_POSITIVE;
            const MAX: $f = <$f>::MAX;

            const MIN_EXP: i32 = <$f>::MIN_EXP;
            const MAX_EXP: i32 = <$f>::MAX_EXP;

            const MIN_10_EXP: i32 = <$f>::MIN_10_EXP;
            const MAX_10_EXP: i32 = <$f>::MAX_10_EXP;

            /* Arc degrees */

            const ARC_DEGREE: $f = Self::FRAC_TAU_360;
            const ARC_MINUTE: $f = 0.00029088820866572159615394846141477;
            const ARC_SECOND: $f = 0.00000484813681109535993589914102358;

            /* Mathematical constants related to Pi (π) */

            const PI: $f = 3.14159265358979323846264338327950288;
            const FRAC_PI_2: $f = 1.57079632679489661923132169163975144;
            const FRAC_PI_3: $f = 1.04719755119659774615421446109316763;
            const FRAC_PI_4: $f = 0.785398163397448309615660845819875721;
            const FRAC_PI_6: $f = 0.52359877559829887307710723054658381;
            const FRAC_PI_8: $f = 0.39269908169872415480783042290993786;
            const SQRT_PI: $f = 1.77245385090551602729816748334114518;
            const FRAC_1_PI: $f = 0.318309886183790671537767526745028724;
            const FRAC_1_SQRT_PI: $f = 0.564189583547756286948079451560772586;
            const FRAC_2_PI: $f = 0.636619772367581343075535053490057448;
            const FRAC_2_SQRT_PI: $f = 1.12837916709551257389615890312154517;

            /* Mathematical constants related to Tau (τ) */

            const TAU: $f = 6.28318530717958647692528676655900577;

            const FRAC_TAU_2: $f = Self::PI;
            const FRAC_TAU_3: $f = 2.09439510239319549230842892218633526;
            const FRAC_TAU_4: $f = Self::FRAC_PI_2;
            const FRAC_TAU_5: $f = 1.25663706143591729538505735331180115;
            const FRAC_TAU_6: $f = Self::FRAC_PI_3;
            const FRAC_TAU_8: $f = Self::FRAC_PI_4;
            const FRAC_TAU_9: $f = 0.69813170079773183076947630739544508;
            const FRAC_TAU_12: $f = Self::FRAC_PI_6;
            const FRAC_TAU_16: $f = Self::FRAC_PI_8;
            const FRAC_TAU_24: $f = 0.26179938779914943653855361527329191;
            const FRAC_TAU_72: $f = 0.08726646259971647884618453842443063;
            const FRAC_TAU_360: $f = 0.01745329251994329576923690768488613;
            const FRAC_360_TAU: $f = 57.29577951308232087679815481410517033;

            const SQRT_TAU: $f = 2.50662827463100050241576528481104525;

            const FRAC_1_TAU: $f = 0.159154943091895335768883763372514362;
            const FRAC_1_SQRT_TAU: $f = 0.398942280401432677939946059934381868;
            const FRAC_2_TAU: $f = Self::FRAC_1_PI;
            const FRAC_2_SQRT_TAU: $f = 0.797884560802865355879892119868763737;

            /* Mathematical constants related to Phi (φ) */

            const PHI: $f = 1.618033988749894848204586834365638118; // last 77 → 8
            const SQ_PHI: $f = 2.618033988749894848204586834365638118; // last 77 → 8
            const FRAC_1_PHI: $f = 0.618033988749894848204586834365638118; // last 77 → 8
            const NEG_FRAC_1_PHI: $f = -0.618033988749894848204586834365638118; // last 77 → 8
            const SQRT_PHI: $f = 1.272019649514068964252422461737491492; // last 17 → 2
            const FRAC_1_SQRT_PHI: $f = 0.786151377757423286069558585842958929; // last 95 = 9
            const TRIBONACCI: $f = 1.83928675521416113255185256465328660; // last 00 = 0

            /* Mathematical constants related to integer roots */

            const SQRT_2: $f = 1.41421356237309504880168872420969808; // last 78 = 8
            const FRAC_1_SQRT_2: $f = 0.707106781186547524400844362104849039;
            const SQRT_3: $f = 1.732050807568877293527446341505872367;
            const FRAC_1_SQRT_3: $f = 0.577350269189625764509148780501957456;
            const SQRT_5: $f = 2.236067977499789696409173668731276235;
            const SQRT_6: $f = 2.449489742783178098197284074705891392;
            const SQRT_7: $f = 2.645751311064590590501615753639260426;
            const SQRT_8: $f = 2.828427124746190097603377448419396157;
            const SQRT_10: $f = 3.162277660168379331998893544432718534;
            const SQRT_11: $f = 3.316624790355399849114932736670686684;
            const SQRT_12: $f = 3.464101615137754587054892683011744734;

            const CBRT_2: $f = 1.259921049894873164767210607278228350;
            const CBRT_3: $f = 1.442249570307408382321638310780109588;
            const FRAC_1_CBRT_3: $f = 0.693361274350634704843352274785961795;

            /* Other mathematical constants */

            const E: $f = 2.71828182845904523536028747135266250;
            const EGAMMA: $f = 0.577215664901532860606512090082402431;
            const LOG2_E: $f = 1.44269504088896340735992468100189214;
            const LOG2_10: $f = 3.32192809488736234787031942948939018;
            const LOG10_E: $f = 0.434294481903251827651128918916605082;
            const LOG10_2: $f = 0.301029995663981195213738894724493027;
            const LN_2: $f = 0.693147180559945309417232121458176568;
            const LN_10: $f = 2.30258509299404568401799145468436421;
        }
    };
}
math_const_impls![f32, f64];
