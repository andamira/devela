// devela::data::array
//
//! Primitive arrays.
#![doc = crate::code::doc_extends!(array)]
//!
//

mod ext; // ExtArray, ArrayFmt
mod init; // array_init!

#[allow(unused_imports)]
pub use {ext::*, init::*};
