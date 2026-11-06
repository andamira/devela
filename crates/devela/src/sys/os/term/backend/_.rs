//
//! Terminal backend adapters.
//!
//! Connects terminal semantics to concrete host environments.
//

crate::mods_in! {
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "linux", feature = "term"))))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(
        hide(linux_syscall_target),
        hide(feature, values("_linux_abi", "unsafe_syscall")),
    )))]
    #[crate::macro_apply(crate::_linux_syscall)]
    mod_ linux;
    // mod macos; // TermMacos
    // mod std; // TermStd
    mod r#trait; // TermBackend
    // mod web; // TermWeb
    // mod windows; // TermWindows
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::r#trait::*;

        #[crate::macro_apply(crate::_linux_syscall)]
        pub use super::linux::_all::*;
    }
    _crate_internals {
        #[crate::macro_apply(crate::_linux_syscall)]
        pub use super::linux::_crate_internals::*;
    }
}
