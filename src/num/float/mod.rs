// devela::num::float
//
//! Floating point functionality.
//

#![cfg_attr(not(feature = "num"), allow(unused))]

mod r#trait;
mod wrapper;

pub use {r#trait::*, wrapper::*};
