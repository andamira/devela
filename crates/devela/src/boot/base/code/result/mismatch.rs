//
//! Defines [`Mismatch`].
//

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

mod mismatch_core_impls {
    use super::Mismatch;
    use ::core::fmt::{Debug, Formatter, Result};
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

    impl<N: PartialEq, H: PartialEq> PartialEq for Mismatch<N, H> {
        fn eq(&self, other: &Self) -> bool {
            self.need == other.need && self.have == other.have && self.info == other.info
        }
    }

    impl<N: Eq, H: Eq> Eq for Mismatch<N, H> {}

    impl<N: Hash, H: Hash> Hash for Mismatch<N, H> {
        fn hash<HR: Hasher>(&self, state: &mut HR) {
            self.need.hash(state);
            self.have.hash(state);
        }
    }
}
