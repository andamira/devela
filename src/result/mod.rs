// devela::result
//
//! Result related functionality, extends
//! `std::{`[`error`], [`option`], [`panic`], [`result`]`}`.
//!
//! [`error`]: std::error
//! [`option`]: std::option
//! [`panic`]: mod@std::panic
//! [`result`]: std::result
//!
//! It re-exports the error and result types defined in other modules.
//

// warnings:
#![cfg_attr(not(feature = "result"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_error", forbid(unsafe_code))]

/* modules */

// always compiled, non-public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "result")))]
mod mismatch;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "result")))]
mod own;
mod reexport;
mod traits;
mod unwrap;
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
#[cfg(not(feature = "std"))]
mod define_no_std_error;

/* re-exports */

// always compiled, non-public
pub use {mismatch::*, own::*, reexport::*, traits::*, unwrap::*};
// always compiled, public
#[doc(no_inline)]
pub use panic::all::*;

// feature-gated, private
#[cfg(feature = "result")]
pub use {ext_result::*, never::*, option::all::*};
//
#[cfg(not(feature = "std"))]
pub use define_no_std_error::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{mismatch::*, own::*, panic::all::*, reexport::*, traits::*, unwrap::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "result")]
    pub use super::{ext_result::*, never::*, option::all::*};
    //
    #[doc(inline)]
    #[cfg(not(feature = "std"))]
    pub use super::define_no_std_error::*;
}
