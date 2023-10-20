// devela::ascii
//
//! ASCII strings and characters, extends [`std::ascii`].
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
