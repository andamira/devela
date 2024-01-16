// devela::num::ops::factors
//
//!
//

#[cfg(doc)]
use crate::num::NumErrors::MismatchedSizes;
use crate::num::{Int, NumResult as Result};

#[cfg(feature = "alloc")]
use ::_alloc::vec::Vec;

/// Numeric factors-related functionality.
///
/// The following methods are only available if the `alloc` feature is enabled,
/// and they all return a `Vec`:
/// - [`factors`][Self::factors]
/// - [`factors_proper`][Self::factors_proper]
/// - [`factors_prime`][Self::factors_prime]
/// - [`factors_prime_unique`][Self::factors_prime_unique]
///
/// There following methods doesn't depend on `alloc`, instead writing to buffers.
/// - [`factors_buf`][Self::factors_buf]
/// - [`factors_proper_buf`][Self::factors_proper_buf]
/// - [`factors_prime_buf`][Self::factors_prime_buf]
/// - [`factors_prime_unique_buf`][Self::factors_prime_unique_buf]
///
/// These methods are also implemented in the [`Int`][crate::num::Int] wrapper.
pub trait NumOpsFactors: Sized {
    /* allocating */

    /// Returns the factors (including 1 and self).
    /// # Examples
    /// ```
    /// # use devela::num::NumOpsFactors;
    /// assert_eq![24_i32.factors(), vec![1, 2, 3, 4, 6, 8, 12, 24]];
    /// assert_eq![(-24_i32).factors(), vec![1, 2, 3, 4, 6, 8, 12, 24]];
    /// assert_eq![0_i32.factors(), vec![]];
    /// assert_eq![1_i32.factors(), vec![1]];
    /// assert_eq![7_i32.factors(), vec![1, 7]];
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    fn factors(&self) -> Vec<Self>;

    /// Returns the proper factors.
    /// # Examples
    /// ```
    /// # use devela::num::NumOpsFactors;
    /// assert_eq![24_i32.factors_proper(), vec![2, 3, 4, 6, 8, 12]];
    /// assert_eq![(-24_i32).factors_proper(), vec![2, 3, 4, 6, 8, 12]];
    /// assert_eq![0_i32.factors_proper(), vec![]];
    /// assert_eq![1_i32.factors_proper(), vec![]];
    /// assert_eq![7_i32.factors_proper(), vec![]];
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    fn factors_proper(&self) -> Vec<Self>;

    /// Returns the prime factors.
    /// # Examples
    /// ```
    /// # use devela::num::NumOpsFactors;
    /// assert_eq![24_i32.factors_prime(), vec![2, 2, 2, 3]];
    /// assert_eq![(-24_i32).factors_prime(), vec![2, 2, 2, 3]];
    /// assert_eq![0_i32.factors_prime(), vec![]];
    /// assert_eq![1_i32.factors_prime(), vec![]];
    /// assert_eq![7_i32.factors_prime(), vec![7]];
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    fn factors_prime(&self) -> Vec<Self>;

    /// Returns the unique prime factors.
    /// # Examples
    /// ```
    /// # use devela::num::NumOpsFactors;
    /// assert_eq![24_i32.factors_prime_unique(), vec![2, 3]];
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    fn factors_prime_unique(&self) -> Vec<Self>;

    /* non-allocating */

    /// Writes the factors in `fbuf`, and the unique prime factors in `upfbuf`.
    ///
    /// Returns a tuple with the number of factors and unique prime factors found.
    ///
    /// Or [`MismatchedSizes`] if the total number of factors is greater
    /// than the length of any buffer.
    /// # Examples
    /// ```
    /// # use devela::num::NumOpsFactors;
    /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
    /// assert_eq![24_i32.factors_buf(&mut fbuf, &mut upbuf), Ok((8, 2))];
    ///
    /// assert_eq![fbuf[..8], [1, 2, 3, 4, 6, 8, 12, 24]];
    /// assert_eq![upbuf[..2], [2, 3]];
    /// ```
    fn factors_buf(&self, fbuf: &mut [Self], upfbuf: &mut [Self]) -> Result<(usize, usize)>;

    /// Writes the proper factors in `fbuf`, and the unique prime factors in `upfbuf`.
    ///
    /// Returns a tuple with the number of factors and unique prime factors found.
    ///
    /// Or [`MismatchedSizes`] if the total number of factors is greater
    /// than the length of any buffer.
    ///
    /// # Examples
    /// ```
    /// # use devela::num::NumOpsFactors;
    /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
    /// assert_eq![24_i32.factors_proper_buf(&mut fbuf, &mut upbuf), Ok((6, 2))];
    ///
    /// assert_eq![fbuf[..6], [2, 3, 4, 6, 8, 12,]];
    /// assert_eq![upbuf[..2], [2, 3]];
    /// ```
    fn factors_proper_buf(&self, fbuf: &mut [Self], upfbuf: &mut [Self]) -> Result<(usize, usize)>;

    /// Writes the prime factors in the given `buffer`.
    ///
    /// Returns the number of factors found, or [`MismatchedSizes`] if the total number
    /// of factors is greater than the length of the `buffer`.
    ///
    /// # Examples
    /// ```
    /// # use devela::num::NumOpsFactors;
    /// let mut buf = [0; 5];
    /// assert_eq![24_i32.factors_prime_buf(&mut buf), Ok(4)];
    ///
    /// assert_eq![buf[..4], [2, 2, 2, 3]];
    /// assert![(24_i32 * 4).factors_prime_buf(&mut buf).is_err()];
    /// assert_eq![buf, [2, 2, 2, 2, 2]]; // the 3 didn't fit
    ///
    /// assert_eq![0_i32.factors_prime_buf(&mut buf), Ok(0)];
    /// assert_eq![1_i32.factors_prime_buf(&mut buf), Ok(0)];
    /// assert_eq![7_i32.factors_prime_buf(&mut buf), Ok(1)];
    /// assert_eq![buf[..1], [7]];
    /// ```
    fn factors_prime_buf(&self, buffer: &mut [Self]) -> Result<usize>;

    /// Writes the prime factors in the given `buffer`.
    ///
    /// The buffer must be large enough to hold all the non-unique factors of `n`.
    /// In that case the function will return the number of unique factors found.
    ///
    /// Otherwise it will return [`MismatchedSizes`], and the buffer will only contain the
    /// non-unique factors that could fit, like [`factors_prime_buf`][Self::factors_prime_buf].
    ///
    /// # Examples
    /// ```
    /// # use devela::num::NumOpsFactors;
    /// let mut uniq = [0; 5];
    /// assert_eq![24_i32.factors_prime_unique_buf(&mut uniq), Ok(2)];
    /// assert_eq![uniq, [2, 3, 2, 3, 0]];
    /// ```
    fn factors_prime_unique_buf(&self, buffer: &mut [Self]) -> Result<usize>;
}

macro_rules! impl_base {
    () => { impl_base![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]; };
    ($($t:ty),+) => { $( impl_base![@$t]; )+ };
    (@$t:ty) => {
        impl NumOpsFactors for $t {
            /* allocating */

            #[inline] #[cfg(feature = "alloc")]
            fn factors(&self) -> Vec<Self> { Int(*self).factors() }
            #[inline] #[cfg(feature = "alloc")]
            fn factors_proper(&self) -> Vec<Self> { Int(*self).factors_proper() }
            #[inline] #[cfg(feature = "alloc")]
            fn factors_prime(&self) -> Vec<Self> { Int(*self).factors_prime() }
            #[inline] #[cfg(feature = "alloc")]
            fn factors_prime_unique(&self) -> Vec<Self> { Int(*self).factors_prime_unique() }

            /* non-allocating */

            #[inline]
            fn factors_buf(&self, fbuf: &mut[Self], upbuf: &mut[Self]) -> Result<(usize, usize)> {
                Int(*self).factors_buf(fbuf, upbuf) }
            #[inline]
            fn factors_proper_buf(&self, fbuf: &mut[Self], upbuf: &mut[Self])
                -> Result<(usize, usize)> { Int(*self).factors_proper_buf(fbuf, upbuf) }
            #[inline]
            fn factors_prime_buf(&self, buffer: &mut[Self]) -> Result<usize> {
                Int(*self).factors_prime_buf(buffer) }
            #[inline]
            fn factors_prime_unique_buf(&self, buffer: &mut[Self]) -> Result<usize> {
                Int(*self).factors_prime_unique_buf(buffer) }
        }
    };
}
impl_base![];
