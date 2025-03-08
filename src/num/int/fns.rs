// devela::num::int::fns

#[cfg(all(not(feature = "std"), feature = "_float_f64"))]
use crate::ExtFloat;

#[doc = crate::TAG_NUM!()]
/// The prime number theorem formula.
///
/// Returns the approximate count of primes less than the given `n`.
///
/// $$ \large \pi(x) \sim \frac{x}{\ln(x)} $$
///
/// # Examples
/// ```
/// use devela::num::prime_number_theorem as pi;
///
/// // Showing the % difference against the real amount, if known.
/// // Note how precision increases in direct relationship to the power.
/// assert_eq![pi(u8::MAX.into()), 46]; // 14.81% < 54
/// assert_eq![pi(u16::MAX.into()), 5909]; // 9.67% < 6542
///
/// #[cfg(feature = "std")] // too slow otherwise
/// {
///     assert_eq![pi(u32::MAX.into()), 193635251]; // 4.74% < 203280221
///     assert_eq![pi(u64::MAX.into()), 415828534307635072]; // 2.30% < 425656284035217743
///     assert_eq![pi(2u128.pow(92)), 77650867634561160386183168]; // 1.59% < 78908656317357166866404346
///     assert_eq![pi(u128::MAX.into()), 3835341275459348115779911081237938176]; // ?% < ?
/// }
/// ```
/// # Links
/// - <https://mathworld.wolfram.com/PrimeNumberTheorem.html>
/// - <https://en.wikipedia.org/wiki/Prime_number_theorem>
/// - The exact prime count till $2^{92}$ is available in <https://oeis.org/A007053>.
//
// IMPROVE: use big int and big float.
#[must_use]
#[cfg(any(feature = "std", feature = "_float_f64"))]
#[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = "_float_f64"))))]
pub fn prime_number_theorem(n: u128) -> u128 {
    #[allow(clippy::cast_precision_loss)]
    let float_n = n as f64;
    let ln_n = float_n.ln();
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    return (float_n / ln_n).round() as u128;
}
