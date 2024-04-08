// devela::data::hash
//
//! Generic hashing support, extends `std::`[`hash`].
//!
//! [`hash`]: std::hash
//

mod fx;
mod reexports;
#[allow(unused_imports)]
pub use {fx::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{fx::*, reexports::*};
}
