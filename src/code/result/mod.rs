// devela::code::result
//
//! Result handling and functional enhancements, extends
//! `std::{`[`error`], [`option`], [`panic`], [`result`]`}`.
//!
//! [`error`]: std::error
//! [`option`]: std::option
//! [`panic`]: mod@std::panic
//! [`result`]: std::result
//!
//! Streamlines error management, result chaining, and introduces utility types
//! and macros.
//!
//! It re-exports the error and result types defined in other modules.
//

// safety:
#![cfg_attr(feature = "safe_result", forbid(unsafe_code))]

/* always compiled, non-public modules */

mod ext_result;
mod mismatch;
mod never;
mod option;
mod own;
mod panic;
mod reexport;
mod traits;
mod unwrap;
mod value_quant;

pub use {
    ext_result::*, mismatch::*, never::*, option::all::*, own::*, panic::all::*, reexport::*,
    traits::*, unwrap::*, value_quant::*,
};

/* feature-gated, non-public modules */

#[cfg(not(feature = "std"))]
mod define_no_std_error;

#[cfg(not(feature = "std"))]
pub use define_no_std_error::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        ext_result::*, mismatch::*, never::*, option::all::*, own::*, panic::all::*, reexport::*,
        traits::*, unwrap::*, value_quant::*,
    };

    // feature-gated
    #[doc(inline)]
    #[cfg(not(feature = "std"))]
    pub use super::define_no_std_error::*;
}
