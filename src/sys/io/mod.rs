// devela::sys::io
//
//! I/O functionality.
//!
#![doc = crate::doc_!(extends: io)]
//!
//! # Derived work
#![doc = include_str!("./define_no_std_io/MODIFICATIONS.md")]
//

// safety:
#![cfg_attr(feature = "safe_io", forbid(unsafe_code))]

mod impls;

#[cfg(not(feature = "std"))]
crate::items! {
    mod define_no_std_io;
    pub use define_no_std_io::*;
}
#[cfg(feature = "std")]
crate::items! {
    mod reexport_std;
    pub use reexport_std::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(not(feature = "std"))]
    pub use super::define_no_std_io::*;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::reexport_std::*;
}
