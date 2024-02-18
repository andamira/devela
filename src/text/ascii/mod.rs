// devela::text::ascii
//
//! ASCII strings and characters, extends `std::`[`ascii`].
//!
//! [`ascii`]: std::ascii
//

/* always compiled, non-public modules */

mod always_fns;

#[allow(unused_imports)]
pub use always_fns::*;

/* feature-gated, non-public modules */

#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod char;
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod fns;

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
