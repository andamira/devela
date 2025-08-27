// devela::code::util::_std_core
//
//! Defines the [`_std_core!`] macro.
//

/// Should return the string literal "std" if `std` is enabled, or "core" otherwise.
///
/// This macro must be defined on each crate where [`_doc!`] is invoked.
///
/// Used by: from [`devela_base::_doc!`],
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
