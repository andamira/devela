// devela_base_core::code::result::own
//
//! Defines [`Own`].
//

mod general;
mod state;
mod value;

#[doc = crate::_TAG_RESULT!()]
/// A return type encapsulating an owned **state** `S` and a **value** `V`.
///
/// It is designed to be used by methods that take ownership of `self`,
/// and return it alongside the operation-specific result.
///
/// By convention methods that return an `Own` should to be prefixed with `own_`,
/// and any [`Result`] or [`Option`] should be part of the `state` field for the
/// constructors, and part of the `value` field for most other methods, allowing
/// `self` to be passed along a chain of operations.
///
/// ## Methods
///
/// ### [General methods](#impl-Own<S,+V>)
/// Additional [*const* methods](#impl-Own<S,+V>-1) are available when both types are `Copy`.
/// - Construct:
///   [`new`](#method.new),
///   [`empty`](#method.empty).
/// - Deconstruct:
///   [`sv`](#method.sv)*([const](#method.sv_const))*
///   [`sv_ref`](#method.sv_ref),
///   [`sv_mut`](#method.sv_mut).
/// - Replace:
///   [`s_replace`](#method.s_replace)*([const](#method.s_const_replace))*,
///   [`v_replace`](#method.v_replace)*([const](#method.v_const_replace))*,
///   [`sv_replace`](#method.sv_replace)*([const](#method.sv_const_replace))*,
/// - Wrap:
///   [`s_wrap_ok`](#method.s_wrap_ok)*([const](#method.s_const_wrap_ok))*,
///   [`s_wrap_some`](#method.s_wrap_some)*([const](#method.s_const_wrap_some))*,
///   [`v_wrap_ok`](#method.v_wrap_ok)*([const](#method.v_const_wrap_ok))*,
///   [`v_wrap_some`](#method.v_wrap_some)*([const](#method.v_const_wrap_some))*,
/// - Map:
///   [`s_map`](#method.s_map),
///   [`v_map`](#method.v_map),
///   [`sv_map`](#method.sv_map).
/// - Query:
///   [`s_eq`](#method.s_eq),
///   [`v_eq`](#method.v_eq),
///   [`sv_eq`](#method.sv_eq).
/// - Assert:
///   [`s_assert`](#method.s_assert)*([or](#method.s_assert_or),
///     [eq](#method.s_assert_eq), [eq_or](#method.s_assert_eq_or))*,
///   [`v_assert`](#method.v_assert)*([or](#method.v_assert_or)
///     [eq](#method.v_assert_eq), [eq_or](#method.v_assert_eq_or))*,
///   [`sv_assert`](#method.sv_assert)*([or](#method.sv_assert_or),
///     [eq](#method.sv_assert_eq), [eq_or](#method.sv_assert_eq_or))*.
///
/// ### [Methods for when `state` is a `Result`](#impl-Own<Result<S,+E>,+V>)
/// - Map:
///   [`s_map_ok`](#method.s_map_ok),
///   [`s_map_err`](#method.s_map_err),
///   [`s_and`](#method.s_and),
///   [`s_and_then`](#method.s_and_then).
/// - Assert:
///   [`s_assert_ok`](#method.s_assert_ok)*([or](#method.s_assert_ok_or))*,
///   [`s_assert_err`](#method.s_assert_err)*([or](#method.s_assert_err_or))*.
/// - Unwrap:
///   [`s_unwrap`](#method.s_unwrap)*([const](#method.s_const_unwrap))*,
///   [`s_unwrap_or`](#method.s_unwrap_or)*([const](#method.s_const_unwrap_or))*,
///   [`s_expect`](#method.s_expect)*([const](#method.s_const_expect))*.
///
/// ### [Methods for when `state` is an `Option`](#impl-Own<Option<S>,+V>)
/// - Map:
///   [`s_map_some`](#method.s_map_some),
///   [`s_and`](#method.s_and-1),
///   [`s_and_then`](#method.s_and_then-1).
/// - Assert:
///   [`s_assert_some`](#method.s_assert_some)*([or](#method.s_assert_some_or))*,
///   [`s_assert_none`](#method.s_assert_none)*([or](#method.s_assert_none_or))*.
/// - Unwrap:
///   [`s_unwrap`](#method.s_unwrap-1)*([const](#method.s_const_unwrap-1))*,
///   [`s_unwrap_or`](#method.s_unwrap_or-1)*([const](#method.s_const_unwrap_or-1))*,
///   [`s_expect`](#method.s_expect-1)*([const](#method.s_const_expect-1))*.
///
/// ### [Methods for when `value` is a `Result`](#impl-Own<S,+Result<V,+E>>)
/// - Map:
///   [`v_map_ok`](#method.v_map_ok),
///   [`v_map_err`](#method.v_map_err),
///   [`v_and`](#method.v_and),
///   [`v_and_then`](#method.v_and_then).
/// - Assert:
///   [`v_assert_ok`](#method.v_assert_ok)*([or](#method.v_assert_ok_or))*,
///   [`v_assert_err`](#method.v_assert_err)*([or](#method.v_assert_err_or))*.
/// - Unwrap:
///   [`v_unwrap`](#method.v_unwrap)*([const](#method.v_const_unwrap))*,
///   [`v_unwrap_or`](#method.v_unwrap_or)*([const](#method.v_const_unwrap_or))*,
///   [`v_expect`](#method.v_expect)*([const](#method.v_const_expect))*.
///
/// ### [Methods for when `value` is an `Option`](#impl-Own<S,+Option<V>>)
/// - Map:
///   [`v_map_some`](#method.v_map_some),
///   [`v_and`](#method.v_and-1),
///   [`v_and_then`](#method.v_and_then-1).
/// - Assert:
///   [`v_assert_some`](#method.v_assert_some)*([or](#method.v_assert_some_or))*,
///   [`v_assert_none`](#method.v_assert_none)*([or](#method.v_assert_none_or))*.
/// - Unwrap:
///   [`v_unwrap`](#method.v_unwrap-1)*([const](#method.v_const_unwrap-1))*,
///   [`v_unwrap_or`](#method.v_unwrap_or-1)*([const](#method.v_const_unwrap_or-1))*,
///   [`v_expect`](#method.v_expect-1)*([const](#method.v_const_expect-1))*.
#[must_use]
pub struct Own<S, V> {
    /// The `state` after the operation.
    pub s: S,

    /// The `value` resulting from the operation.
    pub v: V,
}

mod core_impls {
    use crate::{ConstInitCore, Debug, Display, FmtResult, Formatter, Ordering, Own};

    impl<S: Default, V: Default> Default for Own<S, V> {
        fn default() -> Self {
            Own::new(S::default(), V::default())
        }
    }
    impl<S: ConstInitCore, V: ConstInitCore> ConstInitCore for Own<S, V> {
        const INIT: Self = Own::new(S::INIT, V::INIT);
    }

    impl<S: Debug, V: Debug> Debug for Own<S, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            let mut debug = f.debug_struct("Own");
            debug.field("state", &self.s).field("value", &self.v).finish()
        }
    }
    impl<S: Display, V: Display> Display for Own<S, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write!(f, "State: {}, Value: {}", self.s, self.v)
        }
    }

    impl<S: Clone, V: Clone> Clone for Own<S, V> {
        fn clone(&self) -> Self {
            Own::new(self.s.clone(), self.v.clone())
        }
    }
    impl<S: Copy, V: Copy> Copy for Own<S, V> {}

    impl<S: PartialEq, V: PartialEq> PartialEq for Own<S, V> {
        fn eq(&self, other: &Self) -> bool {
            self.s == other.s && self.v == other.v
        }
    }
    impl<S: Eq, V: Eq> Eq for Own<S, V> {}

    impl<S: PartialOrd, V: PartialOrd> PartialOrd for Own<S, V> {
        /// State's ordering takes precedence over value's ordering.
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.s.partial_cmp(&other.s) {
                Some(Ordering::Equal) => self.v.partial_cmp(&other.v),
                other => other,
            }
        }
    }
    impl<S: Ord, V: Ord> Ord for Own<S, V> {
        /// State's ordering takes precedence over value's ordering.
        fn cmp(&self, other: &Self) -> Ordering {
            match self.s.cmp(&other.s) {
                Ordering::Equal => self.v.cmp(&other.v),
                other => other,
            }
        }
    }
}
