// devela::sys::io
//
//! I/O functionality.
//!
#![doc = crate::_doc!(extends: io)]
//!
#![doc = crate::_doc!(vendor: "no_std_io")]
//
// safety
#![cfg_attr(feature = "safe_io", forbid(unsafe_code))]

#[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
#[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))))]
mod namespace;

#[cfg(not(feature = "std"))]
mod define_no_std;
#[cfg(feature = "std")]
mod reexports_std;

crate::structural_mods! { // _mods
    _mods {
        #[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
        pub use super::namespace::*;

        #[cfg(not(feature = "std"))]
        pub use super::define_no_std::*;
        #[cfg(feature = "std")]
        pub use super::reexports_std::*;
    }
}
