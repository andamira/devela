// devela::text
//
//! Strings, extends
//! `std::{`[`ascii`][std::ascii],
//! [`char`][std::char],
//! [`fmt`][std::fmt],
//! [`str`][std::str],
//! [`string`][std::string]`}`.
//

/* contains always compiled items for internal use */

#[cfg(not(feature = "text"))]
mod ascii;
#[cfg(not(feature = "text"))]
mod char;
#[allow(unused)]
#[cfg(not(feature = "text"))]
pub(crate) use {ascii::*, char::*};

/* only compiled with the `ops` feature */

// public modules
#[cfg(feature = "text")]
pub mod ascii;
#[cfg(feature = "text")]
pub mod char;
#[cfg(feature = "text")]
pub mod egc;
#[cfg(feature = "text")]
pub mod fmt;
#[doc(no_inline)]
#[cfg(feature = "text")]
pub use {ascii::all::*, char::all::*, egc::all::*, fmt::all::*};

// private modules
#[cfg(feature = "text")]
mod array_string;
#[cfg(feature = "text")]
mod error;
#[cfg(feature = "text")]
mod ext;
#[cfg(feature = "text")]
mod helpers;
#[cfg(feature = "text")]
mod non_nul;
#[cfg(feature = "text")]
pub use {array_string::*, error::*, ext::*, non_nul::*};

// reexported items
#[cfg(feature = "text")]
mod reexports;
#[cfg(feature = "text")]
pub use reexports::*;

#[cfg(feature = "text")]
pub(crate) mod all {
    // public
    pub use super::{ascii::all::*, char::all::*, egc::all::*, fmt::all::*};
    // private
    pub use super::{array_string::*, error::*, ext::*, non_nul::*, reexports::*};
}
