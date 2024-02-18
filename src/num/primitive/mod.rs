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
pub struct Primiting<T>(pub T);

mod core_impls {
    use super::Primiting;
    use crate::code::ConstDefault;
    use core::{cmp, fmt};

    impl<T: Clone> Clone for Primiting<T> {
        #[inline]
        #[must_use]
        fn clone(&self) -> Self {
            Primiting(self.0.clone())
        }
    }
    impl<T: Copy> Copy for Primiting<T> {}

    impl<T: Default> Default for Primiting<T> {
        #[inline]
        #[must_use]
        fn default() -> Self {
            Primiting(T::default())
        }
    }

    impl<T: ConstDefault> ConstDefault for Primiting<T> {
        const DEFAULT: Self = Primiting(T::DEFAULT);
    }

    impl<T: PartialEq> PartialEq for Primiting<T> {
        #[inline]
        #[must_use]
        #[allow(clippy::unconditional_recursion)] // NIGHTLY FIX
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<T: Eq> Eq for Primiting<T> {}

    impl<T: PartialOrd> PartialOrd for Primiting<T> {
        #[inline]
        #[must_use]
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: Ord> Ord for Primiting<T> {
        #[inline]
        #[must_use]
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: fmt::Debug> fmt::Debug for Primiting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl<T: fmt::Display> fmt::Display for Primiting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }
}
