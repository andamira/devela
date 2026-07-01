// devela/src/yard/alias.rs
//
//! Defines maintenance aliases for item attributes.
//

crate::macro_apply_alias! {
    /* docs */

    /// Emits an explicit rustdoc cfg marker.
    ///
    /// Overrides `doc(auto_cfg)`. Not affected by `auto_cfg(hide/show)`.
    pub __doc_show($cfg_tree:meta) = #[cfg_attr(nightly_doc, doc(cfg($cfg_tree)))];

    /// Hides auto-generated cfg markers.
    ///
    /// Affects only `doc(auto_cfg)`, not explicit `doc(cfg(...))`.
    /// Pass the contents of `hide(...)` as one parenthesized group.
    //
    // - <https://rust-lang.github.io/rfcs/3631-rustdoc-cfgs-handling.html>
    // - <https://doc.rust-lang.org/rustdoc/unstable-features.html#docauto_cfghide>
    pub __doc_auto_hide($group:tt) = #[cfg_attr(nightly_doc, doc(auto_cfg(hide $group)))];

    /// Re-allows inherited-hidden auto-generated cfg markers.
    ///
    /// This does not emit a marker by itself; use `__doc_cfg` for that.
    /// Pass the contents of `show(...)` as one parenthesized group.
    pub __doc_auto_show($group:tt) =
        #[cfg_attr(nightly_doc, doc(auto_cfg(show $group)))];

    /// Hides one or more feature values from auto-generated cfg markers.
    pub __doc_auto_hide_features($values:tt) =
        #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values $values))))];

    /// Re-allows inherited-hidden feature values in auto-generated cfg markers.
    ///
    /// This does not emit a marker by itself.
    pub __doc_auto_show_features($values:tt) =
        #[cfg_attr(nightly_doc, doc(auto_cfg(show(feature, values $values))))];

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
        #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values($safe, $unsafe)))))];
    /// Compiles an item for the explicit unsafe path, and hides the safety features from docs.
    pub __cfg_item_unsafe_hide($safe:literal, $unsafe:literal) =
        #[cfg(all(not(feature = $safe), feature = $unsafe))]
        #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values($safe, $unsafe)))))];

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

    /* linux & std */

    // unsafe_syscall && !miri
    pub(crate) _unsafe_syscall_not_miri = #[cfg(all(feature = "unsafe_syscall", not(miri)))];

    // (_linux_abi && unsafe_syscall && !miri && supported_linux_arch)
    pub(crate) _linux_syscall = #[cfg(
        all(feature = "_linux_abi", feature = "unsafe_syscall", not(miri), any_target_arch_linux)
    )];

    // !(_linux_abi && unsafe_syscall && !miri && supported_linux_arch)
    pub(crate) _not_linux_syscall = #[cfg(not(
        all(feature = "_linux_abi", feature = "unsafe_syscall", not(miri), any_target_arch_linux)
    ))];

    // !std && (_linux_abi && unsafe_syscall && !miri && supported_linux_arch)
    pub(crate) _linux_syscall_not_std = #[cfg(all(
        not(feature = "std"),
        feature = "_linux_abi", feature = "unsafe_syscall", not(miri), any_target_arch_linux
    ))];

    // std || (_linux_abi && unsafe_syscall && !miri && supported_linux_arch)
    pub(crate) _std_or_linux_syscall = #[cfg(any(
        feature = "std",
        all(feature = "_linux_abi", feature = "unsafe_syscall", not(miri), any_target_arch_linux)
    ))];

    // std && !(_linux_abi && unsafe_syscall && !miri && supported_linux_arch)
    pub(crate) _std_not_linux_syscall = #[cfg(all(
        feature = "std",
        not(all(
            feature = "_linux_abi", feature = "unsafe_syscall", not(miri), any_target_arch_linux
        ))
    ))];

    // !(std || (_linux_abi && unsafe_syscall && !miri && supported_linux_arch))
    pub(crate) _not_std_or_linux_syscall = #[cfg(not(any(
        feature = "std",
        all(feature = "_linux_abi", feature = "unsafe_syscall", not(miri), any_target_arch_linux)
    )))];
}
