// devela::code::util::_env
//
//! Defines private helpers related to the environment.
//!
//! WARNING: #[doc = …] consumers see macros early; avoid rustfmt or macro indirection.
//
// TOC
// - __dbg!
// - __std!
// - _std_core!

#![allow(unused)]

/// Feature-gates all the arguments with the `__dbg` feature.
///
/// If the first argument is @ then it also feature-gates with the `__dbg` feature.
macro_rules! __dbg {
    ($item:item) => { #[cfg(feature = "__dbg")] $item };
    ($($tt:tt)*) => { #[cfg(feature = "__dbg")] { $($tt)* } };
}
pub(crate) use __dbg;

/// Feature-gates all the arguments with the `__std` feature.
///
/// If the first argument is `#` then it also feature-gates with `__dbg`.
macro_rules! __std {
    (# $item:item) => { #[cfg(all(feature = "__std", feature = "__dbg"))] $item };
    (# $($tt:tt)*) => { #[cfg(all(feature = "__std", feature = "__dbg"))] { $($tt)* } };
    ($item:item) => { #[cfg(feature = "__std")] $item };
    ($($tt:tt)*) => { #[cfg(feature = "__std")] { $($tt)* } };
}
pub(crate) use __std;

/// Should return the string literal "std" if `std` is enabled, or "core" otherwise.
///
/// This macro must be defined on each crate where [`_doc!`] is invoked.
///
/// Used in [`devela_base_core::_doc!`],
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg(feature = "std")]
macro_rules! _std_core {
    () => {
        "std"
    };
}

#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg(not(feature = "std"))]
macro_rules! _std_core {
    () => {
        "core"
    };
}
#[doc(hidden)]
pub(crate) use _std_core;
