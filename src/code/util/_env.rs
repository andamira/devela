// devela::code::util::_env
//
//! Defines private helpers related to the environment.
//
// TOC
// - __dbg!
// - _std_core!

crate::sf! {
    /// Feature-gates all the arguments with the `__dbg` feature.
    ///
    /// If the first argument is @ then it also feature-gates with the `__dbg` feature.
    macro_rules! __dbg {
        ($item:item) => { #[cfg(feature = "__dbg")] $item };
        ($($tt:tt)*) => { #[cfg(feature = "__dbg")] { $($tt)* } };
    }
    #[allow(unused_imports)] pub(crate) use __dbg;

    /// Should return the string literal "std" if `std` is enabled, or "core" otherwise.
    ///
    /// This macro must be defined on each crate where [`_doc!`] is invoked.
    ///
    /// Used in [`devela_base_core::_doc!`],
    #[cfg_attr(cargo_primary_package, doc(hidden))]
    #[cfg(feature = "std")]
    macro_rules! _std_core { () => { "std" }; }

    #[cfg_attr(cargo_primary_package, doc(hidden))]
    #[cfg(not(feature = "std"))]
    macro_rules! _std_core { () => { "core" }; }
    #[doc(hidden)]
    pub(crate) use _std_core;
}
