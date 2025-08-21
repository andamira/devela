// devela::sys::io
//
//! I/O functionality.
//!
#![doc = crate::doc_!(extends: io)]
//!
#![doc = crate::doc_!(vendor: "no_std_io")]
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

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        #[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
        pub use super::namespace::*;

        #[cfg(not(feature = "std"))]
        pub use super::define_no_std::*;
        #[cfg(feature = "std")]
        pub use super::reexports_std::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(feature = "std")]
        pub use super::reexports_std::*;
    }
}
