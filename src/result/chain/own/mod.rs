// devela::result::chain::own
//
//!
//

// the order of the modules determines the order of the impl blocks in the docs
mod general;

mod result;

mod option;

/// A return type encapsulating an owned `state` alongside a return `value`.
///
/// It is designed to be returned by methods that take ownership of `self`,
/// and return the new state alongside the operation-specific result.
#[must_use]
pub struct Own<S, V> {
    /// The new state after the operation.
    pub state: S,

    /// The value resulting from the operation.
    pub value: V,
}

mod core_impls {
    use {
        super::Own,
        core::{cmp::Ordering, fmt},
    };

    impl<S: Default, V: Default> Default for Own<S, V> {
        fn default() -> Self {
            Self {
                state: S::default(),
                value: V::default(),
            }
        }
    }

    impl<S: fmt::Debug, V: fmt::Debug> fmt::Debug for Own<S, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut debug = f.debug_struct("Own");
            debug
                .field("state", &self.state)
                .field("value", &self.value)
                .finish()
        }
    }
    impl<S: fmt::Display, V: fmt::Display> fmt::Display for Own<S, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "State: {}, Value: {}", self.state, self.value)
        }
    }

    impl<S: Clone, V: Clone> Clone for Own<S, V> {
        fn clone(&self) -> Self {
            Self {
                state: self.state.clone(),
                value: self.value.clone(),
            }
        }
    }
    impl<S: Copy, V: Copy> Copy for Own<S, V> {}

    impl<S: PartialEq, V: PartialEq> PartialEq for Own<S, V> {
        fn eq(&self, other: &Self) -> bool {
            self.state == other.state && self.value == other.value
        }
    }
    impl<S: Eq, V: Eq> Eq for Own<S, V> {}

    impl<S: PartialOrd, V: PartialOrd> PartialOrd for Own<S, V> {
        /// State's ordering takes precedence over value's ordering.
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.state.partial_cmp(&other.state) {
                Some(Ordering::Equal) => self.value.partial_cmp(&other.value),
                other => other,
            }
        }
    }
    impl<S: Ord, V: Ord> Ord for Own<S, V> {
        /// State's ordering takes precedence over value's ordering.
        fn cmp(&self, other: &Self) -> Ordering {
            match self.state.cmp(&other.state) {
                Ordering::Equal => self.value.cmp(&other.value),
                other => other,
            }
        }
    }
}
