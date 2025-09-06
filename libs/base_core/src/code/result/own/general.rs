// devela_base_core::code::result::own::general
//
//!
//
// methods: 1 + 25 + 8 = 34

use crate::{Own, is};

/* constructors (2) */

impl<S> Own<S, ()> {
    /// A constructor with the given `state` and an empty value.
    pub const fn empty(state: S) -> Own<S, ()> {
        Own::new(state, ())
    }
}

/// # General methods.
impl<S, V> Own<S, V> {
    /// A constructor with the given `state` and `value`.
    pub const fn new(state: S, value: V) -> Self {
        Own { s: state, v: value }
    }

    /* destructors (3) */

    /// Returns both `state` and `value` as a tuple.
    #[rustfmt::skip]
    pub fn sv(self) -> (S, V) {
        (self.s, self.v)
    }

    /// Returns shared references to both `state` and `value`.
    #[rustfmt::skip]
    pub const fn sv_ref(&self) -> (&S, &V) {
        (&self.s, &self.v)
    }

    /// Returns exclusive references to both `state` and `value`.
    #[rustfmt::skip]
    pub fn sv_mut(&mut self) -> (&mut S, &mut V) {
        (&mut self.s, &mut self.v)
    }

    /* replace (3) */

    /// Replaces the existing `state` with a `new_state`.
    pub fn s_replace(self, new_state: S) -> Self {
        Self::new(new_state, self.v)
    }
    /// Replaces the `value` with a `new_value`.
    pub fn v_replace(self, new_value: V) -> Self {
        Self::new(self.s, new_value)
    }
    /// Replaces the existing `state` and `value` with `new_state` and `new_value`, respectively.
    pub fn sv_replace(self, new_state: S, new_value: V) -> Self {
        Self::new(new_state, new_value)
    }

    /* wrap (4) */

    /// Wraps the `state` field into [`Ok`].
    pub fn s_wrap_ok<E>(self) -> Own<Result<S, E>, V> {
        Own::new(Ok(self.s), self.v)
    }
    /// Wraps the `state` field into [`Some`].
    pub fn s_wrap_some(self) -> Own<Option<S>, V> {
        Own::new(Some(self.s), self.v)
    }

    /// Wraps the `value` field into [`Ok`].
    pub fn v_wrap_ok<E>(self) -> Own<S, Result<V, E>> {
        Own::new(self.s, Ok(self.v))
    }
    /// Wraps the `value` field into [`Some`].
    pub fn v_wrap_some(self) -> Own<S, Option<V>> {
        Own::new(self.s, Some(self.v))
    }

    /* map (3) */

    /// Applies a mapping function to the state.
    #[rustfmt::skip]
    pub fn s_map<T, F: FnOnce(S) -> T>(self, f: F) -> Own<T, V> {
        Own::new(f(self.s), self.v)
    }

    /// Applies a mapping function to the value.
    #[rustfmt::skip]
    pub fn v_map<W, F: FnOnce(V) -> W>(self, f: F) -> Own<S, W> {
        Own::new(self.s, f(self.v))
    }

    /// Applies applies a separate mapping function to the state and value.
    #[rustfmt::skip]
    pub fn sv_map<F, G, T, W>(self, sf: F, vf: G) -> Own<T, W>
    where F: FnOnce(S) -> T, G: FnOnce(V) -> W {
        Own::new(sf(self.s), vf(self.v))
    }

    /* equality test (3) */

    /// Returns `true` if the current `state` equals the given `expected` state.
    #[must_use] #[rustfmt::skip]
    pub fn s_eq(&self, expected: &S) -> bool where S: PartialEq {
        self.s == *expected
    }
    /// Returns `true` if the current `value` equals the given `expected` value.
    #[must_use] #[rustfmt::skip]
    pub fn v_eq(&self, expected: &V) -> bool where V: PartialEq {
        self.v == *expected
    }
    /// Returns `true` if the current `state` and `value` equals the corresponding expected ones.
    #[must_use] #[rustfmt::skip]
    pub fn sv_eq(&self, expected_state: &S, expected_value: &V) -> bool
    where S: PartialEq, V: PartialEq {
        self.s == *expected_state && self.v == *expected_value
    }

    /* assert (or, eq, eq_or) (12) */

    /// Asserts the `state` matches the `predicate`, otherwise panics.
    /// # Panics
    /// Panics if the predicate returns `false`.
    #[rustfmt::skip]
    pub fn s_assert<F: FnOnce(&S) -> bool>(self, predicate: F) -> Self {
        is![predicate(&self.s); self; panic![]]
    }
    /// Asserts the `state` matches the `predicate`, otherwise panics with `message`.
    /// # Panics
    /// Panics if the predicate returns `false`.
    #[rustfmt::skip]
    pub fn s_assert_or<F: FnOnce(&S) -> bool>(self, predicate: F, message: &str) -> Self {
        is![predicate(&self.s); self; panic!["{}", message]]
    }
    /// Asserts the `state` equals `expected` and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the `state` doesn't equal the `expected` state.
    #[rustfmt::skip]
    pub fn s_assert_eq(self, expected_state: &S) -> Self where S: PartialEq {
        is![self.s == *expected_state; self; panic![]]
    }
    /// Asserts the `state` equals `expected` and returns `self`, otherwise panics with `message`.
    /// # Panics
    /// Panics if the `state` doesn't equal the `expected` state.
    #[rustfmt::skip]
    pub fn s_assert_eq_or(self, expected_state: &S, message: &str) -> Self where S: PartialEq {
        is![self.s == *expected_state; self; panic!["{}", message]]
    }

    /// Asserts the `value` matches the `predicate`, otherwise panics.
    /// # Panics
    /// Panics if the predicate returns `false`.
    #[rustfmt::skip]
    pub fn v_assert<F: FnOnce(&V) -> bool>(self, predicate: F) -> Self {
        is![predicate(&self.v); self; panic![]]
    }
    /// Asserts the `value` matches the `predicate`, otherwise panics with `message`.
    /// # Panics
    /// Panics if the predicate returns `false`.
    #[rustfmt::skip]
    pub fn v_assert_or<F: FnOnce(&V) -> bool>(self, predicate: F, message: &str) -> Self {
        is![predicate(&self.v); self; panic!["{}", message]]
    }
    /// Asserts the `value` equals `expected` and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if the `value` doesn't equal the `expected` value.
    #[rustfmt::skip]
    pub fn v_assert_eq(self, expected_value: &V) -> Self where V: PartialEq {
        is![self.v == *expected_value; self; panic![]]
    }
    /// Asserts the `value` equals `expected` and returns `self`, otherwise panics with `message`.
    /// # Panics
    /// Panics if the `value` doesn't equal the `expected` value.
    #[rustfmt::skip]
    pub fn v_assert_eq_or(self, expected_value: &V, message: &str) -> Self where V: PartialEq {
        is![self.v == *expected_value; self; panic!["{}", message]]
    }

    /// Asserts both the `state` and `value` matches the corresponding predicates,
    /// otherwise panics.
    /// # Panics
    /// Panics if any predicate returns `false`.
    #[rustfmt::skip]
    pub fn sv_assert<F, G>(self, s_predicate: F, v_predicate: G) -> Self
    where F: FnOnce(&S) -> bool, G: FnOnce(&V) -> bool {
        is![s_predicate(&self.s) && v_predicate(&self.v); self; panic![]]
    }
    /// Asserts both the `state` and `value` matches the corresponding predicates,
    /// otherwise panics with `message`.
    /// # Panics
    /// Panics if any predicate returns `false`.
    #[rustfmt::skip]
    pub fn sv_assert_or<F, G>(self, s_predicate: F, v_predicate: G, message: &str) -> Self
    where F: FnOnce(&S) -> bool, G: FnOnce(&V) -> bool {
        is![s_predicate(&self.s) && v_predicate(&self.v); self; panic!["{}", message]]
    }
    /// Asserts the `state` and `value` equals the corresponding expected ones,
    /// and returns `self`, otherwise panics.
    /// # Panics
    /// Panics if either the `state` or the `value` are not the expected ones.
    #[rustfmt::skip]
    pub fn sv_assert_eq(self, expected_state: &S, expected_value: &V) -> Self
    where S: PartialEq, V: PartialEq {
        is![self.s == *expected_state && self.v == *expected_value; self; panic![]]
    }
    /// Asserts the `state` and `value` equals the corresponding expected ones,
    /// and returns `self`, otherwise panics with `message`
    /// # Panics
    /// Panics if either the `state` or the `value` are not the expected ones.
    #[rustfmt::skip]
    pub fn sv_assert_eq_or(self, expected_state: &S, expected_value: &V, message: &str) -> Self
    where S: PartialEq, V: PartialEq {
        is![self.s == *expected_state && self.v == *expected_value; self; panic!["{}", message]]
    }
}

/// # Additional *const* methods for when everything is `Copy`.
impl<S: Copy, V: Copy> Own<S, V> {
    // (7)
    /// Transforms itself into a tuple, in compile-time.
    #[must_use] #[rustfmt::skip]
    pub const fn sv_const(self) -> (S, V) {
        (self.s, self.v)
    }

    /// Replaces the `state` self with a `new_state`, in compile-time.
    pub const fn s_const_replace(self, s: S) -> Self {
        Self::new(s, self.v)
    }
    /// Replaces the `value` with a `new_value`, in compile-time.
    pub const fn v_const_replace(self, v: V) -> Self {
        Self::new(self.s, v)
    }
    /// Replaces the `state` self with a `new_state` and the `value` with a `new_value`,
    /// in compile-time.
    pub const fn sv_const_replace(self, new_state: S, new_value: V) -> Self {
        Self::new(new_state, new_value)
    }

    /// Wraps the `state` field into a [`Result`], in compile-time.
    pub const fn s_const_wrap_ok<E>(self) -> Own<Result<S, E>, V> {
        Own::new(Ok(self.s), self.v)
    }
    /// Wraps the `state` field into an [`Option`], in compile-time.
    pub const fn s_const_wrap_some(self) -> Own<Option<S>, V> {
        Own::new(Some(self.s), self.v)
    }

    /// Wraps the `value` field into a [`Result`], in compile-time.
    pub const fn v_const_wrap_ok<E>(self) -> Own<S, Result<V, E>> {
        Own::new(self.s, Ok(self.v))
    }
    /// Wraps the `value` field into an [`Option`], in compile-time.
    pub const fn v_const_wrap_some(self) -> Own<S, Option<V>> {
        Own::new(self.s, Some(self.v))
    }
}
