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
mod ext_result;
mod mismatch;
mod never;
mod option;
mod own;
mod panic;
mod reexport;
mod traits;
mod unwrap;

// feature-gated, non-public
#[cfg(not(feature = "std"))]
mod define_no_std_error;

/* re-exports */

// always compiled, non-public
pub use {
    ext_result::*, mismatch::*, never::*, option::all::*, own::*, panic::all::*, reexport::*,
    traits::*, unwrap::*,
};

// feature-gated, private
#[cfg(not(feature = "std"))]
pub use define_no_std_error::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        ext_result::*, mismatch::*, never::*, option::all::*, own::*, panic::all::*, reexport::*,
        traits::*, unwrap::*,
    };

    // feature-gated
    #[doc(inline)]
    #[cfg(not(feature = "std"))]
    pub use super::define_no_std_error::*;
}
