// devela::num::float
//
//! Floating point functionality.
//

#![cfg_attr(not(feature = "num"), allow(unused))]

mod constants; // ExtFloatConst
pub use constants::*;

#[cfg(_float_some)]
mod ext_float; // ExtFloat
#[cfg(_float_some)]
mod wrapper; // Float
#[cfg(_float_some)]
pub use {ext_float::*, wrapper::*};
