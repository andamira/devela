// devela::num::float
//
//! Floating point functionality.
//

#![cfg_attr(not(feature = "num"), allow(unused))]

mod alias;
mod r#trait;
mod wrapper;

pub use {alias::*, r#trait::*, wrapper::*};
