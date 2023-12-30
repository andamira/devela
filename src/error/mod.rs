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

/* contains always compiled items */

pub mod panic;

#[allow(unused)]
#[cfg(not(feature = "error"))]
pub use panic::*;

/* feature-gated */

#[cfg(feature = "error")]
mod ext;
#[cfg(feature = "error")]
mod never;
#[cfg(feature = "error")]
mod option;

// re-export private sub-modules
#[cfg(feature = "error")]
pub use {ext::*, never::*, option::all::*};

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "error")]
pub use panic::all::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::panic::all::*;

    #[doc(inline)]
    #[cfg(feature = "error")]
    pub use super::{ext::*, never::*, option::all::*};
}
