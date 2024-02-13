// devela::result::own::result_state
//
//!
//

use super::Own;

/// # Additional methods for when the `state` field is a `Result`.
impl<S, E, V> Own<Result<S, E>, V> {
    /// A constructor that returns `Ok(state)` alongside `value`.
    #[inline]
    pub const fn new_ok_state(state: S, value: V) -> Own<Result<S, E>, V> {
        Own {
            state: Ok(state),
            value,
        }
    }
    /// A constructor that returns `Err(state)` alongside `value`.
    #[inline]
    pub const fn new_err_state(error_state: E, value: V) -> Own<Result<S, E>, V> {
        Own {
            state: Err(error_state),
            value,
        }
    }

    /* is */

    /// Returns [`true`] if the `state` is [`Ok`].
    #[inline]
    #[must_use]
    pub const fn is_state_ok(&self) -> bool {
        self.state.is_ok()
    }
    /// Returns [`true`] if the `state` is [`Ok`] and it matches a predicate.
    #[inline]
    #[must_use]
    pub fn is_state_ok_and(self, f: impl FnOnce(S) -> bool) -> bool {
        self.state.is_ok_and(f)
    }

    /// Returns [`true`] if the `state` is [`Err`].
    #[inline]
    #[must_use]
    pub const fn is_state_err(&self) -> bool {
        self.state.is_err()
    }
    /// Returns [`true`] if the `state` is [`Err`] and it matches a predicate.
    #[inline]
    #[must_use]
    pub fn is_state_err_and(self, f: impl FnOnce(E) -> bool) -> bool {
        self.state.is_err_and(f)
    }

    /* assert */

    /// Asserts the `state` is [`Ok`] and returns `self`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    #[inline]
    pub const fn assert_state_ok(self) -> Self {
        match self.state {
            Ok(_) => self,
            Err(_) => panic![],
        }
    }

    /// Asserts the `state` is [`Ok`] and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the `state` is `Err`.
    #[inline]
    pub const fn assert_state_ok_or(self, message: &'static str) -> Self {
        match self.state {
            Ok(_) => self,
            Err(_) => panic!["{}", message],
        }
    }

    /// Asserts the `state` is [`Err`] and returns `self`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the `state` is `Ok`.
    #[inline]
    pub const fn assert_state_err(self) -> Self {
        match self.state {
            Err(_) => self,
            Ok(_) => panic![],
        }
    }
    /// Asserts the `state` is [`Err`] and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the `state` is `Ok`.
    #[inline]
    pub const fn assert_state_err_or(self, message: &'static str) -> Self {
        match self.state {
            Err(_) => self,
            Ok(_) => panic!["{}", message],
        }
    }

    /* map */

    /// Maps a `Result<S>` to a `Result<T>` by applying the `op` function
    /// to a contained [`Ok`] value, leaving an [`Err`] value untouched.
    #[inline]
    pub fn state_map<F, T>(self, op: F) -> Own<Result<T, E>, V>
    where
        F: FnOnce(S) -> T,
    {
        Own {
            state: self.state.map(op),
            value: self.value,
        }
    }

    /// Maps a `Result<S, E>` to a `Result<S, F>` by applying the `op` function
    /// to a contained [`Err`] value, leaving an [`Ok`] value untouched.
    #[inline]
    pub fn state_map_err<O, F>(self, op: O) -> Own<Result<S, F>, V>
    where
        O: FnOnce(E) -> F,
    {
        Own {
            state: self.state.map_err(op),
            value: self.value,
        }
    }

    /// Returns `res` if the state [`is`] Ok, otherwise returns the [`Err`] value of `self`.
    #[inline]
    pub fn state_and<T>(self, res: Result<T, E>) -> Own<Result<T, E>, V> {
        Own {
            state: self.state.and(res),
            value: self.value,
        }
    }

    /// Calls `op` if the state is [`Ok`], otherwise returns the [`Err`] value of `self`.
    #[inline]
    pub fn state_and_then<F, T>(self, op: F) -> Own<Result<T, E>, V>
    where
        F: FnOnce(S) -> Result<T, E>,
    {
        Own {
            state: self.state.and_then(op),
            value: self.value,
        }
    }

    /* unwrap */

    /// Unwraps the contained `Ok(state)` or panics.
    ///
    /// See also [`const_state_unwrap`][Self::const_state_unwrap] for `Copy` types.
    ///
    /// # Panics
    /// Panics if the value is `Err`.
    #[inline]
    pub fn state_unwrap(self) -> Own<S, V> {
        if let Ok(state) = self.state {
            Own {
                state,
                value: self.value,
            }
        } else {
            panic![];
        }
    }

    /// Unwraps the contained `Ok(state)` or provides a `default`.
    ///
    /// See also [`const_state_unwrap_or`][Self::const_state_unwrap_or] for `Copy` types.
    #[inline]
    pub fn state_unwrap_or(self, default: S) -> Own<S, V> {
        Own {
            state: self.state.unwrap_or(default),
            value: self.value,
        }
    }

    /// Unwraps the contained `Ok(state)` or panics with a `message`.
    ///
    /// See also [`const_state_expect`][Self::const_state_expect] for `Copy` types.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    #[inline]
    pub fn state_expect(self, message: &str) -> Own<S, V>
    where
        E: core::fmt::Debug,
    {
        Own {
            state: self.state.expect(message),
            value: self.value,
        }
    }

    /* convert to option */

    /// Converts from `Result<S, E>` to `Option<S>`.
    #[inline]
    pub fn state_ok(self) -> Own<Option<S>, V> {
        Own {
            state: self.state.ok(),
            value: self.value,
        }
    }

    /// Converts from `Result<S, E>` to `Option<E>`.
    ///
    /// Converts `state` field into an `Option<E>`, and discarding the success state, if any.
    #[inline]
    pub fn state_err(self) -> Own<Option<E>, V> {
        Own {
            state: self.state.err(),
            value: self.value,
        }
    }
}

/// # *const* methods for when everything is `Copy` and the `state` is a `Result`.
impl<S: Copy, E: Copy, V: Copy> Own<Result<S, E>, V> {
    /* assert and deconstruct */

    /// Asserts the state is [`Ok`] and returns the `state` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    #[inline]
    pub const fn state_ok_state(self) -> S {
        match self.state {
            Ok(state) => state,
            Err(_) => panic![],
        }
    }
    /// Asserts the state is [`Err`] and returns the `state` field, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    #[inline]
    pub const fn state_ok_state_or(self, message: &'static str) -> S {
        match self.state {
            Ok(state) => state,
            Err(_) => panic!["{}", message],
        }
    }
    /// Asserts the state is [`Ok`] and returns the `state` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    #[inline]
    pub const fn state_ok_value(self) -> V {
        match self.state {
            Ok(_) => self.value,
            Err(_) => panic![],
        }
    }
    /// Asserts the state is [`Err`] and returns the `state` field, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    #[inline]
    pub const fn state_ok_value_or(self, message: &'static str) -> V {
        match self.state {
            Ok(_) => self.value,
            Err(_) => panic!["{}", message],
        }
    }

    /// Asserts the state is [`Err`] and returns the `state` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `Ok`.
    #[inline]
    pub const fn state_err_state(self) -> E {
        match self.state {
            Err(state) => state,
            Ok(_) => panic![],
        }
    }
    /// Asserts the state is [`Err`] and returns the `state` field, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the state is `Ok`.
    #[inline]
    pub const fn state_err_state_or(self, message: &'static str) -> E {
        match self.state {
            Err(state) => state,
            Ok(_) => panic!["{}", message],
        }
    }
    /// Asserts the state is [`Err`] and returns the `value` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `Ok`.
    #[inline]
    pub const fn state_err_value(self) -> V {
        match self.state {
            Err(_) => self.value,
            Ok(_) => panic![],
        }
    }
    /// Asserts the state is [`Err`] and returns the `value` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the state is `Ok`.
    #[inline]
    pub const fn state_err_value_or(self, message: &'static str) -> V {
        match self.state {
            Err(_) => self.value,
            Ok(_) => panic!["{}", message],
        }
    }

    /* unwrap */

    /// Unwraps the contained `Ok(state)` or panics.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    #[inline]
    pub const fn const_state_unwrap(self) -> Own<S, V> {
        if let Ok(state) = self.state {
            Own {
                state,
                value: self.value,
            }
        } else {
            panic![];
        }
    }

    /// Unwraps the contained `Ok(state)` or provides a `default`.
    #[inline]
    pub const fn const_state_unwrap_or(self, default: S) -> Own<S, V> {
        if let Ok(state) = self.state {
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

    /// Unwraps the contained `Ok(state)` or panics with the given `message`.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    #[inline]
    pub const fn const_state_expect(self, message: &'static str) -> Own<S, V> {
        if let Ok(state) = self.state {
            Own {
                state,
                value: self.value,
            }
        } else {
            panic!["{}", message];
        }
    }
}
