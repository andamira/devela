//
//! Define the [`Mismatch`] type.
//

use crate::ConstInit;

#[doc(inline)]
pub use crate::boot::base::code::Mismatch;

impl<N: ConstInit, H: ConstInit> ConstInit for Mismatch<N, H> {
    /// Returns a *const* default `Mismatch`.
    const INIT: Self = Self::new(N::INIT, H::INIT);
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
    use ::core::fmt::{Display, Formatter, Result};

    impl<N: Display, H: Display> Display for Mismatch<N, H> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "Mismatch {{ need: {}, have: {}, info: {} }}",
                self.need, self.have, self.info
            )
        }
    }

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
}
