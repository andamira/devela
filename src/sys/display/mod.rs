// devela::sys::display
//
#![doc = crate::_DOC_SYS_DISPLAY!()]
///
/// Unlike hosted environments (e.g. browsers),
/// display backends expose process-owned windows and event loops.
//

// #[cfg(feature = "cocoa")]
// pub mod cocoa;
// #[cfg(feature = "gdi")]
// pub mod gdi;

#[cfg(feature = "x11")]
#[cfg(all(not(feature = "safe_sys"), feature = "unsafe_syscall"))]
pub mod x11;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        // #[cfg(feature = "cocoa")]
        // pub use super::cocoa::*;
        // #[cfg(feature = "gdi")]
        // pub use super::gdi::*;

        #[cfg(feature = "x11")]
        #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_syscall"))]
        pub use super::x11::_all::*;
    }
    _crate_internals {
        #[cfg(feature = "x11")]
        #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_syscall"))]
        pub(crate) use super::x11::_crate_internals::*;
    }
}
