//
//! Workspace compilation and invariant policies.
//
// TOC
// - _devela_policy!
// - __devela_unreachable_unchecked

#[doc = crate::_tags!(internal)]
/// Applies devela-internal policy to expressions, items, and namespaces.
#[cfg_attr(root_yard, doc = crate::_doc_meta!{location("yard", macro _devela_policy)})]
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
/// - `buffer_linear!` (item_gate)
/// - `rand_pcg!` (mod extern)
/// - [`unwrap!`][crate::boot::expr::unwrap] (unreachable)
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! _devela_policy· {
    /* item-level safety policy */

    // Defines one or more items whose availability differs between devela and external builds,
    // enforcing internal safety policies without affecting the public API.
    (item_gate safe:$safe:literal, unsafe:$unsafe:literal, $($item:item)+) => {
        // external crates: emit item directly
        $(
            #[$crate::compile(not(env(__DEVELA_MEMBER)))]
            $item
        )+
        // devela only: delegate to internal arm
        #[$crate::compile(env(__DEVELA_MEMBER))]
        $crate::_devela_policy!{%item_gate devela safe:$safe, unsafe:$unsafe, $($item)+}
    };
    (%item_gate devela safe:$safe:literal, unsafe:$unsafe:literal, $($item:item)+) => {
        $(
            #[cfg(all(not(feature = $safe), feature = $unsafe))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $unsafe)))] // NOTE
            $item
        )+
    };
    (item_gate unsafe:$unsafe:literal, $($item:item)+) => {
        // external crates: emit item directly
        $(
            #[$crate::compile(not(env(__DEVELA_MEMBER)))]
            $item
        )+
        #[$crate::compile(env(__DEVELA_MEMBER))]
        $crate::_devela_policy!{%item_gate devela unsafe:$unsafe, $($item)+}
    };
    (%item_gate devela unsafe:$unsafe:literal, $($item:item)+) => {
        #[cfg(feature = $unsafe)]
        $(
            #[cfg_attr(nightly_doc, doc(cfg(feature = $unsafe)))] // NOTE
            $item
        )+
    };

    /* expression-level branching */

    // Selects the enforcement policy for a caller-established invariant.
    // Does not establish the invariant itself.
    (unreachable) => { $crate::__devela_unreachable_unchecked() };

    /* item-level attribute policy */

    // Emits one item directly for external crates,
    // but attaches devela-only attributes when compiled by a devela workspace member.
    (item_attr devela { $(#[$attr:meta])* } $item:item) => {
        #[$crate::compile_attr(env(__DEVELA_MEMBER) $(, $attr)*)]
        $item
    };

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
#[doc(inline)]
pub use _devela_policy· as _devela_policy;

// Invariant contract (shared by all three implementations):
// The caller must have established that this branch is unreachable.
// A violated invariant panics without unsafe_hint in workspace builds,
// or with debug assertions enabled; otherwise it is undefined behavior.
// The helper's policy is selected when its defining crate is compiled,
// unlike the item/namespace policy arms which run in the consuming crate.
// doc(hidden) is not a safety boundary. The contract is not compiler-enforced.
// TODO: enforce the unchecked contract without requiring unsafe in safe builds.

/// Panics when the invariant is violated (workspace, checked policy).
#[crate::compile(env(__DEVELA_MEMBER))]
#[cfg(not(feature = "unsafe_hint"))]
#[rustfmt::skip] #[doc(hidden)]
pub const fn __devela_unreachable_unchecked() -> ! { ::core::unreachable!() }
/// Panics with debug assertions; otherwise requires the branch to be unreachable.
#[crate::compile(env(__DEVELA_MEMBER))]
#[cfg(feature = "unsafe_hint")]
#[rustfmt::skip] #[doc(hidden)]
pub const fn __devela_unreachable_unchecked() -> ! {
    if cfg!(debug_assertions) { ::core::unreachable!(); }
    else { unsafe { ::core::hint::unreachable_unchecked() } }
}
/// Same unchecked contract when the defining crate uses the external policy.
#[crate::compile(not(env(__DEVELA_MEMBER)))]
#[rustfmt::skip] #[doc(hidden)]
pub const fn __devela_unreachable_unchecked() -> ! {
    if cfg!(debug_assertions) { ::core::unreachable!(); }
    else { unsafe { ::core::hint::unreachable_unchecked() } }
}
