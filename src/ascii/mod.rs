// devela::ascii
//
//! ASCII strings and characters, extends [`core::ascii`].
//!
//! It also reexports some [`const-str`](https://docs.rs/const-str) macros
//! directly related to ASCII, prefixed with `ascii_`, and a new description.
//

/* always compiled for internal use */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "ascii"))]
pub(crate) use always_fns::*;

/* only compiled with the `ascii` feature */

#[cfg(feature = "ascii")]
mod char;
#[cfg(feature = "ascii")]
mod fns;

/* re-exports */

#[cfg(feature = "ascii")]
mod reexport;

#[cfg(feature = "ascii")]
pub use all::*;
#[cfg(feature = "ascii")]
pub(crate) mod all {
    pub use super::{always_fns::*, char::AsciiChar, fns::*};

    pub use super::reexport::*;
}
