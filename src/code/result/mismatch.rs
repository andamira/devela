// devela::code::result::mismatch
//
//! Define the [`Mismatch`] type.
//

use crate::{ConstDefault, Interval};

#[doc = crate::TAG_RESULT!()]
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

impl<N, H> Mismatch<Interval<N>, H> {
    /// Creates a mismatch where `need` is an [`Interval::point`],
    /// and `have` does not match.
    #[must_use] #[rustfmt::skip]
    pub fn in_point_interval(need_point: N, have: H, info: &'static str) -> Self where N: Clone {
        Self { need: Interval::point(need_point), have, info }
    }

    /// Creates a mismatch where `need` is an [`Interval::empty`],
    /// but `have` was provided.
    #[must_use] #[rustfmt::skip]
    pub fn in_empty_interval(have: H, info: &'static str) -> Self where N: Default {
        Self { need: Interval::empty(), have, info }
    }
    /// Creates a mismatch where `need` is an [`Interval::empty_const`],
    /// but `have` was provided.
    #[must_use] #[rustfmt::skip]
    pub const fn in_empty_const_interval(have: H, info: &'static str) -> Self
    where N: ConstDefault {
        Self { need: Interval::empty_const(), have, info }
    }
    /// Creates a mismatch where `need` is an [`Interval::empty_with`],
    /// but `have` was provided.
    #[must_use] #[rustfmt::skip]
    pub fn in_empty_interval_with(value: N, have: H, info: &'static str) -> Self where N: Clone {
        Self { need: Interval::empty_with(value), have, info }
    }

    /// Creates a mismatch where `need` is an [`Interval::closed`],
    /// and `have` is outside it.
    #[must_use]
    pub const fn in_closed_interval(lower: N, upper: N, have: H, info: &'static str) -> Self {
        Self { need: Interval::closed(lower, upper), have, info }
    }

    /// Creates a mismatch where `need` is an [`Interval::closed_open`],
    /// and `have` is outside it.
    #[must_use]
    pub const fn in_closed_open_interval(lower: N, upper: N, have: H, info: &'static str) -> Self {
        Self {
            need: Interval::closed_open(lower, upper),
            have,
            info,
        }
    }

    /// Creates a mismatch where `need` is an [`Interval::open`],
    /// and `have` is outside it.
    #[must_use]
    pub const fn in_open_interval(lower: N, upper: N, have: H, info: &'static str) -> Self {
        Self { need: Interval::open(lower, upper), have, info }
    }
}

mod core_impls {
    use crate::{
        ConstDefault, Debug, Display, FmtResult, Formatter, Hash, Hasher, Mismatch, Ordering,
    };

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
        #[must_use]
        fn default() -> Self {
            Self {
                need: Default::default(),
                have: Default::default(),
                info: "",
            }
        }
    }
    impl<N: ConstDefault, H: ConstDefault> ConstDefault for Mismatch<N, H> {
        /// Returns a *const* default `Mismatch`.
        const DEFAULT: Self = Self { need: N::DEFAULT, have: H::DEFAULT, info: "" };
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
