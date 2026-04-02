// devela::phys::time::source::point
//
//! Defines [`TimePoint`].
//

use crate::Ordering;

#[doc = crate::_tags!(time)]
/// A comparable representation of a point on a time-like timeline.
#[doc = crate::_doc_location!("phys/time/source")]
///
/// `TimePoint` defines ordering and forward elapsed-difference semantics
/// for a concrete timestamp representation.
///
/// The point type itself does not define the source, origin, or sampling policy.
/// Those belong to the time source that produces or interprets it.
pub trait TimePoint: Copy {
    /// The elapsed-difference type between two time points.
    type Elapsed: Copy;

    /// Compares two time points on the same timeline.
    fn time_cmp(a: Self, b: Self) -> Ordering;

    /// Returns the forward elapsed difference from `earlier` to `later`,
    /// or `None` if it is not valid or not representable.
    ///
    /// This models a forward elapsed difference, not a signed directional difference.
    ///
    /// The most common failure case is `later < earlier`.
    fn time_elapsed_checked(later: Self, earlier: Self) -> Option<Self::Elapsed>;

    /* provided */

    /// Returns the forward elapsed difference from `earlier` to `later`.
    ///
    /// # Panics
    /// May panic if `time_elapsed_checked(later, earlier)` would return `None`.
    fn time_elapsed(later: Self, earlier: Self) -> Self::Elapsed {
        match Self::time_elapsed_checked(later, earlier) {
            Some(delta) => delta,
            None => panic!("invalid forward time delta"),
        }
    }
}

impl TimePoint for u32 {
    type Elapsed = u32;
    fn time_cmp(a: Self, b: Self) -> Ordering {
        a.cmp(&b)
    }
    fn time_elapsed_checked(later: Self, earlier: Self) -> Option<Self::Elapsed> {
        later.checked_sub(earlier)
    }
    fn time_elapsed(later: Self, earlier: Self) -> Self::Elapsed {
        later - earlier
    }
}
impl TimePoint for u64 {
    type Elapsed = u64;
    fn time_cmp(a: Self, b: Self) -> Ordering {
        a.cmp(&b)
    }
    fn time_elapsed_checked(later: Self, earlier: Self) -> Option<Self::Elapsed> {
        later.checked_sub(earlier)
    }
    fn time_elapsed(later: Self, earlier: Self) -> Self::Elapsed {
        later - earlier
    }
}

#[rustfmt::skip]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod impl_std {
    use crate::{Duration, SystemInstant, SystemTime};
    use super::*;

    impl TimePoint for SystemTime {
        type Elapsed = Duration;
        fn time_cmp(a: Self, b: Self) -> Ordering {
            a.cmp(&b)
        }
        fn time_elapsed_checked(later: Self, earlier: Self) -> Option<Self::Elapsed> {
            later.duration_since(earlier).ok()
        }
        fn time_elapsed(later: Self, earlier: Self) -> Self::Elapsed {
            later.duration_since(earlier).expect("backwards system time")
        }
    }
    impl TimePoint for SystemInstant {
        type Elapsed = Duration;
        fn time_cmp(a: Self, b: Self) -> Ordering {
            a.cmp(&b)
        }
        fn time_elapsed_checked(later: Self, earlier: Self) -> Option<Self::Elapsed> {
            later.checked_duration_since(earlier)
        }
        fn time_elapsed(later: Self, earlier: Self) -> Self::Elapsed {
            later.duration_since(earlier)
        }
    }
}
