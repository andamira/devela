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

/* re-exports */

// always compiled, non-public
#[allow(unused_imports)]
pub use always_fns::*;

// feature-gated, non-public
#[cfg(feature = "text")]
#[allow(unused_imports)]
pub use {char::AsciiChar, fns::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::always_fns::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "text")]
    #[allow(unused_imports)]
    pub use super::{always_fns::*, char::AsciiChar, fns::*};
}
