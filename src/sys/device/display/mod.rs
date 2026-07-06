// devela/src/sys/device/display/mod.rs
//
#![doc = crate::_DOC_SYS_DEVICE_DISPLAY!()] // public
#![doc = crate::_doc!(modules: crate::sys::device; display: x11)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! Unlike hosted environments (e.g. browsers),
//! display backends expose process-owned windows and event loops.
//

// #[cfg(feature = "cocoa")]
// pub mod cocoa;
// #[cfg(feature = "gdi")]
// pub mod gdi;

#[cfg(all(feature = "x11", not(feature = "safe_sys")))]
pub mod x11;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        // #[cfg(feature = "cocoa")]
        // pub use super::cocoa::*;
        // #[cfg(feature = "gdi")]
        // pub use super::gdi::*;

        #[cfg(all(feature = "x11", not(feature = "safe_sys")))]
        pub use super::x11::_all::*;
    }
    _crate_internals {
        #[cfg(all(feature = "x11", not(feature = "safe_sys")))]
        pub(crate) use super::x11::_crate_internals::*;
    }
}
