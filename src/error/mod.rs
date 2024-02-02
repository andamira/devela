// devela::error
//
//! Error related functionality, extends
//! `std::{`[`error`], [`option`], [`panic`], [`result`]`}`.
//!
//! [`error`]: std::error
//! [`option`]: std::option
//! [`panic`]: mod@std::panic
//! [`result`]: std::result
//

// warnings:
#![cfg_attr(not(feature = "error"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_error", forbid(unsafe_code))]

/* modules */

// always compiled, public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "error")))]
pub mod chain;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "error")))]
pub mod panic;

// feature-gated, non-public
#[cfg(feature = "error")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "error")))]
mod ext_result;
#[cfg(feature = "error")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "error")))]
mod never;
#[cfg(feature = "error")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "error")))]
mod option;
//
#[cfg(feature = "std")]
mod reexport_std;
#[cfg(feature = "no_std")]
#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(all(feature = "error", any(feature = "no_std", feature = "std"))))
)]
mod reimplement_no_std;

/* re-exports */

// always compiled, public
#[doc(no_inline)]
pub use {chain::*, panic::all::*};

// feature-gated, private
#[cfg(feature = "error")]
pub use {ext_result::*, never::*, option::all::*};
//
#[cfg(feature = "std")]
pub use reexport_std::*;
#[cfg(feature = "no_std")]
pub use reimplement_no_std::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{chain::*, panic::all::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "error")]
    pub use super::{ext_result::*, never::*, option::all::*};
    //
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::reexport_std::*;
    #[doc(inline)]
    #[cfg(feature = "no_std")]
    pub use super::reimplement_no_std::*;
}
