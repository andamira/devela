// devela::string::ascii
//
//! ASCII strings and characters, extends [`std::ascii`].
//

/* always compiled for internal use */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "string"))]
pub(crate) use always_fns::*;

/* only compiled with the `ascii` feature */

#[cfg(feature = "string")]
mod char;
#[cfg(feature = "string")]
mod fns;

/* re-exports */

#[cfg(feature = "string")]
mod reexport;

#[cfg(feature = "string")]
pub use all::*;
#[cfg(feature = "string")]
pub(crate) mod all {
    pub use super::{always_fns::*, char::AsciiChar, fns::*};

    pub use super::reexport::*;
}
