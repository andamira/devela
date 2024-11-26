// devela::num::float
//
//! Floating point functionality.
//

#![cfg_attr(not(feature = "num"), allow(unused))]

mod constants; // ExtFloatConst
mod reexports;
pub use {constants::*, reexports::*};

#[cfg(_float_·)]
crate::items! {
    mod ext_float; // ExtFloat
    mod wrapper; // Float
    pub use {ext_float::*, wrapper::*};
}
