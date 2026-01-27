// devela_base_core::num::grain::cast::namespace

#[doc = crate::_tags!(num namespace)]
/// Provides *const* casting, joining and splitting operations between primitives.
#[doc = crate::_doc_location!("num")]
//
// See also the related traits: [`PrimitiveCast`], [`PrimitiveJoin`] and [`PrimitiveSplit`].
//
// [`PrimitiveCast`]: crate::PrimitiveCast
// [`PrimitiveJoin`]: crate::PrimitiveJoin
// [`PrimitiveSplit`]: crate::PrimitiveSplit
#[repr(transparent)]
pub struct Cast<T>(pub T);

mod core_impls {
    use crate::{Cast, ConstInitCore, Ordering};
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
    impl<T: ConstInitCore> ConstInitCore for Cast<T> {
        const INIT: Self = Cast(T::INIT);
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
