// devela::data::hash
//
//! Generic hashing support.
#![doc = crate::doc_!(extends: hash)]
// #![doc = crate::doc_!(modules: crate::data; hash)]
#![doc = crate::doc_!(newline)]
//!
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
    pub use super::{fnv::*, fx::*, reexports::*};
}
