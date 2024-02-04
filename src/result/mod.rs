// devela::result
//
//! Result related functionality, extends
//! `std::{`[`error`], [`option`], [`panic`], [`result`]`}`.
//!
//! [`error`]: std::error
//! [`option`]: std::option
//! [`panic`]: mod@std::panic
//! [`result`]: std::result
//

// warnings:
#![cfg_attr(not(feature = "result"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_error", forbid(unsafe_code))]

/* modules */

// always compiled, non-public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "result")))]
mod chain;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "result")))]
mod mismatch;
mod reexport;
// always compiled, public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "result")))]
pub mod panic;

// feature-gated, non-public
#[cfg(feature = "result")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "result")))]
mod ext_result;
#[cfg(feature = "result")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "result")))]
mod never;
#[cfg(feature = "result")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "result")))]
mod option;
//
#[cfg(feature = "no_std")]
#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(all(feature = "result", any(feature = "no_std", feature = "std"))))
)]
mod reimplement_no_std;

/* re-exports */

// always compiled, non-public
pub use {chain::*, mismatch::*, reexport::*};
// always compiled, public
#[doc(no_inline)]
pub use panic::all::*;

// feature-gated, private
#[cfg(feature = "result")]
pub use {ext_result::*, never::*, option::all::*};
//
#[cfg(feature = "no_std")]
pub use reimplement_no_std::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{chain::*, mismatch::*, panic::all::*, reexport::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "result")]
    pub use super::{ext_result::*, never::*, option::all::*};
    //
    #[doc(inline)]
    #[cfg(feature = "no_std")]
    pub use super::reimplement_no_std::*;
}
