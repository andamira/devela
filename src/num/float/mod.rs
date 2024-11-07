// devela::num::float
//
//! Floating point functionality.
//

#![cfg_attr(not(feature = "num"), allow(unused))]

mod constants; // ExtFloatConst
pub use constants::*;

#[cfg(_float_·)]
crate::items! {
    mod ext_float; // ExtFloat
    mod wrapper; // Float
    pub use {ext_float::*, wrapper::*};
}
