// devela::data::cmp
//
//! Comparing and ordering values, extends `std::`[`cmp`].
//!
//! [`cmp`]: std::cmp
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

#[cfg(_some_cmp)]
mod compare; // `Compare`
#[cfg(_some_cmp)]
pub use compare::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    #[cfg(_some_cmp)]
    pub use super::compare::*;
}
