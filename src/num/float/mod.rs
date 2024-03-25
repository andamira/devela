// devela::num::float
//
//! Floating point functionality.
//

#![cfg_attr(not(feature = "num"), allow(unused))]

mod ext_float_const;
pub use ext_float_const::*;

#[cfg(feature = "num_float")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_float")))]
mod ext_float;
#[cfg(feature = "num_float")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_float")))]
mod wrapper;

#[cfg(feature = "num_float")]
pub use {ext_float::*, wrapper::*};
