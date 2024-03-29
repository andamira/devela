// devela::num::primitive
//
//! Helpers for converting between primitives.
//

#[cfg(test)]
mod tests;

mod casts;
mod join;
mod split;

pub use {casts::*, join::*, split::*};

/// Provides *const* casting, joining and splitting operations between primitives.
///
/// See also the related traits: [`PrimitiveCast`], [`PrimitiveJoin`] and [`PrimitiveSplit`].
#[repr(transparent)]
pub struct Cast<T>(pub T);

mod core_impls {
    use super::Cast;
    use crate::code::ConstDefault;
    use core::{cmp, fmt};

    impl<T: Clone> Clone for Cast<T> {
        #[inline]
        #[must_use]
        fn clone(&self) -> Self {
            Cast(self.0.clone())
        }
    }
    impl<T: Copy> Copy for Cast<T> {}

    impl<T: Default> Default for Cast<T> {
        #[inline]
        #[must_use]
        fn default() -> Self {
            Cast(T::default())
        }
    }

    impl<T: ConstDefault> ConstDefault for Cast<T> {
        const DEFAULT: Self = Cast(T::DEFAULT);
    }

    impl<T: PartialEq> PartialEq for Cast<T> {
        #[inline]
        #[must_use]
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<T: Eq> Eq for Cast<T> {}

    impl<T: PartialOrd> PartialOrd for Cast<T> {
        #[inline]
        #[must_use]
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: Ord> Ord for Cast<T> {
        #[inline]
        #[must_use]
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: fmt::Debug> fmt::Debug for Cast<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl<T: fmt::Display> fmt::Display for Cast<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }
}
