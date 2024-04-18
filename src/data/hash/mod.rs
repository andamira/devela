// devela::data::hash
//
//! Generic hashing support, extends `std::`[`hash`].
//!
//! [`hash`]: std::hash
//

mod fnv;
mod fx;
mod pengy;
mod reexports;
#[allow(unused_imports)]
pub use {fnv::*, fx::*, pengy::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{fx::*, reexports::*};
}
