// devela::num::float::wrapper::impl_consts
//
//! Constants
//!
//! - <https://en.wikipedia.org/wiki/List_of_mathematical_constants>
//

// constants are defined with 37 digits, usually 1 integer and 36 decimals.
#![allow(clippy::excessive_precision)]

use super::Float;

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
     ) => { $crate::code::paste! {
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
    }};
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
    (@$f:ty) => { $crate::code::paste! {
        /// # *Basic mathematical constants*.
        impl Float<$f> {
            /// The multiplicative identity 1.
            pub const ONE: Float<$f> = Float(1.0);
            /// The additive identity 0.
            pub const ZERO: Float<$f> = Float(0.0);
            /// The negative of the multiplicative identity -1.
            pub const NEG_ONE: Float<$f> = Float(-1.0);
            /// The negative of the additive identity -0.
            pub const NEG_ZERO: Float<$f> = Float(-0.0);

            // ...

            /// Not a Number (NaN).
            pub const NAN: Float<$f> = Float(<$f>::NAN);
            /// Infinity (∞).
            pub const INFINITY: Float<$f> = Float(<$f>::INFINITY);
            /// Negative infinity (∞).
            pub const NEG_INFINITY: Float<$f> = Float(<$f>::NEG_INFINITY);

            /// Machine epsilon value.
            pub const EPSILON: Float<$f> = Float(<$f>::EPSILON);

            /// The radix or base of the internal representation.
            pub const RADIX: u32 = <$f>::RADIX;
            /// Approximate number of significant digits in base 10.
            pub const DIGITS: u32 = <$f>::DIGITS;
            /// Number of significant digits in base 2.
            pub const MANTISSA_DIGITS: u32 = <$f>::MANTISSA_DIGITS;

            /// Smallest finite value.
            pub const MIN: Float<$f> = Float(<$f>::MIN);
            /// Smallest positive normal value.
            pub const MIN_POSITIVE: Float<$f> = Float(<$f>::MIN_POSITIVE);
            /// Largest finite value.
            pub const MAX: Float<$f> = Float(<$f>::MAX);

            /// One greater than the minimum possible normal power of 2 exponent.
            pub const MIN_EXP: i32 = <$f>::MIN_EXP;
            /// Maximum possible power of 2 exponent.
            pub const MAX_EXP: i32 = <$f>::MAX_EXP;

            /// Minimum *x* for which 10<sup>*x*</sup> is normal.
            pub const MIN_10_EXP: i32 = <$f>::MIN_10_EXP;
            /// Maximum *x* for which 10<sup>*x*</sup> is normal.
            pub const MAX_10_EXP: i32 = <$f>::MAX_10_EXP;
        }

        /// # *Mathematical constants related to Pi (π)*.
        impl Float<$f> {
            /// $ π = \frac{1}{2} τ = 180º $
            /// ([A000796](https://oeis.org/A000796/constant))
            /// *the ratio of the circumference to the diameter, a half-turn*
            pub const PI: Float<$f> =
                Float(3.14159265358979323846264338327950288);

            /// $ π/2 = τ/4 = 90º $
            /// ([A019669](https://oeis.org/A019669/constant))
            pub const FRAC_PI_2: Float<$f> =
                Float(1.57079632679489661923132169163975144);

            /// $ π/3 = τ/6 = 60º $
            /// ([A019670](https://oeis.org/A019670/constant))
            pub const FRAC_PI_3: Float<$f> =
                Float(1.04719755119659774615421446109316763);

            /// $ π/4 = τ/8 = 45º $
            /// ([A003881](https://oeis.org/A003881/constant))
            pub const FRAC_PI_4: Float<$f> =
                Float(0.785398163397448309615660845819875721);

            /// $ π/6 = τ/12 = 30º $
            /// ([A019673](https://oeis.org/A019673/constant))
            pub const FRAC_PI_6: Float<$f> =
                Float(0.52359877559829887307710723054658381);

            /// $ π/8 = τ/16 = 22.5º $
            /// ([A019675](https://oeis.org/A019675/constant))
            pub const FRAC_PI_8: Float<$f> =
                Float(0.39269908169872415480783042290993786);

            /// $ \sqrt{π} = \sqrt{\frac{1}{2} τ} $
            /// ([A002161](https://oeis.org/A002161/constant))
            pub const SQRT_PI: Float<$f> =
                Float(1.77245385090551602729816748334114518);

            /// $ 1/π = 2/τ $
            /// ([A049541](https://oeis.org/A049541/constant))
            pub const FRAC_1_PI: Float<$f> =
                Float(0.318309886183790671537767526745028724);

            /// $ 1/\sqrt{π} = 1/\sqrt{τ/2} $
            /// ([A087197](https://oeis.org/A087197/constant))
            // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
            pub const FRAC_1_SQRT_PI: Float<$f> =
                Float(0.564189583547756286948079451560772586);

            /// $ 2/π $
            /// ([A060294](https://oeis.org/A060294/constant))
            /// *Buffon's constant*
            pub const FRAC_2_PI: Float<$f> =
                Float(0.636619772367581343075535053490057448);

            /// $ 2/\sqrt{π} $
            /// ([A190732](https://oeis.org/A190732/constant))
            pub const FRAC_2_SQRT_PI: Float<$f> =
                Float(1.12837916709551257389615890312154517);
        }

        /// # *Mathematical constants related to Tau (τ)*.
        impl Float<$f> {
            /// $ τ = 2π = 360º $
            /// ([A019692](https://oeis.org/A019692/constant))
            /// *the ratio of the circumference to the radius, a full-turn*
            pub const TAU: Float<$f> =
                Float(6.28318530717958647692528676655900577);

            /// $ τ/2 = π = 180º $
            /// ([A000796](https://oeis.org/A000796/constant))
            pub const FRAC_TAU_2: Float<$f> =
                Self::PI;

            /// $ τ/3  = 2π/3 = 120º $
            /// ([A019693](https://oeis.org/A019693/constant))
            pub const FRAC_TAU_3: Float<$f> =
                Float(2.09439510239319549230842892218633526);

            /// $ τ/4 = π/2 = 90º $
            /// ([A019693](https://oeis.org/A019693/constant))
            pub const FRAC_TAU_4: Float<$f> =
                Self::FRAC_PI_2;

            /// $ τ/5 = 2π/5 = 72º $
            /// ([A019694](https://oeis.org/A019694/constant))
            pub const FRAC_TAU_5: Float<$f> =
                Float(1.25663706143591729538505735331180115);

            /// $ τ/6 = π/3 = 60º $
            /// ([A019670](https://oeis.org/A019670/constant))
            pub const FRAC_TAU_6: Float<$f> =
                Self::FRAC_PI_3;

            /// $ τ/8 = π/4 = 45º $
            /// ([A003881](https://oeis.org/A003881/constant))
            pub const FRAC_TAU_8: Float<$f> =
                Self::FRAC_PI_4;

            /// $ τ/9 = 2π/9 = 40º $
            /// ([A019696](https://oeis.org/A019696/constant))
            pub const FRAC_TAU_9: Float<$f> =
                Float(0.69813170079773183076947630739544508);

            /// $ τ/12 = π/6 = 30º $
            /// ([A019673](https://oeis.org/A019673/constant))
            pub const FRAC_TAU_12: Float<$f> =
                Self::FRAC_PI_6;

            /// $ τ/16 = π/8 = 22.5º $
            /// ([A019675](https://oeis.org/A019675/constant))
            pub const FRAC_TAU_16: Float<$f> =
                Self::FRAC_PI_8;

            /// $ τ/24 = π/12 = 15º $
            /// ([A019679](https://oeis.org/A019679/constant))
            pub const FRAC_TAU_24: Float<$f> =
                Float(0.26179938779914943653855361527329191);

            /// $ τ/72 = π/36 = 5º $
            pub const FRAC_TAU_72: Float<$f> =
                Float(0.08726646259971647884618453842443063);

            /// $ τ/360 = π/180 = 1º $ *arcdegree*
            /// ([A019685](https://oeis.org/A019685),
            /// [wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
            pub const FRAC_TAU_360: Float<$f> =
                Float(0.01745329251994329576923690768488613);

            /// $ 360/τ = 180/π $
            /// ([A072097](https://oeis.org/A072097/constant))
            pub const FRAC_360_TAU: Float<$f> =
                Float(57.29577951308232087679815481410517033);

            /// $ τ/(360*60) = 1' $ *arcminute*
            /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
            pub const FRAC_TAU_ARCMINUTES: Float<$f> =
                Float(0.00029088820866572159615394846141477);

            /// $ τ/(360 * 60 * 60) = 1'' $ *arcsecond*
            /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
            pub const FRAC_TAU_ARCSECONDS: Float<$f> =
                Float(0.00000484813681109535993589914102358);

            /// $ \sqrt{τ} = \sqrt{2π} $
            /// ([A019727](https://oeis.org/A019727/constant))
            pub const SQRT_TAU: Float<$f> =
                Float(2.50662827463100050241576528481104525);

            /// $ 1/τ = 1/2π $
            /// ([A086201](https://oeis.org/A086201/constant))
            pub const FRAC_1_TAU: Float<$f> =
                Float(0.159154943091895335768883763372514362);

            /// $ 1/\sqrt{τ} = 1/\sqrt{2π} $
            /// ([A231863](https://oeis.org/A231863/constant))
            pub const FRAC_1_SQRT_TAU: Float<$f> =
                Float(0.398942280401432677939946059934381868);

            /// $ 2/τ = 1/π $
            /// ([A049541](https://oeis.org/A049541/constant))
            pub const FRAC_2_TAU: Float<$f> =
                Self::FRAC_1_PI;

            /// $ 2/\sqrt{τ} = \sqrt{2/π} $
            /// ([A231863](https://oeis.org/A231863/constant))
            pub const FRAC_2_SQRT_TAU: Float<$f> =
                Float(0.797884560802865355879892119868763737);
        }

        /// # *Mathematical constants related to Phi (φ)*.
        impl Float<$f> {
            /// $ φ  = (1+\sqrt{5})/2 $
            /// ([A001622](https://oeis.org/A001622/constant))
            /// *the golden ratio*
            ///
            /// Continued fraction: $ [1;1,1,1,…] $
            // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
            pub const PHI: Float<$f> =
                Float(1.618033988749894848204586834365638118); // last 77 → 8

            /// $ φ^2 = φ+1 = (3+\sqrt{5})/2 $
            /// ([A104457](https://oeis.org/A104457/constant))
            pub const SQ_PHI: Float<$f> =
                Float(2.618033988749894848204586834365638118); // last 77 → 8

            /// $ 1/φ = φ-1 $
            /// ([A094214](https://oeis.org/A094214/constant))
            /// *the reciprocal of [φ][Self#PHI]*
            pub const FRAC_1_PHI: Float<$f> =
                Float(0.618033988749894848204586834365638118); // last 77 → 8

            /// $ -1/φ = 1-φ $
            /// *the negative reciprocal of [φ][Self#PHI] and its conjugate in $ x^2-x-1 $*
            pub const NEG_FRAC_1_PHI: Float<$f> =
                Float(-0.618033988749894848204586834365638118); // last 77 → 8

            /// $ \sqrt{φ} $
            /// ([A139339](https://oeis.org/A139339/constant))
            pub const SQRT_PHI: Float<$f> =
                Float(1.272019649514068964252422461737491492); // last 17 → 2

            /// $ 1/\sqrt{φ} = \sqrt{φ/φ^2} = \sqrt{φ^2-2} $
            /// ([A197762](https://oeis.org/A197762/constant))
            pub const FRAC_SQRT_PHI: Float<$f> =
                Float(0.786151377757423286069558585842958929); // last 95 = 9

            /// ([A058265](https://oeis.org/A058265/constant))
            /// *The tribonacci constant*
            pub const TRIBONACCI: Float<$f> =
                Float(1.83928675521416113255185256465328660); // last 00 = 0
        }

        /// # *Mathematical constants related to integer roots*.
        impl Float<$f> {
            /* sqrt */

            /// $ \sqrt{2} $
            /// ([A002193](https://oeis.org/A002193/constant),
            /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2))
            pub const SQRT_2: Float<$f> =
                Float(1.41421356237309504880168872420969808); // last 78 = 8

            /// $ 1/\sqrt{2} = \sqrt{1/2} $
            /// ([A010503](https://oeis.org/A010503/constant),
            /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2#Multiplicative_inverse))
            pub const FRAC_1_SQRT_2: Float<$f> =
                Float(0.707106781186547524400844362104849039);

            /// $ \sqrt{3} $
            /// ([A002194](https://oeis.org/A002194/constant),
            /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_3))
            // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
            pub const SQRT_3: Float<$f> =
                Float(1.732050807568877293527446341505872367);

            /// $ 1/\sqrt{3} = \sqrt{1/3} $
            // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
            pub const FRAC_1_SQRT_3: Float<$f> =
                Float(0.577350269189625764509148780501957456);

            /// $ \sqrt{5} $
            /// ([A002163](https://oeis.org/A002163/constant),
            /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_5))
            pub const SQRT_5: Float<$f> =
                Float(2.236067977499789696409173668731276235);

            /// $ \sqrt{6} $
            /// ([A010464](https://oeis.org/A010464/constant))
            pub const SQRT_6: Float<$f> =
                Float(2.449489742783178098197284074705891392);

            /// $ \sqrt{7} $
            /// ([A010465](https://oeis.org/A010465/constant))
            pub const SQRT_7: Float<$f> =
                Float(2.645751311064590590501615753639260426);

            /// $ \sqrt{8} $
            /// ([A010466](https://oeis.org/A010466/constant))
            pub const SQRT_8: Float<$f> =
                Float(2.828427124746190097603377448419396157);

            /// $ \sqrt{10} $
            /// ([A010467](https://oeis.org/A010467/constant))
            pub const SQRT_10: Float<$f> =
                Float(3.162277660168379331998893544432718534);

            /// $ \sqrt{11} $
            /// ([A010468](https://oeis.org/A010468/constant))
            pub const SQRT_11: Float<$f> =
                Float(3.316624790355399849114932736670686684);

            /// $ \sqrt{12} $
            /// ([A010469](https://oeis.org/A010469/constant))
            pub const SQRT_12: Float<$f> =
                Float(3.464101615137754587054892683011744734);

            /* cbrt */

            /// $ \sqrt[\small 3]{2} $
            /// ([A002580](https://oeis.org/A002580/constant),
            /// [wikipedia](https://en.wikipedia.org/wiki/Doubling_the_cube))
            pub const CBRT_2: Float<$f> =
                Float(1.259921049894873164767210607278228350);

            /// $ \sqrt[\small 3]{3} $
            /// ([A002581](https://oeis.org/A002581/constant))
            pub const CBRT_3: Float<$f> =
                Float(1.442249570307408382321638310780109588);

            /// $ 1/\sqrt[\small 3]{3} = (\normalsize\frac{1}{3})^{\small\frac{1}{3}} $
            /// ([A072365](https://oeis.org/A072365/constant))
            pub const FRAC_1_CBRT_3: Float<$f> =
                Float(0.693361274350634704843352274785961795);
        }

        /// # *Other mathematical constants*.
        impl Float<$f> {
            /// $ e $
            /// ([A001113](https://oeis.org/A001113/constant))
            /// *the Euler number or Napier's constant*
            ///
            /// Continuous fraction: $ [2;1,2,1,1,4,1,1,6,1,…,1,2n,1,…] $
            pub const E: Float<$f> =
                Float(2.71828182845904523536028747135266250);

            /// $ γ $
            /// ([A001620](https://oeis.org/A001620/constant))
            /// *gamma, or the Euler-Mascheroni constant*
            // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
            pub const EGAMMA: Float<$f> =
                Float(0.577215664901532860606512090082402431);

            /// $ \log_2{e} $
            /// ([A007525](https://oeis.org/A007525/constant))
            pub const LOG2_E: Float<$f> =
                Float(1.44269504088896340735992468100189214);

            /// log<sub>2</sub>(10)
            /// ([A020862](https://oeis.org/A020862/constant))
            pub const LOG2_10: Float<$f> =
                Float(3.32192809488736234787031942948939018);

            /// log<sub>10</sub>(e)
            /// ([A002285](https://oeis.org/A002285/constant))
            pub const LOG10_E: Float<$f> =
                Float(0.434294481903251827651128918916605082);

            /// log<sub>10</sub>(2)
            /// ([A007524](https://oeis.org/A007524/constant))
            pub const LOG10_2: Float<$f> =
                Float(0.301029995663981195213738894724493027);

            /// ln(2)
            /// ([A002162](https://oeis.org/A002162/constant))
            pub const LN_2: Float<$f> =
                Float(0.693147180559945309417232121458176568);

            /// ln(10)
            /// ([A002392](https://oeis.org/A002392/constant))
            pub const LN_10: Float<$f> =
                Float(2.30258509299404568401799145468436421);
        }
    }};
}
math_const_impls![f32, f64];
