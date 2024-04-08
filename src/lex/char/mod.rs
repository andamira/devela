// devela::lex::char
//
//! Unicode scalars, extends `std::`[`char`].
//!
//! [`char`]: std::char
//

mod always_fns;
#[allow(unused_imports)]
pub use always_fns::*;

// without re-exports
#[cfg(feature = "lex")]
mod core_impls;
#[cfg(feature = "lex")]
mod impls;
#[cfg(all(feature = "lex", test))]
mod tests;
// with re-exports
#[cfg(feature = "lex")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "lex")))]
mod definitions;
#[cfg(feature = "lex")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "lex")))]
mod fns;
#[cfg(feature = "lex")]
pub use {definitions::*, fns::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    #[cfg(feature = "lex")]
    pub use super::{definitions::*, fns::*};
}
