// devela::text::ascii
//
//! ASCII strings and characters, extends [`std::ascii`].
//

/* always compiled for internal use */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "text"))]
pub(crate) use always_fns::*;

/* only compiled with the `ascii` feature */

#[cfg(feature = "text")]
mod char;
#[cfg(feature = "text")]
mod fns;

/* re-exports */

#[cfg(feature = "text")]
mod reexport;

#[cfg(feature = "text")]
pub use all::*;
#[cfg(feature = "text")]
pub(crate) mod all {
    pub use super::{always_fns::*, char::AsciiChar, fns::*};

    pub use super::reexport::*;
}
