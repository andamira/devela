// devela::result::chain::own::option
//
//!
//

use super::Own;

/// # Additional methods for when the `value` field is an `Option`.
impl<S, V> Own<S, Option<V>> {
    /* is */

    /// Returns [`true`] if the `value` is [`Some`].
    #[inline]
    #[must_use]
    pub const fn is_some(&self) -> bool {
        self.value.is_some()
    }
    /// Returns [`true`] if the `value` is [`Some`] and it matches a predicate.
    #[inline]
    #[must_use]
    pub fn is_some_and(self, f: impl FnOnce(V) -> bool) -> bool {
        self.value.is_some_and(f)
    }

    /// Returns [`true`] if the `value` is [`None`].
    #[inline]
    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.value.is_none()
    }

    /* map */

    /// Maps an `Option<V>` to an `Option<W>` by applying the `op` function
    /// to a contained value (if `Some`), or returns `None` (if `None`).
    #[inline]
    pub fn map<F, W>(self, op: F) -> Own<S, Option<W>>
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
    pub fn and<W>(self, optb: Option<W>) -> Own<S, Option<W>> {
        Own {
            state: self.state,
            value: self.value.and(optb),
        }
    }

    /// Returns [`None`] if the value is `None`,
    /// otherwise calls `op` with the wrapped value and returns the result.
    #[inline]
    pub fn and_then<F, W>(self, op: F) -> Own<S, Option<W>>
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
    #[inline]
    pub fn unwrap(self) -> Own<S, V> {
        Own {
            state: self.state,
            value: self.value.unwrap(),
        }
    }

    /// Unwraps the contained `Some(value)` or provides a `default`.
    #[inline]
    pub fn unwrap_or(self, default: V) -> Own<S, V> {
        Own {
            state: self.state,
            value: self.value.unwrap_or(default),
        }
    }

    /// Unwraps the contained `Some(value)` or panics with the given `message`.
    #[inline]
    pub fn expect(self, message: &str) -> Own<S, V> {
        Own {
            state: self.state,
            value: self.value.expect(message),
        }
    }
}

/// # *const* methods for when everything is `Copy` and the `value` is an `Option`.
impl<S: Copy, V: Copy> Own<S, Option<V>> {
    /* unwrap */

    /// Unwraps the contained `Some(value)` or panics.
    #[inline]
    pub const fn unwrap_const(self) -> Own<S, V> {
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
    pub const fn unwrap_or_const(self, default: V) -> Own<S, V> {
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
    #[inline]
    pub const fn expect_const(self, message: &'static str) -> Own<S, V> {
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
