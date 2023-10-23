// devela::result
//
//! Result, extends
//! `std::{`[`option`][std::option],
//! [`result`][std::result]`}`.
//

/* always compiled for internal use */

/* only compiled with the `result` feature */

#[cfg(feature = "result")]
mod chain;
#[cfg(feature = "result")]
mod ext;
#[cfg(feature = "result")]
mod never;
#[cfg(feature = "result")]
pub use {chain::*, ext::ResultExt, never::*};

#[cfg(feature = "result")]
pub mod option;
#[doc(no_inline)]
#[cfg(feature = "result")]
pub use option::all::*;

/* re-exports */

#[cfg(feature = "result")]
pub(crate) mod all {
    pub use super::option::all::*;
    pub use super::{chain::*, ext::ResultExt, never::*};
}
