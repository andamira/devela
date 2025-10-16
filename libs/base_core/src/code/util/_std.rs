// devela_base_core::code::util::_std
//
//! Defines private helpers related to `std`
//! .
//
// TOC
// - __std!
// - _std_core!

crate::sf! {
    /// Feature-gates all the arguments with the `__std` feature.
    ///
    /// If the first argument is @ then it also feature-gates with the `__dbg` feature.
    macro_rules! __std {
        (@$($tt:tt)*) => { #[cfg(all(feature = "__std", feature = "__dbg"))] { $($tt)* }};
        ($($tt:tt)*) => { #[cfg(feature = "__std")] { $($tt)* }};
    }
    #[allow(unused_imports)] pub(crate) use __std;

    /// Should return the string literal "std" if `std` is enabled, or "core" otherwise.
    macro_rules! _std_core { () => { "core" }; }
    #[allow(unused_imports)] pub(crate) use _std_core;
}
