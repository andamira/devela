// devela/src/sys/os/term/backend/_.rs
//
//! Terminal backend adapters.
//!
//! Connects terminal semantics to concrete host environments.
//

crate::mods_in! {
    #[cfg(all(feature = "linux", not(miri)))]
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

        #[cfg(all(feature = "linux", not(miri)))]
        pub use super::linux::_all::*;
    }
    _crate_internals {
        #[cfg(all(feature = "linux", not(miri)))]
        pub use super::linux::_crate_internals::*;
    }
}
