// devela:: ops
//
//! Operators.
//
// TOC
// - pclamp
// - pmax
// - pmin

/// Returns a [`PartialOrd`]ered `value` clamped between `min` and `max`.
///
/// # Examples
/// ```
/// use devela::ops::pclamp;
///
/// assert_eq![0.4, pclamp(1.0, 0.2, 0.4)];
/// assert_eq![0.2, pclamp(0.0, 0.2, 0.4)];
/// ```
#[inline(always)]
#[rustfmt::skip]
pub fn pclamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    pmin(pmax(value, min), max)
}

/// Returns the maximum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`max`][`core::cmp::max] which requires
/// [`Ord`][core::cmp::Ord].
///
/// # Examples
/// ```
/// use devela::ops::pmax;
///
/// assert_eq![0.4, pmax(0.2, 0.4)];
/// ```
#[inline(always)]
#[rustfmt::skip]
pub fn pmax<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }

/// Returns the minimum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`min`][`core::cmp::min] which requires
/// [`Ord`][core::cmp::Ord].
///
/// # Example
/// ```
/// use devela::ops::pmin;
///
/// assert_eq![0.2, pmin(0.2, 0.4)];
/// ```
#[inline(always)]
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
