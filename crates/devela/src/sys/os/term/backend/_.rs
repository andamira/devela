//
//! Terminal backend adapters.
//!
//! Connects terminal semantics to concrete host environments.
//

crate::mods_in! {
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
