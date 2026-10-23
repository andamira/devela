//
//! Define the [`Mismatch`] type.
//

use crate::ConstInit;

impl<N: ConstInit, H: ConstInit> ConstInit for Mismatch<N, H> {
    /// Returns a *const* default `Mismatch`.
    const INIT: Self = Self::new(N::INIT, H::INIT);
}

#[doc = crate::_tags!(result)]
/// Represents a mismatch between an expected `need` and an encountered `have`.
#[doc = crate::_doc_meta!{
    location("code/result", struct Mismatch),
}]
/// A diagnostic description of a mismatch between an expected requirement and an observed value.
/// This type does not encode failure semantics on its own.
pub struct Mismatch<N, H> {
    /// Information about something that was needed, expected or anticipated.
    pub need: N,

    /// Information about something that was obtained, observed, or encountered.
    pub have: H,

    /// Contextual static information about the mismatch.
    pub info: &'static str,
}

impl<N, H> Mismatch<N, H> {
    /// Creates a new `Mismatch` with the specified `need` and `have`.
    #[must_use]
    pub const fn new(need: N, have: H) -> Self {
        Self { need, have, info: "" }
    }

    /// Creates a new `Mismatch` with the specified `need`, `have`, and `info`.
    #[must_use]
    pub const fn with_info(need: N, have: H, info: &'static str) -> Self {
        Self { need, have, info }
    }
}

mod core_impls {
    use super::Mismatch;
    use ::core::cmp::Ordering;
    use ::core::fmt::{Debug, Display, Formatter, Result};
    use ::core::hash::{Hash, Hasher};

    impl<N: Clone, H: Clone> Clone for Mismatch<N, H> {
        fn clone(&self) -> Self {
            Self {
                need: self.need.clone(),
                have: self.have.clone(),
                info: self.info,
            }
        }
    }
    impl<N: Copy, H: Copy> Copy for Mismatch<N, H> {}

    impl<N: Default, H: Default> Default for Mismatch<N, H> {
        /// Returns a default `Mismatch`.
        fn default() -> Self {
            Self {
                need: Default::default(),
                have: Default::default(),
                info: "",
            }
        }
    }

    impl<N: Debug, H: Debug> Debug for Mismatch<N, H> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let mut debug = f.debug_struct("Mismatch");
            debug.field("need", &self.need);
            debug.field("have", &self.have);
            debug.field("info", &self.info);
            debug.finish()
        }
    }

    impl<N: Display, H: Display> Display for Mismatch<N, H> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "Mismatch {{ need: {}, have: {}, info: {} }}",
                self.need, self.have, self.info
            )
        }
    }

    impl<N: PartialEq, H: PartialEq> PartialEq for Mismatch<N, H> {
        fn eq(&self, other: &Self) -> bool {
            self.need == other.need && self.have == other.have && self.info == other.info
        }
    }
    impl<N: Eq, H: Eq> Eq for Mismatch<N, H> {}

    impl<N: PartialOrd, H: PartialOrd> PartialOrd for Mismatch<N, H> {
        /// Compare need first. If they are equal, then compare have.
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.need.partial_cmp(&other.need) {
                Some(Ordering::Equal) => match self.have.partial_cmp(&other.have) {
                    Some(Ordering::Equal) => self.info.partial_cmp(other.info),
                    other => other,
                },
                other => other,
            }
        }
    }
    impl<N: Ord, H: Ord> Ord for Mismatch<N, H> {
        /// Compare need first. If they are equal, then compare have.
        fn cmp(&self, other: &Self) -> Ordering {
            match self.need.cmp(&other.need) {
                Ordering::Equal => match self.have.cmp(&other.have) {
                    Ordering::Equal => self.info.cmp(other.info),
                    order => order,
                },
                order => order,
            }
        }
    }

    impl<N: Hash, H: Hash> Hash for Mismatch<N, H> {
        fn hash<HR: Hasher>(&self, state: &mut HR) {
            self.need.hash(state);
            self.have.hash(state);
        }
    }
}
