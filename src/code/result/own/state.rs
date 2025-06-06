// devela::code::result::own::state
//
//! Methods .
//
//

use crate::{Debug, Own, is};

/* Result<S, E> */

/// # Additional methods for when the `state` is a `Result`.
impl<S, E, V> Own<Result<S, E>, V> {
    /* map (4) */

    /// Maps a `Result<S>` to a `Result<T>` by applying the `op` function
    /// to a contained [`Ok`] value, leaving an [`Err`] value untouched.
    #[rustfmt::skip]
    pub fn s_map_ok<T, F: FnOnce(S) -> T>(self, op: F) -> Own<Result<T, E>, V> {
        Own::new(self.s.map(op), self.v)
    }

    /// Maps a `Result<S, E>` to a `Result<S, F>` by applying the `op` function
    /// to a contained [`Err`] value, leaving an [`Ok`] value untouched.
    #[rustfmt::skip]
    pub fn s_map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Own<Result<S, F>, V> {
        Own::new(self.s.map_err(op), self.v)
    }

    /// Returns `res` if the state is [`Ok`], otherwise returns the [`Err`] value of `self`.
    pub fn s_and<T>(self, res: Result<T, E>) -> Own<Result<T, E>, V> {
        Own::new(self.s.and(res), self.v)
    }

    /// Calls `op` if the state is [`Ok`], otherwise returns the [`Err`] value of `self`.
    #[rustfmt::skip]
    pub fn s_and_then<T, F: FnOnce(S) -> Result<T, E>>(self, op: F) -> Own<Result<T, E>, V> {
        Own::new(self.s.and_then(op), self.v)
    }

    /* assert (4) */

    /// Asserts the `state` is [`Ok`] and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the state is `Err`.
    pub const fn s_assert_ok(self) -> Self {
        is![let Ok(_) = self.s; self; panic![]]
    }

    /// Asserts the `state` is [`Ok`] and returns `self`, otherwise panics with `message`.
    /// # Panics
    /// Panics if the `state` is `Err`.
    pub const fn s_assert_ok_or(self, message: &'static str) -> Self {
        is![let Ok(_) = self.s; self; panic!["{}", message]]
    }

    /// Asserts the `state` is [`Err`] and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the `state` is `Ok`.
    pub const fn s_assert_err(self) -> Self {
        is![let Err(_) = self.s; self; panic![]]
    }
    /// Asserts the `state` is [`Err`] and returns `self`, otherwise panics with `message`.
    /// # Panics
    /// Panics if the `state` is `Ok`.
    pub const fn s_assert_err_or(self, message: &'static str) -> Self {
        is![let Err(_) = self.s; self; panic!["{}", message]]
    }

    /* unwrap (3) */

    /// Unwraps the contained `Ok(state)` or panics.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    pub fn s_unwrap(self) -> Own<S, V> {
        is![let Ok(s) = self.s; Own::new(s, self.v); panic![]]
    }

    /// Unwraps the contained `Ok(state)` or provides a `default`.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    pub fn s_unwrap_or(self, default: S) -> Own<S, V> {
        Own::new(self.s.unwrap_or(default), self.v)
    }

    /// Unwraps the contained `Ok(state)` or panics with a `message`.
    ///
    /// # Panics
    /// Panics if the state is `Err`.
    #[rustfmt::skip]
    pub fn s_expect(self, message: &str) -> Own<S, V> where E: Debug {
        Own::new(self.s.expect(message), self.v)
    }
}

/// # *const* methods for when everything is `Copy` and the `state` is a `Result`.
impl<S: Copy, E: Copy, V: Copy> Own<Result<S, E>, V> {
    /* unwrap (3) */

    /// Unwraps the contained `Ok(state)` or panics.
    /// # Panics
    /// Panics if the state is `Err`.
    pub const fn s_const_unwrap(self) -> Own<S, V> {
        is![let Ok(s) = self.s; Own::new(s, self.v); panic![]]
    }

    /// Unwraps the contained `Ok(state)` or provides a `default`.
    pub const fn s_const_unwrap_or(self, default: S) -> Own<S, V> {
        is![let Ok(s) = self.s; Own::new(s, self.v); Own::new(default, self.v)]
    }

    /// Unwraps the contained `Ok(state)` or panics with the given `message`.
    /// # Panics
    /// Panics if the state is `Err`.
    pub const fn s_const_expect(self, message: &'static str) -> Own<S, V> {
        is![let Ok(s) = self.s; Own::new(s, self.v); panic!["{}", message]]
    }
}

/* Option<S> */

/// # Additional methods for when the `value` field is an `Option`.
impl<S, V> Own<Option<S>, V> {
    /* map (4) */

    /// Maps an `Option<S>` to an `Option<T>` by applying the `op` function
    /// to a contained state (if `Some`), or returns `None` (if `None`).
    #[rustfmt::skip]
    pub fn s_map_some<T, F: FnOnce(S) -> T>(self, op: F) -> Own<Option<T>, V> {
        Own::new(self.s.map(op), self.v)
    }

    /// Returns [`None`] if the state is `None`, otherwise returns `optb`.
    pub fn s_and<T>(self, res: Option<T>) -> Own<Option<T>, V> {
        Own::new(self.s.and(res), self.v)
    }

    /// Returns [`None`] if the state is `None`,
    /// otherwise calls `op` with the wrapped state and returns the result.
    #[rustfmt::skip]
    pub fn s_and_then<T, F: FnOnce(S) -> Option<T>>(self, op: F) -> Own<Option<T>, V> {
        Own::new(self.s.and_then(op), self.v)
    }

    /* assert (4) */

    /// Asserts the state is [`Some`] and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the state is `None`.
    pub const fn s_assert_some(self) -> Self {
        is![let Some(_) = self.s; self; panic![]]
    }

    /// Asserts the state is [`Some`] and returns `self`, otherwise panics with `message`.
    /// # Panics
    /// Panics if the state is `None`.
    pub const fn s_assert_some_or(self, message: &'static str) -> Self {
        is![let Some(_) = self.s; self; panic!["{}", message]]
    }

    /// Asserts the state is [`None`] and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the state is `Some`.
    pub const fn s_assert_none(self) -> Self {
        is![let None = self.s; self; panic![]]
    }

    /// Asserts the state is [`None`] and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the state is `Some`.
    pub const fn s_assert_none_or(self, message: &'static str) -> Self {
        is![let None = self.s; self; panic!["{}", message]]
    }

    /* unwrap (3) */

    /// Unwraps the contained `Some(state)` or panics.
    /// # Panics
    /// Panics if the state is `None`.
    pub fn s_unwrap(self) -> Own<S, V> {
        Own::new(self.s.unwrap(), self.v)
    }

    /// Unwraps the contained `Some(state)` or provides a `default`.
    pub fn s_unwrap_or(self, default: S) -> Own<S, V> {
        Own::new(self.s.unwrap_or(default), self.v)
    }

    /// Unwraps the contained `Some(state)` or panics with the given `message`.
    /// # Panics
    /// Panics if the state is `None`.
    pub fn s_expect(self, message: &str) -> Own<S, V> {
        Own::new(self.s.expect(message), self.v)
    }
}

/// # *const* methods for when everything is `Copy` and the `value` is an `Option`.
impl<S: Copy, V: Copy> Own<Option<S>, V> {
    /* unwrap (3) */

    /// Unwraps the contained `Some(state)` or panics.
    /// # Panics
    /// Panics if the state is `None`.
    pub const fn s_const_unwrap(self) -> Own<S, V> {
        is![let Some(s) = self.s; Own::new(s, self.v); panic![]]
    }

    /// Unwraps the contained `Some(state)` or provides a `default`.
    pub const fn s_const_unwrap_or(self, default: S) -> Own<S, V> {
        is![let Some(s) = self.s; Own::new(s, self.v); Own::new(default, self.v)]
    }

    /// Unwraps the contained `Some(state)` or panics with the given `message`.
    /// # Panics
    /// Panics if the state is `None`.
    pub const fn s_const_expect(self, message: &'static str) -> Own<S, V> {
        is![let Some(s) = self.s; Own::new(s, self.v); panic!["{}", message]]
    }
}
