// devela::yard::alias
//
//! Defines maintenance aliases for item attributes.
//

crate::macro_apply_alias! {
    /* docs */

    /// Shows a cfg tree on nightly documentation.
    pub __doc_show($cfg_tree:meta) = #[cfg_attr(nightly_doc, doc(cfg($cfg_tree)))];

    /// Hides a cfg item from nightly `auto_cfg` documentation.
    //
    // <https://rust-lang.github.io/rfcs/3631-rustdoc-cfgs-handling.html>
    //
    // NOTE: `doc(auto_cfg(hide(…)))` may fail when emitted directly beside the
    // `#[cfg(…)]` from the same alias expansion. Route it through `macro_apply`
    // so rustdoc observes the hide directive as a separate attribute expansion.
    pub __doc_hide($cfg_item:meta) = #[cfg_attr(nightly_doc, doc(auto_cfg(hide($cfg_item))))];

    /* safety */

    /// Compiles an item for the safe/default implementation path.
    pub __cfg_item_safe($safe:literal, $unsafe:literal) =
        #[cfg(any(feature = $safe, not(feature = $unsafe)))];
    /// Compiles an item for the explicit unsafe implementation path.
    pub __cfg_item_unsafe($safe:literal, $unsafe:literal) =
        #[cfg(all(not(feature = $safe), feature = $unsafe))];

    /// Compiles an item for the safe/default path, and hides the safety features from docs.
    pub __cfg_item_safe_hide($safe:literal, $unsafe:literal) =
        #[cfg(any(feature = $safe, not(feature = $unsafe)))]
        #[$crate::macro_apply($crate::__doc_hide(feature = $unsafe))]
        #[$crate::macro_apply($crate::__doc_hide(feature = $safe))];
    /// Compiles an item for the explicit unsafe path, and hides the safety features from docs.
    pub __cfg_item_unsafe_hide($safe:literal, $unsafe:literal) =
        #[cfg(all(not(feature = $safe), feature = $unsafe))]
        #[$crate::macro_apply($crate::__doc_hide(feature = $unsafe))]
        #[$crate::macro_apply($crate::__doc_hide(feature = $safe))];

    /// Compiles an item for the explicit unsafe path, and shows the unsafe features in docs.
    pub __cfg_item_unsafe_show($safe:literal, $unsafe:literal) =
        #[cfg(all(not(feature = $safe), feature = $unsafe))]
        #[$crate::macro_apply($crate::__doc_show(feature = $unsafe))];

    /* ffi */

    pub(crate) _js_safe_ffi = #[cfg(any(not(feature = "unsafe_ffi"), windows))];
    pub(crate) _js_unsafe_ffi = #[cfg(all(feature = "unsafe_ffi", not(windows)))];
    pub(crate) _js_unsafe_ffi_doc =
        #[cfg(all(feature = "unsafe_ffi", not(windows)))]
        #[$crate::macro_apply($crate::__doc_show(feature = "unsafe_ffi"))];

    /* sys */

    pub(crate) _unsafe_syscall_not_miri = #[cfg(all(feature = "unsafe_syscall", not(miri)))];
}
