// devela::code::interval
//
//!
//

use crate::{iif, Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Represents an interval with a `lower` and an `upper` bound.
///
/// The `Interval` type allows modeling ranges of values with optional inclusion
/// or exclusion at each bound. This is useful for mathematical operations,
/// range checks, and interval arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Interval<T> {
    /// The lower bound (also known as the *start* bound, or the *left* bound).
    pub lower: Bound<T>,
    /// The upper bound (also known as the *end*, bound or the *right* bound).
    pub upper: Bound<T>,
}

/// # Constructors
impl<T> Interval<T> {
    /// Creates a new interval with the given `lower` and `upper` bounds.
    #[must_use]
    pub const fn new(lower: Bound<T>, upper: Bound<T>) -> Self {
        Self { lower, upper }
    }

    // lower-closed

    /// Creates a closed interval $[l, u]$ `lower..=upper` [`RangeInclusive`].
    #[must_use]
    pub const fn closed(lower: T, upper: T) -> Self {
        Self::new(Bound::Included(lower), Bound::Included(upper))
    }
    /// Creates a half-open interval $[l, u)$ `lower..upper` [`Range`].
    #[must_use]
    pub const fn closed_open(lower: T, upper: T) -> Self {
        Self::new(Bound::Included(lower), Bound::Excluded(upper))
    }
    /// Creates an interval $[l, ∞)$ `lower..` [`RangeFrom`].
    #[must_use]
    pub const fn closed_unbounded(lower: T) -> Self {
        Self::new(Bound::Included(lower), Bound::Unbounded)
    }

    // lower-unbounded

    /// Creates an unbounded interval $(-∞, ∞)$ `..` [`RangeFull`].
    #[must_use]
    pub const fn unbounded() -> Self {
        Self::new(Bound::Unbounded, Bound::Unbounded)
    }
    /// Creates an interval $(-∞, u]$ `..upper` [`RangeTo`].
    #[must_use]
    pub const fn unbounded_closed(upper: T) -> Self {
        Self::new(Bound::Unbounded, Bound::Included(upper))
    }
    /// Creates an interval $(-∞, u)$ `..=upper` [`RangeToInclusive`].
    #[must_use]
    pub const fn unbounded_open(upper: T) -> Self {
        Self::new(Bound::Unbounded, Bound::Excluded(upper))
    }

    // lower-open

    /// Creates an open interval $(l, u)$.
    #[must_use]
    pub const fn open(lower: T, upper: T) -> Self {
        Self::new(Bound::Excluded(lower), Bound::Excluded(upper))
    }

    /// Creates a half-open interval $(a, b]$.
    #[must_use]
    pub const fn open_closed(lower: T, upper: T) -> Self {
        Self::new(Bound::Excluded(lower), Bound::Included(upper))
    }

    /// Creates an interval $(l, ∞)$.
    #[must_use]
    pub const fn open_unbounded(lower: T) -> Self {
        Self::new(Bound::Excluded(lower), Bound::Unbounded)
    }
}

impl<T: Copy> Interval<T> {
    /// Returns a copy of both bounds as a tuple `(lower, upper)`.
    ///
    /// # Example
    /// ```
    /// # use devela::Interval;
    /// let r = Interval::from(1..3usize);
    /// assert_eq!("bc", &"abcd"[r.to_tuple()]);
    /// ```
    #[must_use]
    pub const fn to_tuple(self) -> (Bound<T>, Bound<T>) {
        (self.lower, self.upper)
    }
}

impl<T> Interval<T> {
    /// Returns both bounds as a tuple `(lower, upper)`.
    #[must_use]
    pub fn into_tuple(self) -> (Bound<T>, Bound<T>) {
        (self.lower, self.upper)
    }

    /// Returns a reference to both bounds as a tuple `(&lower, &upper)`.
    #[must_use]
    pub fn to_tuple_ref(&self) -> (Bound<&T>, Bound<&T>) {
        (self.lower_ref(), self.upper_ref())
    }

    /// Returns a reference to the lower bound.
    #[must_use]
    pub fn lower_ref(&self) -> Bound<&T> {
        self.lower.as_ref()
    }

    /// Returns a reference to the upper bound.
    #[must_use]
    pub fn upper_ref(&self) -> Bound<&T> {
        self.upper.as_ref()
    }

    /// Checks if the interval is both lower and upper bounded.
    #[must_use]
    pub const fn is_bounded(&self) -> bool {
        self.is_lower_bounded() && self.is_upper_bounded()
    }
    /// Checks if the lower bound is bounded.
    #[must_use]
    pub const fn is_lower_bounded(&self) -> bool {
        !matches!(self.lower, Bound::Unbounded)
    }
    /// Checks if the upper bound is bounded.
    #[must_use]
    pub const fn is_upper_bounded(&self) -> bool {
        !matches!(self.upper, Bound::Unbounded)
    }
    /// Checks if the lower bound is open (excluded).
    #[must_use]
    pub const fn is_lower_open(&self) -> bool {
        matches!(self.lower, Bound::Excluded(_))
    }
    /// Checks if the lower bound is closed (included).
    #[must_use]
    pub const fn is_lower_closed(&self) -> bool {
        matches!(self.lower, Bound::Included(_))
    }
    /// Checks if the upper bound is open (excluded).
    #[must_use]
    pub const fn is_upper_open(&self) -> bool {
        matches!(self.upper, Bound::Excluded(_))
    }
    /// Checks if the upper bound is closed (included).
    #[must_use]
    pub const fn is_upper_closed(&self) -> bool {
        matches!(self.upper, Bound::Included(_))
    }
}

impl<T: PartialOrd> Interval<T> {
    /// Validates that the interval bounds are ordered correctly.
    ///
    /// Returns `true` if the lower bound is less than or equal to the upper bound.
    /// Unbounded intervals are always considered valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        match (&self.lower, &self.upper) {
            (Bound::Unbounded, _) | (_, Bound::Unbounded) => true,
            (Bound::Included(a), Bound::Included(b)) => a <= b,
            (Bound::Included(a), Bound::Excluded(b)) => a < b,
            (Bound::Excluded(a), Bound::Included(b)) => a < b,
            (Bound::Excluded(a), Bound::Excluded(b)) => a < b,
        }
    }

    /// Checks if the interval contains the given value.
    ///
    /// ```
    /// # use devela::Interval;
    /// let interval = Interval::closed(1, 5);
    /// assert!(interval.contains(&3));
    /// assert!(!interval.contains(&6));
    /// ```
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        let lower_check = match &self.lower {
            Bound::Included(ref lower) => *lower <= *value,
            Bound::Excluded(ref lower) => *lower < *value,
            Bound::Unbounded => true,
        };
        let upper_check = match &self.upper {
            Bound::Included(ref upper) => *value <= *upper,
            Bound::Excluded(ref upper) => *value < *upper,
            Bound::Unbounded => true,
        };
        lower_check && upper_check
    }

    /// Returns the size of the interval, if finite.
    #[must_use]
    pub fn size(&self) -> Option<T>
    where
        T: Clone + core::ops::Sub<Output = T>,
    {
        match (&self.lower, &self.upper) {
            (Bound::Included(a), Bound::Included(b)) => {
                iif![a <= b; Some(b.clone() - a.clone()); None]
            }
            (Bound::Included(a), Bound::Excluded(b)) => {
                iif![a < b; Some(b.clone() - a.clone()); None]
            }
            (Bound::Excluded(a), Bound::Included(b)) => {
                iif![a < b; Some(b.clone() - a.clone()); None]
            }
            (Bound::Excluded(a), Bound::Excluded(b)) => {
                iif![a < b; Some(b.clone() - a.clone()); None]
            }
            _ => None, // Unbounded intervals don't have a finite size
        }
    }
}

#[rustfmt::skip]
mod impl_traits {
    use super::*;
    use crate::{NumError, NumError::IncompatibleBounds, NumResult, Ordering, RangeBounds};

    /* infallible conversions */

    // lower-closed
    impl<T> From<RangeInclusive<T>> for Interval<T> {
        #[must_use]
        fn from(r: RangeInclusive<T>) -> Self {
            let (start, end) = r.into_inner();
            Self::closed(start, end)
        }
    }
    impl<T> From<Range<T>> for Interval<T> {
        #[must_use]
        fn from(r: Range<T>) -> Self { Self::closed_open(r.start, r.end) }
    }
    impl<T> From<RangeFrom<T>> for Interval<T> {
        #[must_use]
        fn from(r: RangeFrom<T>) -> Self { Self::closed_unbounded(r.start) }
    }
    // lower-unbounded
    impl<T> From<RangeFull> for Interval<T> {
        #[must_use]
        fn from(_: RangeFull) -> Self { Self::unbounded() }
    }
    impl<T> From<RangeTo<T>> for Interval<T> {
        #[must_use]
        fn from(r: RangeTo<T>) -> Self { Self::unbounded_closed(r.end) }
    }
    impl<T> From<RangeToInclusive<T>> for Interval<T> {
        #[must_use]
        fn from(r: RangeToInclusive<T>) -> Self { Self::unbounded_open(r.end) }
    }

    /* fallible conversions */

    // lower-closed
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeInclusive<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Included(start), Bound::Included(end)) => Ok(start..=end),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for Range<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Included(start), Bound::Excluded(end)) => Ok(start..end),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeFrom<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Included(start), Bound::Unbounded) => Ok(start..),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    // lower-unbounded
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeFull {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Unbounded, Bound::Unbounded) => Ok(..),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeTo<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Unbounded, Bound::Excluded(end)) => Ok(..end),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeToInclusive<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Unbounded, Bound::Included(end)) => Ok(..=end),
                _ => Err(IncompatibleBounds),
            }
        }
    }

    /* other traits */

    impl<T> RangeBounds<T> for Interval<T> {
        fn start_bound(&self) -> Bound<&T> {
            self.lower_ref()
        }
        fn end_bound(&self) -> Bound<&T> {
            self.upper_ref()
        }
    }

    #[cfg(feature = "alloc")]
    crate::items! {
        use crate::{format, String};
        use core::fmt;

        impl<T: fmt::Display> fmt::Display for Interval<T> {
            /// Formats the interval as a human-readable string.
            ///
            /// Examples:
            /// - `(-∞, 5]` for an unbounded lower bound and inclusive upper bound.
            /// - `[1, 3)` for a closed lower bound and open upper bound.
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let lower = match &self.lower {
                    Bound::Included(value) => format!("[{}", value),
                    Bound::Excluded(value) => format!("({}", value),
                    Bound::Unbounded => String::from("(-∞"),
                };
                let upper = match &self.upper {
                    Bound::Included(value) => format!("{}, {}]", lower, value),
                    Bound::Excluded(value) => format!("{}, {})", lower, value),
                    Bound::Unbounded => format!("{}, ∞)", lower),
                };
                write!(f, "{}", upper)
            }
        }
    }

    /// Comparison Logic:
    /// - We compare the lower bounds first.
    /// - If the lower bounds are equal, we compare the upper bounds.
    /// - We define Unbounded as less than any bounded value.
    /// - We define that Included(a) < Excluded(a) at same point a.
    impl<T: PartialOrd> PartialOrd for Interval<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match compare_bounds(&self.lower, &other.lower) {
                Some(Ordering::Equal) => compare_bounds(&self.upper, &other.upper),
                ord => ord,
            }
        }
    }
    /// Comparison Logic:
    /// - We compare the lower bounds first.
    /// - If the lower bounds are equal, we compare the upper bounds.
    /// - We define Unbounded as less than any bounded value.
    /// - We define that Included(a) < Excluded(a) at same point a.
    impl<T: Ord> Ord for Interval<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            match compare_bounds_ord(&self.lower, &other.lower) {
                Ordering::Equal => compare_bounds_ord(&self.upper, &other.upper),
                ord => ord,
            }
        }
    }

    /* helpers */

    fn compare_bounds<T: PartialOrd>(a: &Bound<T>, b: &Bound<T>) -> Option<Ordering> {
        use Bound::{Excluded, Included, Unbounded};
        match (a, b) {
            (Unbounded, Unbounded) => Some(Ordering::Equal),
            (Unbounded, _) => Some(Ordering::Less),
            (_, Unbounded) => Some(Ordering::Greater),
            (Included(ref a_val), Included(ref b_val)) => a_val.partial_cmp(b_val),
            (Excluded(ref a_val), Excluded(ref b_val)) => a_val.partial_cmp(b_val),
            (Included(ref a_val), Excluded(ref b_val)) => {
                match a_val.partial_cmp(b_val) {
                    Some(Ordering::Equal) => Some(Ordering::Less),
                    ord => ord,
                }
            }
            (Excluded(ref a_val), Included(ref b_val)) => {
                match a_val.partial_cmp(b_val) {
                    Some(Ordering::Equal) => Some(Ordering::Greater),
                    ord => ord,
                }
            }
        }
    }
    fn compare_bounds_ord<T: Ord>(a: &Bound<T>, b: &Bound<T>) -> Ordering {
        use Bound::{Excluded, Included, Unbounded};
        match (a, b) {
            (Unbounded, Unbounded) => Ordering::Equal,
            (Unbounded, _) => Ordering::Less,
            (_, Unbounded) => Ordering::Greater,
            (Included(ref a_val), Included(ref b_val)) => a_val.cmp(b_val),
            (Excluded(ref a_val), Excluded(ref b_val)) => a_val.cmp(b_val),
            (Included(ref a_val), Excluded(ref b_val)) => {
                match a_val.cmp(b_val) {
                    Ordering::Equal => Ordering::Less,
                    ord => ord,
                }
            }
            (Excluded(ref a_val), Included(ref b_val)) => {
                match a_val.cmp(b_val) {
                    Ordering::Equal => Ordering::Greater,
                    ord => ord,
                }
            }
        }
    }
}
