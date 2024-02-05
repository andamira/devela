// devela::result::chain::own::result
//
//!
//

use super::Own;

/// # Additional methods for when the `value` field is a `Result`.
impl<S, V, E> Own<S, Result<V, E>> {
    /* is */

    /// Returns [`true`] if the `value` is [`Ok`].
    #[inline]
    #[must_use]
    pub const fn is_ok(&self) -> bool {
        self.value.is_ok()
    }
    /// Returns [`true`] if the `value` is [`Ok`] and it matches a predicate.
    #[inline]
    #[must_use]
    pub fn is_ok_and(self, f: impl FnOnce(V) -> bool) -> bool {
        self.value.is_ok_and(f)
    }

    /// Returns [`true`] if the `value` is [`Err`].
    #[inline]
    #[must_use]
    pub const fn is_err(&self) -> bool {
        self.value.is_err()
    }
    /// Returns [`true`] if the `value` is [`Err`] and it matches a predicate.
    #[inline]
    #[must_use]
    pub fn is_err_and(self, f: impl FnOnce(E) -> bool) -> bool {
        self.value.is_err_and(f)
    }

    /* assert */

    /// Asserts the value is [`Ok`] and returns `self`, otherwise panics.
    #[inline]
    pub const fn assert_ok(self) -> Self {
        match self.value {
            Ok(_) => self,
            Err(_) => panic![],
        }
    }

    /// Asserts the value is [`Ok`] and returns `self`, otherwise panics with `message`.
    #[inline]
    pub const fn assert_ok_or(self, message: &'static str) -> Self {
        match self.value {
            Ok(_) => self,
            Err(_) => panic!["{}", message],
        }
    }

    /// Asserts the value is [`Err`] and returns `self`, otherwise panics.
    #[inline]
    pub const fn assert_err(self) -> Self {
        match self.value {
            Err(_) => self,
            Ok(_) => panic![],
        }
    }
    /// Asserts the value is [`Err`] and returns `self`, otherwise panics with `message`.
    #[inline]
    pub const fn assert_err_or(self, message: &'static str) -> Self {
        match self.value {
            Err(_) => self,
            Ok(_) => panic!["{}", message],
        }
    }

    /* map */

    /// Maps a `Result<V>` to a `Result<W>` by applying the `op` function
    /// to a contained [`Ok`] value, leaving an [`Err`] value untouched.
    #[inline]
    pub fn map<F, W>(self, op: F) -> Own<S, Result<W, E>>
    where
        F: FnOnce(V) -> W,
    {
        Own {
            state: self.state,
            value: self.value.map(op),
        }
    }

    /// Maps a `Result<V, E>` to a `Result<V, F>` by applying the `op` function
    /// to a contained [`Err`] value, leaving an [`Ok`] value untouched.
    #[inline]
    pub fn map_err<O, F>(self, op: O) -> Own<S, Result<V, F>>
    where
        O: FnOnce(E) -> F,
    {
        Own {
            state: self.state,
            value: self.value.map_err(op),
        }
    }

    /// Returns `res` if the result [`is`] Ok, otherwise returns the [`Err`] value of `self`.
    #[inline]
    pub fn and<W>(self, res: Result<W, E>) -> Own<S, Result<W, E>> {
        Own {
            state: self.state,
            value: self.value.and(res),
        }
    }

    /// Calls `op` if the result is [`Ok`], otherwise returns the [`Err`] value of `self`.
    #[inline]
    pub fn and_then<F, W>(self, op: F) -> Own<S, Result<W, E>>
    where
        F: FnOnce(V) -> Result<W, E>,
    {
        Own {
            state: self.state,
            value: self.value.and_then(op),
        }
    }

    /* unwrap */

    /// Unwraps the contained `Ok(value)` or panics.
    #[inline]
    pub fn unwrap(self) -> Own<S, V> {
        if let Ok(value) = self.value {
            Own {
                state: self.state,
                value,
            }
        } else {
            panic![];
        }
    }

    /// Unwraps the contained `Ok(value)` or provides a `default`.
    #[inline]
    pub fn unwrap_or(self, default: V) -> Own<S, V> {
        Own {
            state: self.state,
            value: self.value.unwrap_or(default),
        }
    }

    /// Unwraps the contained `Ok(value)` or panics with a `message`.
    #[inline]
    pub fn expect(self, message: &str) -> Own<S, V>
    where
        E: core::fmt::Debug,
    {
        Own {
            state: self.state,
            value: self.value.expect(message),
        }
    }
}

/// # *const* methods for when everything is `Copy` and the `value` is a `Result`.
impl<S: Copy, V: Copy, E: Copy> Own<S, Result<V, E>> {
    /* assert and deconstruct */

    /// Asserts the value is [`Ok`] and returns the `state` field, otherwise panics.
    #[inline]
    pub const fn state_ok(self) -> S {
        match self.value {
            Ok(_) => self.state,
            Err(_) => panic![],
        }
    }
    /// Asserts the value is [`Ok`] and returns the `state` field, otherwise panics.
    #[inline]
    pub const fn state_ok_or(self, message: &'static str) -> S {
        match self.value {
            Ok(_) => self.state,
            Err(_) => panic!["{}", message],
        }
    }
    /// Asserts the value is [`Ok`] and returns the `value` field, otherwise panics.
    #[inline]
    pub const fn value_ok(self) -> V {
        match self.value {
            Ok(v) => v,
            Err(_) => panic![],
        }
    }
    /// Asserts the value is [`Ok`] and returns the `value` field, otherwise panics.
    #[inline]
    pub const fn value_ok_or(self, message: &'static str) -> V {
        match self.value {
            Ok(v) => v,
            Err(_) => panic!["{}", message],
        }
    }

    /// Asserts the value is [`Err`] and returns the `state` field, otherwise panics.
    #[inline]
    pub const fn state_err(self) -> S {
        match self.value {
            Err(_) => self.state,
            Ok(_) => panic![],
        }
    }
    /// Asserts the value is [`Err`] and returns the `state` field, otherwise panics.
    #[inline]
    pub const fn state_err_or(self, message: &'static str) -> S {
        match self.value {
            Err(_) => self.state,
            Ok(_) => panic!["{}", message],
        }
    }
    /// Asserts the value is [`Err`] and returns the `value` field, otherwise panics.
    #[inline]
    pub const fn value_err(self) -> E {
        match self.value {
            Err(e) => e,
            Ok(_) => panic![],
        }
    }
    /// Asserts the value is [`Err`] and returns the `state` field, otherwise panics.
    #[inline]
    pub const fn value_err_or(self, message: &'static str) -> E {
        match self.value {
            Err(e) => e,
            Ok(_) => panic!["{}", message],
        }
    }

    /* unwrap */

    /// Unwraps the contained `Ok(value)` or panics.
    #[inline]
    pub const fn unwrap_const(self) -> Own<S, V> {
        if let Ok(value) = self.value {
            Own {
                state: self.state,
                value,
            }
        } else {
            panic![];
        }
    }

    /// Unwraps the contained `Ok(value)` or provides a `default`.
    #[inline]
    pub const fn unwrap_or_const(self, default: V) -> Own<S, V> {
        if let Ok(value) = self.value {
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
    #[inline]
    pub const fn expect_const(self, message: &'static str) -> Own<S, V> {
        if let Ok(value) = self.value {
            Own {
                state: self.state,
                value,
            }
        } else {
            panic!["{}", message];
        }
    }
}
