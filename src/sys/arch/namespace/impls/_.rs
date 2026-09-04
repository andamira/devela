// devela/src/sys/arch/namespace/impls/_.rs

crate::mods_in! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_hint")))]
    #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_hint"))]
    mod_ instr; // architecture-specific instructions

    #[cfg(feature = "dep_safe_arch")]
    mod dep_safe_arch;
}
