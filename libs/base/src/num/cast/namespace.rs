// devela_base::num::primitive::namespace

#[doc = crate::TAG_NAMESPACE!()]
/// Provides *const* casting, joining and splitting operations between primitives.
//
// See also the related traits: [`PrimitiveCast`], [`PrimitiveJoin`] and [`PrimitiveSplit`].
//
// [`PrimitiveCast`]: crate::PrimitiveCast
// [`PrimitiveJoin`]: crate::PrimitiveJoin
// [`PrimitiveSplit`]: crate::PrimitiveSplit
#[repr(transparent)]
pub struct Cast<T>(pub T);

mod core_impls {
    use crate::{Cast, Ordering};
    use core::fmt;

    impl<T: Clone> Clone for Cast<T> {
        fn clone(&self) -> Self {
            Cast(self.0.clone())
        }
    }
    impl<T: Copy> Copy for Cast<T> {}

    impl<T: Default> Default for Cast<T> {
        fn default() -> Self {
            Cast(T::default())
        }
    }

    impl<T: PartialEq> PartialEq for Cast<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<T: Eq> Eq for Cast<T> {}

    impl<T: PartialOrd> PartialOrd for Cast<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: Ord> Ord for Cast<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: fmt::Debug> fmt::Debug for Cast<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl<T: fmt::Display> fmt::Display for Cast<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }
}
