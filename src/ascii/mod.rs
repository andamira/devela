// devela::ascii
//
//! ASCII strings and characters, extends [`core::ascii`].
//!
//! It also reexports some [`const-str`](https://docs.rs/const-str) macros
//! directly related to ASCII, prefixed with `ascii_`, and a new description.
//

/* always compiled for internal use */

mod always_fns;
#[cfg(not(feature = "ascii"))]
pub(crate) use always_fns::*;

/* only compiled with the `ascii` feature */

#[cfg(feature = "ascii")]
mod char;
#[cfg(feature = "ascii")]
mod fns;

#[cfg(feature = "ascii")]
pub use {always_fns::*, char::AsciiChar, fns::*};

#[cfg(feature = "ascii")]
mod reexport_const_str;
#[cfg(feature = "ascii")]
pub use reexport_const_str::*;
