// devela::result::own::option_value
//
//!
//

use super::Own;

/// # Additional methods for when the `value` field is an `Option`.
impl<S, V> Own<S, Option<V>> {
    /// A constructor that returns `state` alongside `Some(value)`.
    #[inline]
    pub const fn new_some_value(state: S, value: V) -> Own<S, Option<V>> {
        Own {
            state,
            value: Some(value),
        }
    }

    /// A constructor with the given `state` and no value.
    #[inline]
    pub const fn new_none_value(state: S) -> Own<S, Option<V>> {
        Own { state, value: None }
    }

    /* is */

    /// Returns [`true`] if the `value` is [`Some`].
    #[inline]
    #[must_use]
    pub const fn is_value_some(&self) -> bool {
        self.value.is_some()
    }
    /// Returns [`true`] if the `value` is [`Some`] and it matches a predicate.
    #[inline]
    #[must_use]
    pub fn is_value_some_and(self, f: impl FnOnce(V) -> bool) -> bool {
        self.value.is_some_and(f)
    }

    /// Returns [`true`] if the `value` is [`None`].
    #[inline]
    #[must_use]
    pub const fn is_value_none(&self) -> bool {
        self.value.is_none()
    }

    /* assert */

    /// Asserts the value is [`Some`] and returns `self`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn assert_value_some(self) -> Self {
        match self.value {
            Some(_) => (),
            None => panic![],
        }
        self
    }

    /// Asserts the value is [`Some`] and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn assert_value_some_or(self, message: &'static str) -> Self {
        match self.value {
            Some(_) => (),
            None => panic!["{}", message],
        }
        self
    }

    /// Asserts the value is [`None`] and returns `self`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `Some`.
    #[inline]
    pub const fn assert_value_none(self) -> Self {
        match self.value {
            None => self,
            Some(_) => panic![],
        }
    }

    /// Asserts the value is [`None`] and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the value is `Some`.
    #[inline]
    pub const fn assert_value_none_or(self, message: &'static str) -> Self {
        match self.value {
            None => self,
            Some(_) => panic!["{}", message],
        }
    }

    /* map */

    /// Maps an `Option<V>` to an `Option<W>` by applying the `op` function
    /// to a contained value (if `Some`), or returns `None` (if `None`).
    #[inline]
    pub fn value_map<F, W>(self, op: F) -> Own<S, Option<W>>
    where
        F: FnOnce(V) -> W,
    {
        Own {
            state: self.state,
            value: self.value.map(op),
        }
    }

    /// Returns [`None`] if the value is `None`,
    /// otherwise returns `optb`.
    #[inline]
    pub fn value_and<W>(self, optb: Option<W>) -> Own<S, Option<W>> {
        Own {
            state: self.state,
            value: self.value.and(optb),
        }
    }

    /// Returns [`None`] if the value is `None`,
    /// otherwise calls `op` with the wrapped value and returns the result.
    #[inline]
    pub fn value_and_then<F, W>(self, op: F) -> Own<S, Option<W>>
    where
        F: FnOnce(V) -> Option<W>,
    {
        Own {
            state: self.state,
            value: self.value.and_then(op),
        }
    }

    /* unwrap */

    /// Unwraps the contained `Some(value)` or panics.
    ///
    /// See also [`const_value_unwrap`][Self::const_value_unwrap] for `Copy` types.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub fn value_unwrap(self) -> Own<S, V> {
        Own {
            state: self.state,
            value: self.value.unwrap(),
        }
    }

    /// Unwraps the contained `Some(value)` or provides a `default`.
    ///
    /// See also [`const_value_unwrap`][Self::const_value_unwrap] for `Copy` types.
    #[inline]
    pub fn value_unwrap_or(self, default: V) -> Own<S, V> {
        Own {
            state: self.state,
            value: self.value.unwrap_or(default),
        }
    }

    /// Unwraps the contained `Some(value)` or panics with the given `message`.
    ///
    /// See also [`const_value_expect`][Self::const_value_expect] for `Copy` types.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub fn value_expect(self, message: &str) -> Own<S, V> {
        Own {
            state: self.state,
            value: self.value.expect(message),
        }
    }

    /* convert to result */

    /// Converts from `Option<V>` to `Result<V, E>`.
    #[inline]
    pub fn value_ok_or<E>(self, err: E) -> Own<S, Result<V, E>> {
        Own {
            state: self.state,
            value: self.value.ok_or(err),
        }
    }
    /// Transforms the `Option<V>` into a Result<V, E>,
    /// mapping `Some(v)` to `Ok(v)` and `None` to `Err(err())`.
    #[inline]
    pub fn value_ok_or_else<E, F>(self, err: F) -> Own<S, Result<V, E>>
    where
        F: FnOnce() -> E,
    {
        Own {
            state: self.state,
            value: self.value.ok_or_else(err),
        }
    }
}

/// # *const* methods for when everything is `Copy` and the `value` is an `Option`.
impl<S: Copy, V: Copy> Own<S, Option<V>> {
    /* assert and deconstruct */

    /// Asserts the value is [`Some`] and returns the `state` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn value_some_state(self) -> S {
        match self.value {
            Some(_) => self.state,
            None => panic![],
        }
    }
    /// Asserts the value is [`Some`] and returns the `state` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn value_some_state_or(self, message: &'static str) -> S {
        match self.value {
            Some(_) => self.state,
            None => panic!["{}", message],
        }
    }
    /// Asserts the value is [`Some`] and returns the `value` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn value_some_value(self) -> V {
        match self.value {
            Some(v) => v,
            None => panic![],
        }
    }
    /// Asserts the value is [`Some`] and returns the `value` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn value_some_value_or(self, message: &'static str) -> V {
        match self.value {
            Some(v) => v,
            None => panic!["{}", message],
        }
    }

    /// Asserts the value is [`None`] and returns the `state` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `Some`.
    #[inline]
    pub const fn value_none_state(self) -> S {
        match self.value {
            None => self.state,
            Some(_) => panic![],
        }
    }
    /// Asserts the value is [`None`] and returns the `state` field, otherwise panics.
    ///
    /// # Panics
    /// Panics if the value is `Some`.
    #[inline]
    pub const fn value_none_state_or(self, message: &'static str) -> S {
        match self.value {
            None => self.state,
            Some(_) => panic!["{}", message],
        }
    }

    /* unwrap */

    /// Unwraps the contained `Some(value)` or panics.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn const_value_unwrap(self) -> Own<S, V> {
        if let Some(value) = self.value {
            Own {
                state: self.state,
                value,
            }
        } else {
            panic![];
        }
    }

    /// Unwraps the contained `Some(value)` or provides a `default`.
    #[inline]
    pub const fn const_value_unwrap_or(self, default: V) -> Own<S, V> {
        if let Some(value) = self.value {
            Own {
                state: self.state,
                value,
            }
        } else {
            Own {
                state: self.state,
                value: default,
            }
        }
    }

    /// Unwraps the contained `Some(value)` or panics with the given `message`.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn const_value_expect(self, message: &'static str) -> Own<S, V> {
        if let Some(value) = self.value {
            Own {
                state: self.state,
                value,
            }
        } else {
            panic!["{}", message];
        }
    }
}
