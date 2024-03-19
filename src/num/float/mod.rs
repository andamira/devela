// devela::num::float
//
//! Floating point functionality.
//

#![cfg_attr(not(feature = "num"), allow(unused))]

mod ext_trait;
mod wrapper;

pub use {ext_trait::*, wrapper::*};
