// devela::result::chain::own::general
//
//!
//

use super::Own;
use crate::code::iif;

impl<S> Own<S, ()> {
    /// A constructor with the given `state` and an empty value.
    #[inline]
    pub const fn empty(state: S) -> Own<S, ()> {
        Own { state, value: () }
    }
}

/// # General methods.
impl<S, V> Own<S, V> {
    /// A constructor with the given `state` and `value`.
    #[inline]
    pub const fn new(state: S, value: V) -> Self {
        Own { state, value }
    }

    /// Transforms itself into a tuple.
    ///
    /// See also [`into_tuple_const`][Self::into_tuple_const] for `Copy` values.
    #[inline]
    #[must_use]
    pub fn into_tuple(self) -> (S, V) {
        (self.state, self.value)
    }

    /// Wraps the `value` field into an [`Option`].
    ///
    /// See also [`into_tuple_const`][Self::into_tuple_const] for `Copy` values.
    #[inline]
    pub fn into_option(self) -> Own<S, Option<V>> {
        Own::new_some(self.state, self.value)
    }

    /// Wraps the `value` field into a [`Result`].
    ///
    /// See also [`into_tuple_const`][Self::into_tuple_const] for `Copy` values.
    #[inline]
    pub fn into_result<E>(self) -> Own<S, Result<V, E>> {
        Own::new_ok(self.state, self.value)
    }

    /* references */

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

    /* equality test */

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

    /* assert */

    /// Asserts the `state` matches the `predicate`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the predicate returns `false`.
    #[inline]
    pub fn assert_state<F>(self, predicate: F) -> Self
    where
        F: FnOnce(&S) -> bool,
    {
        iif![predicate(&self.state); self; panic![]]
    }
    /// Asserts the `state` matches the `predicate`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the predicate returns `false`.
    #[inline]
    pub fn assert_state_or<F>(self, predicate: F, message: &str) -> Self
    where
        F: FnOnce(&S) -> bool,
    {
        iif![predicate(&self.state); self; panic!["{}", message]]
    }

    /// Asserts the `value` matches the `predicate`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the predicate returns `false`.
    #[inline]
    pub fn assert_value<F>(self, predicate: F) -> Self
    where
        F: FnOnce(&V) -> bool,
    {
        iif![predicate(&self.value); self; panic![]]
    }
    /// Asserts the `value` matches the `predicate`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the predicate returns `false`.
    #[inline]
    pub fn assert_value_or<F>(self, predicate: F, message: &str) -> Self
    where
        F: FnOnce(&V) -> bool,
    {
        iif![predicate(&self.value); self; panic!["{}", message]]
    }

    /// Asserts both the `state` and `value` matches the corresponding predicates,
    /// otherwise panics.
    ///
    /// # Panics
    /// Panics if any predicate returns `false`.
    #[inline]
    pub fn assert_both<F, G>(self, predicate_state: F, predicate_value: G) -> Self
    where
        F: FnOnce(&S) -> bool,
        G: FnOnce(&V) -> bool,
    {
        iif![predicate_state(&self.state) && predicate_value(&self.value); self; panic![]]
    }
    /// Asserts both the `state` and `value` matches the corresponding predicates,
    /// otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if any predicate returns `false`.
    #[inline]
    pub fn assert_both_or<F, G>(self, predicate_state: F, predicate_value: G, message: &str) -> Self
    where
        F: FnOnce(&S) -> bool,
        G: FnOnce(&V) -> bool,
    {
        if predicate_state(&self.state) && predicate_value(&self.value) {
            self
        } else {
            panic!["{}", message]
        }
    }

    /* assert_eq */

    /// Asserts the `state` equals `expected` and returns `self`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the `state` doesn't equal the `expected` state.
    #[inline]
    pub fn assert_eq_state(self, expected: &S) -> Self
    where
        S: PartialEq,
    {
        iif![self.state == *expected; self; panic![]]
    }
    /// Asserts the `state` equals `expected` and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the `state` doesn't equal the `expected` state.
    #[inline]
    pub fn assert_eq_state_or(self, expected: &S, message: &str) -> Self
    where
        S: PartialEq,
    {
        iif![self.state == *expected; self; panic!["{}", message]]
    }

    /// Asserts the `value` equals `expected` and returns `self`, otherwise panics.
    ///
    /// # Panics
    /// Panics if the `value` doesn't equal the `expected` value.
    #[inline]
    pub fn assert_eq_value(self, expected: &V) -> Self
    where
        V: PartialEq,
    {
        iif![self.value == *expected; self; panic![]]
    }
    /// Asserts the `value` equals `expected` and returns `self`, otherwise panics with `message`.
    ///
    /// # Panics
    /// Panics if the `value` doesn't equal the `expected` value.
    #[inline]
    pub fn assert_eq_value_or(self, expected: &V, message: &str) -> Self
    where
        V: PartialEq,
    {
        iif![self.value == *expected; self; panic!["{}", message]]
    }

    /// Asserts the `state` and `value` equals the corresponding expected ones,
    /// and returns `self`, otherwise panics.
    ///
    /// # Panics
    /// Panics if either the `state` or the `value` are not the expected ones.
    #[inline]
    pub fn assert_eq_both(self, expected_state: &S, expected_value: &V) -> Self
    where
        S: PartialEq,
        V: PartialEq,
    {
        iif![self.state == *expected_state && self.value == *expected_value; self; panic![]]
    }
    /// Asserts the `state` and `value` equals the corresponding expected ones,
    /// and returns `self`, otherwise panics with `message`
    ///
    /// # Panics
    /// Panics if either the `state` or the `value` are not the expected ones.
    #[inline]
    pub fn assert_eq_both_or(self, expected_state: &S, expected_value: &V, message: &str) -> Self
    where
        S: PartialEq,
        V: PartialEq,
    {
        if self.state == *expected_state && self.value == *expected_value {
            self
        } else {
            panic!["{}", message]
        }
    }

    /* replace */

    /// Replaces the existing `state` with a `new_state`.
    ///
    /// See also [`replace_state_const`][Self::replace_state_const] for `Copy` values.
    #[inline]
    pub fn replace_state(self, new_state: S) -> Self {
        Self::new(new_state, self.value)
    }
    /// Replaces the `value` with a `new_value`.
    ///
    /// See also [`replace_value_const`][Self::replace_value_const] for `Copy` values.
    #[inline]
    pub fn replace_value(self, new_value: V) -> Self {
        Self::new(self.state, new_value)
    }
    /// Replaces the existing `state` and `value` with `new_state` and `new_value`, respectively.
    ///
    /// See also [`replace_both_const`][Self::replace_both_const] for `Copy` values.
    #[inline]
    pub fn replace_both(self, new_state: S, new_value: V) -> Self {
        Self::new(new_state, new_value)
    }

    /* map */

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
    /// Transforms itself into a tuple, in compile-time.
    #[inline]
    #[must_use]
    pub const fn into_tuple_const(self) -> (S, V) {
        (self.state, self.value)
    }

    /// Wraps the `value` field into an [`Option`], in compile-time.
    ///
    /// See also [`into_tuple_const`][Self::into_tuple_const] for `Copy` values.
    #[inline]
    pub const fn into_option_const(self) -> Own<S, Option<V>> {
        Own::new_some(self.state, self.value)
    }

    /// Wraps the `value` field into a [`Result`], in compile-time.
    ///
    /// See also [`into_tuple_const`][Self::into_tuple_const] for `Copy` values.
    #[inline]
    pub const fn into_result_const<E>(self) -> Own<S, Result<V, E>> {
        Own::new_ok(self.state, self.value)
    }

    /// Replaces the `state` self with a `new_state`, in compile-time.
    #[inline]
    pub const fn replace_state_const(self, new_state: S) -> Self {
        Self::new(new_state, self.value)
    }
    /// Replaces the `value` with a `new_value`, in compile-time.
    #[inline]
    pub const fn replace_value_const(self, new_value: V) -> Self {
        Self::new(self.state, new_value)
    }
    /// Replaces the `state` self with a `new_state` and the `value` with a `new_value`,
    /// in compile-time.
    #[inline]
    pub const fn replace_both_const(self, new_state: S, new_value: V) -> Self {
        Self::new(new_state, new_value)
    }
}
