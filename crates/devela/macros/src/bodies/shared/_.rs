//
//! Shared functionality for procedural macros.
//

mod common;
mod diag;
mod enumint;
pub(crate) use {common::*, diag::*, enumint::*};
