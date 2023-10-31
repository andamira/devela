// devela::result
//
//! Result, extends
//! `std::{`[`option`][std::option],
//! [`result`][std::result]`}`.
//

/* contains always compiled items */

mod chain;
#[allow(unused)]
#[cfg(not(feature = "result"))]
pub(crate) use chain::*;

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

#[cfg(feature = "result")]
pub(crate) mod all {
    pub use super::{
        chain::{Also, Apply},
        ext::ResultExt,
        never::*,
        option::all::*,
    };
}
