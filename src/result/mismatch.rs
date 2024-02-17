// devela::result::mismatch
//
//!
//

/// Represents a mismatch between an expected `need` and an encountered `have`.
///
/// This struct conveys optional information about the anticipated `need` and
/// the observed `have`, allowing for either, both, or none to be specified.
///
/// It can be particularly useful for error handling and reporting.
pub struct Mismatch<N, H> {
    /// Information about something that was needed, expected or anticipated.
    pub need: N,

    /// Information about something that was obtained, observed, or encountered.
    pub have: H,
}

impl<N, H> Mismatch<N, H> {}

mod core_impls {
    use super::*;
    use core::{
        cmp::Ordering,
        fmt,
        hash::{Hash, Hasher},
    };

    impl<N: Clone, H: Clone> Clone for Mismatch<N, H> {
        #[inline]
        fn clone(&self) -> Self {
            Self {
                need: self.need.clone(),
                have: self.have.clone(),
            }
        }
    }
    impl<N: Copy, H: Copy> Copy for Mismatch<N, H> {}

    impl<N: Default, H: Default> Default for Mismatch<N, H> {
        /// Returns an empty Mismatch with `None` for both fields.
        #[inline]
        #[must_use]
        fn default() -> Self {
            Self {
                need: Default::default(),
                have: Default::default(),
            }
        }
    }

    impl<N: fmt::Debug, H: fmt::Debug> fmt::Debug for Mismatch<N, H> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut debug = f.debug_struct("Mismatch");
            debug.field("need", &self.need);
            debug.field("have", &self.have);
            debug.finish()
        }
    }

    impl<N: PartialEq, H: PartialEq> PartialEq for Mismatch<N, H> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.need == other.need && self.have == other.have
        }
    }
    impl<N: Eq, H: Eq> Eq for Mismatch<N, H> {}

    impl<N: PartialOrd, H: PartialOrd> PartialOrd for Mismatch<N, H> {
        /// Compare need first. If they are equal, then compare have.
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.need.partial_cmp(&other.need) {
                Some(Ordering::Equal) => self.have.partial_cmp(&other.have),
                other => other,
            }
        }
    }
    impl<N: Ord, H: Ord> Ord for Mismatch<N, H> {
        /// Compare need first. If they are equal, then compare have.
        fn cmp(&self, other: &Self) -> Ordering {
            match self.need.cmp(&other.need) {
                Ordering::Equal => self.have.cmp(&other.have),
                order => order,
            }
        }
    }

    impl<N: Hash, H: Hash> Hash for Mismatch<N, H> {
        #[inline]
        fn hash<HR: Hasher>(&self, state: &mut HR) {
            self.need.hash(state);
            self.have.hash(state);
        }
    }
}
