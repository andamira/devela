// devela::result
//
//! Result, extends
//! `std::{`[`error`], [`option`], [`result`]`}`.
//!
//! [`error`]: std::error
//! [`option`]: core::option
//! [`result`]: core::result
//

/* contains always compiled items */

mod chain;
#[allow(unused)]
#[cfg(not(feature = "result"))]
pub use chain::*;

/* feature-gated */

// private sub-modules
#[cfg(feature = "result")]
mod ext;
#[cfg(feature = "result")]
mod never;
#[cfg(feature = "result")]
mod option;

// re-export private sub-modules
#[cfg(feature = "result")]
pub use {
    chain::{Also, Apply},
    ext::ResultExt,
    never::*,
    option::all::*,
};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::chain::{Also, Apply};

    #[doc(inline)]
    #[cfg(feature = "result")]
    pub use super::{ext::ResultExt, never::*, option::all::*};
}
