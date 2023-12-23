// devela::result
//
//! Result, extends
//! `std::{`[`error`], [`option`], [`panic`], [`result`]`}`.
//!
//! [`error`]: std::error
//! [`option`]: core::option
//! [`panic`]: mod@std::panic
//! [`result`]: core::result
//

/* contains always compiled items */

mod chain;
pub mod panic;

#[allow(unused)]
#[cfg(not(feature = "result"))]
pub use {chain::*, panic::*};

/* feature-gated */

#[cfg(feature = "result")]
mod ext;
#[cfg(feature = "result")]
mod never;
#[cfg(feature = "result")]
mod option;

// re-export private sub-modules
#[cfg(feature = "result")]
pub use {chain::*, ext::*, never::*, option::all::*};

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "result")]
pub use panic::all::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{chain::*, panic::all::*};

    #[doc(inline)]
    #[cfg(feature = "result")]
    pub use super::{ext::*, never::*, option::all::*};
}
