// devela::error
//
//! Error related functionality, extends
//! `std::{`[`error`], [`option`], [`panic`], [`result`]`}`.
//!
//! [`error`]: std::error
//! [`option`]: core::option
//! [`panic`]: mod@std::panic
//! [`result`]: core::result
//

#![cfg_attr(not(feature = "error"), allow(unused_imports))]

/* modules */

// always compiled, public
#[cfg_attr(feature = "nightly", doc(cfg(feature = "error")))]
pub mod panic;

// feature-gated, non-public
#[cfg(feature = "error")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "error")))]
mod ext;
#[cfg(feature = "error")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "error")))]
mod never;
#[cfg(feature = "error")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "error")))]
mod option;

/* re-exports */

// always compiled, public
#[doc(no_inline)]
pub use panic::all::*;

// feature-gated, private
#[cfg(feature = "error")]
pub use {ext::*, never::*, option::all::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::panic::all::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "error")]
    pub use super::{ext::*, never::*, option::all::*};
}
