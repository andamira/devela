// devela::num::float::ext_trait
//
//!
//

use crate::num::{Float, Sign};

/// Extension trait for floating-point types.
///
/// This trait can be more convenient to use than the [`Float`] struct,
/// for non-const operations over primitive floating-point types.
///
/// `Float` has a few more methods implemented if the `libm` feature is enabled,
/// and some of its methods are *const* if the `unsafe_const` feature is enabled.
#[rustfmt::skip]
pub trait ExtFloat: Sized {

    /// The largest integer less than or equal to `self`.
    ///
    /// $$ \lfloor x \rfloor = \max \{ n \in \mathbb{Z} \,|\, n \leq x \} $$
    #[must_use]
    fn floor(self) -> Self;

    /// The smallest integer greater than or equal to `self`.
    ///
    /// $$ \lceil x \rceil = \min \{ n \in \mathbb{Z} \,|\, n \geq x \} $$
    #[must_use]
    fn ceil(self) -> Self;

    /// The nearest integer to `self`, default rounding, same as
    /// [`round_ties_away`][ExtFloat::round_ties_away]
    #[must_use]
    fn round(self) -> Self;

    /// The nearest integer to `self`, rounding ties away from `0.0`.
    ///
    /// $$
    /// \text{round\\_ties\\_away}(x) = \begin{cases}
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \text{ or }
    ///     (x - \lfloor x \rfloor = 0.5 \text{ and } x > 0) \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \text{ or }
    ///     (x - \lfloor x \rfloor = 0.5 \text{ and } x < 0)
    /// \end{cases}
    /// $$
    #[must_use]
    fn round_ties_away(self) -> Self;

    /// The nearest integer to `self`, rounding ties to the nearest even integer.
    ///
    /// $$
    /// \text{round\\_ties\\_even}(x) = \begin{cases}
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    ///     \lfloor x \rfloor \text{ is even} \cr
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    ///     \lfloor x \rfloor \text{ is odd}
    /// \end{cases}
    /// $$
    #[must_use]
    fn round_ties_even(self) -> Self;

    /// The nearest integer to `self`, rounding ties to the nearest odd integer.
    ///
    /// $$
    /// \text{round\\_ties\\_odd}(x) = \begin{cases}
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    ///     \lfloor x \rfloor \text{ is odd} \cr
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and }
    ///     \lfloor x \rfloor \text{ is even}
    /// \end{cases}
    /// $$
    #[must_use]
    fn round_ties_odd(self) -> Self;

    /// The integral part of `self`.
    ///
    /// $$
    /// \text{trunc}(x) = \begin{cases}
    /// \lfloor x \rfloor, & \text{if } x \geq 0 \\
    /// \lceil x \rceil, & \text{if } x < 0
    /// \end{cases}
    /// $$
    #[must_use]
    fn trunc(self) -> Self;

    /// The fractional part of `self`.
    ///
    /// $$ \text{fract}(x) = x - \text{trunc}(x) $$
    #[must_use]
    fn fract(self) -> Self;

    /// The integral and fractional parts ox `self`.
    ///
    /// $$ \text{split}(x) = (\text{trunc}(x), \text{fract}(x)) $$
    #[must_use]
    fn split(self) -> (Self, Self);

    /// The absolute value of `self`.
    #[must_use]
    fn abs(self) -> Self;

    /// The negative absolute value of `self`.
    #[must_use]
    fn neg_abs(self) -> Self;

    /// Returns the `Sign` of `self`.
    #[must_use]
    fn sign(self) -> Sign;

    /// Returns the `Sign` of `self`, except for zero.
    #[must_use]
    fn sign_nonzero(self) -> Sign;

    /// A number that represents the sign of `self`.
    #[must_use]
    fn signum(self) -> Self;

    /// Flips the sign of `self`.
    #[must_use]
    fn flip_sign(self) -> Self;

    /// Returns `true` if `self` has a positive sign.
    #[must_use]
    fn is_sign_positive(self) -> bool;

    /// Returns `true` if `self` has a negative sign.
    #[must_use]
    fn is_sign_negative(self) -> bool;

    /// Returns `true` if `self` is either 0.0 or -0.0.
    #[must_use]
    fn is_zero(self) -> bool;

    /// Returns `true` if `self` has a positive sign and is not zero.
    #[must_use]
    fn is_sign_positive_nonzero(self) -> bool;

    /// Returns `true` if `self` has a negative sign and is not zero.
    #[must_use]
    fn is_sign_negative_nonzero(self) -> bool;

    /// A number composed of a magnitude of `self` and the sign of `sign`.
    #[must_use]
    fn copysign(self, sign: Self) -> Self;

    /// Fused multiply-add. Computes `(self * mul) + add` with only one rounding error.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it uses [`mul_add_fallback`][Float::mul_add_fallback].
    #[must_use]
    fn mul_add(self, mul: Self, add: Self) -> Self;

    /// The euclidean division.
    #[must_use]
    fn div_euclid(self, rhs: Self) -> Self;

    /// The least nonnegative remainder of `self % rhs`.
    #[must_use]
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Returns `self` between `[min..=max]` scaled to a new range `[u..=v]`.
    ///
    /// Values of `self` outside `[min..=max]` are not clamped and will result in extrapolation.
    /// # Formula
    /// $$ \large \text{scale}(x, min, max, u, v) = (v - u) \frac{x - min}{max - min} + u $$
    #[must_use]
    fn scale(self, min: Self, max: Self, u: Self, v: Self) -> Self;

    /// Calculates a linearly interpolated value between `u..=v`
    /// based on the percentage `self` between `[0..=1]`.
    ///
    /// Values of `self` outside `[0..=1]` are not clamped and will result in extrapolation.
    /// # Formula
    /// $$ \large \text{lerp}(x, u, v) = (1 - x) \cdot u + x \cdot v $$
    #[must_use]
    fn lerp(self, u: Self, v: Self) -> Self;

    /// Raises `self` to the `y` floating point power.
    #[must_use]
    fn powf(self, y: Self) -> Self;

    /// Raises `self` to the `p` integer power.
    #[must_use]
    fn powi(self, p: i32) -> Self;

    /// The square root.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`sqrt_nr`][Float::sqrt_nr].
    #[must_use]
    fn sqrt(self) -> Self;

    /// The square root calculated using the
    /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
    #[must_use]
    fn sqrt_fisr(self) -> Self;

    /// $ 1 / \sqrt{x} $ the
    /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
    #[must_use]
    fn fisr(self) -> Self;

    /// The cubic root.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`cbrt_nr`][Float::cbrt_nr].
    #[must_use]
    fn cbrt(self) -> Self;

    /// The hypothenuse (the euclidean distance).
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`hypot_nr`][Float::hypot_nr].
    #[must_use]
    fn hypot(self, rhs: Self) -> Self;

    /// $e^x$ (the exponential function).
    ///
    /// The maximum values with a representable result are:
    /// 88.722… for `f32` and 709.782… for `f64`.
    ///
    /// With both `std` and `libm` disabled it leverages [`exp_series`][Float::exp_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn exp(self) -> Self;

    /// $2^x$.
    ///
    /// With both `std` and `libm` disabled it leverages [`exp2_series`][Float::exp2_series]
    /// with [`exp2_series_terms`][Float::exp2_series_terms].
    #[must_use]
    fn exp2(self) -> Self;

    /// The exponential minus 1, more accurately.
    ///
    /// With both `std` and `libm` disabled it leverages [`exp_m1_series`][Float::exp_m1_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn exp_m1(self) -> Self;

    /// The natural logarithm of `self`.
    ///
    /// With both `std` and `libm` disabled it leverages [`ln_series`][Float::ln_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn ln(self) -> Self;

    /// The natural logarithm of `self` plus 1, more accurately.
    ///
    /// With both `std` and `libm` disabled it leverages [`ln_1p_series`][Float::ln_1p_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn ln_1p(self) -> Self;

    /// The logarithm of `self` with respect to an arbitrary `base`.
    ///
    /// With both `std` and `libm` disabled it leverages [`log_series`][Float::log_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn log(self, base: Self) -> Self;

    /// The base 2 logarithm of `self`.
    ///
    /// With both `std` and `libm` disabled it leverages [`log2_series`][Float::log2_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn log2(self) -> Self;

    /// The base 10 logarithm of `self`.
    ///
    /// With both `std` and `libm` disabled it leverages [`log10_series`][Float::log10_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn log10(self) -> Self;

    /// The factorial.
    ///
    /// The maximum values with a representable result are:
    /// 34 for `f32` and 170 for `f64`.
    #[must_use]
    fn factorial(n: u32) -> Self;

    /// The sine.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`sin_series`][Float::sin_series] with 8 terms.
    #[must_use]
    fn sin(self) -> Self;

    /// The cosine.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`cos_series`][Float::cos_series] with 8 terms.
    #[must_use]
    fn cos(self) -> Self;

    /// Both the sine and cosine.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`sin_cos_series`][Float::sin_cos_series] with 8 terms.
    #[must_use]
    fn sin_cos(self) -> (Self, Self);

    /// The tangent.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`tan_series`][Float::tan_series] with 8 terms.
    #[must_use]
    fn tan(self) -> Self;

    /// The arc sine.
    ///
    /// With both `std` and `libm` disabled it leverages [`asin_series`][Float::asin_series]
    /// with [`asin_series_terms`][Float::asin_series_terms].
    #[must_use]
    fn asin(self) -> Self;

    /// The arc cosine.
    ///
    /// With both `std` and `libm` disabled it leverages [`acos_series`][Float::acos_series]
    /// with [`acos_series_terms`][Float::acos_series_terms].
    #[must_use]
    fn acos(self) -> Self;

    /// The arc tangent.
    ///
    /// With both `std` and `libm` disabled it leverages [`atan_series`][Float::atan_series]
    /// with [`atan_series_terms`][Float::atan_series_terms].
    #[must_use]
    fn atan(self) -> Self;

    /// The arc tangent of two variables.
    ///
    /// With both `std` and `libm` disabled it leverages [`atan2_series`][Float::atan2_series]
    /// with [`atan_series_terms`][Float::atan_series_terms].
    #[must_use]
    fn atan2(self, other: Self) -> Self;

    /// The hyperbolic sine.
    ///
    /// With both `std` and `libm` disabled it leverages [`sinh_series`][Float::sinh_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn sinh(self) -> Self;

    /// The hyperbolic cosine.
    ///
    /// With both `std` and `libm` disabled it leverages [`cosh_series`][Float::cosh_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn cosh(self) -> Self;

    /// The hyperbolic tangent.
    ///
    /// With both `std` and `libm` disabled it leverages [`cosh_series`][Float::cosh_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn tanh(self) -> Self;

    /// The inverse hyperbolic sine of `self`.
    ///
    /// With both `std` and `libm` disabled it leverages [`asinh_series`][Float::asinh_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn asinh(self) -> Self;

    /// The inverse hyperbolic cosine of `self`.
    ///
    /// With both `std` and `libm` disabled it leverages [`acosh_series`][Float::acosh_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn acosh(self) -> Self;

    /// The inverse hyperbolic tangent of `self`.
    ///
    /// With both `std` and `libm` disabled it leverages [`atanh_series`][Float::atanh_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn atanh(self) -> Self;

    /// The clamped value, ignoring `NaN`.
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;

    /// The maximum of two numbers, ignoring `NaN`.
    #[must_use]
    fn max(self, other: Self) -> Self;

    /// The minimum of two numbers, ignoring `NaN`.
    #[must_use]
    fn min(self, other: Self) -> Self;

    /// The clamped value, propagating `NaN`.
    #[must_use]
    fn clamp_nan(self, min: Self, max: Self) -> Self;

    /// The maximum of two numbers, propagating `NaN`.
    #[must_use]
    fn max_nan(self, other: Self) -> Self;

    /// The minimum of two numbers, propagating `NaN`.
    #[must_use]
    fn min_nan(self, other: Self) -> Self;

    /// The clamped value, using total order.
    #[must_use]
    fn clamp_total(self, min: Self, max: Self) -> Self;

    /// The maximum of two numbers using total order.
    #[must_use]
    fn max_total(self, other: Self) -> Self;

    /// The minimum of two numbers using total order.
    #[must_use]
    fn min_total(self, other: Self) -> Self;

    /* consts: misc */

    /// The multiplicative identity 1.
    const ONE: Self;
    /// The additive identity 0.
    const ZERO: Self;
    /// The negative of the multiplicative identity -1.
    const NEG_ONE: Self;
    /// The negative of the additive identity -0.
    const NEG_ZERO: Self;

    // TODO

    /* consts: PI */

    /// $ π = \frac{1}{2} τ = 180º $
    /// ([A000796](https://oeis.org/A000796/constant))
    /// *the ratio of the circumference to the diameter, a half-turn*
    const PI: Self;

    /// $ π/2 = τ/4 = 90º $
    /// ([A019669](https://oeis.org/A019669/constant))
    const FRAC_PI_2: Self;

    /// $ π/3 = τ/6 = 60º $
    /// ([A019670](https://oeis.org/A019670/constant))
    const FRAC_PI_3: Self;

    /// $ π/4 = τ/8 = 45º $
    /// ([A003881](https://oeis.org/A003881/constant))
    const FRAC_PI_4: Self;

    /// $ π/6 = τ/12 = 30º $
    /// ([A019673](https://oeis.org/A019673/constant))
    const FRAC_PI_6: Self;

    /// $ π/8 = τ/16 = 22.5º $
    /// ([A019675](https://oeis.org/A019675/constant))
    const FRAC_PI_8: Self;

    /// $ \sqrt{π} = \sqrt{\frac{1}{2} τ} $
    /// ([A002161](https://oeis.org/A002161/constant))
    const SQRT_PI: Self;

    /// $ 1/π = 2/τ $
    /// ([A049541](https://oeis.org/A049541/constant))
    const FRAC_1_PI: Self;

    /// $ 1/\sqrt{π} = 1/\sqrt{τ/2} $
    /// ([A087197](https://oeis.org/A087197/constant))
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const FRAC_1_SQRT_PI: Self;

    /// $ 2/π $
    /// ([A060294](https://oeis.org/A060294/constant))
    /// *Buffon's constant*
    const FRAC_2_PI: Self;

    /// $ 2/\sqrt{π} $
    /// ([A190732](https://oeis.org/A190732/constant))
    const FRAC_2_SQRT_PI: Self;

    /* consts: TAU */

    /// $ τ = 2π = 360º $
    /// ([A019692](https://oeis.org/A019692/constant))
    /// *the ratio of the circumference to the radius, a full-turn*
    const TAU: Self;

    /// $ τ/2 = π = 180º $
    /// ([A000796](https://oeis.org/A000796/constant))
    const FRAC_TAU_2: Self;

    /// $ τ/3  = 2π/3 = 120º $
    /// ([A019693](https://oeis.org/A019693/constant))
    const FRAC_TAU_3: Self;

    /// $ τ/4 = π/2 = 90º $
    /// ([A019693](https://oeis.org/A019693/constant))
    const FRAC_TAU_4: Self;

    /// $ τ/5 = 2π/5 = 72º $
    /// ([A019694](https://oeis.org/A019694/constant))
    const FRAC_TAU_5: Self;

    /// $ τ/6 = π/3 = 60º $
    /// ([A019670](https://oeis.org/A019670/constant))
    const FRAC_TAU_6: Self;

    /// $ τ/8 = π/4 = 45º $
    /// ([A003881](https://oeis.org/A003881/constant))
    const FRAC_TAU_8: Self;

    /// $ τ/9 = 2π/9 = 40º $
    /// ([A019696](https://oeis.org/A019696/constant))
    const FRAC_TAU_9: Self;

    /// $ τ/12 = π/6 = 30º $
    /// ([A019673](https://oeis.org/A019673/constant))
    const FRAC_TAU_12: Self;

    /// $ τ/16 = π/8 = 22.5º $
    /// ([A019675](https://oeis.org/A019675/constant))
    const FRAC_TAU_16: Self;

    /// $ τ/24 = π/12 = 15º $
    /// ([A019679](https://oeis.org/A019679/constant))
    const FRAC_TAU_24: Self;

    /// $ τ/72 = π/36 = 5º $
    const FRAC_TAU_72: Self;

    /// $ τ/360 = π/180 = 1º $ *arcdegree*
    /// ([A019685](https://oeis.org/A019685),
    /// [wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
    const FRAC_TAU_360: Self;

    /// $ 360/τ = 180/π $
    /// ([A072097](https://oeis.org/A072097/constant))
    const FRAC_360_TAU: Self;

    /// $ τ/(360*60) = 1' $ *arcminute*
    /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
    const FRAC_TAU_ARCMINUTES: Self;

    /// $ τ/(360 * 60 * 60) = 1'' $ *arcsecond*
    /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
    const FRAC_TAU_ARCSECONDS: Self;

    /// $ \sqrt{τ} = \sqrt{2π} $
    /// ([A019727](https://oeis.org/A019727/constant))
    const SQRT_TAU: Self;

    /// $ 1/τ = 1/2π $
    /// ([A086201](https://oeis.org/A086201/constant))
    const FRAC_1_TAU: Self;

    /// $ 1/\sqrt{τ} = 1/\sqrt{2π} $
    /// ([A231863](https://oeis.org/A231863/constant))
    const FRAC_1_SQRT_TAU: Self;

    /// $ 2/τ = 1/π $
    /// ([A049541](https://oeis.org/A049541/constant))
    const FRAC_2_TAU: Self;

    /// $ 2/\sqrt{τ} = \sqrt{2/π} $
    /// ([A231863](https://oeis.org/A231863/constant))
    const FRAC_2_SQRT_TAU: Self;

    /* consts: PHI */

    /// $ φ  = (1+\sqrt{5})/2 $
    /// ([A001622](https://oeis.org/A001622/constant))
    /// *the golden ratio*
    ///
    /// Continued fraction: $ [1;1,1,1,…] $
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const PHI: Self;

    /// $ φ^2 = φ+1 = (3+\sqrt{5})/2 $
    /// ([A104457](https://oeis.org/A104457/constant))
    const SQ_PHI: Self;

    /// $ 1/φ = φ-1 $
    /// ([A094214](https://oeis.org/A094214/constant))
    /// *the reciprocal of [φ][Self#PHI]*
    const FRAC_1_PHI: Self;

    /// $ -1/φ = 1-φ $
    /// *the negative reciprocal of [φ][Self#PHI] and its conjugate in $ x^2-x-1 $*
    const NEG_FRAC_1_PHI: Self;

    /// $ \sqrt{φ} $
    /// ([A139339](https://oeis.org/A139339/constant))
    const SQRT_PHI: Self;

    /// $ 1/\sqrt{φ} = \sqrt{φ/φ^2} = \sqrt{φ^2-2} $
    /// ([A197762](https://oeis.org/A197762/constant))
    const FRAC_SQRT_PHI: Self;

    /// ([A058265](https://oeis.org/A058265/constant))
    /// *The tribonacci constant*
    const TRIBONACCI: Self;

    /* consts: sqrt */

    /// $ \sqrt{2} $
    /// ([A002193](https://oeis.org/A002193/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2))
    const SQRT_2: Self;

    /// $ 1/\sqrt{2} = \sqrt{1/2} $
    /// ([A010503](https://oeis.org/A010503/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2#Multiplicative_inverse))
    const FRAC_1_SQRT_2: Self;

    /// $ \sqrt{3} $
    /// ([A002194](https://oeis.org/A002194/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_3))
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const SQRT_3: Self;

    /// $ 1/\sqrt{3} = \sqrt{1/3} $
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const FRAC_1_SQRT_3: Self;

    /// $ \sqrt{5} $
    /// ([A002163](https://oeis.org/A002163/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_5))
    const SQRT_5: Self;

    /// $ \sqrt{6} $
    /// ([A010464](https://oeis.org/A010464/constant))
    const SQRT_6: Self;

    /// $ \sqrt{7} $
    /// ([A010465](https://oeis.org/A010465/constant))
    const SQRT_7: Self;

    /// $ \sqrt{8} $
    /// ([A010466](https://oeis.org/A010466/constant))
    const SQRT_8: Self;

    /// $ \sqrt{10} $
    /// ([A010467](https://oeis.org/A010467/constant))
    const SQRT_10: Self;

    /// $ \sqrt{11} $
    /// ([A010468](https://oeis.org/A010468/constant))
    const SQRT_11: Self;

    /// $ \sqrt{12} $
    /// ([A010469](https://oeis.org/A010469/constant))
    const SQRT_12: Self;

    /* consts: cbrt */

    /// $ \sqrt[\small 3]{2} $
    /// ([A002580](https://oeis.org/A002580/constant),
    /// [wikipedia](https://en.wikipedia.org/wiki/Doubling_the_cube))
    const CBRT_2: Self;

    /// $ \sqrt[\small 3]{3} $
    /// ([A002581](https://oeis.org/A002581/constant))
    const CBRT_3: Self;

    /// $ 1/\sqrt[\small 3]{3} = (\normalsize\frac{1}{3})^{\small\frac{1}{3}} $
    /// ([A072365](https://oeis.org/A072365/constant))
    const FRAC_1_CBRT_3: Self;

    /* consts: other */

    /// $ e $
    /// ([A001113](https://oeis.org/A001113/constant))
    /// *the Euler number or Napier's constant*
    ///
    /// Continuous fraction: $ [2;1,2,1,1,4,1,1,6,1,…,1,2n,1,…] $
    const E: Self;

    /// $ γ $
    /// ([A001620](https://oeis.org/A001620/constant))
    /// *gamma, or the Euler-Mascheroni constant*
    // WAIT: [more_float_constants](https://github.com/rust-lang/rust/issues/103883)
    const EGAMMA: Self;

    /// $ \log_2{e} $
    /// ([A007525](https://oeis.org/A007525/constant))
    const LOG2_E: Self;

    /// log<sub>2</sub>(10)
    /// ([A020862](https://oeis.org/A020862/constant))
    const LOG2_10: Self;

    /// log<sub>10</sub>(e)
    /// ([A002285](https://oeis.org/A002285/constant))
    const LOG10_E: Self;

    /// log<sub>10</sub>(2)
    /// ([A007524](https://oeis.org/A007524/constant))
    const LOG10_2: Self;

    /// ln(2)
    /// ([A002162](https://oeis.org/A002162/constant))
    const LN_2: Self;

    /// ln(10)
    /// ([A002392](https://oeis.org/A002392/constant))
    const LN_10: Self;
}

macro_rules! impl_float_ext {
    // $f: the floating-point type.
    // $ue: unsigned int type with the same bit-size.
    // $ie: the integer type for integer exponentiation.
    ($( ($f:ty, $ue:ty|$ie:ty) ),+) => { $( impl_float_ext![@$f, $ue|$ie]; )+ };
    (@$f:ty, $ue:ty|$ie:ty) => {
        impl ExtFloat for $f {
            /* consts */

            // const BIAS: u32 = [<BIAS_ $f:upper>];
            // const EXPONENT_BITS: u32 = [<EXPONENT_BITS_ $f:upper>];
            // const SIGNIFICAND_BITS: u32 = [<SIGNIFICAND_BITS_ $f:upper>];

            const ONE: Self = Float::<Self>::ONE.0;
            const ZERO: Self = Float::<Self>::ZERO.0;
            const NEG_ONE: Self = Float::<Self>::NEG_ONE.0;
            const NEG_ZERO: Self = Float::<Self>::NEG_ZERO.0;

            // TODO

            // consts: PI related
            const PI: Self = Float::<Self>::PI.0;
            const FRAC_PI_2: Self = Float::<Self>::FRAC_PI_2.0;
            const FRAC_PI_3: Self = Float::<Self>::FRAC_PI_3.0;
            const FRAC_PI_4: Self = Float::<Self>::FRAC_PI_4.0;
            const FRAC_PI_6: Self = Float::<Self>::FRAC_PI_6.0;
            const FRAC_PI_8: Self = Float::<Self>::FRAC_PI_8.0;
            const SQRT_PI: Self = Float::<Self>::SQRT_PI.0;
            const FRAC_1_PI: Self = Float::<Self>::FRAC_1_PI.0;
            const FRAC_1_SQRT_PI: Self = Float::<Self>::FRAC_1_SQRT_PI.0;
            const FRAC_2_PI: Self = Float::<Self>::FRAC_2_PI.0;
            const FRAC_2_SQRT_PI: Self = Float::<Self>::FRAC_2_SQRT_PI.0;
            // TAU related
            const TAU: Self = Float::<Self>::TAU.0;
            const FRAC_TAU_2: Self = Float::<Self>::FRAC_TAU_2.0;
            const FRAC_TAU_3: Self = Float::<Self>::FRAC_TAU_3.0;
            const FRAC_TAU_4: Self = Float::<Self>::FRAC_TAU_4.0;
            const FRAC_TAU_5: Self = Float::<Self>::FRAC_TAU_5.0;
            const FRAC_TAU_6: Self = Float::<Self>::FRAC_TAU_6.0;
            const FRAC_TAU_8: Self = Float::<Self>::FRAC_TAU_8.0;
            const FRAC_TAU_9: Self = Float::<Self>::FRAC_TAU_9.0;
            const FRAC_TAU_12: Self = Float::<Self>::FRAC_TAU_12.0;
            const FRAC_TAU_16: Self = Float::<Self>::FRAC_TAU_16.0;
            const FRAC_TAU_24: Self = Float::<Self>::FRAC_TAU_24.0;
            const FRAC_TAU_72: Self = Float::<Self>::FRAC_TAU_72.0;
            const FRAC_TAU_360: Self = Float::<Self>::FRAC_TAU_360.0;
            const FRAC_360_TAU: Self = Float::<Self>::FRAC_360_TAU.0;
            const FRAC_TAU_ARCMINUTES: Self = Float::<Self>::FRAC_TAU_ARCMINUTES.0;
            const FRAC_TAU_ARCSECONDS: Self = Float::<Self>::FRAC_TAU_ARCSECONDS.0;
            const SQRT_TAU: Self = Float::<Self>::SQRT_TAU.0;
            const FRAC_1_TAU: Self = Float::<Self>::FRAC_1_TAU.0;
            const FRAC_1_SQRT_TAU: Self = Float::<Self>::FRAC_1_SQRT_TAU.0;
            const FRAC_2_TAU: Self = Float::<Self>::FRAC_2_TAU.0;
            const FRAC_2_SQRT_TAU: Self = Float::<Self>::FRAC_2_SQRT_TAU.0;
            // consts PHI related
            const PHI: Self = Float::<Self>::PHI.0;
            const SQ_PHI: Self = Float::<Self>::SQ_PHI.0;
            const FRAC_1_PHI: Self = Float::<Self>::FRAC_1_PHI.0;
            const NEG_FRAC_1_PHI: Self = Float::<Self>::NEG_FRAC_1_PHI.0;
            const SQRT_PHI: Self = Float::<Self>::SQRT_PHI.0;
            const FRAC_SQRT_PHI: Self = Float::<Self>::FRAC_SQRT_PHI.0;
            const TRIBONACCI: Self = Float::<Self>::TRIBONACCI.0;
            // consts integer roots related
            const SQRT_2: Self = Float::<Self>::SQRT_2.0;
            const FRAC_1_SQRT_2: Self = Float::<Self>::FRAC_1_SQRT_2.0;
            const SQRT_3: Self = Float::<Self>::SQRT_3.0;
            const FRAC_1_SQRT_3: Self = Float::<Self>::FRAC_1_SQRT_3.0;
            const SQRT_5: Self = Float::<Self>::SQRT_5.0;
            const SQRT_6: Self = Float::<Self>::SQRT_6.0;
            const SQRT_7: Self = Float::<Self>::SQRT_7.0;
            const SQRT_8: Self = Float::<Self>::SQRT_8.0;
            const SQRT_10: Self = Float::<Self>::SQRT_10.0;
            const SQRT_11: Self = Float::<Self>::SQRT_11.0;
            const SQRT_12: Self = Float::<Self>::SQRT_12.0;
            // cbrt
            const CBRT_2: Self = Float::<Self>::CBRT_2.0;
            const CBRT_3: Self = Float::<Self>::CBRT_3.0;
            const FRAC_1_CBRT_3: Self = Float::<Self>::FRAC_1_CBRT_3.0;
            // consts other
            const E: Self = Float::<Self>::E.0;
            const EGAMMA: Self = Float::<Self>::EGAMMA.0;
            const LOG2_E: Self = Float::<Self>::LOG2_E.0;
            const LOG2_10: Self = Float::<Self>::LOG2_10.0;
            const LOG10_E: Self = Float::<Self>::LOG10_E.0;
            const LOG10_2: Self = Float::<Self>::LOG10_2.0;
            const LN_2: Self = Float::<Self>::LN_2.0;
            const LN_10: Self = Float::<Self>::LN_10.0;

            /* methods */

            #[inline]
            fn floor(self) -> Self { Float(self).floor().0 }

            #[inline]
            fn ceil(self) -> Self { Float(self).ceil().0 }

            #[inline]
            fn round(self) -> Self { Float(self).round_ties_away().0 }

            #[inline]
            fn round_ties_away(self) -> Self { Float(self).round_ties_away().0 }

            #[inline]
            fn round_ties_even(self) -> Self { Float(self).round_ties_even().0 }

            #[inline]
            fn round_ties_odd(self) -> Self { Float(self).round_ties_odd().0 }

            #[inline]
            fn trunc(self) -> Self { Float(self).trunc().0 }

            #[inline]
            fn fract(self) -> Self { Float(self).fract().0 }

            #[inline]
            fn split(self) -> (Self, Self) { let (i, f) = Float(self).split(); (i.0, f.0) }

            #[inline]
            fn abs(self) -> Self { Float(self).abs().0 }

            #[inline]
            fn neg_abs(self) -> Self { Float(self).neg_abs().0 }

            #[inline]
            fn sign(self) -> Sign { Float(self).sign() }

            #[inline]
            fn sign_nonzero(self) -> Sign { Float(self).sign_nonzero() }

            #[inline]
            fn signum(self) -> Self { Float(self).signum().0 }

            #[inline]
            fn flip_sign(self) -> Self { Float(self).flip_sign().0 }

            #[inline]
            fn is_sign_positive(self) -> bool { Float(self).is_sign_positive() }

            #[inline]
            fn is_sign_negative(self) -> bool { Float(self).is_sign_negative() }

            #[inline]
            fn is_zero(self) -> bool { Float(self).is_zero() }

            #[inline]
            fn is_sign_positive_nonzero(self) -> bool {
                Float(self).is_sign_positive_nonzero() }

            #[inline]
            fn is_sign_negative_nonzero(self) -> bool {
                Float(self).is_sign_negative_nonzero() }

            #[inline]
            fn copysign(self, sign: Self) -> Self { Float(self).copysign(sign).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn mul_add(self, mul: Self, add: Self) -> Self {
                Float(self).mul_add(mul, add).0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn mul_add(self, mul: Self, add: Self) -> Self {
                Float(self).mul_add_fallback(mul, add).0
            }

            #[inline]
            fn div_euclid(self, rhs: Self) -> Self { Float(self).div_euclid(rhs).0 }

            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self { Float(self).rem_euclid(rhs).0 }

            #[inline]
            fn scale(self, min: Self, max: Self, u: Self, v: Self) -> Self {
                Float(self).scale(min, max, u, v).0 }
            #[inline]
            fn lerp(self, u: Self, v: Self) -> Self { Float(self).lerp(u, v).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn powf(self, y: Self) -> Self { Float(self).powf(y).0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn powf(self, y: Self) -> Self {
                Float(self).powf_series(y, Float(self).ln_series_terms()).0
            }

            #[inline]
            fn powi(self, p: $ie) -> Self { Float(self).powi(p).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn sqrt(self) -> Self { Float(self).sqrt().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sqrt(self) -> Self { Float(self).sqrt_nr().0 }

            #[inline]
            fn sqrt_fisr(self) -> Self { Float(self).sqrt_fisr().0 }

            #[inline]
            fn fisr(self) -> Self { Float(self).fisr().0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn cbrt(self) -> Self { Float(self).cbrt().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn cbrt(self) -> Self { Float(self).cbrt_nr().0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn hypot(self, rhs: Self) -> Self { Float(self).hypot(rhs).0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn hypot(self, rhs: Self) -> Self { Float(self).hypot_nr(rhs).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn exp(self) -> Self { Float(self).exp().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn exp(self) -> Self {
                Float(self).exp_series(Float(self).exp_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn exp2(self) -> Self { Float(self).exp2().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn exp2(self) -> Self {
                Float(self).exp2_series(Float(self).exp2_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn exp_m1(self) -> Self { Float(self).exp_m1().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn exp_m1(self) -> Self {
                Float(self).exp_m1_series(Float(self).exp_series_terms()).0
            }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn ln(self) -> Self { Float(self).ln().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn ln(self) -> Self {
                Float(self).ln_series(Float(self).ln_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn ln_1p(self) -> Self { Float(self).ln_1p().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn ln_1p(self) -> Self {
                Float(self).ln_1p_series(Float(self).ln_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn log(self, base: Self) -> Self { Float(self).log(base).0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn log(self, base: Self) -> Self {
                Float(self).log_series(base, Float(self).ln_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn log2(self) -> Self { Float(self).log2().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn log2(self) -> Self {
                Float(self).log2_series(Float(self).ln_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn log10(self) -> Self { Float(self).log10().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn log10(self) -> Self {
                Float(self).log10_series(Float(self).ln_series_terms()).0 }

            #[inline]
            fn factorial(a: $ue) -> Self { Float::<Self>::factorial(a).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn sin(self) -> Self { Float(self).sin().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sin(self) -> Self { Float(self).sin_series(8).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn cos(self) -> Self { Float(self).cos().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn cos(self) -> Self { Float(self).cos_series(8).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn sin_cos(self) -> (Self, Self) { let (s, c) = Float(self).sin_cos(); (s.0, c.0) }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sin_cos(self) -> (Self, Self) {
                let (s, c) = Float(self).sin_cos_series(8); (s.0, c.0) }
            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn tan(self) -> Self { Float(self).tan().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn tan(self) -> Self { Float(self).tan_series(8).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn asin(self) -> Self { Float(self).asin().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn asin(self) -> Self {
                Float(self).asin_series(Float(self).asin_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn acos(self) -> Self { Float(self).acos().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn acos(self) -> Self {
                Float(self).acos_series(Float(self).acos_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn atan(self) -> Self { Float(self).atan().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn atan(self) -> Self {
                Float(self).atan_series(Float(self).atan_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn atan2(self, other: Self) -> Self { Float(self).atan2(other).0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn atan2(self, other: Self) -> Self {
                Float(self).atan2_series(other, Float(self).atan_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn sinh(self) -> Self { Float(self).sinh().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sinh(self) -> Self {
                Float(self).sinh_series(Float(self).exp_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn cosh(self) -> Self { Float(self).cosh().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn cosh(self) -> Self {
                Float(self).cosh_series(Float(self).exp_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn tanh(self) -> Self { Float(self).tanh().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn tanh(self) -> Self {
                Float(self).tanh_series(Float(self).exp_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn asinh(self) -> Self { Float(self).asinh().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn asinh(self) -> Self {
                Float(self).asinh_series(Float(self).exp_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn acosh(self) -> Self { Float(self).acosh().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn acosh(self) -> Self {
                Float(self).acosh_series(Float(self).exp_series_terms()).0 }

            #[inline] #[cfg(any(feature = "std", feature = "libm"))]
            fn atanh(self) -> Self { Float(self).atanh().0 }
            #[inline] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn atanh(self) -> Self {
                Float(self).atanh_series(Float(self).exp_series_terms()).0 }

            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self { Float(self).clamp(min, max).0 }

            #[inline]
            fn max(self, other: Self) -> Self { Float(self).max(other).0 }

            #[inline]
            fn min(self, other: Self) -> Self { Float(self).min(other).0 }

            #[inline]
            fn clamp_nan(self, min: Self, max: Self) -> Self { Float(self).clamp_nan(min, max).0 }

            #[inline]
            fn max_nan(self, other: Self) -> Self { Float(self).max_nan(other).0 }

            #[inline]
            fn min_nan(self, other: Self) -> Self { Float(self).min_nan(other).0 }

            #[inline]
            fn clamp_total(self, min: Self, max: Self) -> Self {
                Float(self).clamp_total(min, max).0
            }

            #[inline]
            fn max_total(self, other: Self) -> Self { Float(self).max_total(other).0 }

            #[inline]
            fn min_total(self, other: Self) -> Self { Float(self).min_total(other).0 }
        }
    }
}
impl_float_ext![(f32, u32 | i32), (f64, u32 | i32)];
