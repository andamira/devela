// devela::sys::path
//
//! Paths.
#![doc = crate::doc_!(extends: path)]
#![doc = crate::doc_!(modules: crate::sys; path)]
#![doc = crate::doc_!(newline)]
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
