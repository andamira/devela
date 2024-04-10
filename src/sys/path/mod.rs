// devela::sys::path
//
//! Paths, extends
//! `std::`[`path`][std::path].
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
