// devela::error
//
//! Error management, result handling and utility enhancements,
//! <br/><small>extends `std::{`[`error`], [`option`], [`panic`], [`result`]`}`.</small>
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

mod ext_result;
mod mismatch;
mod never;
mod option;
mod own;
mod panic;
mod reexports;
mod traits;
mod unwrap;
mod value_quant;
#[allow(unused_imports)]
pub use {
    ext_result::*, mismatch::*, never::*, option::all::*, own::*, panic::all::*, reexports::*,
    traits::*, unwrap::*, value_quant::*,
};

#[cfg(not(feature = "std"))]
mod define_no_std_error;
#[allow(unused_imports)]
#[cfg(not(feature = "std"))]
pub use define_no_std_error::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        ext_result::*, mismatch::*, never::*, option::all::*, own::*, panic::all::*, reexports::*,
        traits::*, unwrap::*, value_quant::*,
    };

    #[doc(inline)]
    #[cfg(not(feature = "std"))]
    pub use super::define_no_std_error::*;
}
