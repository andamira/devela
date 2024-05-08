// devela::data::cmp
//
//! Comparing and ordering values, extends `std::`[`cmp`].
//!
//! [`cmp`]: std::cmp
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

#[cfg(_cmp_some)]
mod compare; // `Compare`
#[cfg(_cmp_some)]
pub use compare::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    #[cfg(_cmp_some)]
    pub use super::compare::*;
}
