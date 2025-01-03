// devela::code::result::mismatch
//
//!
//

/// Represents a mismatch between an expected `need` and an encountered `have`.
///
/// With optional contextual information in the `info` field.
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

    /// Contextual static information about the mismatch.
    pub info: &'static str,
}

impl<N, H> Mismatch<N, H> {}

mod core_impls {
    use crate::{Debug, Display, FmtResult, Formatter, Hash, Hasher, Mismatch, Ordering};

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
        /// Returns an empty Mismatch with `None` for both fields.
        #[must_use]
        fn default() -> Self {
            Self {
                need: Default::default(),
                have: Default::default(),
                info: "",
            }
        }
    }

    impl<N: Debug, H: Debug> Debug for Mismatch<N, H> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            let mut debug = f.debug_struct("Mismatch");
            debug.field("need", &self.need);
            debug.field("have", &self.have);
            debug.field("info", &self.info);
            debug.finish()
        }
    }

    impl<N: Display, H: Display> Display for Mismatch<N, H> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
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
