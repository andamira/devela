// devela::math::ops::fns::primes
//
//! factorization funcions
//
// TOC
// - prime_number_theorem
// - sint|uint:
//   - prime_is
//   - prime_nth
//   - prime_pi
//   - totient

use crate::{
    math::{fsize, FloatExt, MathError, MathResult as Result},
    meta::{iif, paste},
};

// MAYBE
// #[cfg(feature = "alloc")]
// use crate::result::Also;
// #[cfg(feature = "alloc")]
// use ::_alloc::{collections::BTreeSet, vec, vec::Vec};

/// The prime number theorem formula.
///
/// Returns the approximate count of primes less than the given `n`.
///
/// $$ \large \pi(x) \sim \frac{x}{\ln(x)} $$
///
/// # Examples
/// ```
/// use devela::math::prime_number_theorem as pi;
///
/// // Showing the % difference against the real amount, if known.
/// // Note how precision increases in direct relationship to the power.
/// assert_eq![pi(u8::MAX.into()), 46]; // 14.81% < 54
/// assert_eq![pi(u16::MAX.into()), 5909]; // 9.67% < 6542
/// assert_eq![pi(u32::MAX.into()), 193635251]; // 4.74% < 203280221
/// assert_eq![pi(u64::MAX.into()), 415828534307635072]; // 2.30% < 425656284035217743
/// assert_eq![pi(2u128.pow(92)), 77650867634561160386183168]; // 1.59% < 78908656317357166866404346
/// assert_eq![pi(u128::MAX.into()), 3835341275459348115779911081237938176]; // ?% < ?
/// ```
/// # Links
/// - <https://mathworld.wolfram.com/PrimeNumberTheorem.html>
/// - <https://en.wikipedia.org/wiki/Prime_number_theorem>
/// - The exact prime count till $2^{92}$ is available in <https://oeis.org/A007053>.
//
// IMPROVE: use big int and big float.
#[cfg(any(feature = "std", feature = "libm"))]
#[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
#[must_use]
pub fn prime_number_theorem(n: u128) -> u128 {
    #[allow(clippy::cast_precision_loss)]
    let float_n = n as f64;
    let ln_n = float_n.ln();
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    return (float_n / ln_n).round() as u128;
}

// $t:   the input/output type
// $up:  the upcasted type to do the operations on (the ones that can overflow)
// $ft:  the floating-point type to do some operations on
macro_rules! impl_ops {
    (signed $( ($t:ty, $up:ty, $ft:ty) ),+) => { $( impl_ops![@signed($t, $up, $ft)]; )+ };
    (unsigned $( ($t:ty, $up:ty, $ft:ty) ),+) => { $( impl_ops![@unsigned($t, $up, $ft)]; )+ };

    // implements signed ops
    (@signed($t:ty, $up:ty, $ft:ty) ) => { paste! {
        /// Returns `true` if `n` is prime.
        ///
        /// This approach uses optimized trial division, which means it checks
        /// only odd numbers starting from 3 and up to the square root of the
        /// given number. This is based on the fact that if a number is
        /// divisible by a number larger than its square root, the result of the
        /// division will be smaller than the square root, and it would have
        /// already been checked in previous iterations.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::prime_is_" $t ";\n\n"]
        #[doc = "assert![prime_is_" $t "(127)];"]
        #[doc = "assert![prime_is_" $t "(2)];"]
        #[doc = "assert![!prime_is_" $t "(1)];"]
        #[doc = "assert![!prime_is_" $t "(-2)];"]
        /// ```
        #[must_use]
        #[inline]
        pub fn [<prime_is_ $t>](number: $t) -> bool {
            match number {
                ..=1 => false,
                2..=3 => true,
                _ => {
                    iif![number % 2 == 0; return false];
                    let limit = (number as $ft).sqrt_fisr() as $t;
                    for i in (3..=limit).step_by(2) {
                        iif![number % i == 0; return false];
                    }
                    true
                }
            }
        }

        /// Finds the 0-indexed `nth` prime number.
        ///
        /// Note: If `nth` is negative, this function treats it as its absolute
        /// value. For example, a value of `-3` will be treated as `3`, and the
        /// function will return the 3rd prime number.
        ///
        #[doc = "Returns [`MathError::Overflow`] if the result would not fit in an `" $t "`."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::prime_nth_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(2), prime_nth_" $t "(0)];"]
        #[doc = "assert_eq![Ok(3), prime_nth_" $t "(1)];"]
        #[doc = "assert_eq![Ok(127), prime_nth_" $t "(30)];"]
        #[doc = "assert_eq![Ok(127), prime_nth_" $t "(-30)];"]
        /// assert![devela::math::prime_nth_i8(121).is_err()];
        /// ```
        #[must_use]
        #[inline]
        pub fn [<prime_nth_ $t>](nth: $t) -> Result<$t> {
            let (nth, mut count, mut i) = (nth.abs(), 1, 2);
            loop {
                if [<prime_is_ $t>](i) {
                    iif![count - 1 == nth; return Ok(i)];
                    count += 1;
                }
                i = i.checked_add(1).ok_or(MathError::Overflow)?;
            }
        }

        /// Counts the number of primes upto and including `n`.
        ///
        /// # Notation
        /// $$\pi(x)$$
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::prime_pi_" $t ";\n\n"]
        #[doc = "assert_eq![1, prime_pi_" $t "(2)];"]
        #[doc = "assert_eq![2, prime_pi_" $t "(3)];"]
        #[doc = "assert_eq![31, prime_pi_" $t "(127)];"]
        #[doc = "assert_eq![0, prime_pi_" $t "(-5)];"]
        /// ```
        /// # Links
        /// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
        /// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
        #[must_use]
        #[inline]
        pub fn [<prime_pi_ $t>](n: $t) -> usize {
            let mut prime_count = 0;
            for i in 0..=n {
                iif![[<prime_is_ $t>](i); prime_count += 1];
            }
            prime_count
        }

        /// Counts the number of integers $<|n|$ that are relatively prime to `n`.
        ///
        /// Note: If `n` is negative, this function treats it as its absolute
        /// value. For example, a value of `-3` will be treated as `3`.
        ///
        /// # Algorithm
        /// This function iterates through all numbers from 2 up to the square
        /// root of $|n|$. If it finds a divisor, it reduces `n` by its factors
        /// and adjusts result accordingly. If after the loop, $n > 1$, it
        /// means `n` has a prime factor greater than its square root, and the
        /// function adjusts result for this last factor.
        ///
        /// $$ \varphi(n) =n \prod_{p\mid |n|} \left(1-\frac{1}{p}\right) $$
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::totient_" $t ";\n\n"]
        #[doc = "assert_eq![2, totient_" $t "(4)];"]
        #[doc = "assert_eq![6, totient_" $t "(9)];"]
        #[doc = "assert_eq![12, totient_" $t "(13)];"]
        #[doc = "assert_eq![22, totient_" $t "(-23)];"]
        #[doc = "assert_eq![2, totient_" $t "(-3)];"]
        /// ```
        /// # Links
        /// - <https://en.wikipedia.org/wiki/Euler%27s_totient_function>.
        #[must_use]
        #[inline]
        pub fn [<totient_ $t>](n: $t) -> $t {
            let (mut n, mut result, mut i) = (n.abs(), n.abs(), 2);
            while i * i <= n {
                if n % i == 0 {
                    while n % i == 0 { n /= i; }
                    result -= result / i;
                }
                i += 1;
            }
            iif![n > 1; result -= result / n];
            result
        }
    }};

    // implements unsigned ops
    (@unsigned($t:ty, $up:ty, $ft:ty) ) => { paste! {
        /// Returns `true` if `n` is prime.
        ///
        /// This approach uses optimized trial division, which means it checks
        /// only odd numbers starting from 3 and up to the square root of the
        /// given number. This is based on the fact that if a number is
        /// divisible by a number larger than its square root, the result of the
        /// division will be smaller than the square root, and it would have
        /// already been checked in previous iterations.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::prime_is_" $t ";\n\n"]
        #[doc = "assert![prime_is_" $t "(127)];"]
        #[doc = "assert![prime_is_" $t "(2)];"]
        #[doc = "assert![!prime_is_" $t "(1)];"]
        /// ```
        #[must_use]
        #[inline]
        pub fn [<prime_is_ $t>](number: $t) -> bool {
            match number {
                ..=1 => false,
                2..=3 => true,
                _ => {
                    iif![number % 2 == 0; return false];
                    let limit = (number as $ft).sqrt_fisr() as $t;
                    for i in (3..=limit).step_by(2) {
                        iif![number % i == 0; return false];
                    }
                    true
                }
            }
        }

        /// Finds the 0-indexed `nth` prime number.
        ///
        #[doc = "Returns [`MathError::Overflow`] if the result would not fit in a `" $t "`."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::prime_nth_" $t ";\n\n"]
        #[doc = "assert_eq![Ok(2), prime_nth_" $t "(0)];"]
        #[doc = "assert_eq![Ok(3), prime_nth_" $t "(1)];"]
        #[doc = "assert_eq![Ok(251), prime_nth_" $t "(53)];"]
        /// assert![devela::math::prime_nth_u8(253).is_err()];
        /// ```
        #[must_use]
        #[inline]
        pub fn [<prime_nth_ $t>](nth: $t) -> Result<$t> {
            let (mut count, mut i) = (1, 2);
            loop {
                if [<prime_is_ $t>](i) {
                    iif![count - 1 == nth; return Ok(i)];
                    count += 1;
                }
                i = i.checked_add(1).ok_or(MathError::Overflow)?;
            }
        }

        /// Counts the number of integers $<n$ that are relatively prime to `n`.
        ///
        /// This function iterates through all numbers from 2 up to the square
        /// root of `n`. If it finds a divisor, it reduces `n` by its factors
        /// and adjusts result accordingly. If after the loop, $n > 1$, it
        /// means `n` has a prime factor greater than its square root, and the
        /// function adjusts result for this last factor.
        ///
        /// $$ \varphi(n) =n \prod_{p\mid n} \left(1-\frac{1}{p}\right) $$
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::totient_" $t ";\n\n"]
        #[doc = "assert_eq![2, totient_" $t "(4)];"]
        #[doc = "assert_eq![6, totient_" $t "(9)];"]
        #[doc = "assert_eq![12, totient_" $t "(13)];"]
        /// ```
        /// # Links
        /// - <https://en.wikipedia.org/wiki/Euler%27s_totient_function>.
        #[must_use]
        #[inline]
        pub fn [<totient_ $t>](mut n: $t) -> $t {
            let (mut result, mut i) = (n, 2);
            while i * i <= n {
                if n % i == 0 {
                    while n % i == 0 { n /= i; }
                    result -= result / i;
                }
                i += 1;
            }
            iif![n > 1; result -= result / n];
            result
        }

        /// Counts the number of primes upto and including `n`.
        ///
        /// # Notation
        /// $$\pi(x)$$
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::prime_pi_" $t ";\n\n"]
        #[doc = "assert_eq![1, prime_pi_" $t "(2)];"]
        #[doc = "assert_eq![2, prime_pi_" $t "(3)];"]
        #[doc = "assert_eq![54, prime_pi_" $t "(251)];"]
        #[doc = "assert_eq![0, prime_pi_" $t "(1)];"]
        /// ```
        /// # Links
        /// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
        /// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
        #[must_use]
        #[inline]
        pub fn [<prime_pi_ $t>](n: $t) -> usize {
            let mut prime_count = 0;
            for i in 0..=n {
                iif![[<prime_is_ $t>](i); prime_count += 1];
            }
            prime_count
        }
    }};
}

impl_ops![
    signed(i8, i16, f32),
    (i16, i32, f32),
    (i32, i64, f32),
    (i64, i128, f64),
    (i128, i128, f64),
    (isize, isize, fsize)
];
impl_ops![
    unsigned(u8, u16, f32),
    (u16, u32, f32),
    (u32, u64, f32),
    (u64, u128, f64),
    (u128, u128, f64),
    (usize, usize, fsize)
];
