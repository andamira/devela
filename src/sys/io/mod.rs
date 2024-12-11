// devela::sys::io
//
//! I/O functionality.
//!
#![doc = crate::doc_!(extends: io)]
//!
//! # Derived work
#![doc = include_str!("./define_no_std_io/MODIFICATIONS.md")]
//
// safety
#![cfg_attr(feature = "safe_io", forbid(unsafe_code))]

#[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(any(feature = "std", all(not(feature = "std"), feature = "io"))))
)]
mod impls;

#[cfg(all(not(feature = "std"), feature = "io"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "io")))]
mod define_no_std_io;
#[cfg(feature = "std")]
mod reexport_std;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        #[cfg(all(not(feature = "std"), feature = "io"))]
        pub use super::define_no_std_io::*;
        #[cfg(feature = "std")]
        pub use super::reexport_std::*;
    }
    pub(super) mod all { #![allow(unused)]
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        #[cfg(feature = "std")]
        pub use super::reexport_std::*;
    }
}
