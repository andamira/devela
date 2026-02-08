// devela_base_core::yard::_policy
//
//! Defines shared helpers related to workspace policies.
//
// TOC
// - _devela_policy!
// - __devela_unreachable_unchecked

#[doc = crate::_tags!(internal)]
/// Applies devela-internal policy to expressions, items, and namespaces.
#[doc = crate::_doc_location!("yard")]
///
/// Centralizes devela-specific rules (such as safety, documentation,
/// or debug behavior) while keeping external expansions unrestricted.
///
/// # Note
/// This macro may emit the unstable attribute `#[doc(cfg(...))]` under
/// `nightly_doc`. When such expansions occur inside doctests, the doctest
/// crate must explicitly enable `feature(doc_cfg)`, for example:
/// ```ignore
/// #![cfg_attr(nightly_doc, feature(doc_cfg))]
/// ```
///
/// # Used in
/// - [`define_bufline!`][crate::define_bufline]
/// - [`define_pcg!`][crate::define_pcg]
/// - [`unwrap!`][crate::unwrap]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[macro_export]
macro_rules! __devela_policy {
    /* item-level safety policy */

    // Defines an item whose availability differs between devela and external builds,
    // enforcing internal safety policies without affecting the public API.
    (safe:$safe:literal, unsafe:$unsafe:literal, $item:item) => {
        // external crates: emit item directly
        #[$crate::compile(not(env(__DEVELA_MEMBER)))]
        $item

        // devela only: delegate to internal arm
        #[$crate::compile(env(__DEVELA_MEMBER))]
        $crate::_devela_policy!{%devela safe:$safe, unsafe:$unsafe, $item}
    };
    (%devela safe:$safe:literal, unsafe:$unsafe:literal, $item:item) => {
        #[cfg(all(not(feature = $safe), feature = $unsafe))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = $unsafe)))] // NOTE
        $item
    };
    (unsafe:$unsafe:literal, $item:item) => {
        // external crates: emit item directly
        #[$crate::compile(not(env(__DEVELA_MEMBER)))]
        $item

        #[$crate::compile(env(__DEVELA_MEMBER))]
        $crate::_devela_policy!{%devela unsafe:$unsafe, $item}
    };
    (%devela unsafe:$unsafe:literal, $item:item) => {
        #[cfg(feature = $unsafe)]
        #[cfg_attr(nightly_doc, doc(cfg(feature = $unsafe)))] // NOTE
        $item
    };

    /* expression-level branching */

    // Selects a policy-controlled implementation of an expression-level primitive,
    // enforcing devela-specific safety or debug behavior.
    (unreachable) => { $crate::__devela_unreachable_unchecked() };

    /* namespace-level policy */

    // Defines a policy-controlled module whose contents differ between
    // devela and external builds, then re-exports it unchanged.
    ($vis:vis mod $mod_name:ident, devela { $($devela:item)* } extern { $($extern:item)* }) => {
        // devela
        #[allow(non_snake_case)]
        #[$crate::compile(env(__DEVELA_MEMBER))]
        mod $mod_name { $($devela)* }

        // extern
        #[allow(non_snake_case)]
        #[$crate::compile(not(env(__DEVELA_MEMBER)))]
        mod $mod_name { $($extern)* }

        $vis use $mod_name::*;
    };
}
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
pub use __devela_policy as _devela_policy;

/// for devela safe
#[crate::compile(env(__DEVELA_MEMBER))] #[cfg(not(unsafe··))]
#[rustfmt::skip] #[doc(hidden)] #[inline(always)]
pub const fn __devela_unreachable_unchecked() -> ! { crate::unreachable!() }
/// for devela unsafe
#[crate::compile(env(__DEVELA_MEMBER))] #[cfg(unsafe··)]
#[rustfmt::skip] #[doc(hidden)] #[inline(always)]
pub const fn __devela_unreachable_unchecked() -> ! {
    if cfg!(debug_assertions) { crate::unreachable!(); }
    else { unsafe { crate::unreachable_unchecked() } }
}
/// for external crates
#[crate::compile(not(env(__DEVELA_MEMBER)))]
#[rustfmt::skip] #[doc(hidden)] #[inline(always)]
pub const fn __devela_unreachable_unchecked() -> ! {
    if cfg!(debug_assertions) { crate::unreachable!(); }
    else { unsafe { crate::unreachable_unchecked() } }
}
