// devela::sys::env
//
//! Inspection and manipulation of the process’s environment.
#![doc = crate::code::doc_extends!(env)]
//!
//

#[cfg(feature = "std")]
mod env;
#[cfg(feature = "std")]
pub use env::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "std")]
    pub use super::env::*;
}
