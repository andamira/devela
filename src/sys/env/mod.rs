// devela::sys::env
//
//! Inspection and manipulation of the process’s environment, extends
//! `std::`[`env`].
//!
//! [`env`]: std::env
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
