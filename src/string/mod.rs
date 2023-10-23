// devela::string
//
//! Strings, extends `std::`[`string`][std::string] +
//! [`ascii`][std::ascii] +
//! [`char`][std::char] +
//! [`str`][std::str].
//

/* always compiled for internal use */

#[cfg(not(feature = "string"))]
mod ascii;
#[cfg(not(feature = "string"))]
mod char;
#[allow(unused)]
#[cfg(not(feature = "string"))]
pub(crate) use {ascii::*, char::*};

/* only compiled with the `ops` feature */

// public modules
#[cfg(feature = "string")]
pub mod ascii;
#[cfg(feature = "string")]
pub mod char;
#[cfg(feature = "string")]
pub mod egc;

#[doc(no_inline)]
#[cfg(feature = "string")]
pub use {ascii::all::*, char::all::*, egc::all::*};

// private modules
#[cfg(feature = "string")]
mod array_string;
#[cfg(feature = "string")]
mod error;
#[cfg(feature = "string")]
mod ext;
#[cfg(feature = "string")]
mod helpers;
#[cfg(feature = "string")]
mod non_nul;

#[cfg(feature = "string")]
pub use {array_string::*, error::*, ext::*, non_nul::*};

/* re-exports */

#[cfg(feature = "string")]
mod reexports;

#[cfg(feature = "string")]
pub use reexports::*;

#[cfg(feature = "string")]
pub(crate) mod all {
    // public
    pub use super::{ascii::all::*, char::all::*, egc::all::*};
    // private
    pub use super::{array_string::*, error::*, ext::*, non_nul::*};
    // reexported
    pub use super::reexports::*;
}
