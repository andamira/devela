// devela::phys::time::source::span
//
//! Defines [`TimeSpan`].
//

#[doc = crate::_tags!(time)]
/// /// A comparable span on a time-like timeline.
#[doc = crate::_doc_location!("phys/time/source")]
///
/// `TimeSpan` defines the basic quantity semantics
/// of an elapsed or interval-like time value.
///
/// Unlike [`TimePoint`], which represents a position on a timeline,
/// `TimeSpan` represents **how much time** lies between positions
/// or is used as a pacing, interval, or accumulation quantity.
///
/// This trait is intentionally small and supports
/// both unsigned and signed span types.
///
/// It does not define a source, origin, or sampling policy.
pub trait TimeSpan: Copy + PartialEq + PartialOrd {
    /* required */

    /// The additive zero span.
    const TIME_ZERO: Self;

    /// Returns the sum of two spans, or `None` if it is not representable.
    fn time_add_checked(self, other: Self) -> Option<Self>;

    /// Returns the difference of two spans, or `None` if it is not representable.
    fn time_sub_checked(self, other: Self) -> Option<Self>;

    /* provided */

    /// Returns whether this span is zero.
    fn time_is_zero(self) -> bool {
        self == Self::TIME_ZERO
    }

    /// Returns the sum of two spans.
    ///
    /// # Panics
    /// May panic if `time_add_checked(self, other)` would return `None`.
    fn time_add(self, other: Self) -> Self {
        match self.time_add_checked(other) {
            Some(sum) => sum,
            None => panic!("invalid time span addition"),
        }
    }

    /// Returns the difference of two spans.
    ///
    /// # Panics
    /// May panic if `time_sub_checked(self, other)` would return `None`.
    fn time_sub(self, other: Self) -> Self {
        match self.time_sub_checked(other) {
            Some(diff) => diff,
            None => panic!("invalid time span subtraction"),
        }
    }
}

#[rustfmt::skip]
mod impls {
    use crate::{Duration, TimeDelta, TimeSpan};
    impl TimeSpan for u32 {
        const TIME_ZERO: Self = 0;
        fn time_add_checked(self, other: Self) -> Option<Self> { self.checked_add(other) }
        fn time_sub_checked(self, other: Self) -> Option<Self> { self.checked_sub(other) }
    }
    impl TimeSpan for u64 {
        const TIME_ZERO: Self = 0;
        fn time_add_checked(self, other: Self) -> Option<Self> { self.checked_add(other) }
        fn time_sub_checked(self, other: Self) -> Option<Self> { self.checked_sub(other) }
    }
    impl TimeSpan for i32 {
        const TIME_ZERO: Self = 0;
        fn time_add_checked(self, other: Self) -> Option<Self> { self.checked_add(other) }
        fn time_sub_checked(self, other: Self) -> Option<Self> { self.checked_sub(other) }
    }
    impl TimeSpan for i64 {
        const TIME_ZERO: Self = 0;
        fn time_add_checked(self, other: Self) -> Option<Self> { self.checked_add(other) }
        fn time_sub_checked(self, other: Self) -> Option<Self> { self.checked_sub(other) }
    }
    impl TimeSpan for Duration {
        const TIME_ZERO: Self = Duration::ZERO;
        fn time_add_checked(self, other: Self) -> Option<Self> { self.checked_add(other) }
        fn time_sub_checked(self, other: Self) -> Option<Self> { self.checked_sub(other) }
    }
    impl TimeSpan for TimeDelta {
        const TIME_ZERO: Self = TimeDelta::ZERO;
        fn time_add_checked(self, other: Self) -> Option<Self> { self.checked_add(other) }
        fn time_sub_checked(self, other: Self) -> Option<Self> { self.checked_sub(other) }
    }
}
