// devela::num::float::consts
//
// - https://en.wikipedia.org/wiki/List_of_mathematical_constants

use super::Floating;

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
        impl Floating<$f> {
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
        /// # *Mathematical constants*.
        #[allow(clippy::excessive_precision)] // 36 decimal points
        impl Floating<$f> {
            /* π related */

            /// $ π $ the ratio of the circumference to the diameter $ = \frac{1}{2} τ = 180º $
            /// ([A000796](https://oeis.org/A000796))
            pub const PI: $f = 3.14159265358979323846264338327950288;

            /// $ π/2 = τ/4 = 90º $
            /// ([A019669](https://oeis.org/A019669))
            pub const FRAC_PI_2: $f = 1.57079632679489661923132169163975144;

            /// $ π/3 = τ/6 = 60º $
            /// ([A019670](https://oeis.org/A019670))
            pub const FRAC_PI_3: $f = 1.04719755119659774615421446109316763;

            /// $ π/4 = τ/8 = 45º $
            /// ([A003881](https://oeis.org/A003881))
            pub const FRAC_PI_4: $f = 0.785398163397448309615660845819875721;

            /// $ π/6 = τ/12 = 30º $
            /// ([A019673](https://oeis.org/A019673))
            pub const FRAC_PI_6: $f = 0.52359877559829887307710723054658381;

            /// $ π/8 = τ/16 = 22.5º $
            /// ([A019675](https://oeis.org/A019675))
            pub const FRAC_PI_8: $f = 0.39269908169872415480783042290993786;

            /// $ \sqrt{π} = \sqrt{\frac{1}{2} τ} $
            /// ([A002161](https://oeis.org/A002161))
            pub const SQRT_PI: $f = 1.77245385090551602729816748334114518;

            /// $ 1/π = 2/τ $
            /// ([A049541](https://oeis.org/A049541))
            pub const FRAC_1_PI: $f = 0.318309886183790671537767526745028724;

            /// $ 1/\sqrt{π} $
            /// ([A087197](https://oeis.org/A087197))
            // WAIT: https://github.com/rust-lang/rust/issues/103883
            pub const FRAC_1_SQRT_PI: $f = 0.564189583547756286948079451560772586;

            /// $ 2/π $
            /// ([A060294](https://oeis.org/A060294))
            pub const FRAC_2_PI: $f = 0.636619772367581343075535053490057448;

            /// $ 2/\sqrt{π} $
            /// ([A190732](https://oeis.org/A190732))
            pub const FRAC_2_SQRT_PI: $f = 1.12837916709551257389615890312154517;

            /* τ related */

            /// $ τ $ the ratio of the circumference to the radius $ = 2π = 360º $
            /// ([A019692](https://oeis.org/A019692))
            pub const TAU: $f = 6.28318530717958647692528676655900577;

            /// $ τ/2 = π = 180º $
            /// ([A000796](https://oeis.org/A000796))
            pub const FRAC_TAU_2: $f = Self::PI;

            /// $ τ/3  = 2π/3 = 120º $
            /// ([A019693](https://oeis.org/A019693))
            pub const FRAC_TAU_3: $f = 2.09439510239319549230842892218633526;

            /// $ τ/4 = π/2 = 90º $
            /// ([A019693](https://oeis.org/A019693))
            pub const FRAC_TAU_4: $f = Self::FRAC_PI_2;

            /// $ τ/5 = 2π/5 = 72º $
            /// ([A019694](https://oeis.org/A019694))
            pub const FRAC_TAU_5: $f = 1.25663706143591729538505735331180115;

            /// $ τ/6 = π/3 = 60º $
            /// ([A019670](https://oeis.org/A019670))
            pub const FRAC_TAU_6: $f = Self::FRAC_PI_3;

            /// $ τ/8 = π/4 = 45º $
            /// ([A003881](https://oeis.org/A003881))
            pub const FRAC_TAU_8: $f = Self::FRAC_PI_4;

            /// $ τ/9 = 2π/9 = 40º $
            /// ([A019696](https://oeis.org/A019696))
            pub const FRAC_TAU_9: $f = 0.69813170079773183076947630739544508;

            /// $ τ/12 = π/6 = 30º $
            /// ([A019673](https://oeis.org/A019673))
            pub const FRAC_TAU_12: $f = Self::FRAC_PI_6;

            /// $ τ/16 = π/8 = 22.5º $
            /// ([A019675](https://oeis.org/A019675))
            pub const FRAC_TAU_16: $f = Self::FRAC_PI_8;

            /// $ τ/24 = π/12 = 15º $
            /// ([A019679](https://oeis.org/A019679))
            pub const FRAC_TAU_24: $f = 0.26179938779914943653855361527329191;

            /// $ τ/72 = π/36 = 5º $
            pub const FRAC_TAU_72: $f = 0.08726646259971647884618453842443063;

            /// $ τ/360 = π/180 = 1º $ *arcdegree*
            /// ([A019685](https://oeis.org/A019685),
            /// [wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
            pub const FRAC_TAU_360: $f = 0.01745329251994329576923690768488613;

            /// $ 360/τ = 180/π $
            /// ([A072097](https://oeis.org/A072097))
            pub const FRAC_360_TAU: $f = 57.29577951308232087679815481410517033;

            /// $ τ/(360*60) = 1' $ *arcminute*
            /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
            pub const FRAC_TAU_ARCMINUTES: $f = 0.00029088820866572159615394846141477;

            /// $ τ/(360 * 60 * 60) = 1'' $ *arcsecond*
            /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
            pub const FRAC_TAU_ARCSECONDS: $f = 0.00000484813681109535993589914102358;

            /// $ \sqrt{τ} = \sqrt{2π} $
            /// ([A019727](https://oeis.org/A019727))
            pub const SQRT_TAU: $f = 2.50662827463100050241576528481104525;

            /// $ 1/τ = 1/2π $
            /// ([A086201](https://oeis.org/A086201))
            pub const FRAC_1_TAU: $f = 0.159154943091895335768883763372514362;

            /// $ 1/\sqrt{τ} = 1/\sqrt{2π} $
            /// ([A231863](https://oeis.org/A231863))
            pub const FRAC_1_SQRT_TAU: $f = 0.398942280401432677939946059934381868;

            /// $ 2/τ = 1/π $
            /// ([A049541](https://oeis.org/A049541))
            pub const FRAC_2_TAU: $f = Self::FRAC_1_PI;

            /// $ 2/\sqrt{τ} = \sqrt{2/π} $
            /// ([A231863](https://oeis.org/A231863))
            pub const FRAC_2_SQRT_TAU: $f = 0.797884560802865355879892119868763737;

            /* φ related */

            /// $ φ $ the golden ratio $\large \frac{1+\sqrt{5}}{2}$
            /// ([A001622](https://oeis.org/A001622))
            // WAIT: https://github.com/rust-lang/rust/issues/103883
            pub const PHI: $f = 1.618033988749894848204586834365638118;

            /// $ \sqrt{φ} $
            /// ([A139339](https://oeis.org/A139339))
            pub const SQRT_PHI: $f = 1.27201964951406896425242246173749149;

            /// The tribonacci constant.
            /// ([A058265](https://oeis.org/A058265))
            pub const TRIBONACCI: $f = 1.83928675521416113255185256465328660;

            /* integer sqrts */

            /// $ \sqrt{2} $
            /// ([A002193](https://oeis.org/A002193),
            /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2))
            pub const SQRT_2: $f = 1.41421356237309504880168872420969808;

            /// $ 1/\sqrt{2} = \sqrt{1/2} $
            /// ([A010503](https://oeis.org/A010503),
            /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2#Multiplicative_inverse))
            pub const FRAC_1_SQRT_2: $f = 0.707106781186547524400844362104849039;

            /// $ \sqrt{3} $
            /// ([A002194](https://oeis.org/A002194),
            /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_3))
            // WAIT: https://github.com/rust-lang/rust/issues/103883
            pub const SQRT_3: $f = 1.732050807568877293527446341505872367;

            /// $ 1/\sqrt{3} = \sqrt{1/3} $
            // WAIT: https://github.com/rust-lang/rust/issues/103883
            pub const FRAC_1_SQRT_3: $f = 0.577350269189625764509148780501957456;

            /// $ \sqrt{5} $
            /// ([A002163](https://oeis.org/A002163),
            /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_5))
            pub const SQRT_5: $f = 2.236067977499789696409173668731276235;

            /// $ \sqrt{6} $
            /// ([A010464](https://oeis.org/A010464))
            pub const SQRT_6: $f = 2.449489742783178098197284074705891392;

            /// $ \sqrt{7} $
            /// ([A010465](https://oeis.org/A010465))
            pub const SQRT_7: $f = 2.645751311064590590501615753639260426;

            /// $ \sqrt{8} $
            /// ([A010466](https://oeis.org/A010466))
            pub const SQRT_8: $f = 2.828427124746190097603377448419396157;

            /// $ \sqrt{10} $
            /// ([A010467](https://oeis.org/A010467))
            pub const SQRT_10: $f = 3.162277660168379331998893544432718534;

            /// $ \sqrt{11} $
            /// ([A010468](https://oeis.org/A010468))
            pub const SQRT_11: $f = 3.316624790355399849114932736670686684;

            /// $ \sqrt{12} $
            /// ([A010469](https://oeis.org/A010469))
            pub const SQRT_12: $f = 3.464101615137754587054892683011744734;

            /* integer cbrts */

            /// $ \sqrt[\small 3]{2} $
            /// ([A002580](https://oeis.org/A002580),
            /// [wikipedia](https://en.wikipedia.org/wiki/Doubling_the_cube))
            pub const CBRT_2: $f = 1.259921049894873164767210607278228350;

            /// $ \sqrt[\small 3]{3} $
            /// ([A002581](https://oeis.org/A002581))
            pub const CBRT_3: $f = 1.442249570307408382321638310780109588;

            /// $ 1/\sqrt[\small 3]{3} = (\normalsize\frac{1}{3})^{\small\frac{1}{3}} $
            /// ([A072365](https://oeis.org/A072365))
            pub const FRAC_1_CBRT_3: $f = 0.693361274350634704843352274785961795;

            /* other */

            /// $ e $ Euler's number
            pub const E: $f = 2.71828182845904523536028747135266250;

            /// $ γ $ The Euler-Mascheroni constant
            /// ([A001620](https://oeis.org/A001620))
            // WAIT: https://github.com/rust-lang/rust/issues/103883
            pub const EGAMMA: $f = 0.577215664901532860606512090082402431;

            /// log<sub>2</sub>(e)
            pub const LOG2_E: $f = 1.44269504088896340735992468100189214;

            /// log<sub>2</sub>(10)
            pub const LOG2_10: $f = 3.32192809488736234787031942948939018;

            /// log<sub>10</sub>(e)
            pub const LOG10_E: $f = 0.434294481903251827651128918916605082;

            /// log<sub>10</sub>(2)
            pub const LOG10_2: $f = 0.301029995663981195213738894724493027;

            /// ln(2)
            pub const LN_2: $f = 0.693147180559945309417232121458176568;

            /// ln(10)
            pub const LN_10: $f = 2.30258509299404568401799145468436421;
        }
    }};
}
math_const_impls![f32, f64];
