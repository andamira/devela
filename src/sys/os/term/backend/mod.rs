// devela::sys::os::term::backend
//
//! Terminal backend adapters.
//!
//! Connects terminal semantics to concrete host environments.
//

#[cfg(all(feature = "linux", feature = "event", not(miri)))]
mod linux;
// mod macos; // TermMacos
// mod std; // TermStd
// mod web; // TermWeb
// mod windows; // TermWindows

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        #[cfg(all(feature = "linux", feature = "event", not(miri)))]
        pub use super::linux::_all::*;
    }
    _crate_internals {
        #[cfg(all(feature = "linux", feature = "event", not(miri)))]
        pub use super::linux::_crate_internals::*;
    }
}
