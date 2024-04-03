// devela::data::array
//
//! Primitive arrays, extends `std::`[`array`].
//!
//! [`array`]: std::array
//

mod ext; // ExtArray, ArrayFmt
mod init; // array_init!

#[allow(unused_imports)]
pub use {ext::*, init::*};
