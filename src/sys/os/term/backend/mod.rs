// devela/src/sys/os/term/backend/mod.rs
//
//! Terminal backend adapters.
//!
//! Connects terminal semantics to concrete host environments.
//

#[cfg(all(feature = "linux", not(miri)))]
mod linux;
// mod macos; // TermMacos
// mod std; // TermStd
mod r#trait; // TermBackend
// mod web; // TermWeb
// mod windows; // TermWindows

crate::structural_mods! { // _mods, _crate_internals
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
