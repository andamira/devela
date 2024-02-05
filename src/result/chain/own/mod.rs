// devela::result::chain::owning
//
//!
//

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

impl<S, V> Own<S, V> {
    /// A constructor with the given `state` and `value`.
    #[inline]
    pub const fn new(state: S, value: V) -> Self {
        Own { state, value }
    }

    /// Destructures itself returning the `state` field.
    #[inline]
    #[must_use]
    pub fn into_state(self) -> S {
        self.state
    }
    /// Destructures itself returning the `value` field.
    #[inline]
    #[must_use]
    pub fn into_value(self) -> V {
        self.value
    }
    /// Destructures itself into a tuple.
    #[inline]
    #[must_use]
    pub fn into_tuple(self) -> (S, V) {
        (self.state, self.value)
    }

    /// Returns a shared reference to the `state` field.
    #[inline]
    #[must_use]
    pub const fn ref_state(&self) -> &S {
        &self.state
    }
    /// Returns a shared reference to the `value` field.
    #[inline]
    #[must_use]
    pub const fn ref_value(&self) -> &V {
        &self.value
    }
    /// Returns shared references to both `state` and `value` fields.
    #[inline]
    #[must_use]
    pub const fn ref_both(&self) -> (&S, &V) {
        (&self.state, &self.value)
    }

    /// Returns an exclusive reference to the `state` field.
    #[inline]
    #[must_use]
    pub fn mut_state(&mut self) -> &mut S {
        &mut self.state
    }
    /// Returns an exclusive reference to the `value` field.
    #[inline]
    #[must_use]
    pub fn mut_value(&mut self) -> &mut V {
        &mut self.value
    }
    /// Returns exclusive references to both `state` and `value` fields.
    #[inline]
    #[must_use]
    pub fn mut_both(&mut self) -> (&mut S, &mut V) {
        (&mut self.state, &mut self.value)
    }

    /// Returns `true` if the current `state` equals the given `expected` state.
    #[inline]
    #[must_use]
    pub fn is_state(&self, expected: &S) -> bool
    where
        S: PartialEq,
    {
        self.state == *expected
    }
    /// Returns `true` if the current `value` equals the given `expected` value.
    #[inline]
    #[must_use]
    pub fn is_value(&self, expected: &V) -> bool
    where
        V: PartialEq,
    {
        self.value == *expected
    }
    /// Returns `true` if the current `state` and `value` equals the corresponding expected ones.
    #[inline]
    #[must_use]
    pub fn are_both(&self, expected_state: &S, expected_value: &V) -> bool
    where
        S: PartialEq,
        V: PartialEq,
    {
        self.state == *expected_state && self.value == *expected_value
    }

    /// Replaces the existing `state` with a `new_state`.
    #[inline]
    pub fn replace_state(self, new_state: S) -> Self {
        Self::new(new_state, self.value)
    }
    /// Replaces the `value` with a `new_value`.
    #[inline]
    pub fn replace_value(self, new_value: V) -> Self {
        Self::new(self.state, new_value)
    }
    /// Replaces the existing `state` and `value` with `new_state` and `new_value`, respectively.
    #[inline]
    pub fn replace_both(self, new_state: S, new_value: V) -> Self {
        Self::new(new_state, new_value)
    }

    /// Applies a mapping function `f` to the `state` field.
    #[inline]
    pub fn map_state<F, NewState>(self, f: F) -> Own<NewState, V>
    where
        F: FnOnce(S) -> NewState,
    {
        Own {
            state: f(self.state),
            value: self.value,
        }
    }
    /// Applies a mapping function `f` to the `value` field.
    #[inline]
    pub fn map_value<F, NewT>(self, f: F) -> Own<S, NewT>
    where
        F: FnOnce(V) -> NewT,
    {
        Own {
            state: self.state,
            value: f(self.value),
        }
    }
    /// Applies the mapping functions `f_*` to the respective `state` and `value` fields.
    pub fn map_both<F, G, NewState, NewT>(self, f_state: F, f_value: G) -> Own<NewState, NewT>
    where
        F: FnOnce(S) -> NewState,
        G: FnOnce(V) -> NewT,
    {
        Own {
            state: f_state(self.state),
            value: f_value(self.value),
        }
    }
}

/// # Additional *const* methods for when everything is `Copy`.
impl<S: Copy, V: Copy> Own<S, V> {
    /// Destructures itself returning its `state` field.
    #[inline]
    #[must_use]
    pub const fn into_state_const(self) -> S {
        self.state
    }
    /// Destructures itself returning its `value` field.
    #[inline]
    #[must_use]
    pub const fn into_value_const(self) -> V {
        self.value
    }
    /// Destructures itself into a tuple.
    #[inline]
    #[must_use]
    pub const fn into_tuple_const(self) -> (S, V) {
        (self.state, self.value)
    }

    /// Replaces the `state` self with a `new_state`.
    #[inline]
    pub const fn replace_state_const(self, new_state: S) -> Self {
        Self::new(new_state, self.value)
    }
    /// Replaces the `value` with a `new_value`.
    #[inline]
    pub const fn replace_value_const(self, new_value: V) -> Self {
        Self::new(self.state, new_value)
    }
    /// Replaces the `state` self with a `new_state` and the `value` with a `new_value`.
    #[inline]
    pub const fn replace_both_const(self, new_state: S, new_value: V) -> Self {
        Self::new(new_state, new_value)
    }
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
