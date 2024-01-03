// devela::text::ascii
//
//! ASCII strings and characters, extends `std::`[`ascii`].
//!
//! [`ascii`]: std::ascii
//

/* contains always compiled items */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "text"))]
pub use always_fns::*;

/* feature-gated */

// private sub-modules
#[cfg(feature = "text")]
mod char;
#[cfg(feature = "text")]
mod fns;
#[cfg(feature = "text")]
mod reexport;

// re-export private sub-modules
#[cfg(feature = "text")]
#[allow(unused_imports)]
pub use {always_fns::*, char::AsciiChar, fns::*, reexport::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    #[cfg(feature = "text")]
    #[allow(unused_imports)]
    pub use super::{always_fns::*, char::AsciiChar, fns::*, reexport::*};
}
