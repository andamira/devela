// devela::text::ascii
//
//! ASCII strings and characters, extends `std::`[`ascii`].
//!
//! [`ascii`]: std::ascii
//

/* always compiled */

mod always_fns;
mod char;
mod fns;

#[allow(unused_imports)]
pub use {always_fns::*, char::AsciiChar, fns::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{always_fns::*, char::AsciiChar, fns::*};
}
