// devela_base_core::num::float::_docs
//
//! Defines constants for shared documentation on [`Float`] and [`FloatExt`].
#![doc = crate::doclink!(custom devela "[`FloatExt`]" "num/trait.FloatExt.html")]
//
// TOC
// - constants: _FLOAT_CONST_*
// - formulas: _FLOAT_FORMULA_*

// documentation constants
crate::CONST! { hidden macro_export,
// identities
_FLOAT_CONST_ONE = r#"The multiplicative identity 1."#;
_FLOAT_CONST_ZERO = r#"The additive identity 0."#;
_FLOAT_CONST_NEG_ONE = r#"The negative of the multiplicative identity -1."#;
_FLOAT_CONST_NEG_ZERO = r#"The negative of the additive identity -0."#;

// representation, precision and range
_FLOAT_CONST_NAN = r#"Not a Number (NaN)."#;
_FLOAT_CONST_INFINITY = r#"Infinity (∞)."#;
_FLOAT_CONST_NEG_INFINITY = r#"Negative infinity (-∞)."#;
_FLOAT_CONST_MIN = r#"Smallest finite value."#;
_FLOAT_CONST_MIN_POSITIVE = r#"Smallest positive normal value."#;
_FLOAT_CONST_MAX = r#"Largest finite value."#;
_FLOAT_CONST_MIN_EXP = r#"One greater than the minimum possible normal power of 2 exponent."#;
_FLOAT_CONST_MAX_EXP = r#"Maximum possible power of 2 exponent."#;
_FLOAT_CONST_MIN_10_EXP = r#"Minimum *x* for which 10<sup>*x*</sup> is normal."#;
_FLOAT_CONST_MAX_10_EXP = r#"Maximum *x* for which 10<sup>*x*</sup> is normal."#;
_FLOAT_CONST_EPSILON = r#"Machine epsilon value.
<p>This is the smallest difference detectable between 1.0 and the next
representable number in the floating-point format.</p>"#;
_FLOAT_CONST_LOW_MARGIN = r#"Allows for minimal deviation; use for high precision needs.."#;
_FLOAT_CONST_MEDIUM_MARGIN =
    r#"Accommodates moderate deviation; balances precision and flexibility."#;
_FLOAT_CONST_HIGH_MARGIN =
    r#"Permits generous deviation; suitable for less precise scenarios."#;
_FLOAT_CONST_RADIX = r#"The radix or base of the internal representation."#;
_FLOAT_CONST_DIGITS = r#"Approximate number of significant digits in base 10."#;
_FLOAT_CONST_MANTISSA_DIGITS = r#"Number of significant digits in base 2."#;
_FLOAT_CONST_SIGNIFICAND_BITS =
    "Number of explicit bits used to represent the significand (or mantissa).";
_FLOAT_CONST_EXPONENT_BIAS =
    r#"Exponent bias for representing both positive and negative exponents."#;
_FLOAT_CONST_EXPONENT_BITS = r#"Number of bits used to represent the exponent."#;

// pi
_FLOAT_CONST_PI = r#"$ π = \frac{1}{2} τ = 180\degree $
([A000796](https://oeis.org/A000796/constant))
`≈ 3.14159265…`
<p>*The ratio of the circumference to the diameter, a half-turn*.</p>"#;
_FLOAT_CONST_FRAC_PI_2 = r#"$ π/2 = τ/4 = 90\degree $
([A019669](https://oeis.org/A019669/constant))
`≈ 1.57079632…`"#;
_FLOAT_CONST_FRAC_PI_3 = r#"$ π/3 = τ/6 = 60\degree $
([A019670](https://oeis.org/A019670/constant))
`≈ 1.04719755…`"#;
_FLOAT_CONST_FRAC_PI_4 = r#"$ π/4 = τ/8 = 45\degree $
([A003881](https://oeis.org/A003881/constant))
`≈ 0.78539816…`"#;
_FLOAT_CONST_FRAC_PI_6 = r#"$ π/6 = τ/12 = 30\degree $
([A019673](https://oeis.org/A019673/constant))
`≈ 0.52359877…`"#;
_FLOAT_CONST_FRAC_PI_8 = r#"$ π/8 = τ/16 = 22.5\degree $
([A019675](https://oeis.org/A019675/constant))
`≈ 0.39269908…`"#;
_FLOAT_CONST_SQRT_PI = r#"$ \sqrt{π} = \sqrt{\frac{1}{2} τ} $
([A002161](https://oeis.org/A002161/constant))
`≈ 1.77245385…`"#;
_FLOAT_CONST_FRAC_1_PI = r#"$ 1/π = 2/τ $
([A049541](https://oeis.org/A049541/constant))
`≈ 0.31830988…`"#;
_FLOAT_CONST_FRAC_1_SQRT_PI = r#"$ 1/\sqrt{π} = 1/\sqrt{τ/2} $
([A087197](https://oeis.org/A087197/constant))
`≈ 0.56418958…`"#;
_FLOAT_CONST_FRAC_1_SQRT_2PI = r#"$ 1/\sqrt{2π} = 1/\sqrt{τ} $
([A231863](https://oeis.org/A231863/constant))
`≈ 0.39894228…`"#;
_FLOAT_CONST_FRAC_2_PI = r#"$ 2/π $
([A060294](https://oeis.org/A060294/constant))
`≈ 0.63661977…`
<p>*Buffon's constant*.</p>"#;
_FLOAT_CONST_FRAC_2_SQRT_PI = r#"$ 2/\sqrt{π} $
([A190732](https://oeis.org/A190732/constant))
`≈ 1.12837916…`"#;

// tau
_FLOAT_CONST_TAU = r#"$ τ = 2π = 360\degree $
([A019692](https://oeis.org/A019692/constant))
`≈ 6.28318530…`
<p>*The ratio of the circumference to the radius, a full-turn*.</p>"#;
_FLOAT_CONST_FRAC_TAU_2 = r#"$ τ/2 = π = 180\degree $
([A000796](https://oeis.org/A000796/constant))
`≈ 3.14159265…`"#;
_FLOAT_CONST_FRAC_TAU_3 = r#"$ τ/3  = 2π/3 = 120\degree $
([A019693](https://oeis.org/A019693/constant))
`≈ 2.09439510…`"#;
_FLOAT_CONST_FRAC_TAU_4 = r#"$ τ/4 = π/2 = 90\degree $
([A019693](https://oeis.org/A019693/constant))
`≈ 1.57079632…`"#;
_FLOAT_CONST_FRAC_TAU_5 = r#"$ τ/5 = 2π/5 = 72\degree $
([A019694](https://oeis.org/A019694/constant))
`≈ 1.25663706…`"#;
_FLOAT_CONST_FRAC_TAU_6 = r#"$ τ/6 = π/3 = 60\degree $
([A019670](https://oeis.org/A019670/constant))
`≈ 1.04719755…`"#;
_FLOAT_CONST_FRAC_TAU_8 = r#"$ τ/8 = π/4 = 45\degree $
([A003881](https://oeis.org/A003881/constant))
`≈ 0.78539816…`"#;
_FLOAT_CONST_FRAC_TAU_9 = r#"$ τ/9 = 2π/9 = 40\degree $
([A019696](https://oeis.org/A019696/constant))
`≈ 0.69813170…`"#;
_FLOAT_CONST_FRAC_TAU_12 = r#"$ τ/12 = π/6 = 30\degree $
([A019673](https://oeis.org/A019673/constant))
`≈ 0.52359877…`"#;
_FLOAT_CONST_FRAC_TAU_16 = r#"$ τ/16 = π/8 = 22.5\degree $
([A019675](https://oeis.org/A019675/constant))
`≈ 0.39269908…`"#;
_FLOAT_CONST_FRAC_TAU_24 = r#"$ τ/24 = π/12 = 15\degree $
([A019679](https://oeis.org/A019679/constant))
`≈ 0.26179938…`"#;
_FLOAT_CONST_FRAC_TAU_72 = r#"$ τ/72 = π/36 = 5\degree $
`≈ 0.08726646…`"#;
_FLOAT_CONST_FRAC_TAU_360 = r#"$ τ/360 = π/180 = 1\degree $ *arc degree*
([A019685](https://oeis.org/A019685),
[wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
`≈ 0.01745329…`"#;
_FLOAT_CONST_FRAC_360_TAU = r#"$ 360/τ = 180/π $
([A072097](https://oeis.org/A072097/constant))
`≈ 57.2957795…`"#;
_FLOAT_CONST_SQRT_TAU = r#"$ \sqrt{τ} = \sqrt{2π} $
([A019727](https://oeis.org/A019727/constant))
`≈ 2.50662827…`"#;
_FLOAT_CONST_FRAC_1_TAU = r#"$ 1/τ = 1/2π $
([A086201](https://oeis.org/A086201/constant))
`≈ 0.15915494…`"#;
_FLOAT_CONST_FRAC_1_SQRT_TAU = r#"$ 1/\sqrt{τ} = 1/\sqrt{2π} $
([A231863](https://oeis.org/A231863/constant))
`≈ 0.39894228…`"#;
_FLOAT_CONST_FRAC_2_TAU = r#"$ 2/τ = 1/π $
([A049541](https://oeis.org/A049541/constant))
`≈ 0.31830988…`"#;
_FLOAT_CONST_FRAC_2_SQRT_TAU = r#"$ 2/\sqrt{τ} = \sqrt{2/π} $
([A076668](https://oeis.org/A076668/constant))
`≈ 0.79788456…`"#;

// arc degrees
_FLOAT_CONST_ARC_DEGREE = r#"$ τ/360 = π/180 = 1\degree $ *arc degree*
([A019685](https://oeis.org/A019685),
[wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
`≈ 0.01745329…`"#;
_FLOAT_CONST_ARC_MINUTE = r#"$ τ/(360*60) = 1^{\prime} $ *arc minute*
([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
`≈ 0.00029088…`"#;
_FLOAT_CONST_ARC_SECOND = r#"$ τ/(360 * 60 * 60) = 1^{\prime\prime} $ *arc second*
([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
`≈ 0.00000484…`"#;

// phi
_FLOAT_CONST_PHI = r#"$ φ  = (1+\sqrt{5})/2 $
([A001622](https://oeis.org/A001622/constant))
`≈ 1.61803398…`
<p>*The golden ratio*.</p>
<p>Continued fraction: $ [1;1,1,1,…] $</p>"#;
_FLOAT_CONST_SQ_PHI = r#"$ φ^2 = φ+1 = (3+\sqrt{5})/2 $
([A104457](https://oeis.org/A104457/constant))
`≈ 2.61803398…`"#;
_FLOAT_CONST_FRAC_1_PHI = r#"$ 1/φ = φ-1 $
([A094214](https://oeis.org/A094214/constant))
`≈ 0.61803398…`
<p>*The reciprocal of [φ][Self#PHI]*.</p>"#;
_FLOAT_CONST_NEG_FRAC_1_PHI = r#"$ -1/φ = 1-φ $
`≈ -0.61803398…`
<p>*The negative reciprocal of [φ][Self#PHI] and its conjugate in $ x^2-x-1 $*.</p>"#;
_FLOAT_CONST_SQRT_PHI = r#"$ \sqrt{φ} $
([A139339](https://oeis.org/A139339/constant))
`≈ 1.27201964…`"#;
_FLOAT_CONST_FRAC_1_SQRT_PHI = r#"$ 1/\sqrt{φ} = \sqrt{φ/φ^2} = \sqrt{φ^2-2} $
([A197762](https://oeis.org/A197762/constant))
`≈ 0.78615137…`"#;
_FLOAT_CONST_TRIBONACCI = r#"([A058265](https://oeis.org/A058265/constant))
`≈ 1.83928675…`
<p>*The tribonacci constant*.</p>"#;

// integer roots
_FLOAT_CONST_SQRT_2 = r#"$ \sqrt{2} $
([A002193](https://oeis.org/A002193/constant),
[wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2))
`≈ 1.41421356…`"#;
_FLOAT_CONST_FRAC_1_SQRT_2 = r#"$ 1/\sqrt{2} = \sqrt{1/2} $
([A010503](https://oeis.org/A010503/constant),
[wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2#Multiplicative_inverse))
`≈ 0.70710678…`"#;
_FLOAT_CONST_SQRT_3 = r#"$ \sqrt{3} $
([A002194](https://oeis.org/A002194/constant),
[wikipedia](https://en.wikipedia.org/wiki/Square_root_of_3))
`≈ 1.73205080…`"#;
_FLOAT_CONST_FRAC_1_SQRT_3 = r#"$ 1/\sqrt{3} = \sqrt{1/3} $
([A020760](https://oeis.org/A002194/constant),
`≈ 0.57735026…`"#;
_FLOAT_CONST_SQRT_5 = r#"$ \sqrt{5} $
([A002163](https://oeis.org/A002163/constant),
[wikipedia](https://en.wikipedia.org/wiki/Square_root_of_5))
`≈ 2.23606797…`"#;
_FLOAT_CONST_SQRT_6 = r#"$ \sqrt{6} $
([A010464](https://oeis.org/A010464/constant))
`≈ 2.44948974…`"#;
_FLOAT_CONST_SQRT_7 = r#"$ \sqrt{7} $
([A010465](https://oeis.org/A010465/constant))
`≈ 2.64575131…`"#;
_FLOAT_CONST_SQRT_8 = r#"$ \sqrt{8} $
([A010466](https://oeis.org/A010466/constant))
`≈ 2.82842712…`"#;
_FLOAT_CONST_SQRT_10 = r#"$ \sqrt{10} $
([A010467](https://oeis.org/A010467/constant))
`≈ 3.16227766…`"#;
_FLOAT_CONST_SQRT_11 = r#"$ \sqrt{11} $
([A010468](https://oeis.org/A010468/constant))
`≈ 3.31662479…`"#;
_FLOAT_CONST_SQRT_12 = r#"$ \sqrt{12} $
([A010469](https://oeis.org/A010469/constant))
`≈ 3.46410161…`"#;
_FLOAT_CONST_CBRT_2 = r#"$ \sqrt[\small 3]{2} $
([A002580](https://oeis.org/A002580/constant),
[wikipedia](https://en.wikipedia.org/wiki/Doubling_the_cube))
`≈ 1.25992104…`"#;
_FLOAT_CONST_CBRT_3 = r#"$ \sqrt[\small 3]{3} $
([A002581](https://oeis.org/A002581/constant))
`≈ 1.44224957…`"#;
_FLOAT_CONST_FRAC_1_CBRT_3 =
r#"$ 1/\sqrt[\small 3]{3} = (\normalsize\frac{1}{3})^{\small\frac{1}{3}} $
([A072365](https://oeis.org/A072365/constant))
`≈ 0.69336127…`"#;

// other
_FLOAT_CONST_E = r#"$ e $
([A001113](https://oeis.org/A001113/constant))
`≈ 2.71828182…`
<p>*The Euler number or Napier's constant*.</p>
<p>Continuous fraction: $ [2;1,2,1,1,4,1,1,6,1,…,1,2n,1,…] $</p>"#;
_FLOAT_CONST_EGAMMA = r#"$ γ $
([A001620](https://oeis.org/A001620/constant))
`≈ 0.57721566…`
<p>*Gamma, or the Euler-Mascheroni constant.*</p>"#;
_FLOAT_CONST_LOG2_E = r#"$ \log_2{e} $
([A007525](https://oeis.org/A007525/constant))
`≈ 1.44269504…`"#;
_FLOAT_CONST_LOG2_10 = r#"log<sub>2</sub>(10)
([A020862](https://oeis.org/A020862/constant))
`≈ 3.32192809…`"#;
_FLOAT_CONST_LOG10_E = r#"log<sub>10</sub>(e)
([A002285](https://oeis.org/A002285/constant))
`≈ 0.43429448…`"#;
_FLOAT_CONST_LOG10_2 = r#"log<sub>10</sub>(2)
([A007524](https://oeis.org/A007524/constant))
`≈ 0.30102999…`"#;
_FLOAT_CONST_LN_2 = r#"ln(2)
([A002162](https://oeis.org/A002162/constant))
`≈ 0.69314718…`"#;
_FLOAT_CONST_LN_10 = r#"ln(10)
([A002392](https://oeis.org/A002392/constant))
`≈ 2.30258509…`"#;
}

// formulas
crate::CONST! { hidden macro_export,
/* Rounding */

_FLOAT_FORMULA_FLOOR = r#"$$
\large \lfloor x \rfloor = \max \{ n \in ℤ \,|\, n \leq x \} $$"#;
_FLOAT_FORMULA_CEIL = r#"$$
\lceil x \rceil = \min \{ n \in ℤ \,|\, n \geq x \} $$"#;
_FLOAT_FORMULA_ROUND_TIES_AWAY = r#"$$
\text{round\\_ties\\_away}(x) = \begin{cases}
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \text{ or }
    (x - \lfloor x \rfloor = 0.5 \text{ and } x > 0) \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \text{ or }
    (x - \lfloor x \rfloor = 0.5 \text{ and } x < 0)
\end{cases}
$$"#;
_FLOAT_FORMULA_ROUND_TIES_EVEN = r#"$$
\text{round\\_ties\\_even}(x) = \begin{cases}
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is even} \cr
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is odd}
\end{cases}
$$"#;
_FLOAT_FORMULA_ROUND_TIES_ODD = r#"$$
\text{round\\_ties\\_odd}(x) = \begin{cases}
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
\lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is odd} \cr
\lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    \lfloor x \rfloor \text{ is even}
\end{cases}
$$"#;
_FLOAT_FORMULA_TRUNC = r#"$$
\text{trunc}(x) = \begin{cases}
\lfloor x \rfloor, & \text{if } x \geq 0 \\
\lceil x \rceil, & \text{if } x < 0
\end{cases}
$$"#;
_FLOAT_FORMULA_FRACT = r#"$$ \text{fract}(x) = x - \text{trunc}(x) $$"#;
_FLOAT_FORMULA_SPLIT = r#"$$ \text{split}(x) = (\text{trunc}(x), \text{fract}(x)) $$"#;


/* Linear interpolation */

_FLOAT_FORMULA_SCALE = r#"$$
\large \text{scale}(x, min, max, u, v) = (v - u) \frac{x - min}{max - min} + u $$"#;
_FLOAT_FORMULA_LERP = r#"$$ \large \text{lerp}(x, u, v) = (1 - x) \cdot u + x \cdot v $$"#;

/* Exponentials and Logarithms */

_FLOAT_FORMULA_POWF_SERIES = r#"$$ \large x^y = e^{y \cdot \ln(x)} $$"#;
_FLOAT_FORMULA_HYPOT_NR = r#"$$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$"#;
_FLOAT_FORMULA_HYPOT_FISR = r#"$$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$"#;

_FLOAT_FORMULA_EXP_SERIES = r#"$$
e^x = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
For values $ x < 0 $ it uses the identity: $$ e^x = \frac{1}{e^-x} $$"#;
_FLOAT_FORMULA_EXP_M1_SERIES = r#"$$
e^x -1 = x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
For values $ x < 0 $ it uses the identity: $$ e^x -1 = -\frac{1}{e^{-x}+1} $$
For values $ x > 0.001 $ it uses [`exp_series`][Self::exp_series]."#;
_FLOAT_FORMULA_EXP2_SERIES = r#"
The series based on the formula $ 2^x = e^{x \ln(2)} $ is:
$$ 2^x = 1 + x \ln(2) + \frac{(x \ln(2))^2}{2!} +
\frac{(x \ln(2))^3}{3!} + \frac{(x \ln(2))^4}{4!} + \cdots $$"#;
_FLOAT_FORMULA_LN_SERIES = r#"$$
\ln(x) = 2 \left( \frac{x-1}{x+1} + \frac{1}{3} \left( \frac{x-1}{x+1} \right)^3 +
\frac{1}{5} \left( \frac{x-1}{x+1} \right)^5 + \cdots \right) $$"#;
_FLOAT_FORMULA_LOG_SERIES = r#"$$ \log_{\text{base}}(x) = \frac{\ln(x)}{\ln(\text{base})} $$"#;
_FLOAT_FORMULA_LOG2_SERIES = r#"$$ \log_{2}(x) = \frac{\ln(x)}{\ln(2)} $$"#;
_FLOAT_FORMULA_LOG10_SERIES = r#"$$ \log_{10}(x) = \frac{\ln(x)}{\ln(10)} $$"#;

/* Trigonometry */

_FLOAT_FORMULA_SIN_MINIMAX = r#"$$ \sin(x) \approx x + x^3\,P(x^2) $$"#;
_FLOAT_FORMULA_COS_MINIMAX = r#"$$ \cos(x) \approx 1 + x^2\,Q(x^2) $$"#;

_FLOAT_FORMULA_SIN_SERIES = r#"$$
\sin(x) = x - \frac{x^3}{3!} + \frac{x^5}{5!} - \frac{x^7}{7!} + \cdots $$"#;
_FLOAT_FORMULA_COS_SERIES = r#"$$
\cos(x) = 1 - \frac{x^2}{2!} + \frac{x^4}{4!} - \frac{x^6}{6!} + \cdots $$"#;
_FLOAT_FORMULA_TAN_SERIES = r#"$$ \tan(x) = \frac{\sin(x)}{\cos(x)} $$"#;
_FLOAT_FORMULA_ASIN_SERIES = r#"$$
\arcsin(x) = x + \left( \frac{1}{2} \right) \frac{x^3}{3} +
\left( \frac{1}{2} \cdot \frac{3}{4} \right) \frac{x^5}{5} +
\left( \frac{1}{2} \cdot \frac{3}{4} \cdot \frac{5}{6} \right) \frac{x^7}{7} +
\cdots $$"#;
_FLOAT_FORMULA_ACOS_SERIES = r#"$$ \arccos(x)=2π-arcsin(x) $$"#;
_FLOAT_FORMULA_ATAN_SERIES = r#"$$
\arctan(x) = x - \frac{x^3}{3} + \frac{x^5}{5} - \frac{x^7}{7} + \cdots $$
For values $ |x| > 1 $ it uses the identity:
$$ \arctan(x) = \frac{\pi}{2} - \arctan(\frac{1}{x}) $$"#;
_FLOAT_FORMULA_SINH_SERIES = r#"$$ \sinh(x) = \frac{e^x - e^{-x}}{2} $$"#;
_FLOAT_FORMULA_COSH_SERIES = r#"$$ \cosh(x) = \frac{e^x + e^{-x}}{2} $$"#;
_FLOAT_FORMULA_TANH_SERIES = r#"$$ \tanh(x) = \frac{\sinh(x)}{\cosh(x)} $$"#;
_FLOAT_FORMULA_ASINH_SERIES = r#"$$ \text{asinh}(x) = \ln(x + \sqrt{x^2 + 1}) $$"#;
_FLOAT_FORMULA_ACOSH_SERIES = r#"$$ \text{acosh}(x) = \ln(x + \sqrt{x^2 - 1}) $$"#;
_FLOAT_FORMULA_ATANH_SERIES = r#"$$
\text{atanh}(x) = \frac{1}{2} \ln\left(\frac{1 + x}{1 - x}\right) $$"#;

/* Calculus */

_FLOAT_FORMULA_DERIVATIVE = r#"$$ f^{\prime}(x) \approx \frac{f(x + h) - f(x)}{h} $$"#;
_FLOAT_FORMULA_INTEGRATE = r#"$$
\int_a^b f(x) \, dx \approx \sum_{i=0}^{n-1} f(x_i) \cdot \Delta x $$
where $$ \Delta x = \frac{b-a}{n} $$"#;
_FLOAT_FORMULA_PARTIAL_DERIVATIVE_X = r#"$$
\frac{\partial f}{\partial x} \approx \frac{f(x + h, y) - f(x, y)}{h} $$"#;
_FLOAT_FORMULA_PARTIAL_DERIVATIVE_Y = r#"$$
\frac{\partial f}{\partial x} \approx \frac{f(x + h, y) - f(x, y)}{h} $$"#;
}
