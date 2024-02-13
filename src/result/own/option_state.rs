// devela::result::own::option_state
//
//!
//

use super::Own;

/// # Additional methods for when the `value` field is an `Option`.
impl<S, V> Own<Option<S>, V> {
    /// A constructor that returns `Some(state)` alongside `value`.
    #[inline]
    pub const fn new_some_state(state: S, value: V) -> Own<Option<S>, V> {
        Own {
            state: Some(state),
            value,
        }
    }

    /// A constructor with `value` and no state.
    #[inline]
    pub const fn new_none_state(value: V) -> Own<Option<S>, V> {
        Own { state: None, value }
    }

    /* is */

    /// Returns [`true`] if the `state` is [`Some`].
    #[inline]
    #[must_use]
    pub const fn is_state_some(&self) -> bool {
        self.state.is_some()
    }
    /// Returns [`true`] if the `state` is [`Some`] and it matches a predicate.
    #[inline]
    #[must_use]
    pub fn is_state_some_and(self, f: impl FnOnce(S) -> bool) -> bool {
        self.state.is_some_and(f)
    }

    /// Returns [`true`] if the `state` is [`None`].
    #[inline]
    #[must_use]
    pub const fn is_state_none(&self) -> bool {
        self.state.is_none()
    }

    /* assert */

    /// Asserts the state is [`Some`] and returns `self`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `None`.
    #[inline]
    pub const fn assert_state_some(self) -> Self {
        match self.state {
            Some(_) => (),
            None => panic![],
        }
        self
    }

    /// Asserts the state is [`Some`] and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the state is `None`.
    #[inline]
    pub const fn assert_state_some_or(self, message: &'static str) -> Self {
        match self.state {
            Some(_) => (),
            None => panic!["{}", message],
        }
        self
    }

    /// Asserts the state is [`None`] and returns `self`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `Some`.
    #[inline]
    pub const fn assert_state_none(self) -> Self {
        match self.state {
            None => self,
            Some(_) => panic![],
        }
    }

    /// Asserts the state is [`None`] and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the state is `Some`.
    #[inline]
    pub const fn assert_state_none_or(self, message: &'static str) -> Self {
        match self.state {
            None => self,
            Some(_) => panic!["{}", message],
        }
    }

    /* map */

    /// Maps an `Option<S>` to an `Option<T>` by applying the `op` function
    /// to a contained state (if `Some`), or returns `None` (if `None`).
    #[inline]
    pub fn state_map<F, T>(self, op: F) -> Own<Option<T>, V>
    where
        F: FnOnce(S) -> T,
    {
        Own {
            state: self.state.map(op),
            value: self.value,
        }
    }

    /// Returns [`None`] if the state is `None`,
    /// otherwise returns `optb`.
    #[inline]
    pub fn state_and<T>(self, optb: Option<T>) -> Own<Option<T>, V> {
        Own {
            state: self.state.and(optb),
            value: self.value,
        }
    }

    /// Returns [`None`] if the state is `None`,
    /// otherwise calls `op` with the wrapped state and returns the result.
    #[inline]
    pub fn state_and_then<F, T>(self, op: F) -> Own<Option<T>, V>
    where
        F: FnOnce(S) -> Option<T>,
    {
        Own {
            state: self.state.and_then(op),
            value: self.value,
        }
    }

    /* unwrap */

    /// Unwraps the contained `Some(state)` or panics.
    ///
    /// See also [`const_state_unwrap`][Self::const_state_unwrap] for `Copy` types.
    ///
    /// # Panics
    /// Panics if the state is `None`.
    #[inline]
    pub fn state_unwrap(self) -> Own<S, V> {
        Own {
            state: self.state.unwrap(),
            value: self.value,
        }
    }

    /// Unwraps the contained `Some(state)` or provides a `default`.
    ///
    /// See also [`const_state_unwrap`][Self::const_state_unwrap] for `Copy` types.
    #[inline]
    pub fn state_unwrap_or(self, default: S) -> Own<S, V> {
        Own {
            state: self.state.unwrap_or(default),
            value: self.value,
        }
    }

    /// Unwraps the contained `Some(state)` or panics with the given `message`.
    ///
    /// See also [`const_state_expect`][Self::const_state_expect] for `Copy` types.
    ///
    /// # Panics
    /// Panics if the state is `None`.
    #[inline]
    pub fn state_expect(self, message: &str) -> Own<S, V> {
        Own {
            state: self.state.expect(message),
            value: self.value,
        }
    }

    /* convert to result */

    /// Converts from `Option<S>` to `Result<S, E>`.
    #[inline]
    pub fn state_ok_or<E>(self, err: E) -> Own<Result<S, E>, V> {
        Own {
            state: self.state.ok_or(err),
            value: self.value,
        }
    }
    /// Transforms the `Option<S>` into a Result<S, E>,
    /// mapping `Some(S)` to `Ok(S)` and `None` to `Err(err())`.
    #[inline]
    pub fn state_ok_or_else<E, F>(self, err: F) -> Own<Result<S, E>, V>
    where
        F: FnOnce() -> E,
    {
        Own {
            state: self.state.ok_or_else(err),
            value: self.value,
        }
    }
}

/// # *const* methods for when everything is `Copy` and the `value` is an `Option`.
impl<S: Copy, V: Copy> Own<Option<S>, V> {
    /* assert and deconstruct */

    /// Asserts the state is [`Some`] and returns the `state` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `None`.
    #[inline]
    pub const fn state_some_state(self) -> S {
        match self.state {
            Some(state) => state,
            None => panic![],
        }
    }
    /// Asserts the state is [`Some`] and returns the `state` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `None`.
    #[inline]
    pub const fn state_some_state_or(self, message: &'static str) -> S {
        match self.state {
            Some(state) => state,
            None => panic!["{}", message],
        }
    }
    /// Asserts the state is [`Some`] and returns the `value` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn state_some_value(self) -> V {
        match self.state {
            Some(_) => self.value,
            None => panic![],
        }
    }
    /// Asserts the state is [`Some`] and returns the `value` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn state_some_value_or(self, message: &'static str) -> V {
        match self.state {
            Some(_) => self.value,
            None => panic!["{}", message],
        }
    }

    /// Asserts the state is [`None`] and returns the `value` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `Some`.
    #[inline]
    pub const fn state_none_value_or(self, message: &'static str) -> V {
        match self.state {
            None => self.value,
            Some(_) => panic!["{}", message],
        }
    }

    /* unwrap */

    /// Unwraps the contained `Some(state)` or panics.
    ///
    /// # Panics
    /// Panics if the state is `None`.
    #[inline]
    pub const fn const_state_unwrap(self) -> Own<S, V> {
        if let Some(state) = self.state {
            Own {
                state,
                value: self.value,
            }
        } else {
            panic![];
        }
    }

    /// Unwraps the contained `Some(state)` or provides a `default`.
    #[inline]
    pub const fn const_state_unwrap_or(self, default: S) -> Own<S, V> {
        if let Some(state) = self.state {
            Own {
                state,
                value: self.value,
            }
        } else {
            Own {
                state: default,
                value: self.value,
            }
        }
    }

    /// Unwraps the contained `Some(state)` or panics with the given `message`.
    ///
    /// # Panics
    /// Panics if the state is `None`.
    #[inline]
    pub const fn const_state_expect(self, message: &'static str) -> Own<S, V> {
        if let Some(state) = self.state {
            Own {
                state,
                value: self.value,
            }
        } else {
            panic!["{}", message];
        }
    }
}
