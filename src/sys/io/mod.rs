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

mod impls;

#[cfg(not(feature = "std"))]
mod define_no_std_io;
#[cfg(feature = "std")]
mod reexport_std;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;
    mod doc_inline {
        #[cfg(not(feature = "std"))]
        pub use super::define_no_std_io::*;
        #[cfg(feature = "std")]
        pub use super::reexport_std::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[cfg(feature = "std")]
        pub use super::reexport_std::*;
    }
}
