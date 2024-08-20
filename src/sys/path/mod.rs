// devela::sys::path
//
//! Paths.
#![doc = crate::code::doc_extends!(path)]
//!
//

#[cfg(feature = "sys")]
mod project;
#[allow(unused_imports)]
#[cfg(feature = "sys")]
pub use project::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "sys")]
    pub use super::project::*;
}
