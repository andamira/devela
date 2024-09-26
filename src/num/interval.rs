// devela::code::interval
//
//!

#![expect(unused_imports)]

use crate::code::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// A generic interval type.
///
/// It tries to follow rust's nomenclature,
/// extends the standard range types to those with excluded left bounds.
///
/// It can serve several purposes like:
/// - A compile-time friendly version of the range types.
/// - A [mathematical interval](#aliases-with-mathematica-terminology)
///   for different kinds of numbers.
#[derive(Clone, Copy, Debug)]
pub enum Interval<T> {
    /* from-inclusive (left-closed) */
    /// From inclusive to inclusive $[a, b]$ `start..=end`, [`RangeInclusive`].
    Inclusive(T, T),
    /// From inclusive to unbounded $[a, ∞)$ `start..` [`RangeFrom`].
    FromInclusive(T),
    /// From inclusive to exclusive $[a, b)$ `start..end` [`Range`].
    FromInclusiveToExclusive(T, T),

    /* from-unbounded (left-unbounded) */
    /// From unbounded to unbonded $(-∞, ∞)$ `..` [`RangeFull`].
    Full,
    /// From unbounded to exclusive $(-∞, b)$ `..end` [`RangeTo`].
    ToExclusive(T),
    /// From unbounded to inclusive $(-∞, b]$ `..=end` [`RangeToInclusive`].
    ToInclusive(T),

    /* from-exclusive (left-open) */
    /// From exclusive to exclusive $(a, b)$.
    Exclusive(T, T),
    /// From exclusive to unbounded $(a, ∞)$.
    FromExclusive(T),
    /// From exclusive to inclusive $(a, b]$.
    FromExclusiveToInclusive(T, T),
}

impl<T: Copy> Interval<T> {
    /// Returns an interval from a pair of bounds.
    #[inline]
    #[must_use]
    // TODO: return Result
    pub const fn from_bounds(from: Bound<T>, to: Bound<T>) -> Self {
        match (from, to) {
            // from included
            (Bound::Included(f), Bound::Included(t)) => Self::Inclusive(f, t),
            (Bound::Included(f), Bound::Unbounded) => Self::FromInclusive(f),
            (Bound::Included(f), Bound::Excluded(t)) => Self::FromInclusiveToExclusive(f, t),
            // from included
            (Bound::Unbounded, Bound::Unbounded) => Self::Full,
            (Bound::Unbounded, Bound::Excluded(t)) => Self::ToExclusive(t),
            (Bound::Unbounded, Bound::Included(t)) => Self::ToInclusive(t),
            // from included
            (Bound::Excluded(f), Bound::Excluded(t)) => Self::Exclusive(f, t),
            (Bound::Excluded(f), Bound::Unbounded) => Self::FromExclusive(f),
            (Bound::Excluded(f), Bound::Included(t)) => Self::FromExclusiveToInclusive(f, t),
        }
    }

    /// Returns both `Bound`s as a tuple `(start, end)`.
    ///
    /// # Example
    /// ```
    /// # use devela::Interval;
    /// let r = Interval::from(1..3usize);
    /// assert_eq!("bc", &"abcd"[r.bounds()]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn bounds(self) -> (Bound<T>, Bound<T>) {
        (self.start_bound(), self.end_bound())
    }

    /// Returns the start `Bound`.
    #[inline]
    #[must_use]
    pub const fn start_bound(self) -> Bound<T> {
        match self {
            // from included
            Self::Inclusive(from, _) => Bound::Included(from),
            Self::FromInclusiveToExclusive(from, _) => Bound::Included(from),
            Self::FromInclusive(from) => Bound::Included(from),
            // from unbounded
            Self::Full => Bound::Unbounded,
            Self::ToExclusive(_) => Bound::Unbounded,
            Self::ToInclusive(_) => Bound::Unbounded,
            // from excluded
            Self::Exclusive(from, _) => Bound::Excluded(from),
            Self::FromExclusiveToInclusive(from, _) => Bound::Excluded(from),
            Self::FromExclusive(from) => Bound::Excluded(from),
        }
    }

    /// Returns the end `Bound`.
    #[inline]
    #[must_use]
    pub const fn end_bound(self) -> Bound<T> {
        match self {
            // from included
            Self::Inclusive(_, to) => Bound::Included(to),
            Self::FromInclusiveToExclusive(_, to) => Bound::Excluded(to),
            Self::FromInclusive(_) => Bound::Unbounded,
            // from unbounded
            Self::Full => Bound::Unbounded,
            Self::ToExclusive(to) => Bound::Excluded(to),
            Self::ToInclusive(to) => Bound::Included(to),
            // from excluded
            Self::Exclusive(_, to) => Bound::Excluded(to),
            Self::FromExclusiveToInclusive(_, to) => Bound::Included(to),
            Self::FromExclusive(_) => Bound::Unbounded,
        }
    }
}
impl<T> Interval<T> {
    /// Returns a reference to both `Bound`s.
    #[must_use]
    pub fn bounds_ref(&self) -> (Bound<&T>, Bound<&T>) {
        (self.start_bound_ref(), self.end_bound_ref())
    }

    /// Returns a reference to the start `Bound`.
    #[must_use]
    pub fn start_bound_ref(&self) -> Bound<&T> {
        match self {
            // from included
            Self::Inclusive(from, _) => Bound::Included(from),
            Self::FromInclusiveToExclusive(from, _) => Bound::Included(from),
            Self::FromInclusive(from) => Bound::Included(from),
            // from unbounded
            Self::Full => Bound::Unbounded,
            Self::ToExclusive(_) => Bound::Unbounded,
            Self::ToInclusive(_) => Bound::Unbounded,
            // from excluded
            Self::Exclusive(from, _) => Bound::Excluded(from),
            Self::FromExclusiveToInclusive(from, _) => Bound::Excluded(from),
            Self::FromExclusive(from) => Bound::Excluded(from),
        }
    }
    /// Returns a reference to the end `Bound`.
    #[must_use]
    pub fn end_bound_ref(&self) -> Bound<&T> {
        match self {
            // from included
            Self::Inclusive(_, to) => Bound::Included(to),
            Self::FromInclusiveToExclusive(_, to) => Bound::Excluded(to),
            Self::FromInclusive(_) => Bound::Unbounded,
            // from unbounded
            Self::Full => Bound::Unbounded,
            Self::ToExclusive(to) => Bound::Excluded(to),
            Self::ToInclusive(to) => Bound::Included(to),
            // from excluded
            Self::Exclusive(_, to) => Bound::Excluded(to),
            Self::FromExclusiveToInclusive(_, to) => Bound::Included(to),
            Self::FromExclusive(_) => Bound::Unbounded,
        }
    }
}

#[rustfmt::skip]
mod conversions {
    use super::*;

    /* from-included (left-closed) */

    impl<T> From<RangeInclusive<T>> for Interval<T> {
        fn from(r: RangeInclusive<T>) -> Self {
            let (start, end) = r.into_inner();
            Self::Inclusive(start, end)
        }
    }
    impl<T> From<RangeFrom<T>> for Interval<T> {
        fn from(r: RangeFrom<T>) -> Self { Self::FromInclusive(r.start) }
    }
    impl<T> From<Range<T>> for Interval<T> {
        fn from(r: Range<T>) -> Self { Self::FromInclusiveToExclusive(r.start, r.end) }
    }

    /* from-unbounded (left-unbounded) */

    impl<T> From<RangeFull> for Interval<T> {
        fn from(_: RangeFull) -> Self { Self::Full }
    }
    impl<T> From<RangeTo<T>> for Interval<T> {
        fn from(r: RangeTo<T>) -> Self { Self::ToExclusive(r.end) }
    }
    impl<T> From<RangeToInclusive<T>> for Interval<T> {
        fn from(r: RangeToInclusive<T>) -> Self { Self::ToInclusive(r.end) }
    }

    // TODO: TryFrom
    // impl<T> TryFrom<Interval> for RangeFull {}
}

/// # Mathematical nomenclature.
///
/// Note that the aliases with parameters can't be used for matching.
#[allow(non_snake_case, non_upper_case_globals)]
#[rustfmt::skip]
impl<T> Interval<T> {
    /* left-closed */

    /// Alias of [`Inclusive`][Self::Inclusive].
    pub const fn Closed(from: T, to: T) -> Self { Self::Inclusive(from, to) }
    /// Alias of [`Inclusive`][Self::Inclusive].
    #[inline]
    pub const fn LeftClosedRightClosed(from: T, to: T) -> Self { Self::Inclusive(from, to) }

    /// Alias of [`FromInclusiveToExclusive`][Self::FromInclusiveToExclusive].
    #[inline]
    pub const fn LeftClosedRightOpen(from: T, to: T) -> Self {
        Self::FromInclusiveToExclusive(from, to) }

    /// Alias of [`FromInclusive`][Self::FromInclusive].
    #[inline]
    pub const fn LeftClosed(from: T) -> Self { Self::FromInclusive(from) }
    /// Alias of [`FromInclusive`][Self::FromInclusive].
    #[inline]
    pub const fn LeftClosedRightUnbounded(from: T) -> Self { Self::FromInclusive(from) }

    /* left-unbounded */

    /// Alias of [`Full`][Self::Full]
    pub const Unbounded: Self = Self::Full;
    /// Alias of [`Full`][Self::Full]
    pub const LeftUnboundedRightUnbounded: Self = Self::Full;

    /// Alias of [`ToExclusive`][Self::ToExclusive].
    #[inline]
    pub const fn RightClosed(to: T) -> Self { Self::ToExclusive(to) }
    /// Alias of [`ToExclusive`][Self::ToExclusive].
    #[inline]
    pub const fn LeftUnboundedRightClosed(to: T) -> Self { Self::ToExclusive(to) }

    /// Alias of [`ToInclusive`][Self::ToInclusive].
    #[inline]
    pub const fn RightOpen(to: T) -> Self { Self::ToInclusive(to) }
    /// Alias of [`ToInclusive`][Self::ToInclusive].
    #[inline]
    pub const fn LeftUnboundedRightOpen(to: T) -> Self { Self::ToInclusive(to) }

    /* left-open */

    /// Alias of [`Exclusive`][Self::Exclusive].
    #[inline]
    pub const fn Open(from: T, to: T) -> Self { Self::Exclusive(from, to) }
    /// Alias of [`Exclusive`][Self::Exclusive].
    #[inline]
    pub const fn LeftOpenRightOpen(from: T, to: T) -> Self { Self::Exclusive(from, to) }

    /// Alias of [`FromExclusiveToInclusive`][Self::FromExclusiveToInclusive].
    #[inline]
    pub const fn LeftOpenRightClosed(from: T, to: T) -> Self {
        Self::FromExclusiveToInclusive(from, to) }

    /// Alias of [`FromExclusive`][Self::FromExclusive].
    #[inline]
    pub const fn LeftOpen(from: T) -> Self { Self::FromExclusive(from) }
    /// Alias of [`FromExclusive`][Self::FromExclusive].
    #[inline]
    pub const fn LeftOpenRightUnbounded(from: T) -> Self { Self::FromExclusive(from) }
}
