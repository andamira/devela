// devela_base_core::num::fin::logic::bool
//
//! Type-level booleans.
//
// TOC
// - trait ConstBool
// - macro const_bool!
// - types True, False

use crate::sf;

#[doc = crate::_tags!(code logic)]
/// Allows to convert compile-time constants into type-level booleans.
#[doc = crate::_doc_location!("num/fin/logic")]
///
/// See also the [`const_bool`] macro, and the [`True`] and [`False`] types.
#[rustfmt::skip]
#[diagnostic::on_unimplemented(
    message = "Only expressions that evaluate to a constant 0 or 1 are valid for `ConstBool`.",
    label = "This expression does not evaluate to a constant 0 or 1 (as usize)."
)]
pub trait ConstBool {
    /// The resulting type-level boolean (`True` or `False`).
    type Value: Sized;
    /// The constant value of the type-level boolean.
    const VALUE: Self::Value;
}
sf! {
    impl ConstBool for False { type Value = False; const VALUE: Self::Value = False; }
    impl ConstBool for [(); 0] { type Value = False; const VALUE: Self::Value = False; }
    impl ConstBool for True { type Value = True; const VALUE: Self::Value = True; }
    impl ConstBool for [(); 1] { type Value = True; const VALUE: Self::Value = True; }
}

#[doc = crate::_tags!(code logic)]
/// Converts a *const* `bool` expression to a type-level boolean.
#[doc = crate::_doc_location!("num/fin/logic")]
///
/// Internally, it leverages the [`ConstBool`] trait and a trick related to array sizes:
/// - Arrays of size `[(); 0]` are mapped to [`False`].
/// - Arrays of size `[(); 1]` are mapped to [`True`].
///
/// # Examples
/// ```
/// # use devela_base_core::{const_bool, True};
/// const _: True = const_bool![4 == 4];
/// ```
/// ```compile_fail
/// # use devela_base_core::{const_bool, True};
/// const _: True = const_bool![3 == 4];
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! const_bool {
    ($bool:expr) => {{ <[(); { $bool as usize }] as $crate::ConstBool>::VALUE }};
}
#[doc(inline)]
pub use const_bool;

#[doc = crate::_tags!(code logic)]
/// A type-level logical *true*.
#[doc = crate::_doc_location!("num/fin/logic")]
///
/// The second state in binary and ternary logic.
///
/// See also the [`ConstBool`] trait, the [`const_bool`] macro, and the [`False`] type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct True;

#[doc = crate::_tags!(code logic)]
/// A type-level logical *false*.
#[doc = crate::_doc_location!("num/fin/logic")]
///
/// The first state in binary and ternary logic.
///
/// See also the [`ConstBool`] trait, the [`const_bool`] macro, and the [`True`] type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct False;

#[rustfmt::skip]
impl True {
    /// Applies the `not` operation, (returns `False`).
    pub const fn not(self) -> False { False }
    /// Applies the `not` operation, (returns `False`).
    pub const fn not_ref(&self) -> &'static False { &False }

    /// Applies the `and` operation to `other`, (returns `other`).
    pub const fn and<T>(self, other: T) -> T { other }
    /// Applies the `and` operation to `other`, (returns `other`).
    pub const fn and_ref<'a, T>(&self, other: &'a T) -> &'a T { other }

    /// Applies the `or` operation to `_other`, (returns `True`).
    pub fn or<T>(self, _other: T) -> True { True }
    /// Applies the `or` operation to `_other`, (returns `True`).
    pub const fn or_ref<T>(&self, _other: &T) -> &'static True { &True }

    /// Returns the value as `bool` (returns `true`).
    pub const fn value(self) -> bool { true }
    /// Returns the value as `bool` (returns `true`).
    pub const fn value_ref(&self) -> bool { true }
}

#[rustfmt::skip]
impl False {
    /// Applies the `not` operation, (returns `True`).
    pub const fn not(self) -> True { True }
    /// Applies the `not` operation, (returns `True`).
    pub const fn not_ref(&self) -> &'static True { &True }

    /// Applies the `and` operation to `_other`, (returns `False`).
    pub fn and<T>(self, _other: T) -> False { False }
    /// Applies the `and` operation to `_other`, (returns `False`).
    pub const fn and_ref<T>(&self, _other: &T) -> &'static False { &False }

    /// Applies the `or` operation to `other`, (returns `other`).
    pub fn or<T>(self, other: T) -> T { other }
    /// Applies the `or` operation to `other`, (returns `other`).
    pub const fn or_ref<'a, T>(&self, other: &'a T) -> &'a T { other }

    /// Returns the value as `bool` (returns `false`).
    pub const fn value(self) -> bool { false }
    /// Returns the value as `bool` (returns `false`).
    pub const fn value_ref(&self) -> bool { false }
}
