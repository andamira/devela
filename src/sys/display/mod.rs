// devela::sys::display
//
//!
//

// #[cfg(feature = "cocoa")]
// pub mod cocoa;
// #[cfg(feature = "gdi")]
// pub mod gdi;

#[cfg(feature = "x11")]
#[cfg(all(not(feature = "safe_sys"), feature = "unsafe_syscall"))]
pub mod x11;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        // #[cfg(feature = "cocoa")]
        // pub use super::cocoa::*;
        // #[cfg(feature = "gdi")]
        // pub use super::gdi::*;

        #[cfg(feature = "x11")]
        #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_syscall"))]
        pub use super::x11::*;
    }
}
