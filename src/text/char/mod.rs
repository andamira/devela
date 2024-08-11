// devela::text::char
//
//! Unicode scalars, extends `std::`[`char`].
//!
//! [`char`]: std::char
//

mod always_fns;
#[allow(unused_imports)]
pub use always_fns::*;

// without re-exports
mod core_impls;
mod impls;
mod tests;

// with re-exports
mod definitions;
mod fns;
pub use {definitions::*, fns::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    pub use super::{definitions::*, fns::*};
}
