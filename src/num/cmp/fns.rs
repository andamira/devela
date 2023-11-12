// devela::num::cmp::fns
//
//! comparison standalone functions.
//

/// Compares and returns a [`PartialOrd`]ered `value` clamped between `min` and `max`.
///
/// # Examples
/// ```
/// use devela::num::pclamp;
///
/// assert_eq![0.4, pclamp(1.0, 0.2, 0.4)];
/// assert_eq![0.2, pclamp(0.0, 0.2, 0.4)];
/// ```
#[inline]
#[must_use]
#[rustfmt::skip]
pub fn pclamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    pmin(pmax(value, min), max)
}

/// Compares and returns the maximum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`max`][`core::cmp::max] which requires
/// [`Ord`]
///
/// # Examples
/// ```
/// use devela::num::pmax;
///
/// assert_eq![0.4, pmax(0.2, 0.4)];
/// ```
#[inline]
#[must_use]
#[rustfmt::skip]
pub fn pmax<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }

/// Compares and returns the minimum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`min`][`core::cmp::min] which requires
/// [`Ord`]
///
/// # Example
/// ```
/// use devela::num::pmin;
///
/// assert_eq![0.2, pmin(0.2, 0.4)];
/// ```
#[inline]
#[must_use]
#[rustfmt::skip]
pub fn pmin<T: PartialOrd>(a: T, b: T) -> T { if a < b { a } else { b } }

#[cfg(test)]
mod test_min_max_clamp {
    use super::{pclamp, pmax, pmin};

    #[test]
    fn min_max_clamp() {
        assert_eq![2, pmin(2, 5)];
        assert_eq![2, pmin(5, 2)];
        assert_eq![2., pmin(2., 5.)];

        assert_eq![5, pmax(2, 5)];
        assert_eq![5, pmax(5, 2)];
        assert_eq![5., pmax(2., 5.)];

        assert_eq![3, pclamp(3, 2, 5)];
        assert_eq![3., pclamp(3., 2., 5.)];
        assert_eq![2, pclamp(1, 2, 5)];
        assert_eq![5, pclamp(7, 2, 5)];
    }
}