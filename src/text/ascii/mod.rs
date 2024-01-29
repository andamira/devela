// devela::text::ascii
//
//! ASCII strings and characters, extends `std::`[`ascii`].
//!
//! [`ascii`]: std::ascii
//

/* modules */

// always compiled, non-public
mod always_fns;

// feature-gated, non-public
#[cfg(feature = "text")]
mod char;
#[cfg(feature = "text")]
mod fns;
#[cfg(feature = "text")]
mod reexport;

/* re-exports */

// always compiled, non-public
pub use always_fns::*;

// feature-gated, non-public
#[cfg(feature = "text")]
#[allow(unused_imports)] // reexport
pub use {char::AsciiChar, fns::*, reexport::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::always_fns::*;

    // feature-gated
    #[doc(inline)]
    #[allow(unused_imports)] // always_fns, reexport
    #[cfg(feature = "text")]
    pub use super::{always_fns::*, char::AsciiChar, fns::*, reexport::*};
}
