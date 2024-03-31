// devela::lex::char
//
//! Unicode scalars, extends `std::`[`char`].
//!
//! [`char`]: std::char
//

/* always compiled */

mod always_fns;

#[allow(unused)]
pub use always_fns::*;

/* feature-gated */

// without re-exports
#[cfg(feature = "lex")]
mod core_impls;
#[cfg(feature = "lex")]
mod impls;
#[cfg(all(feature = "lex", test))]
mod tests;

#[cfg(feature = "lex")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "lex")))]
mod definitions;
#[cfg(feature = "lex")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "lex")))]
mod fns;

#[cfg(feature = "lex")]
pub use {definitions::*, fns::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    #[cfg(feature = "lex")]
    pub use super::{definitions::*, fns::*};
}
