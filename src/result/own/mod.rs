// devela::result::own
//
//!
//

// the order of the modules determines the order of the impl blocks in the docs
mod general;

mod result_state;
mod result_value;

mod option_state;
mod option_value;

/// A return type encapsulating an owned `state` alongside a return `value`.
///
/// It is designed to be used by methods that take ownership of `self`,
/// and return it alongside the operation-specific result.
///
/// By convention methods that return an `Own` should to be prefixed with `own_`,
/// and any [`Result`] or [`Option`] should be part of the `state` field for
/// constructors, and of the `value` field for most other methods, allowing
/// `self` to be passed along a chain of operations.
///
/// ## Available methods
/// - [`Own<S, ()>`](#impl-Own<S,+()>):
/// [`empty`](#method_empty)
///
/// - [`Own<S, V>`](#impl-Own<S,+V>):
///
/// [`new`](#method.new),
/// [`into_tuple`](#method.into_tuple),
/// [`state_into_option`](#method.state_into_option),
/// [`state_into_result`](#method.state_into_result),
/// [`value_into_option`](#method.value_into_option),
/// [`value_into_result`](#method.value_into_result),
/// [`ref_state`](#method.ref_state),
/// [`ref_value`](#method.ref_value),
/// [`ref_both`](#method.ref_both),
/// [`mut_state`](#method.mut_state),
/// [`mut_value`](#method.mut_value),
/// [`mut_both`](#method.mut_both),
/// [`is_state`](#method.is_state),
/// [`is_value`](#method.is_value),
/// [`are_both`](#method.are_both),
/// [`assert_state`](#method.assert_state),
/// [`assert_state_or`](#method.assert_state_or),
/// [`assert_value`](#method.assert_value),
/// [`assert_value_or`](#method.assert_value_or),
/// [`assert_both`](#method.assert_both),
/// [`assert_both_or`](#method.assert_both_or),
/// [`assert_eq_state`](#method.assert_eq_state),
/// [`assert_eq_state_or`](#method.assert_eq_state_or),
/// [`assert_eq_value`](#method.assert_eq_value),
/// [`assert_eq_value_or`](#method.assert_eq_value_or),
/// [`assert_eq_both`](#method.assert_eq_both),
/// [`assert_eq_both_or`](#method.assert_eq_both_or),
/// [`replace_state`](#method.replace_state),
/// [`replace_value`](#method.replace_value),
/// [`replace_both`](#method.replace_both),
/// [`map_state`](#method.map_state),
/// [`map_value`](#method.map_value),
/// [`map_both`](#method.map_both),
///
/// - [`Own<S: Copy, V: Copy>`](#impl-Own<S,+V>-1):
///
/// [`const_into_tuple`](#method.const_into_tuple),
/// [`const_state_into_option`](#method.const_state_into_option),
/// [`const_state_into_result`](#method.const_state_into_result),
/// [`const_value_into_option`](#method.const_value_into_option),
/// [`const_value_into_result`](#method.const_value_into_result),
/// [`const_replace_state`](#method.const_replace_state),
/// [`const_replace_value`](#method.const_replace_value),
/// [`const_replace_both`](#method.const_replace_both).
///
/// ### Implemented when the `state` is a `Result`:
///
/// - [`Own<Result<S, E>, V>`](#impl-Own<Result<S,+E>,+V>):
///
/// [`new_ok_state`](#method.new_ok_state),
/// [`new_err_state`](#method.new_err_state),
/// [`is_state_ok`](#method.is_state_ok),
/// [`is_state_ok_and`](#method.is_state_ok_and),
/// [`is_state_err`](#method.is_state_err),
/// [`is_state_err_and`](#method.is_state_err_and),
/// [`assert_state_ok`](#method.assert_state_ok),
/// [`assert_state_ok_or`](#method.assert_state_ok_or),
/// [`assert_state_err`](#method.assert_state_err),
/// [`assert_state_err_or`](#method.assert_state_err_or),
/// [`state_map`](#method.state_map),
/// [`state_map_err`](#method.state_map_err),
/// [`state_and`](#method.state_and),
/// [`state_and_then`](#method.state_and_then),
/// [`state_unwrap`](#method.state_unwrap),
/// [`state_unwrap_or`](#method.state_unwrap_or),
/// [`state_expect`](#method.state_expect),
/// [`state_ok`](#method.state_ok),
/// [`state_err`](#method.state_err),
///
/// - [`Own<Result<S: Copy, E: Copy>, V: Copy>`](#impl-Own<Result<S,+E>,+V>-1):
///
/// [`state_ok_state`](#method.state_ok_state),
/// [`state_ok_state_or`](#method.state_ok_state_or),
/// [`state_ok_value`](#method.state_ok_value),
/// [`state_ok_value_or`](#method.state_ok_value_or),
/// [`state_err_state`](#method.state_err_state),
/// [`state_err_state_or`](#method.state_err_state_or),
/// [`state_err_value`](#method.state_err_value),
/// [`state_err_value_or`](#method.state_err_value_or),
/// [`const_state_unwrap`](#method.const_state_unwrap),
/// [`const_state_unwrap_or`](#method.const_state_unwrap_or),
/// [`const_state_expect`](#method.const_state_expect),
///
/// ### Implemented when the `value` is a `Result`:
///
/// - [`Own<S, Result<V, E>>`](#impl-Own<S,+Result<V,+E>>):
///
/// [`new_ok_value`](#method.new_ok_value),
/// [`new_err_value`](#method.new_err_value),
/// [`is_value_ok`](#method.is_value_ok),
/// [`is_value_ok_and`](#method.is_value_ok_and),
/// [`is_value_err`](#method.is_value_err),
/// [`is_value_err_and`](#method.is_value_err_and),
/// [`assert_value_ok`](#method.assert_value_ok),
/// [`assert_value_ok_or`](#method.assert_value_ok_or),
/// [`assert_value_err`](#method.assert_value_err),
/// [`assert_value_err_or`](#method.assert_value_err_or),
/// [`value_map`](#method.value_map),
/// [`value_map_err`](#method.value_map_err),
/// [`value_and`](#method.value_and),
/// [`value_and_then`](#method.value_and_then),
/// [`value_unwrap`](#method.value_unwrap),
/// [`value_unwrap_or`](#method.value_unwrap_or),
/// [`value_expect`](#method.value_expect),
/// [`value_ok`](#method.value_ok),
/// [`value_err`](#method.value_err),
///
/// - [`Own<S: Copy, Result<V: Copy, E: Copy>>`](#impl-Own<S,+Result<V,+E>>-1):
///
/// [`value_ok_state`](#method.value_ok_state),
/// [`value_ok_state_or`](#method.value_ok_state_or),
/// [`value_ok_value`](#method.value_ok_value),
/// [`value_ok_value_or`](#method.value_ok_value_or),
/// [`value_err_state`](#method.value_err_state),
/// [`value_err_state_or`](#method.value_err_state_or),
/// [`value_err_value`](#method.value_err_value),
/// [`value_err_value_or`](#method.value_err_value_or),
/// [`const_value_unwrap`](#method.const_value_unwrap),
/// [`const_value_unwrap_or`](#method.const_value_unwrap_or),
/// [`const_value_expect_const`](#method.const_value_expect_const),
///
/// ### Implemented when the `state` is an `Option`:
///
/// - [`Own<Option<S>, V>`](#impl-Own<Option<S>,+V>):
///
/// [`new_some_state`](#method.new_some_state),
/// [`new_none_state`](#method.new_none_state),
/// [`is_state_some`](#method.is_state_some),
/// [`is_state_some_and`](#method.is_state_some_and),
/// [`is_state_none`](#method.is_state_none),
/// [`assert_state_some`](#method.assert_state_some),
/// [`assert_state_some_or`](#method.assert_state_some_or),
/// [`assert_state_none`](#method.assert_state_none),
/// [`assert_state_none_or`](#method.assert_state_none_or),
/// [`state_map`](#method.state_map-1),
/// [`state_and`](#method.state_and-1),
/// [`state_and_then`](#method.state_and_then-1),
/// [`state_unwrap`](#method.state_unwrap-1),
/// [`state_unwrap_or`](#method.state_unwrap_or-1),
/// [`state_expect`](#method.state_expect-1),
/// [`state_ok_or`](#method.state_ok_or),
/// [`state_ok_or_else`](#method.state_ok_or_else),
///
/// - [`Own<S: Copy, Option<V: Copy>>`](#impl-Own<Option<S>,+V>-1):
///
/// [`state_some_state`](#method.state_some_state),
/// [`state_some_state_or`](#method.state_some_state_or),
/// [`state_some_value`](#method.state_some_value),
/// [`state_some_value_or`](#method.state_some_value_or),
/// [`state_none_value_or`](#method.state_none_value_or),
/// [`const_state_unwrap`](#method.const_state_unwrap-1),
/// [`const_state_unwrap_or`](#method.const_state_unwrap_or-1),
/// [`const_state_expect`](#method.const_state_expect-1),
///
/// ### Implemented when the `value` is an `Option`:
///
/// - [`Own<S, Option<V>>`](#impl-Own<S,+Option<V>>):
///
/// [`new_some_value`](#method.new_some_value),
/// [`new_none_value`](#method.new_none_value),
/// [`is_value_some`](#method.is_value_some),
/// [`is_value_some_and`](#method.is_value_some_and),
/// [`is_value_none`](#method.is_value_none),
/// [`assert_value_some`](#method.assert_value_some),
/// [`assert_value_some_or`](#method.assert_value_some_or),
/// [`assert_value_none`](#method.assert_value_none),
/// [`assert_value_none_or`](#method.assert_value_none_or),
/// [`value_map`](#method.value_map-1),
/// [`value_and`](#method.value_and-1),
/// [`value_and_then`](#method.value_and_then-1),
/// [`value_unwrap`](#method.value_unwrap-1),
/// [`value_unwrap_or`](#method.value_unwrap_or-1),
/// [`value_expect`](#method.value_expect-1),
/// [`value_ok_or`](#method.value_ok_or-1),
/// [`value_ok_or_else`](#method.value_ok_or_else),
///
/// - [`Own<S: Copy, Option<V: Copy>>`](#impl-Own<S,+Option<V>>-1):
///
/// [`value_some_state`](#method.value_some_state),
/// [`value_some_state_or`](#method.value_some_state_or),
/// [`value_some_value`](#method.value_some_value),
/// [`value_some_value_or`](#method.value_some_value_or),
/// [`value_none_state`](#method.value_none_state),
/// [`value_none_state_or`](#method.value_none_state_or),
/// [`const_value_unwrap`](#method.const_value_unwrap-1),
/// [`const_value_unwrap_or`](#method.const_value_unwrap_or-1),
/// [`const_value_expect`](#method.const_value_expect-1),

#[must_use]
pub struct Own<S, V> {
    /// The new state after the operation.
    pub state: S,

    /// The value resulting from the operation.
    pub value: V,
}

mod core_impls {
    use {
        super::Own,
        crate::code::ConstDefault,
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
    impl<S: ConstDefault, V: ConstDefault> ConstDefault for Own<S, V> {
        const DEFAULT: Self = Self {
            state: S::DEFAULT,
            value: V::DEFAULT,
        };
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
