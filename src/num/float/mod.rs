// devela::num::float
//
//! Floating point functionality.
//

#![cfg_attr(not(feature = "num"), allow(unused))]

mod constants; // ExtFloatConst
pub use constants::*;

#[cfg(_some_float)]
mod ext_float; // ExtFloat
#[cfg(_some_float)]
mod wrapper; // Float
#[cfg(_some_float)]
pub use {ext_float::*, wrapper::*};
