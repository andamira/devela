// devela::code::result::own::value
//
//!
//

use super::Own;
use crate::code::iif;

/// # Additional methods for when the `value` field is a `Result`.
impl<S, V, E> Own<S, Result<V, E>> {
    /* map (4) */

    /// Maps a `Result<V>` to a `Result<W>` by applying the `op` function
    /// to a contained [`Ok`] value, leaving an [`Err`] value untouched.
    #[inline]
    pub fn v_map_ok<W, F: FnOnce(V) -> W>(self, op: F) -> Own<S, Result<W, E>> {
        Own::new(self.s, self.v.map(op))
    }

    /// Maps a `Result<V, E>` to a `Result<V, F>` by applying the `op` function
    /// to a contained [`Err`] value, leaving an [`Ok`] value untouched.
    #[inline]
    pub fn v_map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Own<S, Result<V, F>> {
        Own::new(self.s, self.v.map_err(op))
    }

    /// Returns `res` if the result is [`Ok`], otherwise returns the [`Err`] value of `self`.
    #[inline]
    pub fn v_and<W>(self, res: Result<W, E>) -> Own<S, Result<W, E>> {
        Own::new(self.s, self.v.and(res))
    }

    /// Calls `op` if the result is [`Ok`], otherwise returns the [`Err`] value of `self`.
    #[inline]
    pub fn v_and_then<W, F: FnOnce(V) -> Result<W, E>>(self, op: F) -> Own<S, Result<W, E>> {
        Own::new(self.s, self.v.and_then(op))
    }

    /* assert (4) */

    /// Asserts the `value` is [`Ok`] and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the `value` is `Err`.
    #[inline]
    pub const fn v_assert_ok(self) -> Self {
        iif![let Ok(_) = self.v; self; panic![]]
    }

    /// Asserts the `value` is [`Ok`] and returns `self`, otherwise panics with `message`.
    /// # Panics
    /// Panics if the `value` is `Err`.
    #[inline]
    pub const fn v_assert_ok_or(self, message: &'static str) -> Self {
        iif![let Ok(_) = self.v; self; panic!["{}", message]]
    }

    /// Asserts the `value` is [`Err`] and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the `value` is `Ok`.
    #[inline]
    pub const fn v_assert_err(self) -> Self {
        iif![let Err(_) = self.v; self; panic![]]
    }
    /// Asserts the `value` is [`Err`] and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the `value` is `Ok`.
    #[inline]
    pub const fn v_assert_err_or(self, message: &'static str) -> Self {
        iif![let Err(_) = self.v; self; panic!["{}", message]]
    }

    /* unwrap (3) */

    /// Unwraps the contained `Ok(value)` or panics.
    /// # Panics
    /// Panics if the value is `Err`.
    #[inline]
    pub fn v_unwrap(self) -> Own<S, V> {
        iif![let Ok(v) = self.v; Own::new(self.s, v); panic![]]
    }

    /// Unwraps the contained `Ok(value)` or provides a `default`.
    #[inline]
    pub fn v_unwrap_or(self, default: V) -> Own<S, V> {
        Own::new(self.s, self.v.unwrap_or(default))
    }

    /// Unwraps the contained `Ok(value)` or panics with a `message`.
    /// # Panics
    /// Panics if the value is `Err`.
    #[inline] #[rustfmt::skip]
    pub fn v_expect(self, message: &str) -> Own<S, V> where E: core::fmt::Debug {
        Own::new(self.s, self.v.expect(message))
    }
}

/// # *const* methods for when everything is `Copy` and the `value` is a `Result`.
impl<S: Copy, V: Copy, E: Copy> Own<S, Result<V, E>> {
    /* unwrap (3) */

    /// Unwraps the contained `Ok(value)` or panics.
    ///
    /// # Panics
    /// Panics if the value is `Err`.
    #[inline]
    pub const fn v_const_unwrap(self) -> Own<S, V> {
        iif![let Ok(v) = self.v; Own::new(self.s, v); panic![]]
    }

    /// Unwraps the contained `Ok(value)` or provides a `default`.
    #[inline]
    pub const fn v_const_unwrap_or(self, default: V) -> Own<S, V> {
        iif![let Ok(v) = self.v; Own::new(self.s, v); Own::new(self.s, default)]
    }

    /// Unwraps the contained `Ok(value)` or panics with the given `message`.
    ///
    /// # Panics
    /// Panics if the value is `Err`.
    #[inline]
    pub const fn v_const_expect_const(self, message: &'static str) -> Own<S, V> {
        iif![let Ok(v) = self.v; Own::new(self.s, v); panic!["{}", message]]
    }
}

/// # Additional methods for when the `value` field is an `Option`.
impl<S, V> Own<S, Option<V>> {
    /* map */

    /// Maps an `Option<V>` to an `Option<W>` by applying the `op` function
    /// to a contained value (if `Some`), or returns `None` (if `None`).
    #[inline]
    pub fn v_map_some<W, F: FnOnce(V) -> W>(self, op: F) -> Own<S, Option<W>> {
        Own::new(self.s, self.v.map(op))
    }

    /// Returns [`None`] if the value is `None`,
    /// otherwise returns `optb`.
    #[inline]
    pub fn v_and<W>(self, optb: Option<W>) -> Own<S, Option<W>> {
        Own::new(self.s, self.v.and(optb))
    }

    /// Returns [`None`] if the value is `None`,
    /// otherwise calls `op` with the wrapped value and returns the result.
    #[inline]
    pub fn v_and_then<W, F: FnOnce(V) -> Option<W>>(self, op: F) -> Own<S, Option<W>> {
        Own::new(self.s, self.v.and_then(op))
    }

    /* assert (4) */

    /// Asserts the value is [`Some`] and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn v_assert_some(self) -> Self {
        iif![let Some(_) = self.v; self; panic![]]
    }

    /// Asserts the value is [`Some`] and returns `self`, otherwise panics with `message`.
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn v_assert_some_or(self, message: &'static str) -> Self {
        iif![let Some(_) = self.v; self; panic!["{}", message]]
    }

    /// Asserts the value is [`None`] and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the value is `Some`.
    #[inline]
    pub const fn v_assert_none(self) -> Self {
        iif![let None = self.v; self; panic![]]
    }

    /// Asserts the value is [`None`] and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the value is `Some`.
    #[inline]
    pub const fn v_assert_none_or(self, message: &'static str) -> Self {
        iif![let None = self.v; self; panic!["{}", message]]
    }

    /* unwrap (3) */

    /// Unwraps the contained `Some(value)` or panics.
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub fn v_unwrap(self) -> Own<S, V> {
        Own::new(self.s, self.v.unwrap())
    }

    /// Unwraps the contained `Some(value)` or provides a `default`.
    #[inline]
    pub fn v_unwrap_or(self, default: V) -> Own<S, V> {
        Own::new(self.s, self.v.unwrap_or(default))
    }

    /// Unwraps the contained `Some(value)` or panics with the given `message`.
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub fn v_expect(self, message: &str) -> Own<S, V> {
        Own::new(self.s, self.v.expect(message))
    }
}

/// # *const* methods for when everything is `Copy` and the `value` is an `Option`.
impl<S: Copy, V: Copy> Own<S, Option<V>> {
    /* unwrap (3) */

    /// Unwraps the contained `Some(value)` or panics.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn v_const_unwrap(self) -> Own<S, V> {
        iif![let Some(v) = self.v; Own::new(self.s, v); panic![]]
    }

    /// Unwraps the contained `Some(value)` or provides a `default`.
    #[inline]
    pub const fn v_const_unwrap_or(self, default: V) -> Own<S, V> {
        iif![let Some(v) = self.v; Own::new(self.s, v); Own::new(self.s, default)]
    }

    /// Unwraps the contained `Some(value)` or panics with the given `message`.
    ///
    /// # Panics
    /// Panics if the value is `None`.
    #[inline]
    pub const fn v_const_expect(self, message: &'static str) -> Own<S, V> {
        iif![let Some(v) = self.v; Own::new(self.s, v); panic!["{}", message]]
    }
}
