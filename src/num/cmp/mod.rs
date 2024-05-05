// devela::data::cmp
//
//! Comparing and ordering values, extends `std::`[`cmp`].
//!
//! [`cmp`]: std::cmp
//

mod compare;
mod reexports;
#[allow(unused_imports)]
pub use {compare::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{compare::*, reexports::*};
}
