// devela_base_core::yard::_policy
//
//! Defines shared helpers related to workspace policies.
//
// TOC
// - _devela_policy!

/// Emits an item with devela-only policy gating.
///
/// Outside the devela workspace, the item is always emitted as-is.
/// Inside devela, the item is conditionally enabled based on feature flags.
///
/// Intended for internal use by `define_*!` macros to avoid duplicating items
/// that differ only by policy-related `cfg` attributes.
#[doc(hidden)]
#[macro_export]
macro_rules! _devela_policy {
    (safe:$safe:literal, unsafe:$unsafe:literal, $item:item) => {
        #[$crate::compile(not(env(__DEVELA_MEMBER)))]
        $item

        #[$crate::compile(env(__DEVELA_MEMBER))]
        #[cfg(all(not(feature = $safe), feature = $unsafe))]
        // #[cfg_attr(nightly_doc, doc(cfg(feature = $unsafe)))]
        $item
    };
    (unsafe:$unsafe:literal, $item:item) => {
        #[$crate::compile(not(env(__DEVELA_MEMBER)))]
        $item

        #[$crate::compile(env(__DEVELA_MEMBER))]
        #[cfg(feature = $unsafe)]
        // #[cfg_attr(nightly_doc, doc(cfg(feature = $unsafe)))]
        $item
    };
}
#[doc(hidden)]
pub use _devela_policy;
