// devela::sys::env
//
//! Inspection and manipulation of the process’s environment.
#![doc = crate::code::doc_extends!(env)]
//!
//

mod reexports;
pub use reexports::*;

#[cfg(feature = "std")]
mod env;
#[cfg(feature = "std")]
pub use env::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::reexports::*;

    #[allow(unused_imports)]
    #[cfg(feature = "std")]
    pub use super::env::*;
}
